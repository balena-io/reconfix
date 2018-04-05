
use ::transform::types::*;
use self::error::*;

use serde_json::Value;

pub mod types;
pub mod default;

mod error {
    error_chain!{}
}

type Map<K, V> = ::std::collections::BTreeMap<K, V>;

trait Mapper {
    fn forward_map(&self, dry: &Value, transforms: &[Transform]) -> Result<Map<Target, Value>>;
    fn reverse_map(&self, wet: &Map<Target, Value>, transforms: &[Transform]) -> Result<Value>;
}