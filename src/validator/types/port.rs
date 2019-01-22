use serde_json::Value;

use super::{
    super::{scope::ScopedSchema, state::ValidationState},
    number::validate_as_integer,
};

pub fn validate_as_port(scope: &ScopedSchema, data: &Value) -> ValidationState {
    let mut state = validate_as_integer(scope, data);

    if state.is_valid() {
        let value = data.as_i64().expect("invalid validate_as_integer");

        if value < 0 || value > 65535 {
            state.push_error(scope.invalid_error("out of range 0 - 65535"));
        }
    }

    state
}
