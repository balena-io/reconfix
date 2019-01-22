use serde_json::Value;

use super::super::{scope::ScopedSchema, state::ValidationState, Validator};

fn validate_object_keys_and_values(_scope: &ScopedSchema, _data: &Value) -> ValidationState {
    // TODO Implement when CDSL -> JSON Schema settles down
    ValidationState::new()
}

fn validate_object_properties(scope: &ScopedSchema, data: &Value) -> ValidationState {
    let object = match data.as_object() {
        Some(x) => x,
        None => return ValidationState::new_with_error(scope.invalid_error("type")),
    };

    let mut state = ValidationState::new();

    for property in scope.schema().properties() {
        let nested_state = scope
            .scope_with_property_name(property.schema(), property.name())
            .validate(object.get(property.name()));
        state.extend(nested_state);
    }

    state
}

pub fn validate_as_object(scope: &ScopedSchema, data: &Value) -> ValidationState {
    let mut state = ValidationState::new();

    if !scope.schema().properties().is_empty() {
        state.extend(validate_object_properties(scope, data));
    }

    if scope.schema().keys().is_some() && scope.schema().values().is_some() {
        state.extend(validate_object_keys_and_values(scope, data));
    }

    state
}
