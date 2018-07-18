use super::error::*;
use json::Pointer as JsonPointer;
use schema::types::*;

use serde_json::{Number, Value};

type Map<K, V> = ::std::collections::BTreeMap<K, V>;

#[derive(Debug)]
pub struct Layer {
    pub values: Vec<(JsonPointer, Leaf)>,
}

impl Layer {
    pub fn new() -> Layer {
        Layer { values: Vec::new() }
    }

    pub fn single(ptr: &JsonPointer, value: Leaf) -> Layer {
        let mut layer = Layer::new();
        layer.values.push((ptr.clone(), value));
        layer
    }

    pub fn from_value(ptr: &JsonPointer, value: &Value) -> Layer {
        let mut layer = Layer::new();
        for (ptr, literal) in produce_literals(ptr.clone(), value) {
            layer.values.push((ptr, Leaf::Literal(literal)));
        }
        layer
    }

    pub fn add_single(&mut self, ptr: &JsonPointer, value: Leaf) {
        self.values.push((ptr.clone(), value));
    }

    pub fn add_many(&mut self, ptr: &JsonPointer, value: &Value) {
        for (ptr, literal) in produce_literals(ptr.clone(), value) {
            self.values.push((ptr, Leaf::Literal(literal)));
        }
    }
}

fn produce_literals(ptr: JsonPointer, value: &Value) -> Vec<(JsonPointer, Literal)> {
    match *value {
        Value::Null => vec![],
        Value::String(ref s) => vec![(ptr, Literal::String(s.to_string()))],
        Value::Number(ref n) => {
            match convert_number(n) {
                Some(l) => vec![(ptr, l)],
                None => vec![],
            }
        },
        Value::Bool(ref b) => vec![(ptr, Literal::Bool(*b))],
        Value::Object(ref o) => {
            o.iter()
                .flat_map(|(name, value)| produce_literals(ptr.extend(name.as_ref()), value))
                .collect()
        },
        Value::Array(ref a) => {
            a.iter()
                .enumerate()
                .flat_map(|(index, value)| produce_literals(ptr.extend(index.to_string()), value))
                .collect()
        },
    }
}

fn convert_number(num: &Number) -> Option<Literal> {
    if let Some(u) = num.as_u64() {
        return Some(Literal::Unsigned(u));
    }
    if let Some(i) = num.as_i64() {
        return Some(Literal::Signed(i));
    }
    if let Some(f) = num.as_f64() {
        return Some(Literal::Float(f));
    }

    None
}

#[derive(Debug)]
pub enum Leaf {
    Literal(Literal),
    Schema(Schema),
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Float(f64),
    Unsigned(u64),
    Signed(i64),
    Bool(bool),
}
