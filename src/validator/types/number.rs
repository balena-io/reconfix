use serde_json::Value;
use serde_yaml::Number;

use crate::validator::{scope::ScopedSchema, state::ValidationState};

fn validate_as<T, F1, F2>(
    scope: &ScopedSchema,
    data: &Value,
    schema_number_value: F1,
    data_value: F2,
) -> ValidationState
where
    F1: Fn(&Number) -> Option<T> + Copy,
    F2: Fn(&Value) -> Option<T> + Copy,
    T: std::fmt::Display + std::cmp::PartialOrd + std::ops::Rem<Output = T> + Copy,
{
    let value = match data_value(data) {
        Some(x) => x,
        None => {
            return ValidationState::new_with_error(scope.error(
                "type",
                format!("expected '{}'", scope.schema().r#type().primitive_type().as_ref()),
            ));
        }
    };

    let schema = scope.schema();
    let mut state = ValidationState::new();

    if let Some(min) = schema.min().and_then(schema_number_value) {
        if value < min {
            state.push_error(scope.error("min", format!("expected '>= {}", min)));
        }
    }

    if let Some(exclusive_min) = schema.exclusive_min().and_then(schema_number_value) {
        if value <= exclusive_min {
            state.push_error(scope.error("exclusiveMin", format!("expected '> {}", exclusive_min)));
        }
    }

    if let Some(max) = schema.max().and_then(schema_number_value) {
        if value > max {
            state.push_error(scope.error("max", format!("expected '<= {}", max)));
        }
    }

    if let Some(exclusive_max) = schema.exclusive_max().and_then(schema_number_value) {
        if value >= exclusive_max {
            state.push_error(scope.error("exclusiveMax", format!("expected '< {}", exclusive_max)));
        }
    }

    let zero = schema_number_value(&Number::from(0)).unwrap();
    if let Some(multiple_of) = schema.multiple_of().and_then(schema_number_value) {
        if multiple_of > zero && value % multiple_of != zero {
            state.push_error(scope.error(
                "multipleOf",
                format!("expected '{} % {} == {}'", value, multiple_of, zero),
            ));
        }
    }

    state
}

pub fn validate_as_integer(scope: &ScopedSchema, data: &Value) -> ValidationState {
    let state = validate_as(scope, data, Number::as_u64, Value::as_u64);
    if state.is_valid() {
        return state;
    }

    validate_as(scope, data, Number::as_i64, Value::as_i64)
}

pub fn validate_as_number(scope: &ScopedSchema, data: &Value) -> ValidationState {
    let state = validate_as_integer(scope, data);
    if state.is_valid() {
        return state;
    }

    validate_as(scope, data, Number::as_f64, Value::as_f64)
}
