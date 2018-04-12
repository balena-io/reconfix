
use std::iter;
use std::str::FromStr;

use ::json::Pointer as JsonPointer;
use ::json::RelativePointer;
use ::schema::types::Schema;

use uuid::Uuid;
use serde_json::Value;
use nom::{self, rest_s};


pub struct Transform {
    pub source: Selector,
    pub target: Target,
    pub destination: JsonPointer,
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

pub enum Case {
    Identity,
    Template { dry: Value, template: Schema },
    Value { dry: Value, wet: Value },
}

#[derive(Clone)]
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

    pub fn pop(&mut self) -> Option<Component> {
        self.components.pop()
    }

    pub fn extend(&self, c: Component) -> Selector {
        let mut extended = self.clone();
        extended.push(c);
        extended
    }

    pub fn select_values<'a>(&self, v: &'a Value) -> Vec<&'a Value> {
        let init = vec![v];
        self.components.iter().fold(init, |state, component| {
            state
                .into_iter()
                .flat_map(|value| {
                    match (value, component) {
                        (&Value::Object(ref o), &Component::Property(ref k)) => match *k {
                            Key::Single(ref s) => o.get(s).into_iter().collect::<Vec<&Value>>(),
                            Key::Wildcard => o.values().collect::<Vec<_>>(),
                        },
                        (&Value::Array(ref a), &Component::Item(ref i)) => match *i {
                            Index::Single(ref i) => a.get(*i as usize).into_iter().collect::<Vec<_>>(),
                            Index::Wildcard => a.iter().collect::<Vec<_>>(),
                        }
                        _ => vec![],
                    }
                })
                .collect::<Vec<_>>()
        })
    }
}

#[derive(Clone)]
pub enum Component {
    Property(Key),
    Item(Index),
}

#[derive(Clone)]
pub enum Key {
    Single(String),
    Wildcard,
}

#[derive(Clone)]
pub enum Index {
    Single(u64),
    Wildcard,
}

pub struct Destination {
    pub parts: Vec<Identifier>,
}

pub enum Identifier {
    String(String),
    Pointer(RelativePointer),
}

impl Destination {
    pub fn new() -> Destination {
        Destination { parts: Vec::new() }
    }
}

pub struct DestinationParseError;

impl FromStr for Destination {
    type Err = DestinationParseError;

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        unimplemented!()
    }
}

// named!(parse_identifiers<&str, Vec<Identifier>>,
//     preceded!(
//         opt!(tag_s!("/")),
//         separated_list!(
//             tag_s!("/"),
//             alt!(
//                 map!(
//                     map_res!(
//                         delimited!(tag_s!("{{"), rest_s, tag_s!("}}")),
//                         RelativePointer::from_str
//                     ),
//                     Identifier::Pointer
//                 ),
//                 map!(rest_s, |s| Identifier::String(String::from(s)))
//             )
//         )
//     )
// );

            // alt!(
            //     map!(
            //         delimited!(tag!("{{"), RelativePointer::from_str, tag!("}}")),
            //         Identifier::Pointer),
            //     map!(rest_s, Identifier::String)
            // )