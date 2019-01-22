use serde_json;
use serde_yaml;

use std::collections::HashMap;

#[derive(Debug)]
pub enum Value<'a> {
    Yaml(&'a serde_yaml::Value),
    Json(&'a serde_json::Value),
}

//
// Any serde_*::Value "conversion"
//
pub trait FromRef<'a, T> {
    fn from_ref(a: &'a T) -> Self;
}

pub trait RefInto<'a, T> {
    fn ref_into(&'a self) -> T;
}

impl<'a, T, U> RefInto<'a, U> for T
where
    U: FromRef<'a, T>,
{
    fn ref_into(&'a self) -> U {
        U::from_ref(self)
    }
}

macro_rules! impl_from_ref {
    ($t:ty, $i:expr) => {
        impl<'a> FromRef<'a, $t> for Value<'a> {
            fn from_ref(a: &$t) -> Value {
                $i(a)
            }
        }
    };
}

impl_from_ref!(serde_yaml::Value, Value::Yaml);
impl_from_ref!(serde_json::Value, Value::Json);

//
// Values extraction
//
macro_rules! impl_value_is {
    ($method:ident) => {
        impl_value_is!(
            $method,
            Json => $method,
            Yaml => $method
        );
    };
    ($method:ident, $( $v:ident => $vm:ident ),+ ) => {
        impl<'a> Value<'a> {
            fn $method(&self) -> bool {
                match *self {
                    $(
                        Value::$v(x) => x.$vm(),
                    )+
                }
            }
        }
    };
}

macro_rules! impl_value_as {
    ($method:ident, $result:ty) => {
        impl_value_as!(
            $method, $result,
            Json => $method,
            Yaml => $method
        );
    };
    ($method:ident, $result:ty, $( $v:ident => $vm:ident ),+ ) => {
        impl<'a> Value<'a> {
            fn $method(&self) -> $result {
                match self {
                    $(
                        Value::$v(x) => x.$vm(),
                    )+
                }
            }
        }
    };
}

impl_value_is!(is_bool, Json => is_boolean, Yaml => is_bool);
impl_value_as!(as_bool, Option<bool>);
impl_value_is!(is_null);
impl_value_as!(as_null, Option<()>);
impl_value_is!(is_i64);
impl_value_as!(as_i64, Option<i64>);
impl_value_is!(is_u64);
impl_value_as!(as_u64, Option<u64>);
impl_value_is!(is_f64);
impl_value_as!(as_f64, Option<f64>);
impl_value_is!(is_string);
impl_value_as!(as_str, Option<&str>);
impl_value_is!(is_array, Json => is_array, Yaml => is_sequence);
impl_value_is!(is_object, Json => is_object, Yaml => is_mapping);

impl<'a> Value<'a> {
    fn as_array(&'a self) -> Option<Vec<Value<'a>>> {
        match self {
            Value::Yaml(yaml) => yaml.as_sequence().map(|x| x.iter().map(Value::from_ref).collect()),
            Value::Json(json) => json.as_array().map(|x| x.iter().map(Value::from_ref).collect()),
        }
    }

    fn as_object(&'a self) -> Option<Result<HashMap<&'a str, Value<'a>>, ()>> {
        match self {
            Value::Yaml(yaml) => yaml.as_mapping().map(|x| {
                x.iter()
                    .map(|(k, v)| k.as_str().map(|k| (k, Value::from_ref(v))).ok_or(()))
                    .collect()
            }),
            Value::Json(json) => json
                .as_object()
                .map(|x| x.iter().map(|(k, v)| Ok((k.as_ref(), Value::from_ref(v)))).collect()),
        }
    }
}

//
// Comparison
//
impl<'a, 'b> PartialEq<Value<'a>> for Value<'b> {
    fn eq(&self, other: &Value) -> bool {
        if self.is_null() {
            return self.as_null() == other.as_null();
        }
        if self.is_bool() {
            return self.as_bool() == other.as_bool();
        }
        if self.is_u64() {
            return self.as_u64() == other.as_u64();
        }
        if self.is_i64() {
            return self.as_i64() == other.as_i64();
        }
        if self.is_f64() {
            // TODO approx crate
            return self.as_f64() == other.as_f64();
        }
        if self.is_string() {
            return self.as_str() == other.as_str();
        }
        if self.is_array() {
            return self.as_array() == other.as_array();
        }
        if self.is_object() {
            match (self.as_object(), other.as_object()) {
                (Some(Ok(x)), Some(Ok(y))) => return x == y,
                _ => return false,
            };
        }
        false
    }
}

pub fn eq<'a, 'b, T, U>(lhs: &'a T, rhs: &'b U) -> bool
where
    T: RefInto<'a, Value<'a>>,
    U: RefInto<'b, Value<'b>>,
{
    let lhs: Value = lhs.ref_into();
    let rhs: Value = rhs.ref_into();

    lhs == rhs
}

pub fn ne<'a, 'b, T, U>(lhs: &'a T, rhs: &'b U) -> bool
where
    T: RefInto<'a, Value<'a>>,
    U: RefInto<'b, Value<'b>>,
{
    !eq(lhs, rhs)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_json_yaml_eq(json: &str, yaml: &str) {
        let j: serde_json::Value = serde_json::from_str(&json).unwrap();
        let y: serde_yaml::Value = serde_yaml::from_str(&yaml).unwrap();
        assert!(eq(&j, &y));
    }

    fn assert_json_yaml_ne(json: &str, yaml: &str) {
        let j: serde_json::Value = serde_json::from_str(&json).unwrap();
        let y: serde_yaml::Value = serde_yaml::from_str(&yaml).unwrap();
        assert!(ne(&j, &y));
    }

    #[test]
    fn to_value() {
        let json: serde_json::Value = serde_json::from_str("true").unwrap();
        let json_to_yaml: serde_yaml::Value = serde_json::from_value(json).unwrap();
        let yaml: serde_yaml::Value = serde_yaml::from_str("true").unwrap();
        assert_eq!(json_to_yaml, yaml);
    }

    #[test]
    fn bool_eq() {
        assert_json_yaml_eq("true", "true");
        assert_json_yaml_eq("false", "false");
        assert_json_yaml_ne("false", "true");
        assert_json_yaml_ne("true", "false");
        assert_json_yaml_eq("[1, 2, 3]", "[1, 2, 3]");
    }

    #[test]
    fn string_eq() {
        assert_json_yaml_eq("\"hallo\"", "\"hallo\"");
    }

    #[test]
    fn u64_eq() {
        assert_json_yaml_eq("18446744073709551614", "18446744073709551614");
    }

    #[test]
    fn i64_eq() {
        assert_json_yaml_eq("-23", "-23");
    }

    #[test]
    fn f64_eq() {
        assert_json_yaml_eq("1.5", "1.5");
    }

    #[test]
    fn array_eq() {
        assert_json_yaml_eq(
            "[1, 2, 3]",
            r#"
            - 1
            - 2
            - 3"#,
        );
    }

    #[test]
    fn object_eq() {
        assert_json_yaml_eq(
            r#"
            {
                "foo": 2,
                "bar": "baz"
            }
            "#,
            r#"
            foo: 2
            bar: baz"#,
        );
    }
}
