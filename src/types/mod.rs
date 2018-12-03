//! DSL `schema.mapping` structures
//!
//! https://github.com/balena-io/balena/blob/63ca3a4b026694750f8d6f4e3eea9792cf344426/specs/configuration-dsl-mapping-extension.md#mapping-object

// TODO: This should be part of the balena-cdsl crate

use std::collections::HashMap;

use balena_cdsl::dsl::schema::{DocumentRoot, Schema};
use serde::Deserialize;
use serde_yaml::Value;

pub use self::mapping::map::Entry;
pub use self::mapping::Mapping;
pub use self::mapping::target::{Format, Location, Partition, Target, Type};

mod mapping;

pub trait MappingExt {
    fn mapping(&self) -> Option<Mapping>;
}

impl MappingExt for Schema {
    fn mapping(&self) -> Option<Mapping> {
        self.mapping
            .as_ref()
            .and_then(|x| Mapping::deserialize(Value::Mapping(x.clone())).ok())
    }
}

impl MappingExt for Option<Schema> {
    fn mapping(&self) -> Option<Mapping> {
        self.as_ref().and_then(|x| x.mapping())
    }
}

pub trait TargetExt {
    fn targets(&self) -> HashMap<String, Target>;
}

impl TargetExt for Schema {
    fn targets(&self) -> HashMap<String, Target> {
        let mut result: HashMap<String, Target> = self.mapping().and_then(|x| x.targets().cloned()).unwrap_or_default();

        if let Some(children) = self.children.as_ref() {
            for named_schema in children.entries() {
                result.extend(named_schema.schema.targets())
            }
        }

        result
    }
}

impl TargetExt for DocumentRoot {
    fn targets(&self) -> HashMap<String, Target> {
        self.schema.as_ref().and_then(|x| Some(x.targets())).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use balena_cdsl::dsl::schema::compiler::compile;
    use serde_yaml::Value;

    use super::*;

    #[test]
    fn root_targets() {
        let yaml = r#"
            version: 1
            mapping:
                targets:
                    config_json:
                        type: file
                        format: json
                        location:
                            partition: 2
                            path: /config.json
        "#;

        let v: Value = serde_yaml::from_str(yaml).unwrap();
        let doc = compile(v).unwrap().compiled();
        let targets = doc.targets();
        assert_eq!(targets.len(), 1);
    }

    #[test]
    fn nested_targets() {
        let yaml = r#"
            version: 1
            mapping:
                targets:
                    config_json:
                        type: file
                        format: json
                        location:
                            partition: 0
                            path: /config.json
            properties:
                - wifi:
                    mapping:
                        targets:
                            wifi:
                                type: file
                                format: json
                                location:
                                    partition: 0
                                    path: /system-connections/wifi
        "#;
        let v: Value = serde_yaml::from_str(yaml).unwrap();
        let doc = compile(v).unwrap().compiled();
        let targets = doc.targets();
        assert_eq!(targets.len(), 2);

        let t = &targets["config_json"];
        assert_eq!(t.location().path(), "/config.json");

        let t = &targets["wifi"];;
        assert_eq!(t.location().path(), "/system-connections/wifi");
    }
}
