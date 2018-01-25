use std::collections::BTreeMap;
use std::cmp::Ordering;

use error::*;
use schema::{File, Location, Mapping, Property, PropertyDefinition, Schema};
use common::{deserialize, serialize};
use json::Pointer;
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
    pub content: Value,
}

/// Transform the raw file data into a dry JSON structure.
/// A matching `Entry` is requird for every `Independent` file
/// in the `Schema`.
pub fn transform_to_dry(config: Vec<Entry>, schema: &Schema) -> Result<Value> {
    let ordered = sort_files(&schema.files);
    let mut mapped = config
        .into_iter()
        .map(|e| (e.name, e.content))
        .collect::<BTreeMap<_, _>>();

    let mut dry_buffer = Vec::new();
    let mut wet_cache = BTreeMap::new();
    let prefix = Pointer::new();

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

                let raw = inner_content.to_string();
                deserialize(raw.as_bytes(), &file.format)?
            }
        };

        generate_dry_values(
            &prefix,
            &wet_content,
            file.properties.as_slice(),
            &mut dry_buffer,
        )?;
        wet_cache.insert(name.to_string(), wet_content);
    }

    generate_dry_object(dry_buffer)
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
    let mut candidates = mappings
        .iter()
        .filter_map(|mapping| match mapping {
            &Mapping::Direct(ref ptr) => follow_pointer(wet, &ptr).map(|val| (None, val)),
            &Mapping::Template {
                ref value,
                ref template,
            } => {
                if ::template::matches(wet, template) {
                    Some((Some(::template::degree(template)), value))
                } else {
                    None
                }
            }
        })
        .collect::<Vec<_>>();

    candidates.sort_unstable_by(|left, right| match (left.0, right.0) {
        (Some(x), Some(y)) => y.cmp(&x),
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    });

    candidates
        .into_iter()
        .fold(Ok(None), |state, candidate| match state {
            Ok(None) => Ok(Some(candidate)),
            Err(e) => Err(e),
            Ok(Some(current)) => {
                match (current, candidate) {
                    ((None, left), (None, right)) => {
                        if left.eq(right) {
                            Ok(Some((None, left)))
                        } else {
                            Err("conflicting direct mapped values".into())
                        }
                    }
                    ((Some(degree_x), x), (Some(degree_y), y)) => match degree_x.cmp(&degree_y) {
                        Ordering::Less => Ok(Some((Some(degree_y), y))),
                        Ordering::Greater => Ok(Some((Some(degree_x), x))),
                        Ordering::Equal => {
                            if x == y {
                                Ok(Some((Some(degree_x), x)))
                            } else {
                                Err("cannot resolve templates with differing values and equal degree".into())
                            }
                        }
                    },
                    ((Some(_), _), x) => Ok(Some(x)),
                    (x, (Some(_), _)) => Ok(Some(x)),
                }
            }
        })
        .map(|o| o.map(|(_, value)| value.clone()))
}

fn generate_conditions(prefix: &Pointer, when: &JsObject, conditions: &mut Vec<(Pointer, Value)>) {
    for (key, value) in when {
        let ptr = prefix.extend(key.as_str());
        match value {
            &Value::Object(ref o) => generate_conditions(&ptr, &o, conditions),
            x => conditions.push((ptr, x.clone())),
        }
    }
}

fn generate_all_conditions(prop: &Property) -> Result<Vec<(Pointer, Value)>> {
    let mut buffer = Vec::new();
    if let Some(ref when) = prop.when {
        let when_obj = when.as_object().ok_or_else(|| "when must be an object")?;
        generate_conditions(&Pointer::new(), when_obj, &mut buffer);
    }

    Ok(buffer)
}

fn condition_match(conditions: &[(Pointer, Value)], values: &[(Pointer, Value)]) -> bool {
    for &(ref cond_ptr, ref cond_val) in conditions.iter() {
        let result = match values.iter().find(|&&(ref ptr, _)| ptr == cond_ptr) {
            Some(&(_, ref val)) => {
                debug!("{}: expected '{}', actual '{}'", cond_ptr, cond_val, val);
                val == cond_val
            }
            None => {
                debug!("{}: expected '{}' but value not found", cond_ptr, cond_val);
                false
            }
        };

        if !result {
            return false;
        }
    }

    true
}

fn condition_match_tree(conditions: &[(Pointer, Value)], tree: &JsObject) -> bool {
    let tree = Value::Object(tree.clone());
    for &(ref ptr, ref val) in conditions {
        let result = match ptr.search(&tree) {
            Some(v) => {
                debug!("{}: expected '{}', actual '{}'", ptr, val, v);
                v == val
            }
            None => {
                debug!("{}: expected '{}' but value not found", ptr, val);
                false
            }
        };

        if !result {
            return false;
        }
    }

    true
}

fn valid_type(def: &PropertyDefinition, val: &Value) -> bool {
    def.types.iter().any(|t| t.is_type(&val))
}

