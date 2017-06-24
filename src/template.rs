//! Templates

#![allow(missing_docs)]

use serde_json::Value;

pub enum Mapping {
    Direct(String),
    Template { value: Value, template: Value },
}

impl Mapping {
    pub fn from_json(v: &Value) -> ::schema::error::Result<Mapping> {
        // TODO
        Ok(
            Mapping::Template {
                value: Value::Bool(true),
                template: Value::Bool(false),
            }
        )
    }
}
