use std::collections::HashSet;

use serde_json::Value;

use crate::validator::{scope::ScopedSchema, state::ValidationState, Validator};

pub fn validate_as_object(scope: &ScopedSchema, data: &Value) -> ValidationState {
    let object = match data.as_object() {
        Some(x) => x,
        None => return ValidationState::new_with_error(scope.error("type", "expected 'object'")),
    };

    let mut state = ValidationState::new();

    let mut remaining_keys: HashSet<&str> = object.keys().map(AsRef::as_ref).collect();

    // Validate .properties first
    for (index, property) in scope.schema().properties().iter().enumerate() {
        let nested_scope = scope.scope_with_property(index, property);
        let nested_state = nested_scope.validate(object.get(property.name()));
        state.extend(nested_state);
        remaining_keys.remove(property.name());
    }

    match (scope.schema().keys(), scope.schema().values()) {
        // Schema contains keys & values, validate pattern properties
        (Some(schema_keys), Some(schema_values)) => {
            for key in remaining_keys {
                let value = object.get(key);
                state.extend(schema_keys.validate(Some(&Value::String(key.to_string()))));
                state.extend(schema_values.validate(value));
            }
        }
        // Schema doesn't contain keys & values, just check for additional properties
        _ => {
            if !remaining_keys.is_empty() && !scope.schema().additional_properties() {
                state.push_error(scope.error("additionalProperties", "not allowed"));
            }
        }
    }

    state
}