fn generate_dry_object(entries: Vec<(Pointer, Value)>) -> Result<Value> {
    let mut tree = json!({});
    for (ptr, val) in entries {
        match ptr.entry(&mut tree)? {
            ::json::Entry::Vacant(v) => {
                v.insert(val);
            }
            ::json::Entry::Occupied(_) => bail!("key '{}' is alread occupied with a value", ptr),
        }
    }

    Ok(tree)
}

/// Recursively process properties, extracting wet values and inserting them
/// into the dry tree.
fn generate_dry_values(
    root: &Pointer,
    wet: &Value,
    props: &[Property],
    dry: &mut Vec<(Pointer, Value)>,
) -> Result<()> {
    for prop in props.iter() {
        let conditions = generate_all_conditions(prop)?;
        if !condition_match(conditions.as_slice(), dry) {
            debug!("condition did not match");
            continue;
        }

        for (name, def) in prop.definition.iter() {
            let ptr = root.extend(name.as_str());

            if !def.mapping.is_empty() {
                let value = extract_value(wet, &def.mapping)?;

                if let Some(val) = value {
                    if !valid_type(def, &val) {
                        bail!("value '{}' is not a valid type for '{}'", val, ptr);
                    }

                    dry.push((ptr.clone(), val.clone()));
                } else if !def.optional {
                    return Err(
                        format!("no valid mapping found for required property '{}'", ptr).into(),
                    );
                }
            }

            generate_dry_values(&ptr, wet, &def.properties, dry)?;
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
            }
            Location::Dependent {
                ref parent,
                ref location,
            } => {
                let mut buffer = Vec::new();
                serialize(wet, &file.format, false, &mut buffer)?;
                let entry = files.get_mut(parent).ok_or("parent file not found")?;
                let value = follow_pointer_mut(&mut entry.1, location);
                let serialized =
                    String::from_utf8(buffer).chain_err(|| "invalid serializer output")?;
                *value = Value::String(serialized);
            }
        }
    }

    let output = files
        .into_iter()
        .map(|(name, (format, wet))| Entry {
            name: name.to_string(),
            content: wet,
        })
        .collect::<Vec<_>>();

    Ok(output)
}

