mod ini;

use std::collections::HashMap;
use std::io::{Read, Write};

#[derive(Debug, Eq, PartialEq)]
pub enum Value {
    Bool(bool),
    Number(String),
    Text(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

type AdaptorError = &'static str;
type AResult<T> = Result<T, AdaptorError>;

pub trait Adaptor<'a> {
    fn serialize<W: Write>(&self, conf: Value, writer: W) -> AResult<()>;
    fn deserialize<R: Read>(&self, reader: R) -> AResult<Value>;
}

