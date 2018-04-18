use adaptor::{Adaptor, IniAdaptor, JsonAdaptor};
use error::*;

use std::io;

use serde_json::Value;

/// Represents a partition within a partition scheme
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Partition(u8);

impl Partition {
    /// Constructor for a partition
    pub fn new(p: u8) -> Partition {
        Partition(p)
    }

    /// Get the partition number for a partition
    pub fn num(&self) -> u8 {
        self.0
    }
}

/// Represents the location of a file in a partition scheme
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct FileNode {
    /// The components of the file path on the filesystem
    pub path: Vec<String>,
    /// The partition containing the file
    pub partition: Partition,
}

/// Supported output file formats
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum FileFormat {
    Ini,
    Json,
}

impl FileFormat {
    pub fn from_str(s: &str) -> Result<FileFormat> {
        match s {
            "ini" => Ok(FileFormat::Ini),
            "json" => Ok(FileFormat::Json),
            _ => bail!("unknown file format"),
        }
    }
}

/// Convert wet JSON into a raw `String` using the formatter appropriate
/// for the provided `FileFormat`.
pub fn serialize<W>(content: Value, format: &FileFormat, pretty: bool, mut out: W) -> Result<()>
where
    W: io::Write,
{
    //let mut buffer = Vec::new();
    match format {
        &FileFormat::Ini => {
            let adaptor = IniAdaptor::new();
            adaptor.serialize(content, &mut out)
        }
        &FileFormat::Json => {
            let adaptor = JsonAdaptor::new(pretty);
            adaptor.serialize(content, &mut out)
        }
    }
    //String::from_utf8(buffer).chain_err(|| "unable to decode utf-8")
}

/// Deserialize raw text using the appropriate formatter for the
/// `FileFormat` and return the wet JSON.
pub fn deserialize<R>(content: R, format: &FileFormat) -> Result<Value>
where
    R: io::Read,
{
    match format {
        &FileFormat::Ini => {
            let adaptor = IniAdaptor::new();
            adaptor.deserialize(content)
        }
        &FileFormat::Json => {
            let adaptor = JsonAdaptor::new(false);
            adaptor.deserialize(content)
        }
    }
}