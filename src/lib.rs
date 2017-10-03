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
pub use io::Plugin;

use common::{serialize, deserialize};
use transform::{Entry, transform_to_dry, transform_to_wet};
use schema::{Schema, Location};
use io::host::HostFile;

use serde_json::{from_reader, Value};

/// The entry point for the Reconfix library
pub struct Reconfix {
    plugin: Box<Plugin>,
}

impl Reconfix {
    /// Initialize reconfix
    pub fn new() -> Reconfix {
        Reconfix {
            plugin: Box::new(HostFile {}),
        }
    }

    fn get_schema(&self, file: &FileNode) -> Result<Schema> {
        let schema_file = self.plugin.open(file).map_err(|e| {
            Error::with_boxed_chain(e, ErrorKind::Plugin)
        })?;

        let schema_json: Value = from_reader(schema_file).chain_err(
            || "unable to read schema file",
        )?;

        Schema::from_json(&schema_json).chain_err(|| "unable to parse schema json")
    }

    /// Read data in data sources and convert to dry
    pub fn read_values(&self, schema: &FileNode) -> Result<Value> {
        let schema = self.get_schema(schema)?;

        let entries = schema
            .files
            .iter()
            .filter_map(|(name, file)| {
                let node = match file.location {
                    Location::Dependent {
                        ..
                    } => return None,
                    Location::Independent(ref node) => node,
                };

                let entry = self.plugin
                    .open(node)
                    .map_err(|e| Error::with_boxed_chain(e, ErrorKind::Plugin))
                    .and_then(|f| deserialize(f, &file.format))
                    .map(|parsed| {
                        Entry {
                            name: name.to_string(),
                            content: parsed,
                        }
                    });

                Some(entry)
            })
            .collect::<Result<Vec<_>>>()?;

        transform_to_dry(entries, &schema)
    }

    /// Convert dry to wet and write to data sources
    pub fn write_values(&self, schema: &FileNode, dry: Value) -> Result<()> {
        let schema = self.get_schema(schema)?;
        let entries = transform_to_wet(dry, &schema)?;

        for entry in entries {
            let file = schema.files.get(&entry.name).ok_or_else(
                || "missing file entry",
            )?;

            if let Location::Independent(ref node) = file.location {
                let content = self.plugin.open(node).map_err(|e| {
                    Error::with_boxed_chain(e, ErrorKind::Plugin)
                })?;

                serialize(entry.content, &file.format, true, content)?;
            }
        }

        Ok(())
    }
}
