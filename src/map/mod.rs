use self::error::*;
use transform::types::*;

use serde_json::Value;

pub mod default;
pub mod types;

mod error {
    error_chain!{}
}

type Map<K, V> = ::std::collections::BTreeMap<K, V>;
//type Entry<K, V> = ::std::collections::btree_map::Entry<_, K, V>;

pub trait Mapper {
    fn forward_map(&self, dry: &Value, transforms: &[Transform]) -> Result<Map<Target, Value>>;
    fn reverse_map(&self, wet: &Map<Target, Value>, transforms: &[Transform]) -> Result<Value>;
}
