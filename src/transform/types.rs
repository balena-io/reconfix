use std::str::FromStr;

use error_chain::bail;
// TODO Rust 2018: Remove * when used macros will import other macros they do
// depend on
use nom::*;
use serde_json::Value;
use uuid::Uuid;

use crate::error::*;
use crate::json::Pointer as JsonPointer;
use crate::json::RelativePointer;
use crate::schema::types::Schema;

pub struct Transform {
    pub source: Selector,
    pub target: Target,
    pub destination: Destination,
    pub map: Vec<Case>,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Target {
    File(File),
    NetworkManager,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct File {
    pub format: Format,
    pub location: Location,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Format {
    Json,
    Ini,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Location {
    Disk(DiskFile),
    Nested(NestedFile),
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct DiskFile {
    pub partition: Partition,
    pub path: String,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Partition {
    Number(u8),
    Label(String),
    Id(Uuid),
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct NestedFile {
    pub file: DiskFile,
    pub path: JsonPointer,
}

#[derive(Debug)]
pub enum Case {
    Identity,
    Stringify,
    Test { dry: Option<Value>, test: Test },
}

#[derive(Debug)]
pub struct Test {
    pub literals: Vec<(Vec<String>, Value)>,
    pub schema: Schema,
}

impl Default for Test {
    fn default() -> Test {
        Test::new()
    }
}

impl Test {
    pub fn new() -> Test {
        Test {
            literals: Vec::new(),
            schema: Schema::Boolean(true),
        }
    }
}

#[derive(Clone, Default)]
pub struct Selector {
    components: Vec<Component>,
}

impl Selector {
    pub fn new() -> Selector {
        Selector {
            components: Vec::new(),
        }
    }

    pub fn push(&mut self, c: Component) {
        self.components.push(c)
    }

    pub fn select_values<'a>(&self, v: &'a Value) -> Vec<&'a Value> {
        let init = vec![v];
        self.components.iter().fold(init, |state, component| {
            state
                .into_iter()
                .flat_map(|value| {
                    match (value, component) {
                        (&Value::Object(ref o), &Component::Property(ref s)) => {
                            o.get(s).into_iter().collect::<Vec<&Value>>()
                        },
                        (&Value::Array(ref a), &Component::Item(ref i)) => {
                            match *i {
                                Index::Single(ref i) => {
                                    a.get(*i as usize).into_iter().collect::<Vec<_>>()
                                },
                                Index::Wildcard => a.iter().collect::<Vec<_>>(),
                            }
                        },
                        _ => vec![],
                    }
                }).collect::<Vec<_>>()
        })
    }

    pub fn get_pointer_for_index(&self, idx: u64) -> Result<JsonPointer> {
        let mut index = Some(idx);
        let mut ptr = JsonPointer::new();

        for component in &self.components {
            match *component {
                Component::Property(ref s) => ptr.push(s.to_string()),
                Component::Item(Index::Single(ref idx)) => ptr.push(idx.to_string()),
                Component::Item(Index::Wildcard) => {
                    match index {
                        Some(idx) => {
                            index = None;
                            ptr.push(idx.to_string());
                        },
                        None => bail!("cannot map more than one array index wildcards"),
                    }
                },
            }
        }

        Ok(ptr)
    }
}

#[derive(Clone)]
pub enum Component {
    Property(String),
    Item(Index),
}

#[derive(Clone)]
pub enum Index {
    // TODO Remove allow(dead_code)
    //
    // Single(_) is never constructed, but is used in matches. Keeping it around for now.
    #[allow(dead_code)]
    Single(u64),
    Wildcard,
}

#[derive(Debug, Clone)]
pub struct Destination {
    pub parts: Vec<Identifier>,
}

#[derive(Debug, Clone)]
pub enum Identifier {
    String(String),
    Pointer(RelativePointer),
}

pub enum MatchKey {
    Property(String),
    Index(u64),
}

impl Destination {
    pub fn get_match_matrix(&self, value: &Value) -> Vec<MatchSet> {
        get_matches(value, &self.parts)
    }

    pub fn get_pointer(&self, value: &Value, current: &JsonPointer) -> Result<JsonPointer> {
        let parts = self
            .parts
            .iter()
            .map(|id| {
                match *id {
                    Identifier::String(ref s) => Ok(s.to_string()),
                    Identifier::Pointer(ref ptr) => {
                        let value = ptr
                            .resolve(value, current)
                            .ok_or_else(|| format!("unable to resolve pointer '{}'", ptr))?;

                        match value {
                            Value::String(s) => Ok(s.to_string()),
                            Value::Number(n) => Ok(n.to_string()),
                            _ => bail!("unsported value type"),
                        }
                    },
                }
            }).collect::<Result<Vec<_>>>()?;

        Ok(parts.into())
    }

    pub fn get_match_pointer(&self, set: &MatchSet) -> Result<JsonPointer> {
        let parts =
            self.parts
                .iter()
                .map(|id| {
                    match *id {
                        Identifier::String(ref s) => Ok(s.to_string()),
                        Identifier::Pointer(ref ptr) => {
                            let found =
                                set.keys.iter().find(|pair| pair.0.eq(ptr)).ok_or_else(|| {
                                    format!("unable to resolve pointer '{}'", ptr)
                                })?;

                            match found.1 {
                                MatchKey::Property(ref s) => Ok(s.to_string()),
                                MatchKey::Index(ref n) => Ok(n.to_string()),
                            }
                        },
                    }
                }).collect::<Result<Vec<_>>>()?;

        Ok(parts.into())
    }
}

fn get_matches(value: &Value, identifiers: &[Identifier]) -> Vec<MatchSet> {
    let split = identifiers.split_first();
    let next = split.map(|pair| pair.0);
    let rest = split.map(|pair| pair.1).unwrap_or_else(|| &[]);

    match (value, next) {
        (&Value::Object(ref o), Some(&Identifier::String(ref s))) => {
            o.get(s)
                .map(|v| get_matches(v, rest))
                .unwrap_or_else(Vec::new)
        },
        (&Value::Array(ref a), Some(&Identifier::String(ref s))) => {
            let idx = u64::from_str(s).ok();
            idx.and_then(|i| a.get(i as usize))
                .map(|v| get_matches(v, rest))
                .unwrap_or_else(Vec::new)
        },
        (&Value::Object(ref o), Some(&Identifier::Pointer(ref ptr))) => {
            o.iter()
                .flat_map(|(key, prop)| {
                    let mut match_sets = get_matches(prop, rest);

                    for match_set in &mut match_sets {
                        let pair = (ptr.clone(), MatchKey::Property(key.to_string()));
                        match_set.keys.push(pair);
                    }

                    match_sets
                }).collect::<Vec<_>>()
        },
        (&Value::Array(ref a), Some(&Identifier::Pointer(ref ptr))) => {
            a.iter()
                .enumerate()
                .flat_map(|(index, item)| {
                    let mut match_sets = get_matches(item, rest);

                    for match_set in &mut match_sets {
                        let pair = (ptr.clone(), MatchKey::Index(index as u64));
                        match_set.keys.push(pair);
                    }

                    match_sets
                }).collect::<Vec<_>>()
        },
        _ => vec![MatchSet { keys: Vec::new() }],
    }
}

pub struct MatchSet {
    keys: Vec<(RelativePointer, MatchKey)>,
}

impl MatchSet {
    pub fn apply_matches(&self, ptr: &JsonPointer) -> Result<Vec<(JsonPointer, Value)>> {
        let mut output = Vec::new();

        for &(ref rel, ref key) in &self.keys {
            let value = match *key {
                MatchKey::Property(ref s) => Value::String(s.to_string()),
                MatchKey::Index(ref i) => Value::Number((*i).into()),
            };

            match rel.normalize(ptr) {
                Some(n) => output.push((n.clone(), value)),
                None => bail!("failed to normalize pointer '{}'", rel),
            }
        }

        Ok(output)
    }
}

pub struct DestinationParseError;

impl FromStr for Destination {
    type Err = DestinationParseError;

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        let ids = match parse_ids(s) {
            IResult::Done(_, i) => i,
            _ => return Err(DestinationParseError),
        };

        Ok(Destination { parts: ids })
    }
}

named_attr!(
    #[allow(unreachable_pub)],
    parse_ids<&str, Vec<Identifier>>,
    many0!(
        alt!(
            map!(
                map_res!(
                    preceded!(tag_s!("/"), delimited!(tag_s!("{{"), rest_s, tag_s!("}}"))),
                    RelativePointer::from_str
                ),
                Identifier::Pointer
            ) |
            map!(preceded!(tag_s!("/"), is_not_s!("/")), |s| Identifier::String(s.to_string()))
        )
    )
);