/// Generate a wet JSON object.
fn generate_wet_file(dry: &JsObject, props: &[Property]) -> Result<Value> {
    let mut root = Value::Object(JsObject::new());

    generate_wet_properties(dry, dry, &mut root, props)?;

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
        }
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
            }
            new_value => {
                let new_wild = parse_wildcard(new_value);
                match tree.entry(key.clone()) {
                    MapEntry::Vacant(v) => {
                        v.insert(new_value.clone());
                    }
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
                            }
                            (None, Some(old)) => {
                                if wildcard_matches(&old, new_value) {
                                    Ok(new_value.clone())
                                } else {
                                    Err(format!(
                                        "wildcard value '{}' does not match new value '{}'",
                                        old_value, new_value
                                    ))
                                }
                            }
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
                            }
                        };

                        o.insert(insert?);
                    }
                }
            }
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
                debug!("inserting '{}' at '{}'", dry, ptr);
                let value = follow_pointer_mut(wet, &ptr);
                *value = dry.clone();
            }
            &Mapping::Template {
                ref value,
                ref template,
            } => {
                if value.eq(dry) {
                    debug!("matched template with value '{}'", value);
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

    Ok(())
}

/// Recursively process dry JSON values and insert them into the wet JSON tree.
fn generate_wet_properties(
    root: &JsObject,
    subtree: &JsObject,
    wet: &mut Value,
    props: &[Property],
) -> Result<()> {
    for prop in props.iter() {
        let conditions = generate_all_conditions(prop)?;
        if !condition_match_tree(conditions.as_slice(), root) {
            continue;
        }

        for (name, definition) in prop.definition.iter() {
            let dry_value = subtree.get(&*name);

            if let Some(val) = dry_value {
                if !definition.mapping.is_empty() && !valid_type(definition, &val) {
                    bail!("value '{}' is not a valid type for '{}'", val, name);
                }

                apply_mappings(val, wet, &definition.mapping)?;

                if let Some(sub) = val.as_object() {
                    generate_wet_properties(root, sub, wet, definition.properties.as_slice())?;
                }
            } else if !definition.optional {
                bail!("no valid mapping found for required property '{}'", name);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    extern crate serde_json;

    use super::*;
    use common::*;
    use schema::*;

    use test::*;

    use serde_json::to_string;
    use serde_json::Value;

    fn generate_dry(wet: &Value, props: &[Property]) -> Option<Value> {
        let mut buffer = Vec::new();
        let prefix = Pointer::new();

        match generate_dry_values(&prefix, wet, props, &mut buffer) {
            Err(e) => return None,
            _ => (),
        }

        generate_dry_object(buffer).ok()
    }

    fn generate_dry_obj(wet: &Value, props: &[Property]) -> Option<JsObject> {
        match generate_dry(wet, props) {
            Some(Value::Object(o)) => Some(o),
            _ => None,
        }
    }

    mod transform {
        use super::*;
        use serde_json::Value;

        fn parse_properties(data: &str) -> (String, Vec<Property>, Value, Option<Value>) {
            parse_test_data(data, |json| {
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
                    let (_, props, dry, wet) = parse_properties(file);

                    let dry = dry.as_object().expect("Dry value must be an object");
                    let wet = wet.unwrap();
                    let result = generate_wet_file(dry, &props[..]).unwrap();
                    assert_eq!(wet, result);

                    let dry_result = generate_dry_obj(&wet, props.as_slice()).unwrap();

                    assert_eq!(dry, &dry_result);
                }
            )* )
        }

        transform_bidi_gen!(bidi_1, bidi_2, bidi_3, bidi_4, bidi_5);

        macro_rules! transform_extract_gen {
            ($($name:ident),*) => ( $(
                #[test]
                fn $name() {
                    let file = include_str!(concat!("../tests/testdata/config/",
                                                            stringify!($name)));

                    let (_, props, wet, dry) = parse_properties(file);

                    let result = generate_dry(&wet, props.as_slice());

                    assert_eq!(dry, result);
                }
            )* )
        }

        transform_extract_gen!(extract_1, extract_2, extract_3);
    }

    mod mapping {
        use super::*;
        use serde_json::Value;

        fn parse_mappings(data: &str) -> (String, Vec<Mapping>, Value, Option<Value>) {
            parse_test_data(data, |json| {
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
                    let (_, mappings, tree, value) = parse_mappings(file);

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

        mapping_bidi_gen!(bidi_1, bidi_2, bidi_3, bidi_4, bidi_5, bidi_6, bidi_7);

        macro_rules! mapping_gtv_gen {
            ($($name:ident),*) => ( $(
                #[test]
                fn $name() {
                    let file = include_str!(concat!("../tests/testdata/mapping/",
                                                            stringify!($name)));
                    let (msg, mappings, tree, value) = parse_mappings(file);

                    let extracted = extract_value(&tree, mappings.as_slice()).ok().and_then(|x| x);
                    assert_eq!(value, extracted, "{}", msg);
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
                    let (_, mappings, value, tree) = parse_mappings(file);

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
                    let (_, mappings, tree, value) = parse_mappings(file);

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
            unmap_6,
            unmap_7
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
                        optional: false,
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
                        optional: false,
                        properties: vec![Property {
                            when: None,
                            definition: btreemap!{
                                "dry".into() => PropertyDefinition {
                                    types: vec![PropertyType::String],
                                    mapping: vec![Mapping::Direct("/wet".into())],
                                    properties: vec![],
                                    optional: false,
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
                        optional: false,
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
                        optional: false,
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
                location: Location::Independent(FileNode {
                    partition: Partition::new(0),
                    path: vec![],
                }),
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
                            optional: false,
                        }
                    },
                }],
            }
        };

        let schema = Schema { files: files };

        let result = transform_to_wet(dry, &schema).unwrap().pop().unwrap();
        let expected = json!({"parent":{"child":"{\"wet\":\"value\"}"}});

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
                        optional: false,
                    },
                },
            },
        ];

        let expected = json!({ "dry": "value" });
        let root = generate_dry(&wet, props.as_slice()).unwrap();

        assert_eq!(expected, root);
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
                        optional: false,
                        properties: vec![Property {
                            when: None,
                            definition: btreemap!{
                                "dry".into() => PropertyDefinition {
                                    types: vec![PropertyType::String],
                                    mapping: vec![Mapping::Direct("/wet".into())],
                                    properties: vec![],
                                    optional: false,
                                }
                            }
                        }],
                    },
                },
            },
        ];

        let expected = json!({ "parent": { "dry": "value" } });
        let root = generate_dry(&wet, props.as_slice()).unwrap();

        assert_eq!(expected, root);
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
                        optional: false,
                    },
                },
            },
        ];

        let expected = json!({ "dry": "value" });
        let root = generate_dry(&wet, props.as_slice()).unwrap();

        assert_eq!(expected, root);
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
                        optional: false,
                        mapping: vec![Mapping::Template {
                            value: json!("yes"),
                            template: template.clone(),
                        }],
                    },
                },
            },
        ];

        let root = generate_dry(&template, props.as_slice()).unwrap();

        let expected = json!({"template": "yes"});
        assert_eq!(expected, root);
    }

    #[test]
    fn wet_to_dry_dependent_json() {
        let inner = r##"{"child":{"wet":"value"}}"##;
        let wet = json!({ "parent": inner });

        let files = btreemap!{
            "independent".into() => File {
                format: FileFormat::Json,
                fileset: false,
                location: Location::Independent(FileNode {
                    partition: Partition::new(0),
                    path: vec![],
                }),
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
                            optional: false,
                        },
                    },
                }],
            },
        };

        let schema = Schema { files: files };
        let entry = Entry {
            name: "independent".into(),
            content: wet,
        };

        let result = transform_to_dry(vec![entry], &schema).unwrap();
        let expected = json!({"key": "value"});
        assert_eq!(expected, result);
    }
}
