//! Schema

#![allow(missing_docs)]
use template;
use std::collections::BTreeMap;
use serde_json::Value;

/// Schema
pub struct Schema {
    files: BTreeMap<String, File>,
}

/// Supported output file formats
pub enum FileFormat {
    Ini,
    Json,
}

/// File
pub struct File {
    /// The file format to generate
    format: FileFormat,
    /// Whether to operate in fileset mode
    fileset: bool,
    /// The location of the file on the filesystem
    location: Location,
    /// The properties defining how this file is transformed
    properties: Vec<Property>,
}

/// A partition where a filesystem can be found
pub enum Partition {
    /// A primary partition
    Primary(u64),
    /// A logical partitoin
    Logical {
        /// The primary partition containing the logical partition table
        on_primary: u64,
        /// The partition number in the logical partition table
        logical_number: u64,
    },
}

/// Location
pub enum Location {
    Independent {
        /// The path on the filesystem where the contents while be written
        path: Vec<String>,
        /// The partition containing the filesystem `path` is relative to
        partition: Partition,
    },
    Dependent {
        /// A JSON Pointer indicating where the file contents will be inlined
        location: String,
    },
}

/// Property
pub struct Property {
    definition: BTreeMap<String, PropertyDefinition>,
    when: Option<Value>,
}

pub enum PropertyType {
    String,
    Number,
    Boolean,
}

/// Property definition
pub struct PropertyDefinition {
    types: Vec<PropertyType>,
    properties: Vec<Property>,
    mapping: Vec<template::Mapping>,
}
