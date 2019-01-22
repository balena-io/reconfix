use serde_json::Value;

use super::super::{scope::ScopedSchema, state::ValidationState};

pub fn validate_as_string(scope: &ScopedSchema, data: &Value) -> ValidationState {
    let string = match data.as_str() {
        Some(x) => x,
        None => return ValidationState::new_with_error(scope.invalid_error("type")),
    };

    let len = string.chars().count();
    let schema = scope.schema();
    let mut state = ValidationState::new();

    if let Some(min) = schema.min_length() {
        if len < min {
            state.push_error(scope.invalid_error("minLength"));
        }
    }

    if let Some(max) = schema.max_length() {
        if len > max {
            state.push_error(scope.invalid_error("maxLength"));
        }
    }

    if let Some(pattern) = schema.pattern() {
        use regex::Regex;

        if let Ok(regex) = Regex::new(pattern) {
            if !regex.is_match(string) {
                state.push_error(scope.invalid_error("pattern"));
            }
        } else {
            // TODO Should be handled on the deserialization level
            state.push_error(scope.invalid_error("pattern"));
        }
    }

    state
}
