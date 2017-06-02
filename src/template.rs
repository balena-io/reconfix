//! Templates

#![allow(missing_docs)]

use serde_json::Value;

pub enum Mapping {
    Direct(String),
    Template { value: Value, template: Value },
}
