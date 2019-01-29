use serde_json::Value;

use super::super::{scope::ScopedSchema, state::ValidationState, Validator};

fn validate_object_keys_and_values(_scope: &ScopedSchema, _data: &Value) -> ValidationState {
    // TODO Implement when CDSL -> JSON Schema settles down
    ValidationState::new()
}

fn validate_object_properties(scope: &ScopedSchema, data: &Value) -> ValidationState {
    let object = match data.as_object() {
        Some(x) => x,
        None => return ValidationState::new_with_error(scope.error("type", "expected 'object'")),
    };

    let mut state = ValidationState::new();

    for property in scope.schema().properties() {
        let nested_scope = scope.scope_with_schema_keyword(property.name());
        let nested_scope = nested_scope.scope_with_data_property(property.name());
        let nested_scope = nested_scope.scope_with_schema(property.schema());
        let nested_state = nested_scope.validate(object.get(property.name()));
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
