use std::net::IpAddr;

use serde_json::Value;

use crate::validator::{scope::ScopedSchema, state::ValidationState, types::validate_as_string};

pub fn validate_as_ipv4(scope: &ScopedSchema, data: &Value) -> ValidationState {
    let mut state = validate_as_string(scope, data);
    if !state.is_valid() {
        return state;
    }

    let ip: Result<IpAddr, _> = data.as_str().expect("invalid validate_as_string").parse();

    match ip {
        Ok(x) if !x.is_ipv4() => state.push_error(scope.error("type", "valid IP address, but not an 'ipv4'")),
        Err(_) => state.push_error(scope.error("type", "unable to parse as 'ipv4'")),
        _ => {}
    };

    state
}

pub fn validate_as_ipv6(scope: &ScopedSchema, data: &Value) -> ValidationState {
    let mut state = validate_as_string(scope, data);
    if !state.is_valid() {
        return state;
    }

    let ip: Result<IpAddr, _> = data.as_str().expect("invalid validate_as_string").parse();

    match ip {
        Ok(x) if !x.is_ipv6() => state.push_error(scope.error("type", "valid IP address, but not an 'ipv6'")),
        Err(_) => state.push_error(scope.error("type", "unable to parse as 'ipv6'")),
        _ => {}
    };

    state
}
