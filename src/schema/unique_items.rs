use std::fmt;

use serde::{
    de,
    ser::{self, SerializeSeq},
};

#[derive(Debug, PartialEq)]
pub enum UniqueItems {
    Boolean(bool),
    Paths(Vec<String>),
}

impl Default for UniqueItems {
    fn default() -> UniqueItems {
        UniqueItems::Boolean(false)
    }
}

impl UniqueItems {
    pub fn is_unique(&self) -> Option<bool> {
        match self {
            UniqueItems::Boolean(v) => Some(*v),
            _ => None,
        }
    }

    pub fn paths(&self) -> Option<&[String]> {
        match self {
            UniqueItems::Paths(ref v) => Some(&v[..]),
            _ => None,
        }
    }
}

struct UniqueItemsVisitor;

impl<'de> de::Visitor<'de> for UniqueItemsVisitor {
    type Value = UniqueItems;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("expected uniqueItems")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(UniqueItems::Boolean(v))
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let mut paths = vec![];
        let mut seq = seq;

        while let Some(s) = seq.next_element()? as Option<String> {
            paths.push(s);
        }

        if paths.is_empty() {
            return Ok(UniqueItems::Boolean(false));
        }

        Ok(UniqueItems::Paths(paths))
    }
}

impl<'de> de::Deserialize<'de> for UniqueItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(UniqueItemsVisitor)
    }
}

impl ser::Serialize for UniqueItems {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match self {
            UniqueItems::Boolean(v) => serializer.serialize_bool(*v),
            UniqueItems::Paths(paths) => {
                let mut seq = serializer.serialize_seq(Some(paths.len()))?;
                for path in paths {
                    seq.serialize_element(path)?;
                }
                seq.end()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bool_value() {
        let ui: UniqueItems = serde_yaml::from_str("true").unwrap();
        assert_eq!(ui.is_unique(), Some(true));
        let ui: UniqueItems = serde_yaml::from_str("false").unwrap();
        assert_eq!(ui.is_unique(), Some(false));
    }

    #[test]
    fn bool_value_does_not_have_paths() {
        let ui: UniqueItems = serde_yaml::from_str("true").unwrap();
        assert!(ui.paths().is_none());
    }

    #[test]
    fn paths() {
        let schema = r#"
        - foo
        - foo.bar
        - foo.bar.baz
        "#;
        let ui: UniqueItems = serde_yaml::from_str(schema).unwrap();

        assert_eq!(
            ui,
            UniqueItems::Paths(vec![
                "foo".to_string(),
                "foo.bar".to_string(),
                "foo.bar.baz".to_string()
            ])
        );
    }

    #[test]
    fn paths_value_does_not_have_uniqueness() {
        let schema = r#"
        - foo
        - foo.bar
        - foo.bar.baz
        "#;
        let ui: UniqueItems = serde_yaml::from_str(schema).unwrap();
        assert!(ui.is_unique().is_none());
    }
}
