use regex::Regex;
use serde_json::Value;

use super::{
    super::{scope::ScopedSchema, state::ValidationState},
    string::validate_as_string,
};

pub fn validate_as_string_with_regex(scope: &ScopedSchema, data: &Value, regex: &Regex) -> ValidationState {
    let mut state = validate_as_string(scope, data);

    if state.is_valid() && !regex.is_match(data.as_str().expect("invalid validate_as_string")) {
        state.push_error(scope.error(
            "type",
            format!("expected '{}'", scope.schema().type_().primitive_type().as_ref()),
        ));
    }

    state
}
