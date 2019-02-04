use serde::de;
use serde_yaml::Value;

#[derive(Debug, PartialEq)]
pub enum FileName {
    /// A real file name
    Name(String),
    /// An expression to evaluate to get a real file name
    Formula(String),
}

impl FileName {
    pub fn name(&self) -> Option<&str> {
        match self {
            FileName::Name(v) => Some(v),
            _ => None,
        }
    }

    pub fn formula(&self) -> Option<&str> {
        match self {
            FileName::Formula(v) => Some(v),
            _ => None,
        }
    }
}

impl<'de> de::Deserialize<'de> for FileName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        match value {
            Value::Mapping(ref mapping) => {
                let eval_key = Value::String("formula".to_string());
                if let Some(value) = mapping.get(&eval_key) {
                    if let Some(s) = value.as_str() {
                        Ok(FileName::Formula(s.to_string()))
                    } else {
                        Err(de::Error::custom("mapping contains eval, but it's not a string"))
                    }
                } else {
                    Err(de::Error::custom("filename must be a string"))
                }
            }
            Value::String(s) => Ok(FileName::Name(s)),
            _ => Err(de::Error::custom("filename must be a string")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name() {
        let f: FileName = serde_yaml::from_str("foo.txt").unwrap();
        assert_eq!(f.name(), Some("foo.txt"));
    }

    #[test]
    fn eval() {
        let f: FileName = serde_yaml::from_str("formula: expr").unwrap();
        assert_eq!(f.formula(), Some("expr"));
    }
}
