use std::fmt;
use std::mem;
use std::str::FromStr;

use error::*;

use serde_json::Value;
type JsObject = ::serde_json::map::Map<String, Value>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Pointer {
    parts: Vec<String>,
}

impl fmt::Display for Pointer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "/")?;
        if let Some((last, rest)) = self.parts.split_last() {
            for name in rest {
                write!(f, "{}/", escape(name))?;
            }
            write!(f, "{}", escape(last))?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct PointerParseError;

impl ::std::error::Error for PointerParseError {
    fn description(&self) -> &'static str {
        "invalid JSON pointer"
    }
}

impl fmt::Display for PointerParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pointer must start with '/'")
    }
}

impl FromStr for Pointer {
    type Err = PointerParseError;

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        if !s.starts_with("/") {
            return Err(PointerParseError);
        }

        let parts = s.trim_left_matches('/').split('/').map(unescape).collect();

        Ok(Pointer { parts: parts })
    }
}

fn escape(s: &str) -> String {
    s.replace("~", "~0").replace("/", "~1")
}

fn unescape(s: &str) -> String {
    s.replace("~1", "/").replace("~0", "~")
}

enum DerefValue<'a> {
    Found(&'a mut Value),
    NotFound(&'a mut JsObject),
}

fn dereference<'a>(v: &'a mut Value, k: &str) -> Result<DerefValue<'a>> {
    let (contains, obj) = v.as_object_mut()
        .map(|obj| (obj.contains_key(k), obj))
        .ok_or_else(|| "expected object")?;

    if contains {
        Ok(DerefValue::Found(obj.get_mut(k).unwrap()))
    } else {
        Ok(DerefValue::NotFound(obj))
    }
}

enum FollowState<'a> {
    Real(&'a mut Value),
    Virtual(&'a mut JsObject, Vec<String>),
}

impl Pointer {
    pub fn new() -> Pointer {
        Pointer { parts: Vec::new() }
    }

    pub fn push<S: Into<String>>(&mut self, s: S) {
        self.parts.push(s.into());
    }

    pub fn extend<S: Into<String>>(&self, s: S) -> Pointer {
        let mut next = self.clone();
        next.push(s);
        next
    }

    pub fn search<'a>(&self, v: &'a Value) -> Option<&'a Value> {
        self.parts.iter().fold(Some(v), |state, name| match state {
            None => None,
            Some(json) => match json {
                &Value::Object(ref obj) => obj.get(name),
                _ => None,
            },
        })
    }

    pub fn entry<'a>(&self, v: &'a mut Value) -> Result<Entry<'a>> {
        let state = self.parts.iter().fold(
            Ok(FollowState::Real(v)),
            |r: Result<FollowState<'a>>, name| {
                let state = match r {
                    Ok(x) => x,
                    e => return e,
                };

                match state {
                    FollowState::Virtual(obj, mut path) => {
                        path.push(name.to_string());
                        Ok(FollowState::Virtual(obj, path))
                    }
                    FollowState::Real(v) => {
                        let state = match dereference(v, name)? {
                            DerefValue::Found(next) => FollowState::Real(next),
                            DerefValue::NotFound(obj) => {
                                FollowState::Virtual(obj, vec![name.to_string()])
                            }
                        };

                        Ok(state)
                    }
                }
            },
        )?;

        match state {
            FollowState::Real(v) => Ok(Entry::Occupied(OccupiedEntry { value: v })),
            FollowState::Virtual(v, p) => Ok(Entry::Vacant(VacantEntry { value: v, path: p })),
        }
    }
}

pub enum Entry<'a> {
    Vacant(VacantEntry<'a>),
    Occupied(OccupiedEntry<'a>),
}

pub struct VacantEntry<'a> {
    value: &'a mut JsObject,
    path: Vec<String>,
}

impl<'a> VacantEntry<'a> {
    pub fn insert(self, v: Value) -> &'a mut Value {
        let (mut path, value) = (self.path, self.value);
        let last = path.pop().expect("vec should never be empty");

        let last_obj = path.into_iter().fold(value, |current, name| {
            match current.entry(name).or_insert(json!({})) {
                &mut Value::Object(ref mut obj) => obj,
                _ => panic!("the value should always be an object"),
            }
        });

        last_obj.entry(last).or_insert(v)
    }
}

pub struct OccupiedEntry<'a> {
    value: &'a mut Value,
}

impl<'a> OccupiedEntry<'a> {
    pub fn get_mut(&'a mut self) -> &'a mut Value {
        self.value
    }

    pub fn insert(self, v: Value) -> Value {
        mem::replace(self.value, v)
    }
}
