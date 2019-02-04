use serde_json::Value;

use crate::validator::{scope::ScopedSchema, state::ValidationState, types::validate_as_string};

pub fn validate_as_text(scope: &ScopedSchema, data: &Value) -> ValidationState {
    validate_as_string(scope, data)
}
