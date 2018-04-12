use std::fmt;
use std::mem;
use std::str::FromStr;

use error::*;

use serde_json::Value;
type JsObject = ::serde_json::map::Map<String, Value>;

use nom::{self, rest_s, IResult};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        if !s.starts_with("/") && s.len() > 0 {
            return Err(PointerParseError);
        }

        let parts = s.trim_left_matches('/').split('/').map(unescape).collect();

        Ok(Pointer { parts: parts })
    }
}

impl Into<Vec<String>> for Pointer {
    fn into(self) -> Vec<String> {
        self.parts
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

pub struct RelativePointer {
    up: u64,
    down: Down,
}

enum Down {
    Position,
    Pointer(Pointer),
}

fn down_from_str(input: &str) -> ::std::result::Result<Down, PointerParseError> {
    match input {
        "#" => Ok(Down::Position),
        ptr => Ok(Down::Pointer(Pointer::from_str(ptr)?))
    }
}

fn is_digit(i: char) -> bool {
    i.is_digit(10)
}

named!(rel_pointer_parser<&str, (u64, Down)>,
    do_parse!(
        up: map_res!(take_while1_s!(is_digit), u64::from_str)
        >> down: map_res!(rest_s, down_from_str)
        >> (up, down)
    )
);

impl FromStr for RelativePointer {
    type Err = PointerParseError;

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        let (up, down) = match rel_pointer_parser(s) {
            IResult::Done(_, o) => o,
            _ => return Err(PointerParseError),
        };

        Ok(RelativePointer { up: up, down: down })
    }
}

impl RelativePointer {
    pub fn normalize(&self, ptr: &Pointer) -> Option<Pointer> {
        match self.down {
            Down::Position => None,
            Down::Pointer(ref rel) => {
                let mut parts: Vec<String> = ptr.clone().into();
                for i in 0..self.up {
                    if let None = parts.pop() {
                        return None;
                    }
                }

                for part in rel.parts.iter() {
                    parts.push(part.clone());
                }

                Some(Pointer { parts: parts })
            }
        }
    }

    pub fn resolve(&self, value: &Value, ptr: &Pointer) -> Option<Value> {
        match self.down {
            Down::Position => {
                let len = ptr.parts.len() as u64;
                if self.up <= len {
                    let boundary = (len - self.up) as usize;
                    let slice = &ptr.parts[0..boundary];
                    let new_ptr = Pointer { parts: slice.to_vec() };
                    let key = &ptr.parts[boundary-1];
                    match new_ptr.search(value) {
                        Some(&Value::Object(_)) => Some(Value::String(key.to_string())),
                        Some(&Value::Array(_)) => match u64::from_str(key) {
                            Ok(idx) => Some(Value::Number(idx.into())),
                            _ => None,
                        }
                        _ => None,
                    }
                } else { None }
            },
            Down::Pointer(_) => {
                self.normalize(ptr)
                    .and_then(|ptr| ptr.search(value))
                    .map(|v| v.clone())
            }
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn relative_resolve_self() {
        let rel = RelativePointer::from_str("0").unwrap();
        let ptr = Pointer::from_str("/foo").unwrap();
        let input = json!({"foo": "bar"});
        let result = rel.resolve(&input, &ptr).unwrap();
        assert_eq!("bar", result);
    }
}