
use std::collections::BTreeMap;

use errors::*;
use schema::{File, FileFormat, Location, Mapping, Property, Schema};
use adaptor::{Adaptor, IniAdaptor, JsonAdaptor};

use serde_json::Value;

type JsObject = ::serde_json::Map<String, Value>;

/// A simple data structure associating file names and content.
/// This struct allows implementing the partition and file reading
/// code in a seperate module.
pub struct Entry {
    /// The file name associated with this entry.
    pub name: String,
    /// The file content associated with this entry.
    pub content: String,
}

/// Transform the raw file data into a dry JSON structure.
/// A matching `Entry` is requird for every `Independent` file
/// in the `Schema`.
pub fn transform_wet(config: Vec<Entry>, schema: &Schema) -> Result<Value> {
    let mut root = JsObject::new();
    let ordered = sort_files(&schema.files);
    let mut mapped = config
        .into_iter()
        .map(|e| (e.name, e.content))
        .collect::<BTreeMap<_, _>>();

    let mut wet_cache = BTreeMap::new();

    for (name, file) in ordered {
        let wet_content = match &file.location {
            &Location::Independent {
                ..
            } => mapped.remove(name).ok_or("file not found")?,
            &Location::Dependent {
                ref parent,
                ref location,
            } => {
                let parent_wet_content = wet_cache.get(parent).ok_or("parent not found")?;
                let value = follow_pointer(parent_wet_content, location.as_ref());
                let inner_content = value.as_str().ok_or("value is not a string")?;
                inner_content.to_string()
            },
        };

        let wet = deserialize(wet_content, &file.format)?;
        generate_dry_property(&mut root, &wet, file.properties.as_slice())?;
        wet_cache.insert(name.to_string(), wet);
    }

    Ok(Value::Object(root))
}

/// Order files for processing. Currently, this just sorts `Independent` files
/// before `Dependent` files. This does not allow for chained dependencies.
fn sort_files<'a>(files: &'a BTreeMap<String, File>) -> Vec<(&'a str, &'a File)> {
    let mut ordered = files
        .iter()
        .map(|(name, file)| (name.as_ref(), file))
        .collect::<Vec<_>>();
    ordered.sort_by_key(|&(_, ref file)| file.location.clone());
    ordered
}

/// Recursively process properties, extracting wet values and inserting them
/// into the dry tree.
fn generate_dry_property(dry: &mut JsObject, wet: &Value, props: &[Property]) -> Result<()> {
    for prop in props.iter() {
        for (name, def) in prop.definition.iter() {
            //TODO: perform type checkes

            //TODO: determine behavior when multiple mappings are found
            for mapping in def.mapping.iter() {
                let value = match mapping {
                    &Mapping::Direct(ref ptr) => Some(follow_pointer(wet, &ptr)),
                    &Mapping::Template {
                        ref value,
                        ref template,
                    } => {
                        if ::template::matches(wet, template) {
                            Some(value)
                        } else {
                            None
                        }
                    },
                };

                if let Some(val) = value {
                    dry.insert(name.clone(), val.clone());
                }
            }

            if let &mut Value::Object(ref mut inner) =
                dry.entry(name.as_ref()).or_insert(json!({}))
            {
                generate_dry_property(inner, wet, &def.properties)?;
            }
        }
    }

    Ok(())
}

/// Transform a dry configuration structure into the raw file content
/// for the configuration files defined in the `Schema`.
pub fn transform_dry(config: Value, schema: &Schema) -> Result<Vec<Entry>> {
    let ordered = sort_files(&schema.files);

    let dry = config.as_object().unwrap();

    let mut files = BTreeMap::new();
    for (name, file) in ordered.into_iter() {
        let wet = generate_wet_file(dry, &file.properties)?;

        match file.location {
            Location::Independent {
                ..
            } => {
                files.insert(name.to_string(), (file.format.clone(), wet));
            },
            Location::Dependent {
                ref parent,
                ref location,
            } => {
                let serialized = serialize(wet, &file.format)?;
                let entry = files.get_mut(parent).ok_or("parent file not found")?;
                let mut value = follow_pointer_mut(&mut entry.1, location);
                *value = Value::String(serialized);
            },
        }
    }

    let output = files
        .into_iter()
        .map(|(name, (format, wet))| {
            serialize(wet, &format).map(|c| {
                Entry {
                    name: name.to_string(),
                    content: c,
                }
            })
        })
        .collect::<Result<Vec<_>>>();

    output
}

