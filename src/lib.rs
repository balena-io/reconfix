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
extern crate nom;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate regex;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;

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
            Plugin(cause: Box<::std::error::Error + Sync + Send + 'static>) { }

        }
    }
}

pub use error::*;
pub use common::FileNode;

use common::{FileFormat, deserialize};
use transform::{Entry, transform_to_dry};
use schema::{Schema, Location};
use io::{HostFile, Plugin};

use serde_json::{from_reader, Value};

use std::path::Path;

/// The entry point for the Reconfix library
pub struct Reconfix {
    plugin: Box<Plugin>,
}

impl Reconfix {
    /// Initialize reconfix
    pub fn new() -> Reconfix {
        Reconfix {
            plugin: Box::new(HostFile { })
        }
    }

    /// Read data in data sources and convert to dry
    pub fn read_values<P>(&self, schema: &FileNode) -> Result<Value>
    {
        let schema_file = self.plugin.open(schema)
            .map_err(|e| Error::from(ErrorKind::Plugin(e)))?;

        let schema_json: Value = from_reader(schema_file)
            .chain_err(|| "unable to read schema file")?;

        let schema = Schema::from_json(&schema_json)
            .chain_err(|| "unable to parse schema json")?;

        let entries = schema.files.iter().filter_map(|(name, file)| {
            let node = match file.location {
                Location::Dependent { .. } => return None,
                Location::Independent(ref node) => node,
            };

            let entry = self.plugin.open(node)
                .map_err(|e| ErrorKind::Plugin(e).into())
                .and_then(|f| deserialize(f, &file.format))
                .map(|parsed| Entry { name: name.to_string(), content: parsed });

            Some(entry)
        })
        .collect::<Result<Vec<_>>>()?;

        transform_to_dry(entries, &schema)
    }
}
