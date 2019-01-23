use serde_json::Value;

use super::super::{scope::ScopedSchema, state::ValidationState};
use super::array::validate_as_array;

pub fn validate_as_stringlist(scope: &ScopedSchema, data: &Value) -> ValidationState {
    validate_as_array(scope, data)
}
