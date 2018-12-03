//! DSL `schema.mapping` structures
//!
//! https://github.com/balena-io/balena/blob/63ca3a4b026694750f8d6f4e3eea9792cf344426/specs/configuration-dsl-mapping-extension.md#mapping-object

// TODO: This should be part of the balena-cdsl crate

use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};
use serde_yaml::Value;

use crate::utils::deref::OptionDeref;

use self::target::Target;

pub(crate) mod target;
pub(crate) mod map;

/// Mapping target filename
#[derive(Debug, PartialEq)]
pub enum Filename {
    /// Actual file name
    Name(String),
    /// Something to be evaluated first to get a real file name
    Evaluate(String),
}

/// Mapping structure
#[derive(Debug, Deserialize, Serialize)]
pub struct Mapping {
    #[serde(skip_serializing_if = "Option::is_none")]
    targets: Option<HashMap<String, Target>>,

    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    target: Option<String>,

    // TODO: Implement
    //
    // https://github.com/balena-io/balena/blob/63ca3a4b026694750f8d6f4e3eea9792cf344426/specs/configuration-dsl-mapping-extension.md#keyword-map
    #[serde(default, skip_serializing_if = "Option::is_none")]
    map: Option<Vec<map::Entry>>,

    // TODO: Implement
    //
    // https://github.com/balena-io/balena/blob/63ca3a4b026694750f8d6f4e3eea9792cf344426/specs/configuration-dsl-mapping-extension.md#keyword-filename
    //
    // Can be a string or a dictionary with `eval`.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "filename_de_serializer")]
    filename: Option<Filename>,

    // TODO: Implement
    //
    // https://github.com/balena-io/balena/blob/63ca3a4b026694750f8d6f4e3eea9792cf344426/specs/configuration-dsl-mapping-extension.md#keyword-path
    //
    // Must be parsable as temen's AST Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,

    // TODO: Implement
    //
    // https://github.com/balena-io/balena/blob/63ca3a4b026694750f8d6f4e3eea9792cf344426/specs/configuration-dsl-mapping-extension.md#keyword-template
    //
    // Probably nothing to do here as the value should be dictionary (pure JSON value).
    #[serde(skip_serializing_if = "Option::is_none")]
    template: Option<Value>,
}

impl Mapping {
    /// Returns target name
    pub fn target(&self) -> Option<&str> {
        self.target.as_deref()
    }

    /// Returns target path
    pub fn filename(&self) -> Option<&Filename> {
        self.filename.as_ref()
    }

    /// Returns target path
    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }

    /// Returns target path
    pub fn template(&self) -> Option<&Value> {
        self.template.as_ref()
    }

    /// Returns map
    pub fn map(&self) -> Option<&Vec<map::Entry>> {
        self.map.as_ref()
    }

    /// Returns number of targets
    pub fn target_count(&self) -> usize {
        match &self.targets {
            Some(targets) => targets.len(),
            _ => 0,
        }
    }

    /// Returns target by name
    ///
    /// # Arguments
    ///
    /// * `name` - Target name
    pub fn named_target(&self, name: &str) -> Option<&Target> {
        match &self.targets {
            Some(targets) => targets.get(name),
            _ => None,
        }
    }

    /// Returns all targets
    pub fn targets(&self) -> Option<&HashMap<String, Target>> {
        self.targets.as_ref()
    }
}

mod filename_de_serializer {
    use serde::{Deserialize, Deserializer, Serializer};
    use serde::de::Error;
    use serde::ser::SerializeMap;
    use serde_yaml::Value;

    use super::Filename;