/// Convert wet JSON into a raw `String` using the formatter appropriate
/// for the provided `FileFormat`.
fn serialize(wet: Value, format: &FileFormat) -> Result<String> {
    let mut buffer = Vec::new();
    match format {
        &FileFormat::Ini => {
            let adaptor = IniAdaptor::new();
            adaptor.serialize(wet, &mut buffer)?;
        },
        &FileFormat::Json => {
            let adaptor = JsonAdaptor::new();
            adaptor.serialize(wet, &mut buffer)?;
        },
    }
    String::from_utf8(buffer).chain_err(|| "unable to decode utf-8")
}

/// Deserialize raw text using the appropriate formatter for the
/// `FileFormat` and return the wet JSON.
fn deserialize(content: String, format: &FileFormat) -> Result<Value> {
    let buffer = content.into_bytes();
    match format {
        &FileFormat::Ini => {
            let adaptor = IniAdaptor::new();
            adaptor.deserialize(buffer.as_slice())
        },
        &FileFormat::Json => {
            let adaptor = JsonAdaptor::new();
            adaptor.deserialize(buffer.as_slice())
        },
    }
}

/// Generate a wet JSON object.
fn generate_wet_file(dry: &JsObject, props: &[Property]) -> Result<Value> {
    let mut root = Value::Object(JsObject::new());

    for prop in props {
        generate_wet_property(dry, &mut root, prop)?;
    }

    Ok(root)
}

/// Follow a JSON pointer, returning a reference to the refered `Value`.
fn follow_pointer<'a>(v: &'a Value, pointer: &str) -> &'a Value {
    let names = pointer.trim_left_matches('/').split('/');

    names.fold(v, |state, name| match state {
        &Value::Object(ref obj) => obj.get(name).unwrap(),
        _ => panic!("invalid value type"),
    })
}

/// Follow a JSON pointer, returning a mutable reference. If any property in
/// the pointer chain does not exist, a new `Object` will be inserted.
fn follow_pointer_mut<'a>(v: &'a mut Value, pointer: &str) -> &'a mut Value {
    let names = pointer.trim_left_matches('/').split('/');

    names.fold(v, |state, name| match state {
        &mut Value::Object(ref mut obj) => obj.entry(name).or_insert(json!({})),
        _ => panic!("invalid value type"),
    })
}

/// Insert the specified template value into the wet JSON tree.
/// NOTE: currently does not ignore wildcard values.
fn insert_template(tree: &mut JsObject, template: &JsObject) -> Result<()> {
    for (key, value) in template {
        match value {
            &Value::Object(ref subtemplate) => {
                if let &mut Value::Object(ref mut inner) =
                    tree.entry(key.clone()).or_insert(json!({}))
                {
                    insert_template(inner, subtemplate)?;
                } else {
                    bail!("cannot insert template: key already has value")
                }
            },
            x => {
                if let Some(old) = tree.insert(key.clone(), x.clone()) {
                    //TODO check for wildcards
                    bail!("cannot insert ({}, {}): key already has value {}", key, x, old);
                }
            },
        }
    }

    Ok(())
}

