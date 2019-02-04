//!
//! mapping.map support was intentionally removed. I'd like to see if we can
//! do it without it. If not, I'll put it back. But less stuff we have, more
//! better it is.
//!
use std::collections::HashMap;

use serde_derive::Deserialize;
use serde_yaml::Value;

use crate::utils::deref::OptionDeref;

pub use self::{
    filename::FileName,
    target::{LocationPartition, RawTarget, Target, TargetFormat, TargetLocation, TargetType},
};

mod filename;
mod target;

/// Mapping structure
#[derive(Debug, Deserialize)]
pub struct Mapping {
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    targets: HashMap<String, RawTarget>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    target: Option<Target>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    filename: Option<FileName>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    path: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    template: Option<Value>,
}

impl Mapping {
    pub fn targets(&self) -> &HashMap<String, RawTarget> {
        &self.targets
    }

    pub fn target(&self) -> Option<&Target> {
        self.target.as_ref()
    }

    pub fn filename(&self) -> Option<&FileName> {
        self.filename.as_ref()
    }

    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }

    pub fn template(&self) -> Option<&Value> {
        self.template.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path() {
        let m: Mapping = serde_yaml::from_str("path: /config.json").unwrap();
        assert_eq!(m.path(), Some("/config.json"));
    }

    #[test]
    fn filename() {
        let m: Mapping = serde_yaml::from_str("filename: foo.txt").unwrap();
        assert_eq!(m.filename().unwrap().name(), Some("foo.txt"));
    }

    #[test]
    fn target_as_ref() {
        // Other target options are tested inside target.rs
        let m: Mapping = serde_yaml::from_str("target: config_json").unwrap();
        assert_eq!(m.target().unwrap().reference(), Some("config_json"));
    }

    #[test]
    fn template() {
        let schema = r#"
        template: string
        "#;
        // Other target options are tested inside target.rs
        let m: Mapping = serde_yaml::from_str(schema).unwrap();

        assert_eq!(m.template(), Some(&Value::String("string".to_string())));
    }

    #[test]
    fn targets() {
        let schema = r#"
        targets:
          config_json:
            type: file
            format: json
            location:
              partition: resin-boot
              path: /config.json
        "#;
        let m: Mapping = serde_yaml::from_str(schema).unwrap();

        let t = &m.targets["config_json"];
        assert_eq!(t.type_(), &TargetType::File);
        assert_eq!(t.format(), &TargetFormat::Json);
        assert_eq!(t.location().partition().label(), Some("resin-boot"));
        assert_eq!(t.location().path(), "/config.json");
    }
}
