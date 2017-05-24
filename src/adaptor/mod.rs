mod ini;

use std::collections::HashMap;
use std::io::{Read, Write};

#[derive(Debug, Eq, PartialEq)]
pub enum Config {
    Bool(bool),
    Number(String),
    Text(String),
    Array(Vec<Config>),
    Object(HashMap<String, Config>),
}

type AdaptorError = &'static str;

pub trait Adaptor<'a> {
    fn serialize<W: Write>(&self, conf: Config, writer: W) -> Result<(), AdaptorError>;
    fn deserialize<R: Read>(&self, reader: R) -> Result<Config, AdaptorError>;
}

