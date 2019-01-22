use serde_json::Value;

use super::super::{scope::ScopedSchema, state::ValidationState};

pub fn validate_as_stringlist(scope: &ScopedSchema, _data: &Value) -> ValidationState {
    ValidationState::new_with_error(scope.invalid_error("TODO"))
}
