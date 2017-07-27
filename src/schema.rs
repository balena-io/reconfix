//! Schema

#![allow(missing_docs)]
use std::collections::BTreeMap;
use std::cmp::{Ord, Ordering};

use serde_json::Value;
use serde_json::map::Map;

use regex::Regex;

pub mod error {
    error_chain!{
        errors {
            MissingKey(name: String)
            InvalidFileName(name: String)
            InvalidSchema(message: String)
            UnknownValue(message: String)
        }
    }
}

use self::error::*;
use self::error::ErrorKind::InvalidSchema;

/// Schema
pub struct Schema {
    pub files: BTreeMap<String, File>,
}

fn filename_is_valid(s: &str) -> bool {
    lazy_static !{
        static ref RE: Regex = Regex::new("([[:alnum:]]|_)+")
            .expect("failed compiling regex!".into());
    }

    RE.is_match(s)
}

impl Schema {
    pub fn from_json(v: &Value) -> Result<Schema> {
        let mut files = BTreeMap::new();
        match v.as_object() {
            Some(obj) => {
                for (k, v) in obj {
                    if !filename_is_valid(k) {
                        bail!(InvalidSchema("filename invalid".into()))
                    }
                    files.insert(k.to_owned(), File::from_json(v)?);
                }
            },
            None => bail!(InvalidSchema("schema is not an object".into())),
        }
        Ok(Schema {
            files: files,
        })
    }
}

/// Supported output file formats
#[derive(Eq, PartialEq, Clone)]
pub enum FileFormat {
    Ini,
    Json,
}

impl FileFormat {
    pub fn from_str(s: &str) -> Result<FileFormat> {
        match s {
            "ini" => Ok(FileFormat::Ini),
            "json" => Ok(FileFormat::Json),
            _ => bail!(ErrorKind::UnknownValue("unknown file format".into())),
        }
    }
}
/// File
pub struct File {
    /// The file format to generate
    pub format: FileFormat,
    /// Whether to operate in fileset mode
    pub fileset: bool,
    /// The location of the file on the filesystem
    pub location: Location,
    /// The properties defining how this file is transformed
    pub properties: Vec<Property>,
}

fn get<'a>(v: &'a Value, k: &str) -> Result<&'a Value> {
    match v.get(k) {
        Some(v) => Ok(v),
        None => bail!(InvalidSchema(format!("missing key {}", k))),
    }
}

fn get_array<'a>(v: &'a Value, k: &str) -> Result<&'a Vec<Value>> {
    match get(v, k)?.as_array() {
        Some(v) => Ok(v),
        None => bail!(InvalidSchema(format!("expected an array for {}", k))),
    }
}

fn get_i64(v: &Value, k: &str) -> Result<i64> {
    match get(v, k)?.as_i64() {
        Some(v) => Ok(v),
        None => bail!(InvalidSchema(format!("expected an int for {}", k))),
    }
}

fn get_u64(v: &Value, k: &str) -> Result<u64> {
    match get(v, k)?.as_u64() {
        Some(v) => Ok(v),
        None => bail!(InvalidSchema(format!("expected non-negative int for {}", k))),
    }
}

fn expect_object<'a>(v: &'a Value) -> Result<&'a Map<String, Value>> {
    match v.as_object() {
        Some(o) => Ok(o),
        None => bail!(InvalidSchema("expected object, found different kind of value".into())),
    }
}

fn expect_string<'a>(v: &'a Value) -> Result<&'a str> {
    match v.as_str() {
        Some(s) => Ok(s),
        None => bail!(InvalidSchema("expected string, found different kind of value".into())),
    }
}

fn get_object<'a>(v: &'a Value, k: &str) -> Result<&'a Map<String, Value>> {
    expect_object(get(v, k)?)
}

fn get_string<'a>(v: &'a Value, k: &str) -> Result<&'a str> {
    match get(v, k)?.as_str() {
        Some(s) => Ok(s),
        None => bail!(InvalidSchema(format!("expected a string for {}", k))),
    }
}
impl File {
    pub fn from_json(v: &Value) -> Result<File> {
        expect_object(v)?;
        let format = FileFormat::from_str(get_string(v, "type")?)?;
        let fileset = v.get("fileset").and_then(Value::as_bool).unwrap_or(false);
        let location = Location::from_json(get(v, "location")?)?;
        let json_properties = get_array(v, "properties")?;
        let mut properties = Vec::with_capacity(json_properties.len());

        for prop in json_properties {
            properties.push(Property::from_json(prop)?);
        }

        Ok(File {
            format: format,
            fileset: fileset,
            location: location,
            properties: properties,
        })
    }
}

/// A partition where a filesystem can be found
#[derive(Eq, PartialEq, Clone)]
pub enum Partition {
    /// A primary partition
    Primary(u8),
    /// A logical partitoin
    Logical {
        /// The primary partition containing the logical partition table
        on_primary: u8,
        /// The partition number in the logical partition table
        logical_number: u64,
    },
}

impl Partition {
    pub fn from_json(v: &Value) -> Result<Partition> {
        let o = expect_object(v)?;
        let primary = get_u64(v, "primary")?;
        if primary > 4 {
            bail!(InvalidSchema("primary partition number must be less than 4".into()))
        }
        let primary = primary as u8;
        if o.contains_key("logical") {
            let logical = get_u64(v, "logical")?;
            Ok(Partition::Logical {
                on_primary: primary,
                logical_number: logical,
            })
        } else {
            Ok(Partition::Primary(primary))
        }
    }
}

