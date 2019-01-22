use serde_json::Value;

use super::{
    super::{scope::ScopedSchema, state::ValidationState},
    string::validate_as_string,
};

pub fn validate_as_password(scope: &ScopedSchema, data: &Value) -> ValidationState {
    validate_as_string(scope, data)
}
