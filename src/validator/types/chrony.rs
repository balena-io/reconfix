use serde_json::Value;

use crate::validator::{
    scope::ScopedSchema,
    state::ValidationState,
    types::{validate_as_hostname, validate_as_ipv4, validate_as_ipv6},
};

// https://github.com/balena-os/meta-balena/blob/v2.29.2/meta-resin-common/recipes-connectivity/resin-ntp-config/resin-ntp-config/resin-ntp-config#L19
//
//  add server address [option]...
//
//     The add server command allows a new NTP server to be added whilst chronyd is running.
//     Following the words add server, the syntax of the following parameters and options is similar to that
//     for the server directive in the configuration file. The following server options can be set in the
//     command: port, minpoll, maxpoll, presend, maxdelayratio, maxdelay, key.
//     An example of using this command is shown below:
//     add server foo.example.net minpoll 6 maxpoll 10 key 25
//
// Additional options not supported, validate either as ipv4, ipv6 or hostname

pub fn validate_as_chrony_address(scope: &ScopedSchema, data: &Value) -> ValidationState {
    let state = validate_as_hostname(scope, data);
    if state.is_valid() {
        return state;
    }

    let state = validate_as_ipv4(scope, data);
    if state.is_valid() {
        return state;
    }

    validate_as_ipv6(scope, data)
}
