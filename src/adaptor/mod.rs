mod ini;

use std::collections::BTreeMap;
use std::io::{Read, Write};

use serde_json::Value;

// #[derive(Debug, Eq, PartialEq)]
// pub enum Value {
//     Bool(bool),
//     Number(String),
//     Text(String),
//     Array(Vec<Value>),
//     Object(BTreeMap<String, Value>),
// }

type AdaptorError = &'static str;
type AResult<T> = Result<T, AdaptorError>;

pub trait Adaptor<'a> {
    fn serialize<W: Write>(&self, conf: Value, writer: W) -> AResult<()>;
    fn deserialize<R: Read>(&self, reader: R) -> AResult<Value>;
}

