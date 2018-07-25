pub mod default;
pub mod types;

use serde_json::Value;

use self::error::*;
use crate::transform::types::*;

mod error {
    // TODO Rust 2018: Remove when error_chain will be fixed
    #![allow(
        renamed_and_removed_lints,
        bare_trait_objects,
        unreachable_pub
    )]

    use error_chain::*;
    error_chain!{}
}

type Map<K, V> = ::std::collections::BTreeMap<K, V>;

pub trait Mapper {
    fn forward_map(&self, dry: &Value, transforms: &[Transform]) -> Result<Map<Target, Value>>;
    fn reverse_map(&self, wet: &Map<Target, Value>, transforms: &[Transform]) -> Result<Value>;
}