/// Recursively process dry JSON values and insert them into the wet JSON tree.
fn generate_wet_property(dry: &JsObject, wet: &mut Value, prop: &Property) -> Result<()> {
    //TODO: check when values
    for (name, definition) in prop.definition.iter() {
        let dry_value = dry.get(&*name).ok_or("dry value not found")?;

        for mapping in definition.mapping.iter() {
            match mapping {
                &Mapping::Direct(ref ptr) => {
                    let mut value = follow_pointer_mut(wet, &ptr);
                    *value = dry_value.clone();
                },
                &Mapping::Template {
                    ref value,
                    ref template,
                } => {
                    if value.eq(dry_value) {
                        let template_obj =
                            template.as_object().ok_or("template must be an object")?;
                        if let &mut Value::Object(ref mut wet_obj) = wet {
                            insert_template(wet_obj, &template_obj)?;
                        } else {
                            bail!("wet value must be an object");
                        }
                    }
                },
            }
        }

        if let Some(sub) = dry_value.as_object() {
            for property in definition.properties.iter() {
                generate_wet_property(sub, wet, &property)?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    extern crate serde_json;

    use super::*;
    use schema::*;

    use serde_json::to_string;

    #[test]
    fn dry_to_wet_simple_direct() {
        let dry = json!({ "dry": "value" });
        let props = vec![Property {
            when: None,
            definition: btreemap!{
                "dry".into() => PropertyDefinition {
                    types: vec![PropertyType::String],
                    properties: vec![],
                    mapping: vec![Mapping::Direct("/wet".into())],
                },
            },
        }];

        let expected = json!({ "wet": "value" });

        let result = generate_wet_file(dry.as_object().unwrap(), &props).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn dry_to_wet_dry_heirarchy_direct() {
        let dry = json!({ "parent": { "dry": "value" } });
        let props = vec![Property {
            when: None,
            definition: btreemap!{
                "parent".into() => PropertyDefinition {
                    types: vec![PropertyType::String],
                    mapping: vec![],
                    properties: vec![Property {
                        when: None,
                        definition: btreemap!{
                            "dry".into() => PropertyDefinition {
                                types: vec![PropertyType::String],
                                mapping: vec![Mapping::Direct("/wet".into())],
                                properties: vec![],
                            }
                        }
                    }],
                },
            },
        }];

        let expected = json!({ "wet": "value" });

        let result = generate_wet_file(dry.as_object().unwrap(), &props).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn dry_to_wet_wet_heirarchy_direct() {
        let dry = json!({ "dry": "value" });
        let props = vec![Property {
            when: None,
            definition: btreemap!{
                "dry".into() => PropertyDefinition {
                    types: vec![PropertyType::String],
                    properties: vec![],
                    mapping: vec![Mapping::Direct("/parent/wet".into())],
                },
            },
        }];

        let expected = json!({ "parent": { "wet": "value" } });

        let result = generate_wet_file(dry.as_object().unwrap(), &props).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn dry_to_wet_simple_template() {
        let dry = json!({ "template": "yes" });
        let template = json!({"parent": { "key": "value" } });
        let props = vec![Property {
            when: None,
            definition: btreemap!{
                "template".into() => PropertyDefinition {
                    types: vec![PropertyType::String],
                    properties: vec![],
                    mapping: vec![Mapping::Template {
                        value: json!("yes"),
                        template: template.clone(),
                    }],
                },
            },
        }];

        let result = generate_wet_file(dry.as_object().unwrap(), &props).unwrap();

        assert_eq!(template, result);

        let dry = json!({"template": "no"});
        let result = generate_wet_file(dry.as_object().unwrap(), &props).unwrap();

        assert_eq!(json!({}), result);
    }

    #[test]
    fn dry_to_wet_dependent_json() {
        let dry = json!({"key": "value" });
        let files =
            btreemap!{
            "independent".into() => File {
                format: FileFormat::Json,
                fileset: false,
                location: Location::Independent {
                    partition: Partition::Primary(0),
                    path: vec![],
                },
                properties: vec![],
            },
            "dependent".into() => File {
                format: FileFormat::Json,
                fileset: false,
                location: Location::Dependent {
                    parent: "independent".into(),
                    location: "/parent/child".into(),
                },
                properties: vec![Property {
                    when: None,
                    definition: btreemap!{
                        "key".into() => PropertyDefinition {
                            types: vec![PropertyType::String],
                            properties: vec![],
                            mapping: vec![Mapping::Direct("/wet".into())],
                        }
                    },
                }],
            }
        };

        let schema = Schema {
            files: files,
        };

        let result = transform_dry(dry, &schema).unwrap().pop().unwrap();
        let expected = r##"{"parent":{"child":"{\"wet\":\"value\"}"}}"##;

        assert_eq!(expected, result.content);
    }

    #[test]
    fn wet_to_dry_simple_direct() {
        let wet = json!({ "wet": "value" });
        let props = vec![Property {
            when: None,
            definition: btreemap!{
                "dry".into() => PropertyDefinition {
                    types: vec![PropertyType::String],
                    properties: vec![],
                    mapping: vec![Mapping::Direct("/wet".into())],
                },
            },
        }];

        let expected = json!({ "dry": "value" });
        let mut root = JsObject::new();
        generate_dry_property(&mut root, &wet, &props).unwrap();

        assert_eq!(expected, Value::Object(root));
    }

    #[test]
    fn wet_to_dry_dry_heirarchy_direct() {
        let wet = json!({ "wet": "value" });
        let props = vec![Property {
            when: None,
            definition: btreemap!{
                "parent".into() => PropertyDefinition {
                    types: vec![PropertyType::String],
                    mapping: vec![],
                    properties: vec![Property {
                        when: None,
                        definition: btreemap!{
                            "dry".into() => PropertyDefinition {
                                types: vec![PropertyType::String],
                                mapping: vec![Mapping::Direct("/wet".into())],
                                properties: vec![],
                            }
                        }
                    }],
                },
            },
        }];

        let expected = json!({ "parent": { "dry": "value" } });
        let mut root = JsObject::new();
        generate_dry_property(&mut root, &wet, &props).unwrap();

        assert_eq!(expected, Value::Object(root));
    }

    #[test]
    fn wet_to_dry_wet_heirarchy_direct() {
        let wet = json!({ "parent": { "wet": "value" } });
        let props = vec![Property {
            when: None,
            definition: btreemap!{
                "dry".into() => PropertyDefinition {
                    types: vec![PropertyType::String],
                    properties: vec![],
                    mapping: vec![Mapping::Direct("/parent/wet".into())],
                },
            },
        }];

        let expected = json!({ "dry": "value" });
        let mut root = JsObject::new();
        generate_dry_property(&mut root, &wet, &props).unwrap();

        assert_eq!(expected, Value::Object(root));
    }

    #[test]
    fn wet_to_dry_simple_template() {
        let template = json!({"parent": { "key": "value" } });
        let props = vec![Property {
            when: None,
            definition: btreemap!{
                "template".into() => PropertyDefinition {
                    types: vec![PropertyType::String],
                    properties: vec![],
                    mapping: vec![Mapping::Template {
                        value: json!("yes"),
                        template: template.clone(),
                    }],
                },
            },
        }];

        let mut dry = JsObject::new();
        generate_dry_property(&mut dry, &template, &props).unwrap();

        let expected = json!({"template": "yes"});
        assert_eq!(expected, Value::Object(dry));
    }

    #[test]
    fn wet_to_dry_dependent_json() {
        let inner = r##"{"child":{"wet":"value"}}"##;
        let wet = json!({"parent": inner});
        let wet_content = to_string(&wet).unwrap();

        let files =
            btreemap!{
            "independent".into() => File {
                format: FileFormat::Json,
                fileset: false,
                location: Location::Independent {
                    partition: Partition::Primary(0),
                    path: vec![],
                },
                properties: vec![],
            },
            "dependent".into() => File {
                format: FileFormat::Json,
                fileset: false,
                location: Location::Dependent {
                    parent: "independent".into(),
                    location: "/parent".into(),
                },
                properties: vec![Property {
                    when: None,
                    definition: btreemap!{
                        "key".into() => PropertyDefinition {
                            types: vec![PropertyType::String],
                            properties: vec![],
                            mapping: vec![Mapping::Direct("/child/wet".into())],
                        },
                    },
                }],
            },
        };

        let schema = Schema {
            files: files,
        };
        let entry = Entry {
            name: "independent".into(),
            content: wet_content,
        };

        let result = transform_wet(vec![entry], &schema).unwrap();
        let expected = json!({"key": "value"});
        assert_eq!(expected, result);
    }
}
