
use std::cmp::Ordering;
use std::collections::btree_map::Entry;

use ::transform::types::*;
use ::schema::types::{Schema, ObjectSchema};
use ::json::{Pointer, Entry as PointerEntry};
use super::Mapper;
use super::error::*;
use super::types::*;

use itertools::Itertools;
use serde_json::{Value, Number};
use valico::json_schema;

type Map<K, V> = ::std::collections::BTreeMap<K, V>;

pub struct DefaultMapper;

impl Mapper for DefaultMapper {
    fn forward_map(&self, dry: &Value, transforms: &[Transform]) -> Result<Map<Target, Value>> {
        let mut transforms = transforms
            .iter()
            .map(|t| apply_tranform_forward(dry, t).map(|l| (t, l)))
            .collect::<Result<Vec<_>>>()?;

        transforms.sort_unstable_by(|a, b| a.0.target.cmp(&b.0.target));

        let wet = transforms.into_iter()
            .group_by(|t| &t.0.target)
            .into_iter()
            .map(|(key, group)| {
                let layers = group
                    .map(|(_, layer)| layer)
                    .collect::<Vec<_>>();
                let value = flatten_layers(layers)?;
                Ok((key.clone(), value))
            })
            .collect::<Result<Map<_, _>>>()?;

        Ok(wet)
    }

    fn reverse_map(&self, wet: &Map<Target, Value>, transforms: &[Transform]) -> Result<Value> {
        let layers = transforms
            .iter()
            .map(|t| {
                let wet_document = wet.get(&t.target)
                    .ok_or_else(|| "unable to find target")?;
                let layer = apply_transform_reverse(wet_document, t)?;
                Ok(layer)
            })
            .collect::<Result<Vec<_>>>()?;
        
        let dry = flatten_layers(layers)?;
        Ok(dry)
    }
}

fn apply_tranform_forward(dry: &Value, transform: &Transform) -> Result<Layer> {
    let inputs = transform.source.select_values(dry);

    if inputs.is_empty() {
        bail!("no values found on required transform");
    }

    let mut layer = Layer::new();

    for (index, input) in inputs.iter().enumerate() {
        let dry_pointer = transform.source.get_pointer_for_index(index as u64)
            .chain_err(|| "unable to generate dry JSON pointer")?;

        let wet_pointer = transform.destination.get_pointer(dry, &dry_pointer)
            .chain_err(|| "unable to generate wet JSON pointer")?;

        let mut found = false;
        for element in &transform.map {
            match *element {
                Case::Identity => {
                    layer.add_many(&wet_pointer, input);
                },
                Case::Template { ref dry, ref template } if dry.eq(*input) => {
                    layer.add_single(&wet_pointer, Leaf::Schema(template.clone()));
                },
                Case::Value { ref dry, ref wet } if dry.eq(*input) => {
                    layer.add_many(&wet_pointer, wet);
                },
                _ => continue,
            }

            found = true;
            break;
        }

        if !found {
            bail!("unable to match dry value '{}'", input);
        }
    }

    Ok(layer)
}

fn apply_transform_reverse(wet: &Value, transform: &Transform) -> Result<Layer> {
    let matches = transform.destination.get_match_matrix(&wet);
    let mut layer = Layer::new();

    for (idx, match_set) in matches.iter().enumerate() {
        let dry_pointer = transform.source.get_pointer_for_index(idx as u64)
            .chain_err(|| "unable to resolve dry JSON pointer")?;
        let parameters = match_set.apply_matches(&dry_pointer)
            .chain_err(|| "unable to resolve path parameters")?;

        for (ptr, val) in parameters {
            layer.add_many(&ptr, &val);
        }

        let wet_pointer = transform.destination.get_match_pointer(match_set)
            .chain_err(|| "unable to generate wet JSON pointer")?;
        let output = match wet_pointer.search(wet) {
            Some(v) => v,
            None => bail!("unable to find value at transform destination '{}'", wet_pointer),
        };

        let mut found = false;
        for element in &transform.map {
            match *element {
                Case::Identity => {
                    layer.add_many(&dry_pointer, output);
                },
                Case::Value { ref dry, ref wet } if wet == output => {
                    layer.add_many(&dry_pointer, dry);
                },
                Case::Template { ref dry, ref template } => {
                    match validate(output, template)? {
                        true => layer.add_many(&dry_pointer, dry),
                        false => continue,
                    }
                },
                _ => continue,
            }

            found = false;
            break;
        }

        if !found {
            bail!("unable to match wet value at '{}'", wet_pointer);
        }
    }

    Ok(layer)
}

