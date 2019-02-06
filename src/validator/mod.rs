use serde_json::{self, Value};

use crate::{
    schema::{PrimitiveType, Schema},
    utils::value,
};

pub use error::ValidationError;
use scope::ScopedSchema;
pub use state::ValidationState;

mod error;
mod path;
mod scope;
mod state;
mod types;

pub trait Validator {
    fn validate(&self, data: Option<&Value>) -> ValidationState;
}

macro_rules! bail_if_invalid {
    ($state:expr) => {{
        let state = $state;
        if !state.is_valid() {
            return state;
        }
    }};
}

fn validate_optional(scope: &ScopedSchema, data: Option<&Value>) -> ValidationState {
    let value_exists = match data {
        Some(Value::Null) => false,
        None => false,
        _ => true,
    };

    if !value_exists && scope.schema().r#type().is_required() {
        return scope
            .error(
                "type",
                format!("'{}' is not an optional type", scope.schema().r#type().to_string()),
            )
            .into();
    }

    ValidationState::new()
}

fn validate_const(scope: &ScopedSchema, data: &Value) -> ValidationState {
    match scope.schema().r#const() {
        Some(constant) if value::ne(constant, data) => scope.error("const", "value does not match").into(),
        _ => ValidationState::new(),
    }
}

fn validate_enum(scope: &ScopedSchema, data: &Value) -> ValidationState {
    let enum_entries = scope.schema().r#enum();

    if enum_entries.is_empty() {
        return ValidationState::new();
    }

    let valid_count = enum_entries
        .iter()
        .fold(0, |acc, item| acc + value::eq(item.value(), data) as usize);

    match valid_count {
        1 => ValidationState::new(),
        0 => scope.error("enum", "does not match any value").into(),
        _ => scope.error("enum", "matches multiple values").into(),
    }
}

impl<'a> Validator for ScopedSchema<'a> {
    fn validate(&self, data: Option<&Value>) -> ValidationState {
        bail_if_invalid!(validate_optional(self, data));

        let data = match data {
            Some(x) => x,
            None => return ValidationState::new(),
        };

        bail_if_invalid!(validate_const(self, data));
        bail_if_invalid!(validate_enum(self, data));

        match self.schema().r#type().primitive_type() {
            PrimitiveType::String => types::validate_as_string(self, data),
            PrimitiveType::Array => types::validate_as_array(self, data),
            PrimitiveType::Boolean => types::validate_as_boolean(self, data),
            PrimitiveType::Integer => types::validate_as_integer(self, data),
            PrimitiveType::Number => types::validate_as_number(self, data),
            PrimitiveType::Object => types::validate_as_object(self, data),
            PrimitiveType::Port => types::validate_as_port(self, data),
            PrimitiveType::Text => types::validate_as_text(self, data),
            PrimitiveType::Password => types::validate_as_password(self, data),
            PrimitiveType::Hostname => types::validate_as_hostname(self, data),
            PrimitiveType::Date => types::validate_as_date(self, data),
            PrimitiveType::DateTime => types::validate_as_datetime(self, data),
            PrimitiveType::Time => types::validate_as_time(self, data),
            PrimitiveType::Email => types::validate_as_email(self, data),
            PrimitiveType::IPv4 => types::validate_as_ipv4(self, data),
            PrimitiveType::IPv6 => types::validate_as_ipv6(self, data),
            PrimitiveType::Uri => types::validate_as_uri(self, data),
            PrimitiveType::File => types::validate_as_file(self, data),
            PrimitiveType::ChronyAddress => types::validate_as_chrony_address(self, data),
            PrimitiveType::DNSMasqAddress => types::validate_as_dnsmasq_address(self, data),
            PrimitiveType::IPTablesAddress => types::validate_as_iptables_address(self, data),
            PrimitiveType::StringList => types::validate_as_stringlist(self, data),
        }
    }
}

impl Validator for Schema {
    fn validate(&self, data: Option<&Value>) -> ValidationState {
        ScopedSchema::new(self).validate(data)
    }
}

pub fn validate(schema: &Schema, data: &Value) -> ValidationState {
    schema.validate(Some(data))
}