/// Location
#[derive(Eq, PartialEq, Clone)]
pub enum Location {
    Independent {
        /// The path on the filesystem where the contents while be written
        path: Vec<String>,
        /// The partition containing the filesystem `path` is relative to
        partition: Partition,
    },
    Dependent {
        /// The file name where the file will be inlined
        parent: String,
        /// A JSON Pointer indicating where the file contents will be inlined
        location: String,
    },
}

impl Location {
    pub fn from_json(v: &Value) -> Result<Location> {
        let o = expect_object(v)?;
        match o.get("parent") {
            Some(p) => {
                let json_path = o.get("path").ok_or("path missing")?;
                Ok(Location::Dependent {
                    parent: expect_string(p)?.to_owned(),
                    location: expect_string(json_path)?.to_owned(),
                })
            },
            None => {
                let json_path = get_array(v, "path")?;
                let mut path = Vec::with_capacity(json_path.len());
                for p in json_path {
                    path.push(expect_string(p)?.to_owned());
                }
                let partition = Partition::from_json(get(v, "partition")?)?;
                Ok(Location::Independent {
                    path: path,
                    partition: partition,
                })
            },
        }
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (&Location::Independent {
                 ..
             },
             &Location::Dependent {
                 ..
             }) => Ordering::Less,
            (&Location::Dependent {
                 ..
             },
             &Location::Independent {
                 ..
             }) => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Property
pub struct Property {
    pub definition: BTreeMap<String, PropertyDefinition>,
    pub when: Option<Value>,
}

impl Property {
    pub fn from_json(v: &Value) -> Result<Property> {
        let o = expect_object(v)?;
        let when = o.get("when").map(|p| p.to_owned());
        let json_definition = get_object(v, "definition")?;
        let mut definition = BTreeMap::new();
        for (k, v) in json_definition {
            definition.insert(k.to_owned(), PropertyDefinition::from_json(v)?);
        }
        Ok(Property {
            definition: definition,
            when: when,
        })
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum PropertyType {
    String,
    Number,
    Boolean,
}

impl PropertyType {
    pub fn from_json(v: &Value) -> Result<PropertyType> {
        let text = expect_string(v)?;
        let prop_type = match text {
            "string" => PropertyType::String,
            "number" => PropertyType::Number,
            "boolean" => PropertyType::Boolean,
            _ => {
                bail!(
                    InvalidSchema("property types must be either string, number, or boolean".into())
                )
            },
        };

        Ok(prop_type)
    }
}

#[derive(PartialEq, Debug)]
pub enum Mapping {
    Direct(String),
    Template { value: Value, template: Value },
}

impl Mapping {
    pub fn from_json(v: &Value) -> Result<Mapping> {
        let mapping = match *v {
            // Value::Array(ref elems) => {
            //     let map = elems.iter()
            //         .map(|elem| {
            //             elem.as_str().map(String::from)
            //                 .ok_or(InvalidSchema(
            //                     "direct mapping elements must be strings".into()).into()
            //                  )
            //         })
            //         .collect::<Vec<_>>()
            //         .into_iter()
            //         .collect::<Result<Vec<_>>>()?;
            //     Mapping::Direct(map)
            // },
            Value::String(ref s) => Mapping::Direct(s.to_owned()),
            Value::Object(ref obj) => {
                let value = obj.get("value").ok_or(InvalidSchema(
                    "template object must contain a value property"
                        .into(),
                ))?;
                let template = obj.get("template").ok_or(InvalidSchema(
                    "template must contain a template property"
                        .into(),
                ))?;
                Mapping::Template {
                    value: value.to_owned(),
                    template: template.to_owned(),
                }
            },
            _ => bail!(InvalidSchema("mapping must be a string or object".into())),
        };

        Ok(mapping)
    }
}

/// Property definition
pub struct PropertyDefinition {
    pub types: Vec<PropertyType>,
    pub properties: Vec<Property>,
    pub mapping: Vec<Mapping>,
}

impl PropertyDefinition {
    pub fn from_json(v: &Value) -> error::Result<PropertyDefinition> {
        let json_types = get_array(v, "type")?;
        let mut types = Vec::with_capacity(json_types.len());
        for t in json_types {
            types.push(PropertyType::from_json(t)?);
        }

        let properties = match get_array(v, "properties") {
            Ok(props) => {
                let mut properties = Vec::with_capacity(props.len());
                for prop in props {
                    properties.push(Property::from_json(prop)?);
                }
                properties
            },
            Err(_) => vec![],
        };

        let mapping = match *get(v, "mapping")? {
            // Value::String(ref s) => vec![Mapping::Direct(vec![s.to_owned()])],
            Value::Array(ref a) => a.iter().map(Mapping::from_json).collect::<Result<Vec<_>>>()?,
            _ => bail!(InvalidSchema("mapping must be an array".into())),
        };

        Ok(PropertyDefinition {
            types: types,
            properties: properties,
            mapping: mapping,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn property_type_parse_valid() {
        let json = json!("string");
        let result = PropertyType::from_json(&json);
        assert_eq!(result.unwrap(), PropertyType::String);
    }

    #[test]
    fn property_type_parse_invalid() {
        let json = json!([]);
        let result = PropertyType::from_json(&json);
        assert!(result.is_err());
    }

    #[test]
    fn mapping_parse_direct() {
        let json = json!("direct");
        let result = Mapping::from_json(&json);
        assert_eq!(result.unwrap(), Mapping::Direct("direct".into()));
    }
}