    pub fn serialize<S>(filename: &Option<Filename>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(filename) = filename {
            match filename {
                Filename::Name(ref name) => serializer.serialize_str(name),
                Filename::Evaluate(ref eval) => {
                    let mut map = serializer.serialize_map(Some(1))?;
                    map.serialize_entry("eval", eval)?;
                    map.end()
                }
            }
        } else {
            serializer.serialize_none()
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Filename>, D::Error>
    where
        D: Deserializer<'de>,
    {
        match Value::deserialize(deserializer)? {
            Value::String(name) => Ok(Some(Filename::Name(name))),
            Value::Mapping(mut map) => {
                let eval_key = Value::String("eval".to_string());
                let eval = map.remove(&eval_key).ok_or_else(|| Error::custom("missing eval key"))?;

                match eval {
                    Value::String(eval) => Ok(Some(Filename::Evaluate(eval))),
                    _ => Err(Error::custom("eval key must be a string")),
                }
            }
            _ => Err(Error::custom("invalid filename value")),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_yaml::Value;

    use crate::types::*;

    use super::*;

    #[test]
    fn named_target() {
        let yaml = r#"
            targets:
                config_json:
                    type: fileset
                    format: json
                    location:
                        path: /somewhere
                        partition: 0
        "#;
        let mapping: Mapping = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(mapping.target_count(), 1);

        let target = mapping.named_target("config_json").unwrap();
        assert_eq!(target.type_(), Type::FileSet);
        assert_eq!(target.format(), Format::Json);
        assert_eq!(target.location().path(), "/somewhere");
        assert_eq!(target.location().partition(), &Partition::Index(0));
    }

    #[test]
    fn filename_as_string() {
        let yaml = r#"
            filename: foo
        "#;
        let mapping: Mapping = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(mapping.filename(), Some(&Filename::Name("foo".to_string())));
    }

    #[test]
    fn filename_as_object_with_eval() {
        let yaml = r#"
            filename:
                eval: super.id
        "#;
        let mapping: Mapping = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(mapping.filename(), Some(&Filename::Evaluate("super.id".to_string())));
    }

    #[test]
    fn fail_on_filename_as_object_without_eval() {
        let yaml = r#"
            filename:
                foo: bar
        "#;
        let mapping: Result<Mapping, _> = serde_yaml::from_str(yaml);
        assert!(mapping.is_err());
    }

    #[test]
    fn path() {
        let yaml = r#"
            path: foo
        "#;
        let mapping: Mapping = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(mapping.path(), Some("foo"));
    }

    #[test]
    fn target() {
        let yaml = r#"
            target: foo
        "#;
        let mapping: Mapping = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(mapping.target(), Some("foo"));
    }

    #[test]
    fn map() {
        let yaml = r#"
            map:
                - - true
                  - "yes"
                - - false
                  - "no"
                - - identity
                  - identity
                - identity
        "#;
        let mapping: Mapping = serde_yaml::from_str(yaml).unwrap();
        let map = mapping.map().unwrap();
        assert_eq!(map.len(), 4);

        let mut i = map.iter();
        let e = i.next().unwrap();
        assert_eq!(e.matcher(), &Value::Bool(true));
        assert_eq!(e.output(), &Value::String("yes".to_string()));

        let e = i.next().unwrap();
        assert_eq!(e.matcher(), &Value::Bool(false));
        assert_eq!(e.output(), &Value::String("no".to_string()));

        assert!(i.next().unwrap().is_identity());
        assert!(i.next().unwrap().is_identity());
    }

    #[test]
    fn all_fields_all_optional() {
        let yaml = "{}";
        let mapping: Result<Mapping, _> = serde_yaml::from_str(yaml);
        assert!(mapping.is_ok());
    }

    #[test]
    fn deserialize_from_value() {
        // This is something that shouldn't be included in cdsl, it's just for
        // me to check that the Mapping can be deserialized from yaml::Value
        let yaml = r#"
            target: foo
            filename: bar
            path: baz
            targets:
                config_json:
                    type: file
                    format: json
                    location:
                        partition: 2
                        path: /config.json
        "#;

        let value: Value = serde_yaml::from_str(yaml).unwrap();
        let mapping = Mapping::deserialize(value).unwrap();

        assert_eq!(mapping.target(), Some("foo"));
        assert_eq!(mapping.filename(), Some(&Filename::Name("bar".to_string())));
        assert_eq!(mapping.path(), Some("baz"));

        let t = mapping.named_target("config_json").unwrap();
        assert_eq!(t.type_(), Type::File);
        assert_eq!(t.format(), Format::Json);
        assert_eq!(t.location().partition(), &Partition::Index(2));
        assert_eq!(t.location().path(), "/config.json");
    }
}
