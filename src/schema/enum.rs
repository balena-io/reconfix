use serde::de;
use serde_yaml::Value;

#[derive(Clone, Debug, PartialEq)]
pub struct EnumEntry {
    title: Option<String>,
    value: Value,
}

impl EnumEntry {
    pub fn title(&self) -> String {
        match &self.title {
            Some(v) => v.clone(),
            None => match &self.value {
                Value::Bool(x) => format!("{}", x),
                Value::Number(x) => format!("{}", x),
                Value::String(x) => x.clone(),
                // See deserializer, title is required for other types
                _ => unreachable!(),
            },
        }
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
}

impl<'de> de::Deserialize<'de> for EnumEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let v = Value::deserialize(deserializer)?;
        match v {
            Value::Bool(_) | Value::Number(_) | Value::String(_) => Ok(EnumEntry { title: None, value: v }),
            Value::Null | Value::Sequence(_) => Err(de::Error::custom("title is required for null or sequence value")),
            Value::Mapping(mut m) => {
                let title_keyword = Value::String("title".to_string());
                let title = m
                    .remove(&title_keyword)
                    .ok_or_else(|| de::Error::custom("missing title keyword"))?;

                let title = match title {
                    Value::String(s) => s,
                    _ => return Err(de::Error::custom("title is not a string")),
                };

                let value_keyword = Value::String("value".to_string());
                let value = m
                    .remove(&value_keyword)
                    .ok_or_else(|| de::Error::custom("missing value keyword"))?;

                Ok(EnumEntry {
                    title: Some(title),
                    value,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_value_as_title() {
        let e: EnumEntry = serde_yaml::from_str("foo").unwrap();
        assert_eq!(e.title(), "foo".to_string());
    }

    #[test]
    fn bool_value_as_title() {
        let e: EnumEntry = serde_yaml::from_str("true").unwrap();
        assert_eq!(e.title(), "true".to_string());
        let e: EnumEntry = serde_yaml::from_str("false").unwrap();
        assert_eq!(e.title(), "false".to_string());
    }

    #[test]
    fn number_value_as_title() {
        let e: EnumEntry = serde_yaml::from_str("123").unwrap();
        assert_eq!(e.title(), "123".to_string());
        let e: EnumEntry = serde_yaml::from_str("1.5").unwrap();
        assert_eq!(e.title(), "1.5".to_string());
    }

    #[test]
    fn custom_title() {
        let schema = r#"
            title: Foo
            value: Bar
        "#;
        let e: EnumEntry = serde_yaml::from_str(schema).unwrap();
        assert_eq!(e.title(), "Foo".to_string());
        assert_eq!(e.value(), &Value::String("Bar".to_string()));
    }

    #[test]
    fn require_title_for_null_value() {
        let x: Result<EnumEntry, _> = serde_yaml::from_str("~");
        assert!(x.is_err());
    }

    #[test]
    fn require_title_for_sequence_value() {
        let x: Result<EnumEntry, _> = serde_yaml::from_str("[1, 2, 3]");
        assert!(x.is_err());
    }
}
