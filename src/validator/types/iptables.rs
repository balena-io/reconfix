use serde_json::Value;

use crate::validator::{
    scope::ScopedSchema,
    state::ValidationState,
    types::{validate_as_ipv4, validate_as_ipv6},
};

//
// https://github.com/balena-os/meta-balena/blob/v2.29.2/meta-resin-common/recipes-connectivity/resin-proxy-config/resin-proxy-config/resin-proxy-config#L66-L73
//
//   -d, --destination address[/mask][,...]
//          Destination specification.  See the description of the -s (source) flag for a detailed description
//          of the syntax.  The flag --dst is an alias for this option.
//
// ip tables address is (ipv4|ipv6)[/mask]
//
// -d also accepts "ipv4/mask,ipv4/mask,...", but then, it should be stringlist of iptables-address

pub fn validate_as_iptables_address(scope: &ScopedSchema, data: &Value) -> ValidationState {
    let state = validate_as_ipv4(scope, data);
    if state.is_valid() {
        return state;
    }

    validate_as_ipv6(scope, data)
}
