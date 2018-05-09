
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
    #[serde(rename = "$id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
    pub ref_: Option<String>,
    #[serde(rename = "$comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub examples: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_items: Option<Schema>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<TypeKind<Schema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_items: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique_items: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains: Option<Schema>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_properties: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_properties: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<Schema>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definitions: Option<Map<String, Schema>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Map<String, Schema>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern_properties: Option<Map<String, Schema>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Map<String, Schema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_names: Option<Schema>,
    #[serde(rename = "const", skip_serializing_if = "Option::is_none")]
    pub const_: Option<Value>,
    #[serde(default, rename = "enum", skip_serializing_if = "Option::is_none")]
    pub enum_: Option<Vec<Value>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<TypeKind<Type>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_media_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_encoding: Option<String>,
    #[serde(rename = "if", skip_serializing_if = "Option::is_none")]
    pub if_: Option<Schema>,
    #[serde(rename = "then", skip_serializing_if = "Option::is_none")]
    pub then_: Option<Schema>,
    #[serde(rename = "else", skip_serializing_if = "Option::is_none")]
    pub else_: Option<Schema>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_of: Option<Vec<Schema>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub any_of: Option<Vec<Schema>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub one_of: Option<Vec<Schema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not: Option<Schema>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map: Option<TypeKind<Case>>,
    #[serde(rename = "const", skip_serializing_if = "Option::is_none")]
    pub const_: Option<Schema>,
    pub output: Output,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Case {
    Identity,
    Stringify,
    Tuple(Value, Schema),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Output {
    pub target: String,
    pub path: String,
}
