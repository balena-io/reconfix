use serde_json::Value;

use super::super::{scope::ScopedSchema, state::ValidationState};

fn validate_as_u64(scope: &ScopedSchema, data: &Value) -> ValidationState {
    let integer = match data.as_u64() {
        Some(x) => x,
        None => {
            return ValidationState::new_with_error(scope.invalid_error("type"));
        }
    };

    let schema = scope.schema();
    let mut state = ValidationState::new();

    if let Some(min) = schema.min().and_then(|x| x.as_u64()) {
        if integer < min {
            state.push_error(scope.invalid_error("min"));
        }
    }

    if let Some(exclusive_min) = schema.exclusive_min().and_then(|x| x.as_u64()) {
        if integer <= exclusive_min {
            state.push_error(scope.invalid_error("exclusiveMin"));
        }
    }

    if let Some(max) = schema.max().and_then(|x| x.as_u64()) {
        if integer > max {
            state.push_error(scope.invalid_error("max"));
        }
    }

    if let Some(exclusive_max) = schema.exclusive_max().and_then(|x| x.as_u64()) {
        if integer >= exclusive_max {
            state.push_error(scope.invalid_error("exclusiveMax"));
        }
    }

    if let Some(multiple_of) = schema.multiple_of().and_then(|x| x.as_u64()) {
        if multiple_of > 0 && integer % multiple_of != 0 {
            state.push_error(scope.invalid_error("multipleOf"));
        }
    }

    state
}

fn validate_as_i64(scope: &ScopedSchema, data: &Value) -> ValidationState {
    let integer = match data.as_i64() {
        Some(x) => x,
        None => {
            return ValidationState::new_with_error(scope.invalid_error("type"));
        }
    };

    let schema = scope.schema();
    let mut state = ValidationState::new();

    if let Some(min) = schema.min().and_then(|x| x.as_i64()) {
        if integer < min {
            state.push_error(scope.invalid_error("min"));
        }
    }

    if let Some(exclusive_min) = schema.exclusive_min().and_then(|x| x.as_i64()) {
        if integer <= exclusive_min {
            state.push_error(scope.invalid_error("exclusiveMin"));
        }
    }

    if let Some(max) = schema.max().and_then(|x| x.as_i64()) {
        if integer > max {
            state.push_error(scope.invalid_error("max"));
        }
    }

    if let Some(exclusive_max) = schema.exclusive_max().and_then(|x| x.as_i64()) {
        if integer >= exclusive_max {
            state.push_error(scope.invalid_error("exclusiveMax"));
        }
    }

    if let Some(multiple_of) = schema.multiple_of().and_then(|x| x.as_i64()) {
        if multiple_of > 0 && integer % multiple_of != 0 {
            state.push_error(scope.invalid_error("multipleOf"));
        }
    }

    state
}

fn validate_as_f64(scope: &ScopedSchema, data: &Value) -> ValidationState {
    let integer = match data.as_f64() {
        Some(x) => x,
        None => {
            return ValidationState::new_with_error(scope.invalid_error("type"));
        }
    };

    let schema = scope.schema();
    let mut state = ValidationState::new();

    if let Some(min) = schema.min().and_then(|x| x.as_f64()) {
        if integer < min {
            state.push_error(scope.invalid_error("min"));
        }
    }

    if let Some(exclusive_min) = schema.exclusive_min().and_then(|x| x.as_f64()) {
        if integer <= exclusive_min {
            state.push_error(scope.invalid_error("exclusiveMin"));
        }
    }

    if let Some(max) = schema.max().and_then(|x| x.as_f64()) {
        if integer > max {
            state.push_error(scope.invalid_error("max"));
        }
    }

    if let Some(exclusive_max) = schema.exclusive_max().and_then(|x| x.as_f64()) {
        if integer >= exclusive_max {
            state.push_error(scope.invalid_error("exclusiveMax"));
        }
    }

    if let Some(multiple_of) = schema.multiple_of().and_then(|x| x.as_f64()) {
        if multiple_of > 0.0 && integer % multiple_of != 0.0 {
            state.push_error(scope.invalid_error("multipleOf"));
        }
    }

    state
}

pub fn validate_as_integer(scope: &ScopedSchema, data: &Value) -> ValidationState {
    let state = validate_as_u64(scope, data);
    if state.is_valid() {
        return state;
    }

    validate_as_i64(scope, data)
}

pub fn validate_as_number(scope: &ScopedSchema, data: &Value) -> ValidationState {
    let state = validate_as_integer(scope, data);
    if state.is_valid() {
        return state;
    }

    validate_as_f64(scope, data)
}
