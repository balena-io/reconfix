
use ::transform::types::*;
use super::{Map, Mapper};
use super::error::*;
use super::types::*;

use serde_json::Value;

struct DefaultMapper;

impl Mapper for DefaultMapper {
    fn forward_map(&self, dry: &Value, transforms: &[Transform]) -> Result<Map<Target, Value>> {
        unimplemented!()
    }

    fn reverse_map(&self, wet: &Map<Target, Value>, transforms: &[Transform]) -> Result<Value> {
        unimplemented!()
    }
}

fn apply_tranform_forward(dry: &Value, transform: &Transform) -> Result<Layer> {
    let input = match transform.source.search(dry) {
        Some(value) => value,
        None => bail!("unable to find transform input"),
    };
    
    for element in &transform.map {
        let layer = match *element {
            Case::Identity => Some(Layer::from_value(&transform.source, input)),
            Case::Value { ref dry, ref wet } if dry == input => {
                Some(Layer::single(&transform.source, Leaf::Schema(wet.clone())))
            },
            _ => None,
        };

        if let Some(layer) = layer {
            return Ok(layer);
        }
    }
    
    bail!("unable to match dry value at '{}'", transform.source);
}
