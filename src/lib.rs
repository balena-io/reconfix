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

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;
#[macro_use]
extern crate nom;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_json;

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
            Plugin { }

        }
    }
}

pub use error::*;
pub use common::FileNode;
pub use io::{Plugin, Content};

use common::{serialize, deserialize, FileFormat};
use transform::{Entry, transform_to_dry, transform_to_wet};
use schema::{Schema, Location};
use io::host::HostFile;

use std::ops::{Deref, DerefMut};

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
        let schema_json: Value = from_reader(r).chain_err(
            || "unable to read schema file",
        )?;

        let schema = Schema::from_json(&schema_json)
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
        read_values(&schema, &mut self.default)
    }

    /// Write data using default `Plugin` implementation
    pub fn write_values(&mut self, dry: Value) -> Result<()> {
        let schema = self.get_schema()?;
        write_values(&schema, dry, &mut self.default)
    }

    /// Read data in data sources and convert to dry
    pub fn read_values_plugin<P: Plugin + DerefMut>(&self, mut plugin: P) -> Result<Value> 
        where for<'a> &'a mut <P as Deref>::Target: Plugin
    {
        let schema = self.get_schema()?;
        read_values(&schema, plugin)
    }

    /// Convert dry to wet and write to data sources
    pub fn write_values_plugin<P: Plugin + DerefMut>(&self, dry: Value, mut plugin: P) -> Result<()> 
        where for<'a> &'a mut <P as Deref>::Target: Plugin
    {
        let schema = self.get_schema()?;
        write_values(&schema, dry, plugin)
    }
}

fn read_values<P: Plugin + DerefMut>(schema: &Schema, mut plugin: P) -> Result<Value> 
    where for<'a> &'a mut <P as Deref>::Target: Plugin
{
    let mut entries = Vec::new();
    for (name, file) in schema.files.iter() {
        let node = match file.location {
            Location::Dependent {
                ..
            } => continue,
            Location::Independent(ref node) => node,
        };

        // let wet = read_single(&file.format, node, &mut plugin)?;
        let content = (&mut *plugin).open(node)
            .map_err(|e| Error::with_boxed_chain(e, ErrorKind::Plugin))?;
        
        let wet = deserialize(content, &file.format)?;
        
        let entry = Entry {
            name: name.to_string(),
            content: wet,
        };

        entries.push(entry)
    }

    transform_to_dry(entries, &schema)
}

// fn read_single<P: Plugin>(format: &FileFormat, node: &FileNode, plugin: &mut P) -> Result<Value> {
//     let content = plugin.open(node)
//                 .map_err(|e| Error::with_boxed_chain(e, ErrorKind::Plugin))?;

//     deserialize(content, format)
// }

fn write_values<P: Plugin + DerefMut>(schema: &Schema, dry: Value, mut plugin: P) -> Result<()> 
    where for<'a> &'a mut <P as Deref>::Target: Plugin
{
    let entries = transform_to_wet(dry, &schema)?;
    //let plugin = &mut plugin;

    for entry in entries {
        let file = schema.files.get(&entry.name).ok_or_else(
            || "missing file entry",
        )?;

        if let Location::Independent(ref node) = file.location {
            //write_single(entry.content, &file.format, node, &mut plugin)?;
            (&mut *plugin).open(node).map_err(|e| {
                Error::with_boxed_chain(e, ErrorKind::Plugin)
            })
            .and_then(move |content| {
                serialize(entry.content, &file.format, true, content)
            });
        }
    }

    Ok(())
}

// fn write_single<P: Plugin>(wet: Value, format: &FileFormat, node: &FileNode, plugin: &mut P) -> Result<()> {
//     plugin.open(node).map_err(|e| {
//         Error::with_boxed_chain(e, ErrorKind::Plugin)
//     })
//     .and_then(move |content| {
//         serialize(wet, format, content)
//     })
// }
