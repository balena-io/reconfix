Schema Specification
====================

This document aims to describe in detail the syntax of schemas.

Schemas are based on the JSON format, and declaratively describe how clients
should move among the various transformation phases.

Example
-------

This is a complete example of a configuration schema:

```json
{
  "system_connections": {
    "fileset": true,
    "type": "ini",
    "location": {
      "path": [ "system-connections" ],
      "partition": {
        "primary": 4,
        "logical": 1
      }
    },
    "properties": [
      {
        "definition": {
          "cellularConnectionName": {
            "type": [ "string" ],
            "mapping": [
              [ "cellular", "connection", "name" ]
            ]
          },
          "ethernetConnectionName": {
            "type": [ "string" ],
            "mapping": [
              [ "ethernetConnectionName", "connection", "name" ]
            ]
          },
          "wifiConnectionName": {
            "type": [ "string" ],
            "mapping": [
              [ "wifi", "connection", "name" ]
            ]
          }
        }
      }
    ]
  }
}
```

Structure
---------

Schemas are modelled with a top level JSON object, including various object
properties that describe each of the files the schema is concerned with.

For example:

```js
{
  "file_foo", {
    ...
  },
  "file_bar", {
    ...
  },
  "file_baz", {
    ...
  }
}
```

File Names
----------

Schemas contain object properties defining files. The property names should
adhere to the following rules:

- Must be unique in the schema
- Must consist of alphanumeric characters and underscores only

Valid file names:

- `config_txt`
- `resin_config_network_config`

File Objects
------------

File object properties consist of the following properties:

### `String type`

The name of a valid connector, which is able to serialize and deserialize the
desired file format.

For example:

- `ini`
- `json`

If the referenced connector doesn't exist, then the schema is considered
invalid.

### `Boolean fileset` (default to `false`)

This property determines whether the selected connector should behave in
"fileset" mode.

If this is the case, `location.path` should be a directory, and each file
inside it is parsed as its own independent file.

### `Object location`

This object describes the location of the in question. Note that a file can be
independent, or can be inlined inside of another file (dependent).

Note that mixing properties from independent and dependent location is
considered invalid.

#### Independent Location

For example:

```json
"location": {
  "path": [ "settings", "config.json" ],
  "partition": {
    "primary": 1
  }
}
```

When an object location is independent, then the following properties are expected:

##### `String[] path`

The path of the file, where each portion of the path is a different element of
the array.

- **This propery can't be an empty array**
- If the directory name of the specified path doesn't exist, then the schema
  client should recursively create it

For example

```json
"path": [ "foo", "bar", "baz.json" ]
```

##### `Object partition`

The partition where the file is located. The partition object might include the
following properties:

- `Number primary`: the primary partition number, which must be less or equal
  than 4
- `Number logical`: the logical partition number

For example:

```json
"partition": {
  "primary": 1
}
```

***

```json
"partition": {
  "primary": 4,
  "logical": 2
}
```

#### Dependent Location

When an object location is independent, then the following properties are expected:

##### `String parent`

The name of the parent file. This must equal one of the top level schema
properties.

For example

```js
{
  "top_level_file": {
    ...
  },
  "nested_file": {
    "location": {
      "parent": "top_level_file"
    }
  }
}
```

If the referenced file doesn't exist, then the schema is considered invalid.

##### `String[] property`

The path of the property of the parent file where this file should be inlined.

- **This propery can't be an empty array**

For example:

```js
{
  "nested_file": {
    "location": {
      "parent": "top_level_file",
      "path": [ "foo" ]
    }
  }
}
```

This means that `nested_file` will be serialised, and will be stored as a
string inside the `foo` property of the `top_level_file` file.

### `Object[] properties`

This is the crux of schemas, and describes how to map dry and wet properties.

The array contains items that include the following properties:

#### `Object definition`

The property definition, which contains top level objects whose property names:

- Must be unique in the described file
- Must consist of alphanumeric characters and underscores only

For example:

```js
"properties": [
  {
    "definition": {
      "myFooProperty", {
        ...
      },
      "myBarProperty", {
        ...
      }
    }
  }
]
```

#### `Object when` (optional)

An object describing when this particular properties object should be applied.
This object can refer to any dry property.

For example:

```
"properties": [
  {
    "when": {
      "wifi": true
    },
    "definition": {
      "myFooProperty": {
        ...
      },
      "myBarProperty": {
        ...
      }
    }
  },
  {
    "when": {
      "wifi": false
    },
    "definition": {
      "myFooProperty": {
        ...
      },
      "myBarProperty": {
        ...
      }
    }
  }
]
```

If the `when` property is not defined, then the properties in the object item
in question always applies.

Note that many object items can be true at the same time unless they disagree
in one or more properties.

Property Objects
----------------

Property objects consist of the following properties:

### `String[] type`

- **This propery can't be an empty array**

The types of the property. Valid property types are:

- `string`
- `number`
- `boolean`

This property is an array of strings since a single property may support
multiple types.

For example:

```json
"properties": {
  "myProperty": {
    "type": [ "string", "number" ]
  }
}
```

### `(Array|Object)[] mapping`

- **This propery can't be an empty array**

This property defines the mapping between the dry and wet states. Schemas
support two types of mappings:

- Direct mappings
- Template mappings

#### Direct Mappings

Direct mappings are describes as array of strings.

For example:

```json
"mapping": [
  [ "path", "to", "property" ]
]
```

A `mapping` property may contain multiple direct mappings, and empty direct
mappings are considered invalid.

#### Template Mappings

For example:

```json
"mapping": [
  {
    "value": "foo",
    "template": {
      "my": {
        "template": {
          "is": "foo"
        }
      }
    }
  },
  {
    "value": "bar",
    "template": {
      "my": {
        "template": {
          "is": "bar"
        }
      }
    }
  }
]
```

Template mappings are described with objects that may contain the following
properties:

##### `Any value`

The value that must hold for the template to be applied. The value may be of
any supported type:

- `string`
- `number`
- `boolean`

##### `Object template`

The actual template that must map to the dry representation. This object can't
be empty, and is user specific.

#### Template Wildcards

When using template mappings, you might need to match the existence of a
certain property, but you don't know the value in advance.

For these cases, templates values may include string wildcards in the following
form:

```
[[<type/s>]]
```

Where `type` can be:

- `string`
- `number`
- `boolean`
- `object`
- `array`
- `*` (matches any type)

Wildcards may also include multiple types separated by `|` symbols. For
example:

```
[[string|number]]
```

Note that wildcards can't be part of other strings. The following notation is
considered invalid:

```json
"mapping": [
  {
    "value": "foo",
    "template": {
      "hello": "foo [[string]]"
    }
  }
]
```

See the following table for some examples of how wildcards are applied:

| Template                       | Source            | Matches |
|--------------------------------|-------------------|---------|
| `"foo": "[[string]]"`          | `"foo": "bar"`    | `true`  |
| `"foo": "[[string]]"`          | `"foo": 1`        | `false` |
| `"foo": "[[*]]"`               | `"foo": false`    | `true`  |
| `"foo": "[[number\|boolean]]"` | `"foo": true`     | `true`  |
| `"foo": "[[number\|boolean]]"` | `"foo": 5`        | `true`  |
| `"foo": "[[number\|boolean]]"` | `"foo": "bar"`    | `false` |
| `"foo": "[[object]]"`          | `"foo": [ 1, 2 ]` | `false` |
