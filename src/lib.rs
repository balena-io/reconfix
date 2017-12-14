//! # `reconfix-core`
//!
//! This crate implements the core of reconfix schema handling and bidirectional transformation.
#![deny(missing_docs)]
#![recursion_limit = "1024"]

mod common;
mod adaptor;
mod schema;
mod template;
mod transform;
mod io;

#[cfg(test)]
mod test;

#[macro_use] extern crate log;
#[macro_use] extern crate error_chain;
extern crate futures;
#[macro_use] extern crate nom;
extern crate serde;
#[macro_use] extern crate serde_json;
extern crate regex;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate maplit;

mod error {
    error_chain! {
        links {
            Schema(::schema::Error, ::schema::ErrorKind)
            /// Indicates an error during schema processing
            ;
        }

        errors {
            /// Indicates a parsing error
            Parse { }
            /// Indicates a plugin error
            Plugin(inner: Box<::std::error::Error + Send>) { }
            /// Indicates an IO error
            Io(inner: Box<::std::io::Error>) { }

        }
    }
}

pub use error::*;
pub use common::FileNode;
pub use io::{Plugin};

use common::{serialize, deserialize};
use transform::{Entry, transform_to_dry, transform_to_wet};
use schema::{Schema, Location};
use io::host::HostFile;

use std::ops::{Deref, DerefMut};

use futures::{Future, Stream};
use futures::stream;

use serde_json::{from_reader, Value};

/// The entry point for the Reconfix library
#[derive(Clone)]
pub struct Reconfix {
    default: HostFile,
    schema: Option<Schema>,
}

impl Reconfix {
    /// Initialize reconfix
    pub fn new() -> Reconfix {
        Reconfix {
            default: HostFile::new(),
            schema: None,
        }
    }

    /// Initialize reconfix from plugin
    // pub fn from_plugin(p: Box<Plugin>) -> Self
    // {
    //     Reconfix { 
    //         plugin: p,
    //         schema: None,
    //     }
    // }

    /// Load the schema from the specified `Read` implementation.
    pub fn load_schema<R>(&mut self, r: R) -> Result<()> 
        where R: std::io::Read
    {
        debug!("reading JSON...");

        let schema_json: Value = from_reader(r)
            .map_err(|e| {
                error!("failed to read JSON: {}", e);
                e
            })
            .chain_err(|| "unable to read schema file")?;

        debug!("parsing schema structure...");

        let schema = Schema::from_json(&schema_json)
            .map_err(|e| {
                error!("failed to parse schema: {}", e);
                e
            })
            .chain_err(|| "unable to parse schema json")?;
        
        self.schema = Some(schema);

        Ok(())
    }

    fn get_schema(&self) -> Result<Schema> {
        if let Some(ref s) = self.schema {
            Ok(s.clone())
        } else {
            bail!("schema required");
        }
    }

    /// Read data using default `Plugin` implementation
    pub fn read_values(&mut self) -> Result<Value> {
        let schema = self.get_schema()?;
        read_values(schema, &mut self.default)
    }

    /// Write data using default `Plugin` implementation
    pub fn write_values(&mut self, dry: Value) -> Result<()> {
        let schema = self.get_schema()?;
        write_values(schema, dry, &mut self.default)
    }

    /// Read data in data sources and convert to dry
    pub fn read_values_plugin<P: Plugin + DerefMut>(&self, plugin: P) -> Result<Value> 
        where for<'a> &'a mut <P as Deref>::Target: Plugin
    {
        let schema = self.get_schema()?;
        read_values(schema, plugin)
    }

    /// Convert dry to wet and write to data sources
    pub fn write_values_plugin<P: Plugin + DerefMut>(&self, dry: Value, plugin: P) -> Result<()> 
        where for<'a> &'a mut <P as Deref>::Target: Plugin
    {
        let schema = self.get_schema()?;
        write_values(schema, dry, plugin)
    }
}

fn read_values<P: Plugin + DerefMut>(schema: Schema, mut plugin: P) -> Result<Value> 
    where for<'a> &'a mut <P as Deref>::Target: Plugin
{
    let data = schema.files.iter().map(|(name, file)| {
        (name.clone(), file.location.clone(), file.format.clone())
    });

    let entries = stream::iter_ok(data)
        .filter_map(|(name, location, format)| {
            match location {
                Location::Dependent {
                    ..
                } => None,
                Location::Independent(node) => Some((name, node, format)),
            }
        })
        .and_then(|(name, node, format)| {
            (&mut *plugin).read(node)
                .map(|content| (name, content, format))
                .map_err(|e| ErrorKind::Plugin(e).into())
        })
        .and_then(|(name, content, format)| {
            let wet = deserialize(content.as_slice(), &format)?;

            Ok(Entry { name: name.to_string(), content: wet })
        })
        .collect()
        .and_then(|entries| {
            transform_to_dry(entries, &schema)
        });

    entries.wait()
}

// fn read_single<P: Plugin>(format: &FileFormat, node: &FileNode, plugin: &mut P) -> Result<Value> {
//     let content = plugin.open(node)
//                 .map_err(|e| Error::with_boxed_chain(e, ErrorKind::Plugin))?;

//     deserialize(content, format)
// }

fn write_values<P: Plugin + DerefMut>(schema: Schema, dry: Value, mut plugin: P) -> Result<()> 
    where for<'a> &'a mut <P as Deref>::Target: Plugin
{
    let entries = transform_to_wet(dry, &schema)?;
    //let plugin = &mut plugin;

    let future = stream::iter_ok(entries)
        .and_then(|entry| {
            schema.files.get(&entry.name).ok_or_else(
                || "missing file entry".into(),
            )
            .map(|file| (entry, file))
        })
        .filter_map(|(entry, file)| {
            match file.location {
                Location::Independent(ref node) => Some((entry, node, &file.format)),
                _ => None,
            }
        })
        .and_then(|(entry, node, format)| {
            let mut buf = Vec::new();
            serialize(entry.content, format, &mut buf)?;
            Ok((node, buf))
        })
        .and_then(|(node, buf)| {
            (&mut *plugin).write(node.clone(), buf)
                .map_err(|e| ErrorKind::Plugin(e).into())
        })
        .collect()
        .map(|_| ());

    future.wait()
}

// fn write_single<P: Plugin>(wet: Value, format: &FileFormat, node: &FileNode, plugin: &mut P) -> Result<()> {
//     plugin.open(node).map_err(|e| {
//         Error::with_boxed_chain(e, ErrorKind::Plugin)
//     })
//     .and_then(move |content| {
//         serialize(wet, format, content)
//     })
// }
