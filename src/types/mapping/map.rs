//! DSL `schema.mapping.map` structures
//!
//! https://github.com/balena-io/balena/blob/63ca3a4b026694750f8d6f4e3eea9792cf344426/specs/configuration-dsl-mapping-extension.md#keyword-map

use std::fmt;

use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, Serializer, SerializeSeq};
use serde_yaml::Value;

const IDENTITY_KEYWORD: &str = "identity";

#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    matcher: Value,
    output: Value,
}

impl Entry {
    /// Creates new entry
    pub fn new(matcher: Value, output: Value) -> Entry {
        Entry { matcher, output }
    }

    /// Creates new identity mapping entry
    pub fn identity() -> Entry {
        Entry::new(
            Value::String(IDENTITY_KEYWORD.to_string()),
            Value::String(IDENTITY_KEYWORD.to_string()),
        )
    }

    /// Checks if a mapping is identity
    pub fn is_identity(&self) -> bool {
        self.matcher.as_str() == Some(IDENTITY_KEYWORD) && self.output.as_str() == Some(IDENTITY_KEYWORD)
    }

    pub fn matcher(&self) -> &Value {
        &self.matcher
    }

    pub fn output(&self) -> &Value {
        &self.output
    }
}

impl Serialize for Entry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.is_identity() {
            serializer.serialize_str(IDENTITY_KEYWORD)
        } else {
            let mut seq = serializer.serialize_seq(Some(2))?;
            seq.serialize_element(&self.matcher)?;
            seq.serialize_element(&self.output)?;
            seq.end()
        }
    }
}

impl<'de> Deserialize<'de> for Entry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EntryVisitor;

        impl<'de> Visitor<'de> for EntryVisitor {
            type Value = Entry;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("`identity` or two values sequence")
            }

            fn visit_str<E>(self, value: &str) -> Result<Entry, E>
            where
                E: de::Error,
            {
                if value == IDENTITY_KEYWORD {
                    Ok(Entry::identity())
                } else {
                    Err(de::Error::custom("invalid entry value"))
                }
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Entry, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let matcher = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let output = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(Entry::new(matcher, output))
            }
        }

        deserializer.deserialize_any(EntryVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::{Entry, Value};

    #[test]
    fn identity() {
        let yaml = "identity";
        let entry: Entry = serde_yaml::from_str(yaml).unwrap();
        assert!(entry.is_identity());
    }

    #[test]
    fn array_identity() {
        let yaml = r#"
        - identity
        - identity
        "#;
        let entry: Entry = serde_yaml::from_str(yaml).unwrap();
        assert!(entry.is_identity());
    }

    #[test]
    fn entries() {
        let yaml = r#"
        - - true
          - "true"
        - identity
        "#;

        let entries: Vec<Entry> = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(entries.len(), 2);
        assert_eq!(
            entries.first(),
            Some(&Entry::new(Value::Bool(true), Value::String("true".to_string())))
        );
        assert_eq!(entries.last(), Some(&Entry::identity()));
    }
}
