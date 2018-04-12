
use std::collections::BTreeMap;

use ::json::Pointer as JsonPointer;

use serde_json::Value;

pub type Map<K, V> = BTreeMap<K, V>;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Type {
    Array,
    Boolean,
    Integer,
    Null,
    Number,
    Object,
    String,
}

//#[derive(Deserialize)]
#[derive(Clone, Debug, PartialEq)]
pub enum Schema {
    Boolean(bool),
    Object(Box<ObjectSchema>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeKind<T> {
    Single(T),
    Set(Vec<T>),
}

#[derive(Clone, Debug, Deserialize, Default, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectSchema {
    #[serde(rename = "$id")]
    pub id: Option<String>,
    #[serde(rename = "$schema")]
    pub schema: Option<String>,
    #[serde(rename = "$ref")]
    pub ref_: Option<String>,
    #[serde(rename = "$comment")]
    pub comment: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub default: Option<Value>,
    #[serde(default)]
    pub read_only: bool,
    #[serde(default)]
    pub examples: Vec<Value>,
    pub multiple_of: Option<f64>,
    pub maximum: Option<f64>,
    pub exclusive_maximum: Option<f64>,
    pub minimum: Option<f64>,
    pub exclusive_minimum: Option<f64>,
    pub max_length: Option<u64>,
    #[serde(default)]
    pub min_length: u64,
    pub pattern: Option<String>,
    pub additional_items: Option<Schema>,
    pub items: Option<TypeKind<Schema>>,
    pub max_items: Option<u64>,
    #[serde(default)]
    pub min_items: u64,
    #[serde(default)]
    pub unique_items: bool,
    pub contains: Option<Schema>,
    pub max_properties: Option<u64>,
    pub min_properties: Option<u64>,
    #[serde(default)]
    pub required: Vec<String>,
    pub additional_properties: Option<Schema>,
    #[serde(default)]
    pub definitions: Map<String, Schema>,
    #[serde(default)]
    pub properties: Map<String, Schema>,
    #[serde(default)]
    pub pattern_properties: Map<String, Schema>,
    #[serde(default)]
    pub dependencies: Map<String, Schema>,
    pub property_names: Option<Schema>,
    #[serde(rename = "const")]
    pub const_: Option<Value>,
    #[serde(rename = "enum")]
    #[serde(default)]
    pub enum_: Vec<Value>,
    #[serde(rename = "type")]
    pub type_: Option<TypeKind<Type>>,
    pub format: Option<String>,
    pub content_media_type: Option<String>,
    pub content_encoding: Option<String>,
    #[serde(rename = "if")]
    pub if_: Option<Schema>,
    #[serde(rename = "then")]
    pub then_: Option<Schema>,
    #[serde(rename = "else")]
    pub else_: Option<Schema>,
    #[serde(default)]
    pub all_of: Vec<Schema>,
    #[serde(default)]
    pub any_of: Vec<Schema>,
    #[serde(default)]
    pub one_of: Vec<Schema>,
    pub not: Option<Schema>,
    pub reconfix: Option<Reconfix>,
}

#[derive(Clone, Debug, Deserialize, Default, PartialEq, Serialize)]
pub struct Reconfix {
    #[serde(default)]
    pub targets: Map<String, Target>,
    #[serde(default)]
    pub transforms: Vec<Transform>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum Target {
    #[serde(rename = "file")]
    File { 
        format: Format,
        location: Location,
    },
    #[serde(rename = "network_manager")]
    NetworkManager,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Format {
    Json,
    Ini,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Location {
    Disk {
        partition: Partition,
        path: String,
    },
    Nested {
        file: String,
        path: String,
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Partition {
    Number(u8),
    String(String),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Transform {
    pub map: Option<TypeKind<Case>>,
    pub output: Output,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Case {
    Identity,
    Tuple(Value, Schema),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Output {
    pub target: String,
    pub path: String,
}
