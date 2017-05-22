mod ini;

use std::io::{Read, Write};

pub enum Config {
    Bool(bool),
    Number(f64),
    Text(String),
    Array(Vec<Config>),
    Object(Vec<(String, Config)>),
}

pub trait Adaptor {
    fn read<R: Read>(reader: Read) -> Result<Config, String>;
    fn write<W: Write>(conf: Config, writer: W) -> Result<(), String>;
}
