use serde_json::Value;

use super::super::{scope::ScopedSchema, state::ValidationState, Validator};

pub fn validate_as_array(scope: &ScopedSchema, data: &Value) -> ValidationState {
    let array = match data.as_array() {
        Some(x) => x,
        None => return ValidationState::new_with_error(scope.invalid_error("type")),
    };

    let schema = scope.schema();
    let mut state = ValidationState::new();

    if let Some(min) = schema.min_items() {
        if array.len() < min {
            state.push_error(scope.invalid_error("minItems"));
        }
    }

    if let Some(max) = schema.max_items() {
        if array.len() > max {
            state.push_error(scope.invalid_error("maxItems"));
        }
    }

    for (idx, item) in array.iter().enumerate() {
        let mut valid_count = 0;

        for array_schema in scope.schema().items() {
            let nested_state = scope
                .scope_with_array_index(array_schema, idx as isize)
                .validate(Some(item));

            if nested_state.is_valid() {
                valid_count += 1;
            }
        }

        match valid_count {
            0 => state.push_error(scope.invalid_error("items")),
            1 => {}
            _ => state.push_error(scope.invalid_error("items")),
        };
    }

    state
}
