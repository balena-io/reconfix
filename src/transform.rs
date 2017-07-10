
use std::collections::BTreeMap;

use ::errors::*;
use ::schema::{FileFormat, Location, Mapping, Property, Partition, Schema};
use ::adaptor::{Adaptor, IniAdaptor, JsonAdaptor};

use serde_json::Value;

type JsObject = ::serde_json::Map<String, Value>;

pub struct Entry {
    pub parition: Partition,
    pub path: Vec<String>,
    pub content: String,
}

pub fn transform(config: Value, schema: Schema) -> Result<Vec<Entry>> 
{
    let mut ordered = schema.files.into_iter().collect::<Vec<_>>();
    ordered.sort_by_key(|&(_, ref file)| file.location.clone());

    let dry = config.as_object().unwrap();

    let mut files = BTreeMap::new();   
    for (name, file) in ordered.into_iter() {
        let wet = generate_wet_file(dry, &file.properties)?;

        match file.location {
            Location::Independent { path, partition } => {
                files.insert(name, (partition, path, file.format, wet));
            },
            Location::Dependent { parent, location } => {
                let serialized = serialize(wet, file.format)?;
                let entry = files.get_mut(&parent).ok_or("parent file not found")?;
                let mut value = follow_pointer(&mut entry.3, &location);
                *value = Value::String(serialized);
            }
        }
    }

    let output = files.into_iter()
        .map(|(_, (partition, path, format, wet))| {
            serialize(wet, format)
                .map(|c| Entry {
                    parition: partition,
                    path: path,
                    content: c,
                })
        })
        .collect::<Result<Vec<_>>>();

    output
}

fn serialize(wet: Value, format: FileFormat) -> Result<String> {
    let mut buffer = Vec::new();
    match format {
        FileFormat::Ini => {
            let adaptor = IniAdaptor::new();
            adaptor.serialize(wet, &mut buffer)?;
        },
        FileFormat::Json => {
            let adaptor = JsonAdaptor::new();
            adaptor.serialize(wet, &mut buffer)?;
        }
    }   
    String::from_utf8(buffer).chain_err(|| "unable to decode utf-8")
}

fn generate_wet_file(dry: &JsObject, props: &[Property]) -> Result<Value>
{   
    let mut root = Value::Object(JsObject::new());

    for prop in props {
        generate_wet_property(dry, &mut root, prop)?;
    }

    Ok(root)
}

fn follow_pointer<'a>(v: &'a mut Value, pointer: &str) -> &'a mut Value {
    let names = pointer.trim_left_matches('/').split('/');
    
    names.fold(v, |state, name| {
        match state {
            &mut Value::Object(ref mut obj) => {
                obj.entry(name).or_insert(json!({}))
            }
            _ => panic!("invalid value type")
        }
    })
}

fn insert_template(tree: &mut JsObject, template: &JsObject) -> Result<()> {
    for (key, value) in template {
        match value {
            &Value::Object(ref subtemplate) => {
                if let &mut Value::Object(ref mut inner) = tree.entry(key.clone()).or_insert(json!({})) {
                    insert_template(inner, subtemplate)?;
                } else {
                    bail!("cannot insert template: key already has value")
                }
            }
            x => {
                if let Some(old) = tree.insert(key.clone(), x.clone()) {
                    //TODO check for wildcards
                    bail!("cannot insert ({}, {}): key already has value {}", key, x, old);
                }
            }
        }
    }

    Ok(())
}

fn generate_wet_property(dry: &JsObject, wet: &mut Value, prop: &Property) -> Result<()> {
    //TODO: check when values
    for (name, definition) in prop.definition.iter() {
        let dry_value = dry.get(&*name).ok_or("dry value not found")?;

        for mapping in definition.mapping.iter() {
            match mapping {
                &Mapping::Direct(ref ptr) => {
                    let mut value = follow_pointer(wet, &ptr);
                    *value = dry_value.clone();
                },
                &Mapping::Template { ref value, ref template } => {
                    if value.eq(dry_value) {
                        let template_obj = template.as_object().ok_or("template must be an object")?;
                        if let &mut Value::Object(ref mut wet_obj) = wet {
                            insert_template(wet_obj, &template_obj)?;
                        } else {
                            bail!("wet value must be an object");
                        }
                    }
                }
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

    use super::*;
    use ::schema::*;

    

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
        let files = btreemap!{
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

        let schema = Schema { files: files };

        let result = transform(dry, schema).unwrap().pop().unwrap();
        let expected = r##"{"parent":{"child":"{\"wet\":\"value\"}"}}"##;

        assert_eq!(expected, result.content);
    }
}


