pub mod generator;
pub mod types;

mod error {
    #![allow(renamed_and_removed_lints)] // unused_doc_comment -> unused_doc_comments
    error_chain!{}
}

use self::types::Transform;
use error::*;
use schema::types::Schema;

pub trait Generator {
    fn generate(&self, schema: &Schema) -> Result<Vec<Transform>>;
}
