# Reconfix JSON Schema Extensions

This document describes the Reconfix extensions to the JSON Schema format. These extensions are meant to allow the use of JSON Schema as a description of a generic JSON structure that represents the configuration state of a computer system and it's software.

This document makes use of the existing draft specification for [JSON Schema Core](https://tools.ietf.org/html/draft-handrews-json-schema-00) and [JSON Schema Validation](https://tools.ietf.org/html/draft-handrews-json-schema-validation-00).

## Introduction

Reconfix is a library and a schema for mapping a generic JSON document into one or more documents with varying file formats. This generic "dry" JSON document represents the configuration state of one or more pieces of software. The Reconfix library's purpose is to use the schema to interpret the "dry" JSON document and convert it into formatted files that can be ingested by a variety of software programs.

This extension seeks to leverage the extensive capabilities of JSON Schema for verifying that a "dry" JSON document is in a format that is considered "valid" by the system. The validation information in the JSON Schema can be used as metadata for a variety of purposes, such as dynamically rendered UX.

# Assumptions

This extension to the JSON Schema specification relies on the assumption that other implementations of JSON Schema will ignore unknown keywords on JSON Schema objects. If a JSON Schema implementation does not fulfill this requirement, it will not be interoperable with Reconfix extended schemas.

Additionally, this exension will encapsulate all details in a special key on schema objects called "reconfix". If any JSON Schema implementation or extension uses or modifies the contents of the special key, it may not be compatible with Reconfix.

# JSON Pointers

This specification uses [JSON pointers](https://tools.ietf.org/html/rfc6901) for certain values. JSON pointers are used to refer to a specific value in a JSON document by navigating from the root. Additionally, this specification also makes use of [relative JSON Pointers](https://tools.ietf.org/html/draft-handrews-relative-json-pointer-01).

# Dry and Wet Formats

This document refers to "dry" and "wet" representations. The "dry" representation is intended to be an abstract representation of the configuration of a system, and is encoded in standard JSON syntax. The Reconfix schema contains instructions for transforming between a single "dry" state and multiple "wet" representations.

The "wet" representation is an intermediate representation of the content one or more application specific configuration files, such as `json` or `ini`. This "wet" state is internal to Reconfix. The format of this representation is an implementation detail. For the purposes of  specifying conversions, the structure of a "wet" representation can be assumed to follow a JSON document. However, the types and structure may restricted based on the final format of the application-specific configuration file. The "wet" representation is converted into the application-specific format as a final action by Reconfix. 

# Wildcards

When specifying "wet" JSON values in the Reconfix schema, there may be situations where it is known that a field will contain a type of value, but the exact value is unknown. In this situation, a "wildcard" value may be used. Wildcard values are JSON strings that contain only openning and closing double brackets, surrounding the JSON type name that is expected. For example, a number wildcard is represented as `"[[number]]"` and a string is represented as `"[[string]]"`. 

Available wildcards:

* `[[boolean]]`
* `[[string]]`
* `[[number]]`
* `[[object]]`
* `[[array]]`
* `[[any]]`

# Substitution

When processing the schema rules, there may be situtations where the value to be output will depend on another value. To reference other values in the "dry" JSON structure, a relative JSON pointer may be included in any wet values specified in "map" or "const". Relative JSON pointers may also be used in the "path" field of the the "output" key. Substitutions must wrap the relative JSON pointer in double brackets. 

Example:

```json
{
    "reconfix": {
        "output": {
            "file": { "$ref": "#/reconfix/files/connections" },
            "path": "[[1/name]]/wifi"
        }
    }
}
```


# The Reconfix Object

All Reconfix extension data is encapsulated in the Reconfix object. The Reconfix object MUST always be contained in the special key "reconfix". This special key may exist on any JSON Schema object. The existence of the special key indicates that the JSON Schema contains extended Reconfix metadata, and that the Reconfix library should make use of this schema when performing transformations. While there is no restriction on which JSON Schema objects may contain the special key, the process of transforming data may be contextual, based on the location of the Reconfix object within the JSON Schema document.

Example: 

```json
{
    "$schema": "http://resin.io/reconfix/schema#",
    "title": "A JSON chema instance with Reconfix extensions",
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

The file object contains information about file format and location. The object must have a "format" key, which can contain the values "json" or "ini". More file formats may be added in the future. The file object must also contain a "location" property with an object value. This object is the Location object.

Example:

```json
{
    "config_json": {
        "format": "json",
        "location": { ... }
    }
}
```

### Location Object Properties

Location objects are categorized into two implicit types: disk file and nested. Disk file location objects are identified by containing a "partition" field with a positive integer value and a "path" field with a UNIX-style file path in a string. The "partition" field is used to indicate which 1-indexed partition the file is located in. The "path" field is used to locate the file within the partition.

Example:

```json
{
    "location": {
        "partition": 1,
        "path": "/config.json",
    }
}
```

Disk files may also operate in "fileset" mode. In this mode, the path on a disk file location must refer to a directory. When in this mode, the file contents of all files in the directory are combined into one "wet" JSON document. Each file

Nested location objects represent formatted files which are packaged in a string within another file. The nested location object is identified by containing a "parent" filed with a JSON Schema fragment reference to the file where this file will be embedded. The object must also contain a "path" field, which will must be a JSON pointer directed to the location in the "parent" file document where the nested file will be included.

```json
{
    "location": {
        "parent": { "$ref": "#/reconfix/files/config.json" },
        "path": "/foo/bar"
    }
}
```

## Transforms

Transforms describe bi-directional mappings between the "dry" JSON document representing the configuration state and the "wet" intermediate JSON representation. Transforms can have various rules and conditions for processing, allowing for a variety of ways to convert configuration values. Transforms will perform their operations on the "current" JSON value that is being verified by the JSON Schema. Transforms will have access to a variety of contextual information through certain variables.

### The "transforms" Keyword

Transforms are defined using the "transforms" keyword on the Reconfix object. The "transforms" property will contain an array value. This array will be populated with transform objects. These objects will represent individual transform operations that are processed in the order they appear in the array.

### The Transform Object

The Transform object contains multiple keys that instruct the Reconfix library how to map the current JSON value into one or more "wet" JSON documents. 

### The "when" Keyword

First, the transform object may contain a "when" field, that is used as a predicate to determine if the transform should be applied or skipped. The value for the when property must be either a valid JSON Schema object or a reference to a JSON Schema fragment. The referenced schema will be tested against the root "dry" JSON document that is undergoing transformation. When converting backwards from the "wet" JSON document intermediate representation to the "dry" JSON document, all transforms will be speculatively processed. All "when" schemas will then be tested at the end to ensure all conditions are satisfied. If there is no "when" property, the transform will always be executed.

Example 1:

```json
{
    "when": {
        "type": "object",
        "properties": {
            "wifi": {
                "type": "object",
                "properties": {
                    "enable": {
                        "value": true
                    }
                }
            }
        }
    }
}
```

Example 2:

```json
{
    "when": { "$ref": "#/definitions/wifi_enable" }
}
```

### The "map" Keyword

Second, the tranform object may contain a "map" field, which describes how data is mapped between "dry" and "wet" representation. There are multiple types of values that can be used with this field. The first is a "match ladder", which is an array of 2-tuples. The value in the first position of each tuple is compared with the current "dry" JSON value. If the values match, the value in the second position is used in the target "wet" JSON document. The value is compared either as a literal JSON value or as a wildcard value. The special value `"identiy"` can also be used inside the match ladder or in place of the match ladder. If the "map" field is left out, the `"identity"` mapping is implied.

Example 1:

```json
{
    "map": "identity"
}
```

Example 2:

```json
{
    "map": [
        [true, "yes"],
        [false, "no"],
        "identity",
    ]
}
```

Example 3:

```json
{
    "map": [
        [false, {}],
        [true, {
            "wifi": {
                "ssid": "[[string]]"
            },
            "wifi-security": {
                "psk": "[[string]]"
            }
        }],
    ]
}
```

### The "output" keyword

Third, the transform object must contain an "output" field. The output field must contain the output object. The output object must contain a "file" field which refers must contain a file object or a JSON Schema fragment reference to a file object. This file is the destination file where the "wet" value will be placed. The output object may also contain a "path" field. The "path" field must contain a JSON pointer value, which points to the location in the "wet" document to insert the selected "wet" value. If the "path" field is ommitted, the path will default to the root path `"/"`.

Example:

```json
{
    "output": {
        "file": { "$ref": "#/reconfix/files/config_json" },
        "path": "/foo/bar"
    }
}
```

### The "const" keyword

If the "wet" JSON value is static and does not require any mapping to a "dry" JSON value, the "const" key may be used. The value of the "const" key will always be inserted if any dry value exists. The value may also contain wildcard values. If any wildcard values fail validation, the entire process will fail.

# Mapping Examples

Provided here are some full examples of a Reconfix schema, the "dry" JSON document that is input, and the "wet" output document.

Schema:

```json
{
    "$schema": "http://resin.io/schemas/v1/reconfix.json",
    "reconfix": {
        "files": {
            "config_json": {
                "format": "json",
                "location": {
                    "partition": 1,
                    "path": "/config.json"
                }
            },
            "system_connections": {
                "format": "ini",
                "location": {
                    "partition": 1,
                    "path": "/system-connections",
                    "fileset": true
                }
            }
        }
    },
    "type": "object",
    "properties": {
        "persistentLogging": {
            "type": "boolean",
            "reconfix": {
                "map": [
                    [true, "true"],
                    [false, "false"]
                ],
                "output": {
                    "file": { "$ref": "#/reconfix/files/config_json" },
                    "path": "/persistentLogging"
                }
            }
        },
        "wifi": {
            "type": "array",
            "items": {
                "type": "object",
                "reconfix": {
                    "const": {
                        "connection": {
                            "id": "[[string]]",
                            "type": "wifi"
                        },
                        "wifi": {
                            "hidden": true,
                            "mode": "infrastructure",
                            "ssid": "[[string]]"
                        },
                        "wifi-security": {
                            "auth-alg": "open",
                            "key-mgmt": "wpa-psk",
                            "psk": "[[string]]"
                        },
                        "ipv4": {
                            "method": "auto"
                        },
                        "ipv6": {
                            "addr-gen-mode": "stable-privacy",
                            "method": "auto"
                        }
                    }
                },
                "properties": {
                    "name": {
                        "type": "string",
                        "reconfix": {
                            "output": {
                                "file": {"$ref": "#/reconfix/files/system_connections"},
                                "path": "/[[1/name]]/connection/id"
                            }
                        }
                    },
                    "ssid": {
                        "type": "string",
                        "reconfix": {
                            "output": {
                                "file": {"$ref": "#/reconfix/files/system_connections"},
                                "path": "/[[1/name]]/wifi/ssid"
                            }
                        }
                    },
                    "key": {
                        "type": "string",
                        "reconfix": {
                            "output": {
                                "file": {"$ref": "#/reconfix/files/system_connections"},
                                "path": "/[[1/name]]/wifi/ssid"
                            }
                        }
                    }
                }
            }
        }
    }
}
```

Dry JSON:

```json
{
    "peristentLogging": true,
    "wifi": [
        {
            "name": "resin-wifi",
            "ssid": "resin",
            "key": "supersecret"
        },
        {
            "name": "other-wifi",
            "ssid": "other",
            "key": "password"
        }
    ]
}
```

Wet JSON for `config_json`:

```json
{
    "persistentLogging": "true"
}
```

Wet JSON for `system_connections`:

```json
{
    "resin-wifi": {
        "connection": {
            "id": "resin-wifi",
            "type": "wifi"
        },
        "wifi": {
            "hidden": true,
            "mode": "infrastructure",
            "ssid": "resin",
        },
        "wifi-security": {
            "auth-alg": "open",
            "key-mgmt": "wpa-psk",
            "psk": "supersecret",
        },
        "ipv4": {
            "method": "auto",
        },
        "ipv6": {
            "addr-gen-mode": "stable-privacy",
            "method": "auto",
        }
    },
    "other-wifi": {
        "connection": {
            "id": "other-wifi",
            "type": "wifi"
        },
        "wifi": {
            "hidden": true,
            "mode": "infrastructure",
            "ssid": "other",
        },
        "wifi-security": {
            "auth-alg": "open",
            "key-mgmt": "wpa-psk",
            "psk": "password",
        },
        "ipv4": {
            "method": "auto",
        },
        "ipv6": {
            "addr-gen-mode": "stable-privacy",
            "method": "auto",
        }
    }
}
```

File `/config.json`:

```json
{
    "persistentLogging": "true"
}
```

File `/system-connections/resin-wifi`:

```ini
[connection]
id=resin-wifi
type=wifi

[wifi]
hidden=true
mode=infrastructure
ssid=resin

[wifi-security]
auth-alg=open
key-mgmt=wpa-psk
psk=supersecret

[ipv4]
method=auto

[ipv6]
addr-gen-mode=stable-privacy
method=auto
```

File `/system-connections/other-wifi`:

```ini
[connection]
id=other-wifi
type=wifi

[wifi]
hidden=true
mode=infrastructure
ssid=other

[wifi-security]
auth-alg=open
key-mgmt=wpa-psk
psk=password

[ipv4]
method=auto

[ipv6]
addr-gen-mode=stable-privacy
method=auto
```


