use std::collections::BTreeMap;

use errors::*;
use schema::{File, FileFormat, Location, Mapping, Property, PropertyDefinition, Schema};
use adaptor::{Adaptor, IniAdaptor, JsonAdaptor};
use template::Wildcard;

use serde_json::Value;
use serde_json::map::Entry as MapEntry;

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
pub fn transform_to_dry(config: Vec<Entry>, schema: &Schema) -> Result<Value> {
    let mut root = JsObject::new();
    let ordered = sort_files(&schema.files);
    let mut mapped = config
        .into_iter()
        .map(|e| (e.name, e.content))
        .collect::<BTreeMap<_, _>>();

    let mut wet_cache = BTreeMap::new();

    for (name, file) in ordered {
        let wet_content = match &file.location {
            &Location::Independent { .. } => mapped.remove(name).ok_or("file not found")?,
            &Location::Dependent {
                ref parent,
                ref location,
            } => {
                let parent_wet_content = wet_cache.get(parent).ok_or("parent not found")?;
                let value = follow_pointer(parent_wet_content, location.as_ref());
                let inner_content = value
                    .ok_or("value not found")?
                    .as_str()
                    .ok_or("value is not a string")?;
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

fn extract_value(wet: &Value, mappings: &[Mapping]) -> Result<Option<Value>> {
    let mut current: Option<Value> = None;

    for mapping in mappings {
        let value = match mapping {
            &Mapping::Direct(ref ptr) => {
                let value = follow_pointer(wet, &ptr).ok_or("direct mapping not found")?;
                Some(value)
            },
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

        let next = match (current, value) {
            (Some(c), Some(v)) => {
                if c.eq(v) {
                    Some(c)
                } else {
                    return Err("found non-matching values".into());
                }
            },
            (Some(c), None) => Some(c),
            (None, Some(v)) => Some(v.clone()),
            _ => None,
        };

        current = next;
    }

    Ok(current)
}

fn valid_type(def: &PropertyDefinition, val: &Value) -> bool {
    def.types.iter().any(|t| t.is_type(&val))
}

/// Recursively process properties, extracting wet values and inserting them
/// into the dry tree.
fn generate_dry_property(dry: &mut JsObject, wet: &Value, props: &[Property]) -> Result<()> {
    for prop in props.iter() {
        for (name, def) in prop.definition.iter() {
            let value = extract_value(wet, &def.mapping)?;

            if let Some(val) = value {
                if !valid_type(def, &val) {
                    return Err("selected value is not a valid type".into());
                }

                dry.insert(name.clone(), val.clone());
            } else {
                //TODO: generate a warning?
            }

            if let &mut Value::Object(ref mut inner) = dry.entry(name.as_ref()).or_insert(json!({}))
            {
                generate_dry_property(inner, wet, &def.properties)?;
            }
        }
    }

    Ok(())
}

/// Transform a dry configuration structure into the raw file content
/// for the configuration files defined in the `Schema`.
pub fn transform_to_wet(config: Value, schema: &Schema) -> Result<Vec<Entry>> {
    let ordered = sort_files(&schema.files);

    let dry = config.as_object().unwrap();

    let mut files = BTreeMap::new();
    for (name, file) in ordered.into_iter() {
        let wet = generate_wet_file(dry, &file.properties)?;

        match file.location {
            Location::Independent { .. } => {
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
            serialize(wet, &format).map(|c| Entry {
                name: name.to_string(),
                content: c,
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
            let adaptor = JsonAdaptor::new(false);
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
            let adaptor = JsonAdaptor::new(false);
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

    if find_wildcards(&root) {
        bail!("wildcard values found in output");
    }

    Ok(root)
}

/// Follow a JSON pointer, returning a reference to the refered `Value`.
fn follow_pointer<'a>(v: &'a Value, pointer: &str) -> Option<&'a Value> {
    let names = pointer.trim_left_matches('/').split('/');

    names.fold(Some(v), |state, name| match state {
        None => None,
        Some(json) => {
            match json {
                &Value::Object(ref obj) => obj.get(name),
                _ => None, //TODO: return an error or warning?
            }
        },
    })
}

/// Follow a JSON pointer, returning a mutable reference. If any property in
/// the pointer chain does not exist, a new `Object` will be inserted.
fn follow_pointer_mut<'a>(v: &'a mut Value, pointer: &str) -> &'a mut Value {
    let names = pointer.trim_left_matches('/').split('/');

    names.fold(v, |state, name| match state {
        &mut Value::Object(ref mut obj) => obj.entry(name).or_insert(json!({})),
        _ => panic!("invalid value type"), //TODO: report this error rather than panic
    })
}

/// Recursively search a JSON `Value` for any values that look like wildcards.
fn find_wildcards(v: &Value) -> bool {
    match v {
        &Value::Object(ref o) => o.values().any(find_wildcards),
        &Value::Array(ref a) => a.iter().any(find_wildcards),
        x => parse_wildcard(x).is_some(),
    }
}

/// Parse a `Value` for wildcards.
fn parse_wildcard(v: &Value) -> Option<Vec<Wildcard>> {
    v.as_str().and_then(::template::type_wildcards)
}

/// Check if a `Value` matches a slice of `Wildcard`s.
fn wildcard_matches(wilds: &[Wildcard], v: &Value) -> bool {
    wilds.iter().any(|w| w.matches(v))
}

/// Insert the specified template value into the wet JSON tree.
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
            new_value => {
                let new_wild = parse_wildcard(new_value);
                match tree.entry(key.clone()) {
                    MapEntry::Vacant(v) => {
                        v.insert(new_value.clone());
                    },
                    MapEntry::Occupied(mut o) => {
                        let old_value = o.insert(Value::Bool(false));
                        let old_wild = parse_wildcard(&old_value);
                        let insert = match (new_wild, old_wild) {
                            (Some(new), None) => {
                                if wildcard_matches(&new, &old_value) {
                                    Ok(old_value)
                                } else {
                                    Err(format!(
                                        "wildcard value '{}' does not match original value '{}'",
                                        new_value, old_value
                                    ))
                                }
                            },
                            (None, Some(old)) => {
                                if wildcard_matches(&old, new_value) {
                                    Ok(new_value.clone())
                                } else {
                                    Err(format!(
                                        "wildcard value '{}' does not match new value '{}'",
                                        old_value, new_value
                                    ))
                                }
                            },
                            (None, None) => Err(format!(
                                "cannot replace value '{}' with '{}'",
                                old_value, new_value
                            )),
                            (Some(new), Some(old)) => {
                                if new.eq(&old) {
                                    Ok(old_value)
                                } else {
                                    Err("wildcard values do not match".into())
                                }
                            },
                        };

                        o.insert(insert?);
                    },
                }
            },
        }
    }

    Ok(())
}

/// Applies a slice of mappings to a wet JSON `Value`.
fn apply_mappings(dry: &Value, wet: &mut Value, mappings: &[Mapping]) -> Result<()> {
    if dry.eq(&Value::Null) {
        return Ok(());
    }

    for mapping in mappings {
        match mapping {
            &Mapping::Direct(ref ptr) => {
                let mut value = follow_pointer_mut(wet, &ptr);
                *value = dry.clone();
            },
            &Mapping::Template {
                ref value,
                ref template,
            } => {
                if value.eq(dry) {
                    let template_obj = template.as_object().ok_or("template must be an object")?;
                    if let &mut Value::Object(ref mut wet_obj) = wet {
                        insert_template(wet_obj, &template_obj)?;
                    } else {
                        bail!("wet value must be an object");
                    }
                }
            },
        }
    }

    Ok(())
}

/// Recursively process dry JSON values and insert them into the wet JSON tree.
fn generate_wet_property(dry: &JsObject, wet: &mut Value, prop: &Property) -> Result<()> {
    for (name, definition) in prop.definition.iter() {
        let dry_value = dry.get(&*name).ok_or("dry value not found")?;

        if !definition.mapping.is_empty() && !valid_type(definition, dry_value) {
            return Err("selected value is not a valid type".into());
        }

        apply_mappings(dry_value, wet, &definition.mapping)?;

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

    use test::*;

    use serde_json::to_string;
    use serde_json::Value;

    fn parse_data<T, F>(data: &str, convert: F) -> (T, Value, Option<Value>)
    where
        F: FnOnce(&Value) -> T,
    {
        let lines = read_sections(data.as_bytes());
        let schema = lines[1].as_value().expect("Invalid JSON on line 2!");
        let tree = lines[2]
            .as_value()
            .map(|x| x.clone())
            .expect("Invalid JSON on line 3!");
        let value = lines[3].as_value().map(|x| x.clone());

        let parsed = convert(schema);
        (parsed, tree, value)
    }

    mod transform {
        use super::*;
        use serde_json::Value;

        fn parse_properties(data: &str) -> (Vec<Property>, Value, Option<Value>) {
            parse_data(data, |json| {
                json.as_array()
                    .expect("Properties must be an array")
                    .into_iter()
                    .map(|s| Property::from_json(s).unwrap())
                    .collect::<Vec<_>>()
            })
        }

        macro_rules! transform_bidi_gen {
            ($($name:ident),*) => ( $(
                #[test]
                fn $name() {
                    let file = include_str!(concat!("../tests/testdata/config/",
                                                            stringify!($name)));
                    let (props, dry, wet) = parse_properties(file);

                    let dry = dry.as_object().expect("Dry value must be an object");
                    let wet = wet.unwrap();
                    let result = generate_wet_file(dry, &props[..]).unwrap();
                    assert_eq!(wet, result);

                    let mut dry_result = JsObject::new();
                    generate_dry_property(&mut dry_result, &wet, &props[..]).unwrap();
                    assert_eq!(dry, &dry_result);
                }
            )* )
        }

        transform_bidi_gen!(bidi_1, bidi_2, bidi_3);

        macro_rules! transform_extract_gen {
            ($($name:ident),*) => ( $(
                #[test]
                fn $name() {
                    let file = include_str!(concat!("../tests/testdata/config/",
                                                            stringify!($name)));

                    let (props, wet, dry) = parse_properties(file);

                    let mut tree = JsObject::new();
                    let result = generate_dry_property(&mut tree, &wet, &props[..])
                        .ok().map(|_| Value::Object(tree));

                    assert_eq!(dry, result);
                }
            )* )
        }

        transform_extract_gen!(extract_1, extract_2, extract_3);
    }

    mod mapping {
        use super::*;
        use serde_json::Value;

        fn parse_mappings(data: &str) -> (Vec<Mapping>, Value, Option<Value>) {
            parse_data(data, |json| {
                json.as_array()
                    .expect("List of mappings required!")
                    .into_iter()
                    .map(|i| Mapping::from_json(i).expect("Invalid mapping format!"))
                    .collect::<Vec<_>>()
            })
        }

        macro_rules! mapping_bidi_gen {
            ($($name:ident),*) => ( $(
                #[test]
                fn $name() {
                    let file = include_str!(concat!("../tests/testdata/mapping/",
                                                            stringify!($name)));
                    let (mappings, tree, value) = parse_mappings(file);

                    let value = value.unwrap();
                    let mut wet = json!({});
                    apply_mappings(&value, &mut wet, mappings.as_slice())
                        .expect("Mapping application failed!");
                    assert_eq!(tree, wet);

                    let extracted = extract_value(&tree, mappings.as_slice())
                        .expect("Unable to extract value!");
                    assert_eq!(Some(value), extracted);
                }
            )* )
        }

        mapping_bidi_gen!(
            bidi_1,
            bidi_2,
            bidi_3,
            bidi_4,
            bidi_5,
            bidi_6 // bidi_7 TODO: clarify the 'degree' heuristic
        );

        macro_rules! mapping_gtv_gen {
            ($($name:ident),*) => ( $(
                #[test]
                fn $name() {
                    let file = include_str!(concat!("../tests/testdata/mapping/",
                                                            stringify!($name)));
                    let (mappings, tree, value) = parse_mappings(file);

                    let extracted = extract_value(&tree, mappings.as_slice()).ok().and_then(|x| x);
                    assert_eq!(value, extracted);
                }
            )* )
        }

        mapping_gtv_gen!(gtv_1, gtv_2, gtv_3);

        macro_rules! mapping_map_gen {
            ($($name:ident),*) => ( $(
                #[test]
                fn $name() {
                    let file = include_str!(concat!("../tests/testdata/mapping/",
                                                            stringify!($name)));
                    let (mappings, value, tree) = parse_mappings(file);

                    let mut wet = json!({});
                    let result = apply_mappings(&value, &mut wet, mappings.as_slice())
                        .ok().map(|_| wet);
                    assert_eq!(tree, result);
                }
            )* )
        }

        mapping_map_gen!(map_1, map_2, map_3);

        macro_rules! mapping_unmap_gen {
            ($($name:ident),*) => ( $(
                #[test]
                fn $name() {
                    let file = include_str!(concat!("../tests/testdata/mapping/",
                                                            stringify!($name)));
                    let (mappings, tree, value) = parse_mappings(file);

                    let extracted = extract_value(&tree, mappings.as_slice()).ok().and_then(|x| x);
                    assert_eq!(value, extracted);
                }
            )* )
        }

        mapping_unmap_gen!(
            //unmap_1, undefined does not exist in the serde JSON model
            unmap_2,
            unmap_3,
            unmap_4,
            unmap_5,
            unmap_6
        );
    }

    #[test]
    fn dry_to_wet_simple_direct() {
        let dry = json!({ "dry": "value" });
        let props = vec![
            Property {
                when: None,
                definition: btreemap!{
                    "dry".into() => PropertyDefinition {
                        types: vec![PropertyType::String],
                        properties: vec![],
                        mapping: vec![Mapping::Direct("/wet".into())],
                    },
                },
            },
        ];

        let expected = json!({ "wet": "value" });

        let result = generate_wet_file(dry.as_object().unwrap(), &props).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn dry_to_wet_dry_heirarchy_direct() {
        let dry = json!({ "parent": { "dry": "value" } });
        let props = vec![
            Property {
                when: None,
                definition: btreemap!{
                    "parent".into() => PropertyDefinition {
                        types: vec![],
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
            },
        ];

        let expected = json!({ "wet": "value" });

        let result = generate_wet_file(dry.as_object().unwrap(), &props).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn dry_to_wet_wet_heirarchy_direct() {
        let dry = json!({ "dry": "value" });
        let props = vec![
            Property {
                when: None,
                definition: btreemap!{
                    "dry".into() => PropertyDefinition {
                        types: vec![PropertyType::String],
                        properties: vec![],
                        mapping: vec![Mapping::Direct("/parent/wet".into())],
                    },
                },
            },
        ];

        let expected = json!({ "parent": { "wet": "value" } });

        let result = generate_wet_file(dry.as_object().unwrap(), &props).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn dry_to_wet_simple_template() {
        let dry = json!({ "template": "yes" });
        let template = json!({"parent": { "key": "value" } });
        let props = vec![
            Property {
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
            },
        ];

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

        let result = transform_to_wet(dry, &schema).unwrap().pop().unwrap();
        let expected = r##"{"parent":{"child":"{\"wet\":\"value\"}"}}"##;

        assert_eq!(expected, result.content);
    }

    #[test]
    fn wet_to_dry_simple_direct() {
        let wet = json!({ "wet": "value" });
        let props = vec![
            Property {
                when: None,
                definition: btreemap!{
                    "dry".into() => PropertyDefinition {
                        types: vec![PropertyType::String],
                        properties: vec![],
                        mapping: vec![Mapping::Direct("/wet".into())],
                    },
                },
            },
        ];

        let expected = json!({ "dry": "value" });
        let mut root = JsObject::new();
        generate_dry_property(&mut root, &wet, &props).unwrap();

        assert_eq!(expected, Value::Object(root));
    }

    #[test]
    fn wet_to_dry_dry_heirarchy_direct() {
        let wet = json!({ "wet": "value" });
        let props = vec![
            Property {
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
            },
        ];

        let expected = json!({ "parent": { "dry": "value" } });
        let mut root = JsObject::new();
        generate_dry_property(&mut root, &wet, &props).unwrap();

        assert_eq!(expected, Value::Object(root));
    }

    #[test]
    fn wet_to_dry_wet_heirarchy_direct() {
        let wet = json!({ "parent": { "wet": "value" } });
        let props = vec![
            Property {
                when: None,
                definition: btreemap!{
                    "dry".into() => PropertyDefinition {
                        types: vec![PropertyType::String],
                        properties: vec![],
                        mapping: vec![Mapping::Direct("/parent/wet".into())],
                    },
                },
            },
        ];

        let expected = json!({ "dry": "value" });
        let mut root = JsObject::new();
        generate_dry_property(&mut root, &wet, &props).unwrap();

        assert_eq!(expected, Value::Object(root));
    }

    #[test]
    fn wet_to_dry_simple_template() {
        let template = json!({"parent": { "key": "value" } });
        let props = vec![
            Property {
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
            },
        ];

        let mut dry = JsObject::new();
        generate_dry_property(&mut dry, &template, &props).unwrap();

        let expected = json!({"template": "yes"});
        assert_eq!(expected, Value::Object(dry));
    }

    #[test]
    fn wet_to_dry_dependent_json() {
        let inner = r##"{"child":{"wet":"value"}}"##;
        let wet = json!({ "parent": inner });
        let wet_content = to_string(&wet).unwrap();

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

        let schema = Schema { files: files };
        let entry = Entry {
            name: "independent".into(),
            content: wet_content,
        };

        let result = transform_to_dry(vec![entry], &schema).unwrap();
        let expected = json!({"key": "value"});
        assert_eq!(expected, result);
    }
}
