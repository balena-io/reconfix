use serde::{de, ser};
use serde_yaml::Number;

#[derive(Debug, PartialEq)]
pub struct Version {
    value: u8,
}

impl Version {
    /// Returns schema version value
    pub fn value(&self) -> u8 {
        self.value
    }
}

impl Default for Version {
    fn default() -> Version {
        Version { value: 1 }
    }
}

impl<'de> de::Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let v = Number::deserialize(deserializer)?;

        if v.as_u64() != Some(1) {
            Err(de::Error::custom("unsuppored version number"))
        } else {
            Ok(Version { value: 1 })
        }
    }
}

impl ser::Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_u8(self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_one() {
        assert_eq!(Version::default().value(), 1);
    }

    #[test]
    fn one() {
        let v: Version = serde_yaml::from_str("1").unwrap();
        assert_eq!(v.value(), 1);
    }

    #[test]
    fn fail_on_unsupported_version() {
        let v: Result<Version, _> = serde_yaml::from_str("2");
        assert!(v.is_err());
        let v: Result<Version, _> = serde_yaml::from_str("0");
        assert!(v.is_err());
    }

    #[test]
    fn fail_on_string() {
        let v: Result<Version, _> = serde_yaml::from_str("'1'");
        assert!(v.is_err());
    }
}
