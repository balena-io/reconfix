use serde_json::Value;
use serde_json::Value::*;

/// The types of wildcard patterns in a schema
#[derive(Debug, Eq, PartialEq)]
pub enum Wildcard {
    /// Match any string
    String,
    /// Match any number
    Number,
    /// Match any object (key-value map)
    Object,
    /// Match any boolean
    Boolean,
    /// Match any array
    Array,
    /// Match only null
    ///
    /// This is included for completness, you could also just use null in the schema.
    Null,
}

impl Wildcard {
    /// Check whether this wildcard matches some JSON value
    pub fn matches(&self, data: &Value) -> bool {
        match *self {
            Wildcard::String => data.is_string(),
            Wildcard::Number => data.is_number(),
            Wildcard::Object => data.is_object(),
            Wildcard::Boolean => data.is_boolean(),
            Wildcard::Array => data.is_array(),
            Wildcard::Null => data == &Value::Null,
        }
    }
}

/// Determine if a string represents a wildcard.
///
/// If so, returns Some. Otherwise, returns None.
pub fn type_wildcards(s: &str) -> Option<Vec<Wildcard>> {
    // FIXME: Result with error messages
    // FIXME: parse multiple wildcards
    match s {
        "[[string]]" => Some(vec![Wildcard::String]),
        "[[number]]" => Some(vec![Wildcard::Number]),
        "[[object]]" => Some(vec![Wildcard::Object]),
        "[[boolean]]" => Some(vec![Wildcard::Boolean]),
        "[[array]]" => Some(vec![Wildcard::Array]),
        "[[null]]" => Some(vec![Wildcard::Null]),
        _ => None,
    }
}

/// Test if a pattern matches some data.
///
/// A pattern matches if any of:
///
/// 1. The data and the pattern are of the same kind of object and are structurally equal
/// 2. The pattern is a wildcard string and the data is of the corresponding type
/// 3. The pattern and data are an array and each element matches
/// 4. The pattern and data are an object and each key of the pattern exists in the data and the
///    values match
pub fn matches(data: &Value, pattern: &Value) -> bool {
    match *pattern {
        String(ref s) => match type_wildcards(s) {
            None => data == pattern,
            Some(wildcards) => wildcards.iter().any(|v| v.matches(data)),
        },
        Array(ref a) => match data.as_array() {
            Some(d) => {
                if a.len() == d.len() {
                    a.iter().zip(d.iter()).all(|(a, b)| matches(a, b))
                } else {
                    false
                }
            },
            None => false,
        },
        Object(ref o) => match data.as_object() {
            Some(d) => o.iter()
                .all(|(k, pattern)| d.get(k).map_or(false, |data| matches(data, pattern))),
            None => false,
        },
        _ => data == pattern,
    }
}

/// Get the "degree" of a pattern.
///
/// The degree is the total number of fields in a value.
pub fn degree(pattern: &Value) -> u64 {
    1 + match *pattern {
        Object(ref o) => o.values().map(degree).sum(),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {

    use super::matches;
    use serde_json::Value;

    fn template_matches(data: Vec<String>) -> Option<String> {
        let msg = &data[0];
        let value = data[1].parse::<Value>().expect("Invalid JSON value!");
        let pattern = data[2].parse::<Value>().expect("Invalid JSON pattern!");
        let expected = &*data[3];
        let result = matches(&value, &pattern);
        match (expected, result) {
            ("true", true) => None,
            ("false", false) => None,
            _ => Some(msg.clone()),
        }
    }

    macro_rules! template_matches_gen {
        ($($name:ident),*) => ( $(
            #[test]
            fn $name() {
                let file_contents = include_str!(concat!("../tests/testdata/template/",
                                                         stringify!($name)));
                match template_matches(file_contents.split('\n').map(String::from).collect()) {
                    None => { },
                    Some(s) => assert!(false, s),
                }
            }
        )* )
    }

    template_matches_gen!(
        matches_1,
        matches_2,
        matches_3,
        matches_4,
        matches_5,
        matches_6,
        /* FIXME: invalid wildcards matches_7 , */
        matches_8,
        matches_9,
        matches_10,
        matches_11,
        matches_12,
        matches_13,
        matches_14,
        matches_15,
        matches_16
    );
}
