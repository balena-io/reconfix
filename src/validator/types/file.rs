use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;

use crate::validator::{scope::ScopedSchema, state::ValidationState};

lazy_static! {
    // data:text/plain;name=test.txt;base64,aGV...
    static ref FILE_REGEX: Regex =
        Regex::new(r"^data:.*;name=(.*);([a-zA-Z0-9]+),(.*)$").unwrap();
}

pub fn validate_as_file(scope: &ScopedSchema, data: &Value) -> ValidationState {
    let s = match data.as_str() {
        Some(x) => x,
        None => return scope.error("type", "expected `file`").into(),
    };

    let captures = match FILE_REGEX.captures(s) {
        Some(x) => x,
        _ => return scope.error("type", "expected `file`").into(),
    };

    if (&captures[1]).is_empty() {
        return scope.error("type", "file name is missing").into();
    }

    if &captures[2] != "base64" {
        return scope.error("type", "only base64 is supported").into();
    }

    match base64::decode(&captures[3]) {
        Ok(_) => ValidationState::new(),
        Err(_) => scope.error("type", "unable to decode file data").into(),
    }
}
