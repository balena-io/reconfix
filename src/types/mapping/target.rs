//! DSL `schema.mapping.targets` structures
//!
//! https://github.com/balena-io/balena/blob/63ca3a4b026694750f8d6f4e3eea9792cf344426/specs/configuration-dsl-mapping-extension.md#keyword-targets

// TODO: This should be part of the balena-cdsl crate

use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

/// Target type
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub enum Type {
    /// Single file
    #[serde(rename = "file")]
    File,
    /// List of files
    ///
    /// This type allows you to use glob patterns in `Location.path`.
    #[serde(rename = "fileset")]
    FileSet,
}

/// Target file format
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub enum Format {
    /// INI file
    #[serde(rename = "ini")]
    Ini,
    /// JSON file
    #[serde(rename = "json")]
    Json,
}

/// Target file partition
#[derive(Debug, Clone, PartialEq)]
pub enum Partition {
    /// Partition index
    Index(u8),
    /// Partition UUID (GPT tables)
    Uuid(Uuid),
    /// Partition label
    Label(String),
}

impl Partition {
    /// Returns partition index
    pub fn index(&self) -> Option<u8> {
        match self {
            Partition::Index(index) => Some(*index),
            _ => None,
        }
    }

    /// Returns partition UUID
    pub fn uuid(&self) -> Option<&Uuid> {
        match self {
            Partition::Uuid(uuid) => Some(uuid),
            _ => None,
        }
    }

    /// Returns partition label
    pub fn label(&self) -> Option<&str> {
        match self {
            Partition::Label(label) => Some(label),
            _ => None,
        }
    }
}

/// Target location
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Location {
    /// Absolute file path or folder path with glob pattern (depends on `Type`)
    path: String,
    /// Partition
    #[serde(with = "partition_de_serializer")]
    partition: Partition,
}

impl Location {
    /// Returns target location absolute path inside partition
    ///
    /// It's not an absolute system path!
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Returns target location partition info
    pub fn partition(&self) -> &Partition {
        &self.partition
    }
}

/// Target
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Target {
    /// Target type
    #[serde(rename = "type")]
    type_: Type,
    /// Target file format
    format: Format,
    /// Target location
    location: Location,
}

impl Target {
    /// Returns target type
    pub fn type_(&self) -> Type {
        self.type_
    }

    /// Returns target file format
    pub fn format(&self) -> Format {
        self.format
    }

    /// Returns target location
    pub fn location(&self) -> &Location {
        &self.location
    }
}

mod partition_de_serializer {
    use serde::{Deserialize, Deserializer, Serializer};
    use serde::de::Error;
    use serde_yaml::Value;
    use uuid::Uuid;

    use super::Partition;

    pub fn serialize<S>(partition: &Partition, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match partition {
            Partition::Index(index) => serializer.serialize_u8(*index),
            Partition::Uuid(uuid) => serializer.serialize_str(&uuid.to_hyphenated_ref().to_string()),
            Partition::Label(label) => serializer.serialize_str(label),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Partition, D::Error>
    where
        D: Deserializer<'de>,
    {
        // This can be much simpler, but because I'm deserializing from string and
        // from Value as well, it's more complicated.
        //
        // See deserialize_from_value test in types::mapping::mod
        match Value::deserialize(deserializer)? {
            Value::Number(num) => Ok(Partition::Index(
                num.as_u64()
                    .and_then(|x| Some(x as u8))
                    .ok_or_else(|| Error::custom("invalid partition number"))?,
            )),
            Value::String(s) => {
                if let Ok(index) = s.parse::<u8>() {
                    return Ok(Partition::Index(index));
                }

                if let Ok(uuid) = s.parse::<Uuid>() {
                    return Ok(Partition::Uuid(uuid));
                }

                Ok(Partition::Label(s))
            }
            _ => Err(Error::custom("invalid partition value")),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_yaml;

    use super::*;

    #[test]
    fn location_partition_index_deserialization() {
        let yaml = r#"
            type: file
            format: ini
            location:
                path: /somewhere
                partition: 2
        "#;
        let target: Target = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(target.location().partition(), &Partition::Index(2));
    }

    #[test]
    fn location_partition_label_deserialization() {
        let yaml = r#"
            type: file
            format: ini
            location:
                path: /somewhere
                partition: some-label
        "#;
        let target: Target = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(
            target.location().partition(),
            &Partition::Label("some-label".to_string())
        );
    }

    #[test]
    fn location_partition_uuid_deserialization() {
        let yaml = r#"
            type: file
            format: ini
            location:
                path: /somewhere
                partition: 7dc28be7-0620-463b-b4a0-ba260a694cbc
        "#;
        let target: Target = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(
            target.location().partition(),
            &Partition::Uuid("7dc28be7-0620-463b-b4a0-ba260a694cbc".parse().unwrap())
        );
    }

    #[test]
    fn fail_on_missing_location_partition() {
        let yaml = r#"
            type: file
            format: ini
            location:
                path: /somewhere
        "#;
        let target: Result<Target, _> = serde_yaml::from_str(yaml);
        assert!(target.is_err());
    }

    #[test]
    fn fail_on_missing_location_path() {
        let yaml = r#"
            type: file
            format: ini
            location:
                partition: 7dc28be7-0620-463b-b4a0-ba260a694cbc
        "#;
        let target: Result<Target, _> = serde_yaml::from_str(yaml);
        assert!(target.is_err());
    }

    #[test]
    fn fail_on_missing_location() {
        let yaml = r#"
            type: file
            format: ini
        "#;
        let target: Result<Target, _> = serde_yaml::from_str(yaml);
        assert!(target.is_err());
    }

    #[test]
    fn file_type_id() {
        let yaml = r#"
            type: file
            format: ini
            location:
                path: /somewhere
                partition: 0
        "#;
        let target: Target = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(target.type_(), Type::File);
    }

    #[test]
    fn fileset_type_id() {
        let yaml = r#"
            type: fileset
            format: ini
            location:
                path: /somewhere
                partition: 0
        "#;
        let target: Target = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(target.type_(), Type::FileSet);
    }

    #[test]
    fn fail_on_invalid_type_id() {
        let yaml = r#"
            type: foo
            format: ini
            location:
                path: /somewhere
                partition: 0
        "#;
        let target: Result<Target, _> = serde_yaml::from_str(yaml);
        assert!(target.is_err());
    }

    #[test]
    fn fail_on_missing_type_id() {
        let yaml = r#"
            format: ini
            location:
                path: /somewhere
                partition: 0
        "#;
        let target: Result<Target, _> = serde_yaml::from_str(yaml);
        assert!(target.is_err());
    }

    #[test]
    fn ini_format() {
        let yaml = r#"
            type: file
            format: ini
            location:
                path: /somewhere
                partition: 0
        "#;
        let target: Target = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(target.format(), Format::Ini);
    }

    #[test]
    fn json_format() {
        let yaml = r#"
            type: fileset
            format: json
            location:
                path: /somewhere
                partition: 0
        "#;
        let target: Target = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(target.format(), Format::Json);
    }

    #[test]
    fn fail_on_invalid_format() {
        let yaml = r#"
            type: file
            format: foo
            location:
                path: /somewhere
                partition: 0
        "#;
        let target: Result<Target, _> = serde_yaml::from_str(yaml);
        assert!(target.is_err());
    }

    #[test]
    fn fail_on_missing_format() {
        let yaml = r#"
            type: file
            location:
                path: /somewhere
                partition: 0
        "#;
        let target: Result<Target, _> = serde_yaml::from_str(yaml);
        assert!(target.is_err());
    }
}
