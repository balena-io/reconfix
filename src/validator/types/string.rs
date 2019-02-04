use serde_json::Value;

use crate::validator::{scope::ScopedSchema, state::ValidationState};

pub fn validate_as_string(scope: &ScopedSchema, data: &Value) -> ValidationState {
    let string = match data.as_str() {
        Some(x) => x,
        None => {
            return ValidationState::new_with_error(scope.error(
                "type",
                format!("expected '{}'", scope.schema().r#type().primitive_type().as_ref()),
            ));
        }
    };

    let len = string.chars().count();
    let schema = scope.schema();
    let mut state = ValidationState::new();

    if let Some(min) = schema.min_length() {
        if len < min {
            state.push_error(scope.error("minLength", format!("expected '>= {}'", min)));
        }
    }

    if let Some(max) = schema.max_length() {
        if len > max {
            state.push_error(scope.error("maxLength", format!("expected '<= {}'", max)));
        }
    }

    if let Some(regex) = schema.pattern() {
        if !regex.is_match(string) {
            state.push_error(scope.error("pattern", "does not match"));
        }
    }

    state
}
