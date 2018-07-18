pub mod generator;
pub mod types;

mod error {
    error_chain!{}
}

use self::types::Transform;
use error::*;
use schema::types::Schema;

pub trait Generator {
    fn generate(&self, schema: &Schema) -> Result<Vec<Transform>>;
}
