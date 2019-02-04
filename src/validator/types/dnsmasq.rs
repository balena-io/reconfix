use serde_json::Value;

use crate::validator::{
    scope::ScopedSchema,
    state::ValidationState,
    types::{validate_as_ipv4, validate_as_ipv6},
};

// https://github.com/balena-os/meta-balena/blob/v2.29.2/meta-resin-common/recipes-connectivity/resin-net-config/resin-net-config/resin-net-config#L34-L39
//
// This dnsmasq server address can be pretty complex:
//
// --server=[/[<domain>]/[domain/]][<ipaddr>[#<port>][@<source-ip>|<interface>[#<port>]]
//
// For now, we're going to validate it as either ipv4 or ipv6.

pub fn validate_as_dnsmasq_address(scope: &ScopedSchema, data: &Value) -> ValidationState {
    let state = validate_as_ipv4(scope, data);
    if state.is_valid() {
        return state;
    }

    validate_as_ipv6(scope, data)
}
