
pub mod types;
pub mod generator;

mod error {
    error_chain!{}
}

use error::*;
use schema::types::Schema;
use self::types::Transform;

pub trait Generator {
    fn generate(&self, schema: &Schema) -> Result<Vec<Transform>>;
}