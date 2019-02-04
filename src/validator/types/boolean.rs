use serde_json::Value;

use crate::validator::{scope::ScopedSchema, state::ValidationState};

pub fn validate_as_boolean(scope: &ScopedSchema, data: &Value) -> ValidationState {
    if !data.is_boolean() {
        ValidationState::new_with_error(scope.error("type", "expected 'boolean'"))
    } else {
        ValidationState::new()
    }
}
