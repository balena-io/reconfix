pub use array::validate_as_array;
pub use boolean::validate_as_boolean;
pub use chrony::validate_as_chrony_address;
pub use datetime::{validate_as_date, validate_as_datetime, validate_as_time};
pub use dnsmasq::validate_as_dnsmasq_address;
pub use email::validate_as_email;
pub use file::validate_as_file;
pub use hostname::validate_as_hostname;
pub use ip::{validate_as_ipv4, validate_as_ipv6};
pub use iptables::validate_as_iptables_address;
pub use number::{validate_as_integer, validate_as_number};
pub use object::validate_as_object;
pub use password::validate_as_password;
pub use port::validate_as_port;
pub use string::validate_as_string;
pub use stringlist::validate_as_stringlist;
pub use text::validate_as_text;
pub use uri::validate_as_uri;

mod array;
mod boolean;
mod chrony;
mod datetime;
mod dnsmasq;
mod email;
mod file;
mod hostname;
mod ip;
mod iptables;
mod number;
mod object;
mod password;
mod port;
mod string;
mod stringlist;
mod text;
mod uri;

//
// Shared helpers
//

use regex::Regex;
use serde_json::Value;

use crate::validator::{scope::ScopedSchema, ValidationState};

pub(crate) fn validate_as_string_with_regex(scope: &ScopedSchema, data: &Value, regex: &Regex) -> ValidationState {
    let mut state = validate_as_string(scope, data);

    if state.is_valid() && !regex.is_match(data.as_str().expect("invalid validate_as_string")) {
        state.push_error(scope.error(
            "type",
            format!("expected '{}'", scope.schema().r#type().primitive_type().as_ref()),
        ));
    }

    state
}
