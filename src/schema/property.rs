use std::fmt;

use serde::de;

use crate::schema::Schema;

#[derive(Debug)]
pub struct Property {
    name: String,
    schema: Schema,
}

impl Property {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }
}

struct PropertyVisitor;

impl<'de> de::Visitor<'de> for PropertyVisitor {
    type Value = Property;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("expected property")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: de::MapAccess<'de>,
    {
        if let Some((name, schema)) = access.next_entry()? {
            return Ok(Property { name, schema });
        }

        Err(de::Error::custom("unable to deserialize property from empty map"))
    }
}

impl<'de> de::Deserialize<'de> for Property {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(PropertyVisitor)
    }
}
