//! # `reconfix-core`
//!
//! This crate implements the core of reconfix schema handling and bidirectional transformation.
#![deny(missing_docs)]
#![recursion_limit = "1024"]

mod common;
mod json;
mod adaptor;
mod schema;
mod transform;
mod map;
mod io;

#[cfg(test)]
mod test;

#[macro_use]
extern crate error_chain;
extern crate futures;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate maplit;
extern crate itertools;
#[macro_use]
extern crate nom;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate valico;
extern crate uuid;

mod error {
    error_chain! {
        // links {
        //     Schema(::schema::Error, ::schema::ErrorKind)
        //     /// Indicates an error during schema processing
        //     ;
        // }

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
pub use common::{FileNode, Partition, FileFormat};
pub use io::Plugin;

use common::{deserialize, serialize};
use json::Entry;
// use transform::{transform_to_dry, transform_to_wet, Entry};
// use schema::{Location, Schema};
use schema::types::Schema;
use schema::parse;
use transform::Generator;
use transform::types::{Transform, Target, DiskFile, Location, Format};
use map::Mapper;
use map::default::DefaultMapper;
use io::host::HostFile;

use std::ops::{Deref, DerefMut};
use std::collections::BTreeMap;

use futures::{Future, Stream};
use futures::stream;

use serde_json::Value;

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

    /// Load the schema from the specified `Read` implementation.
    pub fn load_schema<R>(&mut self, r: R) -> Result<()>
    where
        R: std::io::Read,
    {
        debug!("reading JSON...");

        let schema = schema::parse::from_reader(r)
            .chain_err(|| "unable to read Reconfix schema")?;

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
    where
        for<'a> &'a mut <P as Deref>::Target: Plugin,
    {
        let schema = self.get_schema()?;
        read_values(schema, plugin)
    }

    /// Convert dry to wet and write to data sources
    pub fn write_values_plugin<P: Plugin + DerefMut>(&self, dry: Value, plugin: P) -> Result<()>
    where
        for<'a> &'a mut <P as Deref>::Target: Plugin,
    {
        let schema = self.get_schema()?;
        write_values(schema, dry, plugin)
    }
}

fn wet_to_dry<F>(schema: Schema, mut wet: F) -> Result<Value> 
    where F: FnMut(&DiskFile, &FileFormat) -> Result<Value>
{
    let generator = transform::generator::DefaultGenerator;
    let transforms = generator.generate(&schema)
        .chain_err(|| "unable to generate transforms")?;

    let targets = get_targets(&transforms);

    let mut phy_files = BTreeMap::new();
    let mut entries = BTreeMap::new();

    for target in targets.iter() {
        match *target {
            Target::NetworkManager => {
                bail!("networkmanager backend not supported")
            },
            Target::File(ref file) => {
                let format = match file.format {
                    Format::Ini => common::FileFormat::Ini,
                    Format::Json => common::FileFormat::Json,
                };
                match file.location {
                    Location::Disk(ref d) => {
                        let content = wet(d, &format)?;
                        phy_files.insert(d.clone(), content.clone());
                        entries.insert(target.clone(), content.clone());
                    },
                    Location::Nested(ref n) => {
                        let content = phy_files.get(&n.file)
                            .ok_or_else(|| "unable to find nested file")?;
                        let wet = n.path.search(&content)
                            .ok_or_else(|| "unable to find search nested path")?;
                        entries.insert(target.clone(), wet.clone());
                    }
                }
            }
        }
    }

    let mapper = map::default::DefaultMapper;
    let dry = mapper.reverse_map(&entries, &transforms)
        .chain_err(|| "unable to perform reverse map")?;

    Ok(dry)
}

fn read_values<P: Plugin + DerefMut>(schema: Schema, mut plugin: P) -> Result<Value>
where
    for<'a> &'a mut <P as Deref>::Target: Plugin,
{
    wet_to_dry(schema, |file, format| {
        let node = get_node(file)?;
        let content: Vec<u8> = (&mut *plugin)
            .read(node)
            .map_err(|e| ErrorKind::Plugin(e))?;
        
        let wet = deserialize(content.as_slice(), &format)?;
        Ok(wet)
    })
}

fn dry_to_wet(schema: Schema, dry: Value) -> Result<Vec<(DiskFile, FileFormat, Value)>> {
    let generator = transform::generator::DefaultGenerator;
    let transforms = generator.generate(&schema)
        .chain_err(|| "unable to generate transforms")?;

    let mapper = map::default::DefaultMapper;
    let entries = mapper.forward_map(&dry, &transforms)
        .chain_err(|| "unable to perform forward map")?;

    let mut disk_entries = BTreeMap::new();

    for (target, value) in entries {
        match target {
            Target::NetworkManager => {
                bail!("networkmanager backend not supported")
            },
            Target::File(file) => {
                let format = match file.format {
                    Format::Ini => common::FileFormat::Ini,
                    Format::Json => common::FileFormat::Json,
                };
                match file.location {
                    Location::Disk(d) => {
                        disk_entries.insert(d, (format, value));
                    },
                    Location::Nested(ref n) => {
                        let mut buffer = Vec::new();
                        serialize(value, &format, false, &mut buffer)?;
                        let content = String::from_utf8(buffer)
                            .chain_err(|| "invalid utf8 output from serializer")?;
                        let &mut (_, ref mut disk_file) = disk_entries.get_mut(&n.file)
                            .ok_or_else(|| "nested file destination not found")?;
                        match n.path.entry(disk_file)? {
                            Entry::Vacant(e) => { e.insert(Value::String(content)); },
                            _ => bail!("cannot overwrite value"),
                        }
                    },
                }
            }
        }
    }

    let list = disk_entries
        .into_iter()
        .map(|(file, (format, value))| (file, format, value))
        .collect::<Vec<_>>();

    Ok(list)
}

fn write_values<P: Plugin + DerefMut>(schema: Schema, dry: Value, mut plugin: P) -> Result<()>
where
    for<'a> &'a mut <P as Deref>::Target: Plugin,
{
    let disk_entries = dry_to_wet(schema, dry)?;

    for (disk_file, format, value) in disk_entries {
        let mut buffer = Vec::new();
        serialize(value, &format, true, &mut buffer)?;
        let node = get_node(&disk_file)?;
        (&mut *plugin)
            .write(node, buffer)
            .map_err(|e| ErrorKind::Plugin(e))?;
    }

    Ok(())
}

fn get_targets(transforms: &[Transform]) -> Vec<Target> {
    let mut targets = transforms
        .iter()
        .map(|t| t.target.clone())
        .collect::<Vec<_>>();
    
    targets.sort_unstable();
    targets.dedup();

    targets
}

fn get_node(file: &DiskFile) -> Result<FileNode> {
    let parts = file.path
        .trim_left_matches('/')
        .split("/")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let partition = match file.partition {
        transform::types::Partition::Number(p) => p,
        _ => bail!("unsupported parition identifier"),
    };
    Ok(FileNode {
        path: parts,
        partition: Partition::new(partition),
    })
}

mod tests {
    use super::*;
    use serde_json::Value;

    fn end_to_end_test(data: &str) {
        let json: Value = ::serde_json::from_str(data).expect("unable to parse json file");
        let schema_json = json.get("schema").expect("unable to read schema");
        let schema_ast = ::schema::parse::from_value(schema_json.clone()).expect("unable to parse schema AST");
        let dry_expected = json.get("dry").expect("unable to read dry");
        let wet_expected = json.get("wet")
            .and_then(|x| x.as_object())
            .expect("unable to read wet");
        
        let wet_actual = dry_to_wet(schema_ast.clone(), dry_expected.clone()).expect("unable to convert dry to wet");
        let dry_actual = wet_to_dry(schema_ast.clone(), |file, _| {
            let content = wet_expected
                .get(&file.path)
                .expect("unable to find wet json");
            Ok(content.clone())
        }).expect("unable to convert wet to dry");

        assert_eq!(dry_expected.clone(), dry_actual);

        let mut wet_actual_map = wet_actual
            .into_iter()
            .map(|(disk, _, content)| (disk.path, content))
            .collect::<BTreeMap<_, _>>();

        for (path, content_expected) in wet_expected {
            let content_actual = wet_actual_map
                .remove(path)
                .expect("unable to find wet json");
            
            assert_eq!(content_expected.clone(), content_actual);
        }
    }

    macro_rules! end_to_end_gen {
        ($($name:ident),*) => ( $(
            #[test]
            fn $name() {
                let file_contents = include_str!(concat!("../tests/e2e/json/", stringify!($name), ".json"));
                end_to_end_test(file_contents);
            }
        )* )
    }

    end_to_end_gen!(
        identity_map,
        template_map,
        const_map
    );
}