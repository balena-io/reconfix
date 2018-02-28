# Reconfix JSON Schema Extensions

This document describes the Reconfix extensions to the JSON Schema format. These extensions are meant to allow the use of JSON Schema as a description of a generic JSON structure that represents the configuration state of a computer system and it's software.

This document makes use of the existing draft specification for [JSON Schema Core](https://tools.ietf.org/html/draft-handrews-json-schema-00) and [JSON Schema Validation](https://tools.ietf.org/html/draft-handrews-json-schema-validation-00).

## Introduction

Reconfix is a library and a schema for mapping a generic JSON document into one or more documents with varying file formats. This generic "dry" JSON document represents the configuration state of one or more pieces of software. The Reconfix library's purpose is to use the schema to interpret the "dry" JSON document and convert it into formatted files that can be ingested by a variety of software programs.

This extension seeks to leverage the extensive capabilities of JSON Schema for verifying that a "dry" JSON document is in a format that is considered "valid" by the system. The validation information in the JSON Schema can be used as metadata for a variety of purposes, such as dynamically rendered UX.

# Assumptions

This extension to the JSON Schema specification relies on the assumption that other implementations of JSON Schema will ignore unknown keywords on JSON Schema objects. If a JSON Schema implementation does not fulfill this requirement, it will not be interoperable with Reconfix extended schemas.

Additionally, this exension will encapsulate all details in a special key on schema objects called "reconfix". If any JSON Schema implementation or extension uses or modifies the contents of the special key, it may not be compatible with Reconfix.

# The Reconfix Object

All Reconfix extension data is encapsulated in the Reconfix object. The Reconfix object MUST always be contained in the special key "reconfix". This special key may exist on any JSON Schema object. The existence of the special key indicates that the JSON Schema contains extended Reconfix metadata, and that the Reconfix library should make use of this schema when performing transformations. While there is no restriction on which JSON Schema objects may contain the special key, the process of transforming data may be contextual, based on the location of the Reconfix object within the JSON Schema document.

Example: 

```json
{
    "$id": "http://example.com/reconfix.json",
    "title": "A Reconfix extended schema",
    "reconfix": { ... }
}
```

## Files

### The "files" Keyword

The "files" keyword defines a list of target files into which transformed and formatted data may be mapped into. The "files" key must contain an object. Each key in this object represents the "name" or lexical identifier of a file. The file name does not have any bearing on a file's physical location in a filesystem. It is purely for reference. As such, file names must be unique within the same object. Each file name identifier property must have an object value. This object contains information about the file type and location.

Example:

```json
{
    "files": {
        "config_json": { ... },
        "config_txt": { ... }
    }
}
```

### File Object Properties

The file object contains information about file format and location. File formats




