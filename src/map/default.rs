use std::cmp::Ordering;
use std::collections::btree_map::Entry;
use std::mem;

use super::error::*;
use super::types::*;
use super::Mapper;
use json::{Entry as PointerEntry, Pointer};
use schema::types::{ObjectSchema, Schema};
use transform::types::*;

use itertools::Itertools;
use serde_json::{Number, Value};
use valico::json_schema;

type Map<K, V> = ::std::collections::BTreeMap<K, V>;

pub struct DefaultMapper;

impl Mapper for DefaultMapper {
    fn forward_map(&self, dry: &Value, transforms: &[Transform]) -> Result<Map<Target, Value>> {
        debug!("processing {} transforms...", transforms.len());

        let mut transforms = transforms
            .iter()
            .map(|t| apply_tranform_forward(dry, t).map(|l| (t, l)))
            .collect::<Result<Vec<_>>>()?;

        transforms.sort_unstable_by(|a, b| a.0.target.cmp(&b.0.target));

        let wet = transforms
            .into_iter()
            .group_by(|t| &t.0.target)
            .into_iter()
            .map(|(key, group)| {
                let layers = group.map(|(_, layer)| layer).collect::<Vec<_>>();
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
                let wet_document = wet.get(&t.target).ok_or_else(|| "unable to find target")?;
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

    let mut layer = Layer::new();

    for (index, input) in inputs.iter().enumerate() {
        let dry_pointer = transform
            .source
            .get_pointer_for_index(index as u64)
            .chain_err(|| "unable to generate dry JSON pointer")?;

        debug!("dry pointer: '{}'", dry_pointer);

        let wet_pointer = transform
            .destination
            .get_pointer(dry, &dry_pointer)
            .chain_err(|| "unable to generate wet JSON pointer")?;

        debug!("wet pointer: '{}'", wet_pointer);

        let mut found = false;
        for element in &transform.map {
            match *element {
                Case::Identity => {
                    layer.add_many(&wet_pointer, input);
                },
                Case::Stringify => {
                    let string = match stringify(input) {
                        Ok(s) => s,
                        Err(_) => continue,
                    };
                    layer.add_many(&wet_pointer, &Value::String(string));
                },
                Case::Test { ref dry, ref test } => {
                    let pass = match *dry {
                        Some(ref d) => d.eq(*input),
                        None => true,
                    };

                    if !pass {
                        continue;
                    }

                    for &(ref dest, ref value) in test.literals.iter() {
                        let ptr = wet_pointer.extend_all(dest);
                        debug!("adding literal '{}', destination '{}'", value, ptr);
                        layer.add_many(&ptr, &value);
                    }
                    layer.add_single(&wet_pointer, Leaf::Schema(test.schema.clone()));
                },
            }

            found = true;
            break;
        }

        if !found {
            bail!("unable to match dry value '{}'", input);
        }
    }

    debug!("layer: {:?}", layer);

    Ok(layer)
}

fn apply_transform_reverse(wet: &Value, transform: &Transform) -> Result<Layer> {
    let matches = transform.destination.get_match_matrix(&wet);
    let mut layer = Layer::new();

    for (idx, match_set) in matches.iter().enumerate() {
        let dry_pointer = transform
            .source
            .get_pointer_for_index(idx as u64)
            .chain_err(|| "unable to resolve dry JSON pointer")?;
        let parameters = match_set
            .apply_matches(&dry_pointer)
            .chain_err(|| "unable to resolve path parameters")?;

        debug!("dry pointer: '{}'", dry_pointer);

        for (ptr, val) in parameters {
            layer.add_many(&ptr, &val);
        }

        let wet_pointer = transform
            .destination
            .get_match_pointer(match_set)
            .chain_err(|| "unable to generate wet JSON pointer")?;
        let output = match wet_pointer.search(wet) {
            Some(v) => v,
            None => continue,
        };

        debug!("wet pointer: '{}'", wet_pointer);
        debug!("wet value: '{:?}'", output);

        let mut found = false;
        for case in &transform.map {
            match *case {
                Case::Identity => {
                    layer.add_many(&dry_pointer, output);
                },
                Case::Stringify => {
                    let string = match *output {
                        Value::String(ref s) => s.as_ref(),
                        _ => continue,
                    };
                    layer.add_many(&dry_pointer, &unstringify(string));
                },
                Case::Test { ref dry, ref test } => {
                    debug!("test literals: {:?}", test.literals);
                    let lit_pass = test.literals.iter().fold(
                        true,
                        |prev, &(ref dest, ref value)| {
                            debug!("test value: {:?}", value);
                            let local_ptr = Pointer::from(dest.clone());
                            debug!("testing against wet path: '{}'", local_ptr);
                            let test_result = match local_ptr.search(output) {
                                Some(v) => v.eq(value),
                                None => false,
                            };

                            prev && test_result
                        },
                    );

                    match lit_pass && validate(output, &test.schema)? {
                        true => {
                            if let Some(ref dry_value) = *dry {
                                layer.add_many(&dry_pointer, dry_value);
                            }
                        },
                        false => continue,
                    }
                },
            }

            found = true;
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

    if !validate(&value, &schema)? {
        bail!("JSON failed validation");
    }

    Ok(value)
}

fn flatten_literals(literals: Vec<(Pointer, Literal)>) -> Result<Value> {
    let mut root = literals
        .iter()
        .find(|&&(ref ptr, _)| ptr.is_root())
        .and_then(|&(_, ref lit)| literal_to_value(lit.clone()))
        .unwrap_or_else(|| json!({}));

    for (key, value) in literals {
        let entry = key
            .entry(&mut root)
            .chain_err(|| "unable to search navigate JSON")?;
        match entry {
            PointerEntry::Vacant(e) => {
                let converted =
                    literal_to_value(value).ok_or_else(|| "invalid floating point value")?;

                e.insert(converted);
            },
            PointerEntry::Occupied(_) => unimplemented!(),
        }
    }

    Ok(root)
}

fn literal_to_value(literal: Literal) -> Option<Value> {
    let value = match literal {
        Literal::Bool(b) => Value::Bool(b),
        Literal::Unsigned(u) => Value::Number(u.into()),
        Literal::Signed(i) => Value::Number(i.into()),
        Literal::Float(f) => {
            match Number::from_f64(f) {
                Some(num) => Value::Number(num),
                None => return None,
            }
        },
        Literal::String(s) => Value::String(s),
    };

    Some(value)
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
            (None, None) => Some(schema),
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
    schema: Schema,
) {
    let mut properties = mem::replace(&mut root.properties, None).unwrap_or_else(|| Map::new());
    let mut required = mem::replace(&mut root.required, None).unwrap_or_else(|| Vec::new());
    {
        let entry = properties.entry(path.clone());
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

    required.push(path);
    root.required = Some(required);
    root.properties = Some(properties);
}

fn validate(value: &Value, schema: &Schema) -> Result<bool> {
    debug!("test json: {:?}", value);
    debug!("test schema: {:?}", schema);

    let schema_value =
        ::serde_json::to_value(schema).chain_err(|| "unable to serialize schema fragment")?;
    if let Value::Bool(ref b) = schema_value {
        return Ok(*b);
    }

    let mut scope = json_schema::Scope::new();
    let schema = match scope.compile_and_return(schema_value, true) {
        Ok(s) => s,
        Err(e) => {
            bail!("unable to compile schema fragment: {:?}", e);
        },
    };

    let state = schema.validate(&value);
    for err in state.errors.iter() {
        debug!("validation error: {:?}", err);
    }
    Ok(state.errors.is_empty())
}

fn stringify(value: &Value) -> Result<String> {
    let out = match *value {
        Value::Bool(true) => "true".to_string(),
        Value::Bool(false) => "false".to_string(),
        Value::Number(ref n) => format!("{}", n),
        Value::String(ref s) => s.to_string(),
        _ => bail!("invalid stringify value"),
    };

    Ok(out)
}

fn unstringify(value: &str) -> Value {
    if let Ok(b) = value.parse() {
        return Value::Bool(b);
    }

    if let Ok(n) = value.parse::<u64>() {
        return Value::Number(n.into());
    }

    if let Ok(n) = value.parse::<i64>() {
        return Value::Number(n.into());
    }

    if let Ok(n) = value.parse::<f64>() {
        if let Some(n) = Number::from_f64(n) {
            return Value::Number(n);
        }
    }

    Value::String(value.into())
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
