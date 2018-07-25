pub mod generator;
pub mod types;

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

use self::types::Transform;
use crate::error::*;
use crate::schema::types::Schema;

pub trait Generator {
    fn generate(&self, schema: &Schema) -> Result<Vec<Transform>>;
}
