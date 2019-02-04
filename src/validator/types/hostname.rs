use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;

use crate::validator::{scope::ScopedSchema, state::ValidationState, types::validate_as_string_with_regex};

lazy_static! {
    // ajv v6.7.0 compatible
    // https://github.com/epoberezkin/ajv/blob/v6.7.0/lib/compile/formats.js
    static ref HOSTNAME_REGEX: Regex =
        Regex::new(r"^(?i)[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:\.[a-z0-9](?:[-0-9a-z]{0,61}[0-9a-z])?)*$").unwrap();
}

pub fn validate_as_hostname(scope: &ScopedSchema, data: &Value) -> ValidationState {
    let mut state = validate_as_string_with_regex(scope, data, &HOSTNAME_REGEX);

    if state.is_valid() {
        let len = data
            .as_str()
            .expect("invalid validate_as_string_with_regex")
            .chars()
            .count();

        if len > 255 {
            state.push_error(scope.error("type", "'hostname' must not be longer than 255 characters"));
        }
    }

    state
}
