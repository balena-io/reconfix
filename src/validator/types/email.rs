use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;

use crate::validator::{scope::ScopedSchema, state::ValidationState, types::validate_as_string_with_regex};

lazy_static! {
    // ajv v6.7.0 compatible
    // https://github.com/epoberezkin/ajv/blob/v6.7.0/lib/compile/formats.js
    static ref EMAIL_REGEX: Regex =
        Regex::new(r"^(?i)[a-z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:\.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)*$").unwrap();
}

pub fn validate_as_email(scope: &ScopedSchema, data: &Value) -> ValidationState {
    validate_as_string_with_regex(scope, data, &EMAIL_REGEX)
}
