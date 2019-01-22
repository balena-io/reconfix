use std::fmt;

use serde::de;
use serde_derive::Deserialize;
use uuid::Uuid;

use crate::utils::deref::OptionDeref;

/// Target type
#[derive(Debug, Copy, Clone, PartialEq, Deserialize)]
pub enum TargetType {
    /// Single file
    #[serde(rename = "file")]
    File,
    /// List of files
    ///
    /// This type allows you to use glob patterns in `Location.path`.
    #[serde(rename = "fileset")]
    FileSet,
}

impl TargetType {
    pub fn is_file(self) -> bool {
        match self {
            TargetType::File => true,
            _ => false,
        }
    }

    pub fn is_file_set(self) -> bool {
        match self {
            TargetType::FileSet => true,
            _ => false,
        }
    }
}

/// Target file format
#[derive(Debug, Copy, Clone, PartialEq, Deserialize)]
pub enum TargetFormat {
    #[serde(rename = "ini")]
    Ini,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "binary")]
    Binary,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "redsocks")]
    Redsocks,
}

impl TargetFormat {
    pub fn is_ini(self) -> bool {
        match self {
            TargetFormat::Ini => true,
            _ => false,
        }
    }

    pub fn is_json(self) -> bool {
        match self {
            TargetFormat::Json => true,
            _ => false,
        }
    }

    pub fn is_binary(self) -> bool {
        match self {
            TargetFormat::Binary => true,
            _ => false,
        }
    }

    pub fn is_text(self) -> bool {
        match self {
            TargetFormat::Text => true,
            _ => false,
        }
    }

    pub fn is_redsocks(self) -> bool {
        match self {
            TargetFormat::Redsocks => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LocationPartition {
    Index(u8),
    Uuid(Uuid),
    Label(String),
}

impl LocationPartition {
    pub fn index(&self) -> Option<u8> {
        match self {
            LocationPartition::Index(index) => Some(*index),
            _ => None,
        }
    }

    pub fn uuid(&self) -> Option<&Uuid> {
        match self {
            LocationPartition::Uuid(uuid) => Some(uuid),
            _ => None,
        }
    }

    pub fn label(&self) -> Option<&str> {
        match self {
            LocationPartition::Label(label) => Some(label),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TargetLocation {
    path: String,
    partition: LocationPartition,
}

impl TargetLocation {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn partition(&self) -> &LocationPartition {
        &self.partition
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct RawTarget {
    #[serde(rename = "type")]
    type_: TargetType,
    format: TargetFormat,
    glob: Option<String>,
    location: TargetLocation,
}

impl RawTarget {
    pub fn type_(&self) -> &TargetType {
        &self.type_
    }

    pub fn format(&self) -> &TargetFormat {
        &self.format
    }

    pub fn glob(&self) -> Option<&str> {
        self.glob.as_deref()
    }

    pub fn location(&self) -> &TargetLocation {
        &self.location
    }
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum Target {
    Reference(String),
    Raw(RawTarget),
}

impl Target {
    pub fn reference(&self) -> Option<&str> {
        match self {
            Target::Reference(r) => Some(r),
            _ => None,
        }
    }

    pub fn raw(&self) -> Option<&RawTarget> {
        match self {
            Target::Raw(r) => Some(r),
            _ => None,
        }
    }
}

struct PartitionVisitor;

impl<'de> de::Visitor<'de> for PartitionVisitor {
    type Value = LocationPartition;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("expected property")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v > u64::from(std::u8::MAX) {
            return Err(de::Error::custom("partition index out of bounds"));
        }
        Ok(LocationPartition::Index(v as u8))
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if let Ok(uuid) = Uuid::parse_str(s) {
            return Ok(LocationPartition::Uuid(uuid));
        }

        Ok(LocationPartition::Label(s.to_string()))
    }
}

impl<'de> de::Deserialize<'de> for LocationPartition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(PartitionVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_reference() {
        let t: Target = serde_yaml::from_str("foo").unwrap();
        assert_eq!(t.reference(), Some("foo"));
    }

    #[test]
    fn target_raw() {
        let schema = r#"
            type: file
            format: json
            location:
                partition: 0
                path: /config.json

        "#;
        let t: Target = serde_yaml::from_str(schema).unwrap();
        let t = t.raw().unwrap();
        assert!(t.type_().is_file());
        assert!(t.format().is_json());
        assert_eq!(t.location().partition().index(), Some(0));
        assert_eq!(t.location().path(), "/config.json");
    }

    #[test]
    fn raw_target() {
        let schema = r#"
            type: file
            format: json
            location:
                partition: 0
                path: /config.json

        "#;
        let t: RawTarget = serde_yaml::from_str(schema).unwrap();
        assert!(t.type_().is_file());
        assert!(t.format().is_json());
        assert_eq!(t.location().partition().index(), Some(0));
        assert_eq!(t.location().path(), "/config.json");
    }

    #[test]
    fn format_ini() {
        let f: TargetFormat = serde_yaml::from_str("ini").unwrap();
        assert!(f.is_ini());
    }

    #[test]
    fn format_json() {
        let f: TargetFormat = serde_yaml::from_str("json").unwrap();
        assert!(f.is_json());
    }

    #[test]
    fn format_binary() {
        let f: TargetFormat = serde_yaml::from_str("binary").unwrap();
        assert!(f.is_binary());
    }

    #[test]
    fn type_file() {
        let t: TargetType = serde_yaml::from_str("file").unwrap();
        assert!(t.is_file());
    }

    #[test]
    fn type_file_set() {
        let t: TargetType = serde_yaml::from_str("fileset").unwrap();
        assert!(t.is_file_set());
    }

    #[test]
    fn partition_index() {
        let p: LocationPartition = serde_yaml::from_str("0").unwrap();
        assert_eq!(p.index(), Some(0));
    }

    #[test]
    fn partition_label() {
        let p: LocationPartition = serde_yaml::from_str("foo").unwrap();
        assert_eq!(p.label(), Some("foo"));
    }

    #[test]
    fn partition_uuid() {
        const UUID: &str = "20dd882d-7042-4213-ba7b-88638ea34b37";
        let p: LocationPartition = serde_yaml::from_str(UUID).unwrap();
        let uuid: Uuid = Uuid::parse_str(UUID).unwrap();
        assert_eq!(p.uuid(), Some(&uuid));
    }
}
