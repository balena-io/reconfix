use self::error::*;
use crate::transform::types::*;

use serde_json::Value;

pub mod default;
pub mod types;

mod error {
    // TODO Rust 2018: Remove when error_chain will be fixed
    #![allow(
        renamed_and_removed_lints,
        bare_trait_objects,
        unreachable_pub
    )]
    error_chain!{}
}

type Map<K, V> = ::std::collections::BTreeMap<K, V>;

pub trait Mapper {
    fn forward_map(&self, dry: &Value, transforms: &[Transform]) -> Result<Map<Target, Value>>;
    fn reverse_map(&self, wet: &Map<Target, Value>, transforms: &[Transform]) -> Result<Value>;
}
