mod ini;

use std::io::{Read, Write};

use errors::*;

use serde_json::Value;

// #[derive(Debug, Eq, PartialEq)]
// pub enum Value {
//     Bool(bool),
//     Number(String),
//     Text(String),
//     Array(Vec<Value>),
//     Object(BTreeMap<String, Value>),
// }

// type AdaptorError = &'static str;
// type AResult<T> = Result<T, ErrorKind>;

pub trait Adaptor<'a> {
    fn serialize<W: Write>(&self, conf: Value, writer: W) -> Result<()>;
    fn deserialize<R: Read>(&self, reader: R) -> Result<Value>;
}
