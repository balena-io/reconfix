use crate::schema::Schema;

use super::error::ValidationError;
use super::path::Path;

#[derive(Debug, Clone)]
pub struct ScopedSchema<'a> {
    schema: &'a Schema,
    path: Path,
}

impl<'a> ScopedSchema<'a> {
    pub fn new(schema: &Schema) -> ScopedSchema {
        ScopedSchema {
            schema,
            path: Path::new(),
        }
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl<'a> ScopedSchema<'a> {
    pub fn scope_with_array_index(&'a self, schema: &'a Schema, index: isize) -> ScopedSchema {
        let mut path = self.path.clone();
        path.push_array_index(index);
        ScopedSchema { schema, path }
    }

    pub fn scope_with_property_name<S>(&'a self, schema: &'a Schema, name: S) -> ScopedSchema
    where
        S: Into<String>,
    {
        let mut path = self.path.clone();
        path.push_property_name(name);
        ScopedSchema { schema, path }
    }
}

impl<'a> ScopedSchema<'a> {
    pub fn missing_error(&self) -> ValidationError {
        ValidationError::Missing(self.path().clone())
    }

    pub fn invalid_error<S>(&self, message: S) -> ValidationError
    where
        S: Into<String>,
    {
        ValidationError::Invalid(self.path().clone(), message.into())
    }
}
