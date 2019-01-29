use serde_json::Value;

use super::super::{scope::ScopedSchema, state::ValidationState, Validator};

pub fn validate_as_array(scope: &ScopedSchema, data: &Value) -> ValidationState {
    let data_array = match data.as_array() {
        Some(x) => x,
        None => return scope.error("type", "expected 'array'").into(),
    };

    let schema = scope.schema();

    let mut state = ValidationState::new();

    if let Some(min) = schema.min_items() {
        if data_array.len() < min {
            state.push_error(scope.error("minItems", format!("should contain at least '{}' items", min)));
        }
    }

    if let Some(max) = schema.max_items() {
        if data_array.len() > max {
            state.push_error(scope.error("maxItems", format!("should contain up to '{}' items", max)));
        }
    }

    let scope = scope.scope_with_schema_keyword("items");

    for (idx, item) in data_array.iter().enumerate() {
        let mut valid_count = 0;

        let data_scope = scope.scope_with_data_index(idx);

        let mut data_item_state = ValidationState::new();

        for (idx, array_schema) in scope.schema().items().iter().enumerate() {
            let nested_scope = data_scope.scope_with_schema_index(idx, array_schema);

            let nested_state = nested_scope.validate(Some(item));

            if nested_state.is_valid() {
                valid_count += 1;
            } else {
                data_item_state.extend(nested_state);
            }
        }

        match valid_count {
            0 => state.extend(data_item_state),
            1 => {}
            _ => state.push_error(data_scope.error("items", "valid against multiple schemas")),
        };
    }

    state
}
