use serde_json::Value;

use super::super::{scope::ScopedSchema, state::ValidationState};

pub fn validate_as_file(scope: &ScopedSchema, _data: &Value) -> ValidationState {
    ValidationState::new_with_error(scope.error("type", "not implemented"))
}
