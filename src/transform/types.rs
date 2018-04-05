
use ::json::Pointer as JsonPointer;
use ::schema::types::Schema;

use uuid::Uuid;
use serde_json::Value;


pub struct Transform {
    pub source: JsonPointer,
    pub target: Target,
    pub map: Vec<Case>,
}

pub enum Target {
    File(File),
    NetworkManager,
}

pub struct File {
    pub format: Format,
    pub location: Location,
}

pub enum Format {
    Json,
    Ini,
}

pub enum Location {
    Disk(DiskFile),
    Nested(NestedFile),
}

pub struct DiskFile {
    pub partition: Partition,
    pub path: String,
}

pub enum Partition {
    Number(u8),
    Label(String),
    Id(Uuid),
}

pub struct NestedFile {
    pub file: DiskFile,
    pub path: JsonPointer,
}

pub enum Case {
    Identity,
    Value { dry: Value, wet: Schema }
}