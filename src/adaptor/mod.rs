mod ini;

use std::io::{Read, Write};

use errors::*;

use serde_json::{from_reader, to_writer_pretty};
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

pub use self::ini::IniAdaptor;

pub trait Adaptor<'a> {
    fn serialize<W: Write>(&self, conf: Value, writer: W) -> Result<()>;
    fn deserialize<R: Read>(&self, reader: R) -> Result<Value>;
}

pub struct JsonAdaptor {}

impl JsonAdaptor {
    pub fn new() -> JsonAdaptor {
        JsonAdaptor {}
    }
}

impl<'a> Adaptor<'a> for JsonAdaptor {
    fn deserialize<R>(&self, reader: R) -> Result<Value>
    where
        R: Read,
    {
        from_reader(reader).chain_err(|| "unable to deserialize JSON")
    }

    fn serialize<W>(&self, value: Value, writer: W) -> Result<()>
    where
        W: Write,
    {
        to_writer_pretty(writer, &value).chain_err(|| "unable to serialize JSON")
    }
}