fn flatten_layers(layers: Vec<Layer>) -> Result<Value> {
    let (literals, schemas) = seperate(layers);
    let schema = normalize_schemas(schemas);
    let value = flatten_literals(literals)?;

    if validate(&value, &schema)? {
        bail!("JSON failed validation");
    }
        
    Ok(value)
}

fn flatten_literals(literals: Vec<(Pointer, Literal)>) -> Result<Value> {
    let mut root = json!({});

    for (key, value) in literals {
        let entry = key.entry(&mut root)
            .chain_err(|| "unable to search navigate JSON")?;
        match entry {
            PointerEntry::Vacant(e) => {
                let converted = match value {
                    Literal::Bool(b) => Value::Bool(b),
                    Literal::Number(n) => {
                        Value::Number(Number::from_f64(n).unwrap())
                    },
                    Literal::String(s) => Value::String(s),
                };

                e.insert(converted);
            },
            PointerEntry::Occupied(_) => unimplemented!(),
        }
    }

    Ok(root)
}

fn seperate(layers: Vec<Layer>) -> (Vec<(Pointer, Literal)>, Vec<(Pointer, Schema)>) {
    let mut literals = Vec::new();
    let mut schemas = Vec::new();
    for layer in layers {
        for (ptr, leaf) in layer.values {
            match leaf {
                Leaf::Literal(l) => literals.push((ptr, l)),
                Leaf::Schema(s) => schemas.push((ptr, s)),
            }
        }
    }

    (literals, schemas)
}

fn normalize_schemas(schemas: Vec<(Pointer, Schema)>) -> Schema {

    let combined = schemas.into_iter().fold(None, |root, (ptr, schema)| {
        let mut parts: Vec<String> = ptr.into();
        parts.reverse();

        match (root, parts.pop()) {
            (None, Some(next)) => {
                let mut obj: ObjectSchema = Default::default();
                normalize_schema(&mut obj, next, parts, schema);
                Some(Schema::Object(Box::new(obj)))
            },
            (None, None) => {
                Some(schema)
            },
            (Some(Schema::Object(mut obj)), Some(next)) => {
                normalize_schema(&mut obj, next, parts, schema);
                Some(Schema::Object(obj))
            },
            _ => unimplemented!(),
        }
    });

    match combined {
        Some(x) => x,
        None => Schema::Boolean(true),
    }
}

fn normalize_schema(
    root: &mut ObjectSchema, 
    path: String, 
    mut remaining: Vec<String>, 
    schema: Schema) 
{
    let entry = root.properties.entry(path);
    let next = remaining.pop();
    match (entry, next) {
        (Entry::Occupied(mut e), Some(next)) => {
            match e.get_mut() {
                &mut Schema::Object(ref mut o) => {
                    normalize_schema(&mut **o, next, remaining, schema)
                },
                _ => unimplemented!(),
            }
        },
        (Entry::Vacant(e), Some(next)) => {
            let mut obj: ObjectSchema = Default::default();
            normalize_schema(&mut obj, next, remaining, schema);
            e.insert(Schema::Object(Box::new(obj)));
        },
        (Entry::Vacant(e), None) => {
            e.insert(schema);
        },
        _ => unimplemented!(),
    }
}

fn validate(value: &Value, schema: &Schema) -> Result<bool> {
    let schema_value = ::serde_json::to_value(schema)
        .chain_err(|| "unable to serialize schema fragment")?;
    let mut scope = json_schema::Scope::new();
    let schema = match scope.compile_and_return(schema_value, true) {
        Ok(s) => s,
        Err(_) => bail!("unable to compile schema fragment"),
    };

    let state = schema.validate(&value);

    Ok(!state.errors.is_empty())
}

//     let combined = layers
//         .flat_map(|layer| layer.values)
//         .collect();

//     // combined.sort_by(|(_, a), (_, b)| {
//     //     match (a, b) {
//     //         (Leaf::Literal(_), Leaf::Schema(_)) => Ordering::Less,
//     //         (Leaf::Schema(_), Leaf::Literal(_)) => Ordering::Greater,
//     //         _ => Ordering::Equal,
//     //     }
//     // })
// }

// fn valid_literal(literal: Literal, Value) -> bool {

// }
