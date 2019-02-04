use serde_json::Value;

use crate::validator::{scope::ScopedSchema, state::ValidationState, types::validate_as_array};

pub fn validate_as_stringlist(scope: &ScopedSchema, data: &Value) -> ValidationState {
    validate_as_array(scope, data)
}
