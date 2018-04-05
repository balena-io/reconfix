


use error::*;
use super::types::*;

trait Generator {
    fn generate(&self, schema: &Schema) -> Result<Vec<Tranform>>;
}

pub struct DefaultGenerator;

impl Generator for DefaultGenerator {
    fn generate(&self, schema: &Schema) -> Result<Vec<Transform>> {
        let root = match *schema {
            Schema::Boolean(_) => return Ok(Vec::new()),
            Schema::Object(obj) => obj,
        };

        Ok(vec![])
    }
}

struct GeneratorContext {
}
