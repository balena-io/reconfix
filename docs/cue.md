# The CUE Language

CUE (https://cuelang.org/) is a data interchange format with an unified data and type model. Values (i.e. the string "text" or the number 10) and types (i.e. strings or non-negative numbers) are treated the same. CUE also has a procedural scripting language and advanced tooling.

This document is meant to be a practical guide to CUE, independent of the official documentation.

## Basic Syntax

A CUE document (file extension `.cue`) has a structure similar to JSON. In fact, every JSON document is a valid CUE document. But CUE extends JSON syntax by:

- Allowing single-line C-style comments (`//`)
- Allowing field names to be written unquoted if they are valid identifiers. Identifiers are only composed of letters, numbers, `_` and `$`, must start with a letter, and are not one of the single characters `_` or `$`
- Allowing commas at the end of lists
- Making commas at the end of fields optional before a newline
- Making the outermost curly braces optional
- Making curly braces optional when there is only one field
- Allowing duplicate field names

As a comparison, this JSON document:

```json
{
    "id": 1234,
    "tags": [
        "release"
    ],
    "version": {
        "major": 0,
        "minor": 1,
        "patch": 78
    }
}
```

Is equivalent to this CUE document:

```
id: 1234
tags: [
    "release",
]
version: {
    major: 0
    minor: 1
}
version: patch: 78
```

## Basic Tooling Usage

The official tools are all encapsulated by the `cue` command. CUE requires [Go](https://golang.org/), and `cue` can be installed with:

```
$ go get -u cuelang.org/go/cmd/cue
```

Alternatively, clone CUE from [GitHub](https://github.com/cuelang/cue) and run `cue` as:

```
$ go run ./cmd/cue/
```

from the root of the repository.

`cue` has multiple subcommands but the most important for this document are:

- `cue def`: consolidates one or more CUE files
- `cue export`: export a CUE file to other formats
- `cue vet`: verify whether a file is valid according to a CUE schema

## Constraints

In CUE, values and types are treated as a single concept. This is expressed as constraints. Thus, CUE files can both transport data and define schemas in a single language. In a way this is similar to how JSON Schema uses plain JSON, but CUE extends JSON and so defining these constraints are much nicer syntactically.

Constraints are defined by values, types, comparison operators, logical operators, lattice operators, and the optional operator (`?`). For example:

```
id: =~"^[[:xdigit:]]{8}(?:-[[:xdigit:]]{4}){3}-[[:xdigit:]]{12}$"
name: string & !=""
version?: {
    major: 1
    minor: uint
    patch: uint
    tag?: string & !=""
}
```

Lots of things happening here. Let's go line by line:

```
id: =~"^[[:xdigit:]]{8}(?:-[[:xdigit:]]{4}){3}-[[:xdigit:]]{12}$"
```

This declares a required field named `id` which must match a regex (`=~` operator). The regex in this example matches an UUID. Note that we only specify the right side of a comparison operator since we are always comparing that to the field itself.

```
name: string & !=""
```

This declares a required field `name`, which must be a non-empty string. In CUE just a type name is a constraint, meaning that any value must be of that type. `&` is called the *unification* operator, and is one of two *lattice* operators in CUE (the other being disjunction, `|`). A treatment of [lattice theory](https://en.wikipedia.org/wiki/Lattice_(order)) and [how CUE applies lattices](https://cuelang.org/docs/concepts/logic/) is out of scope for this document, but the gist of it is that `&` and `|` are similar to `&&` and `||`, the difference being that `&`/`|` are applied on constraints, while `&&`/`||` are applied on boolean values.

```
version?: {
    major: 1
    minor: uint
    patch: uint
    tag?: string & !=""
}
```

This declares an optional (`?` operator) field called `version`. If it exists, it must be a structure containing `major`, `minor`, `patch`, and optionally `tag` fields. `major` must be equal to the number `1`. Similar to types, values are constraints in and of themselves.

### Constraints, Values, Lattice Operators, and `cue export`

Since CUE is a data interchange format, you might be wondering how constraints fit in this paradigm. If I export CUE to plain JSON for example, how does `cue export` deal with constraints? The answer is, it doesn't most of the time. For example, if I have this constraint in a `example.cue` file:

```
>9 & <11
```

And try to export this to JSON:

```
$ cue export example.cue
incomplete value >9 & <11
exit status 1
```

`cue` will just flat out refuse to export. However, if we narrow our constraint enough so that it resolves to a single value by asserting that we want a non-fractional number:

```
int & >9 & <11
```

CUE will simplify that to the value `10` and the export will proceed as expected:

```
$ cue export example.cue
10
```

With `cue export` we can also see the practical difference between `&`/`|` and `&&`/`||`. When exporting:

```
false && true
```

the operator is simply evaluated using normal boolean rules:

```
$ cue export example.cue
false
```

On the other hand, for an unification:

```
false & true
```

CUE tries to find a value that is equal to `false` and `true` at the same time (`false` and `true` are treated as constraints):

```
$ cue export example.cue
conflicting values true and false:
    example.cue:1:1
    example.cue:1:9
exit status 1
```

### Duplicate Fields

As stated earlier, CUE accepts duplicate fields. Semantically, all duplicates are resolved by applying unification (`&`) recursively. For example:

```
name: string
name: !=""
version: major: int
version: major: >=0
version: minor: uint
version: {
    patch: uint
    tag?: string & !=""
}
```

Is equivalent to:

```
name: string & !=""
version: {
        major: >=0 & int
        minor: uint
        patch: uint
        tag?: !=""
}
```

### Top and Bottom

CUE has two special entities: top (written `_`) and bottom (written `_|_`). Top is a constraint that matches anything, and bottom is a constraint that matches nothing. Bottoms are treated as errors.

## Structs

**TODO**

### Open and Closed Structs

**TODO**

### Identifiers and Cycles

Every field name that is a valid identifier (i.e. not quoted) can be referenced. For example:

```
x: 10
y: x + 1.5
```

Will be exported as:

```
$ cue export example.cue
{
    "x": 10,
    "y": 11.5
}
```

Reference cycles are permitted as long as the cycle can be broken off with a value. For example:

```
x: y
y: x & 10
```

Is valid since there is only one possible value for `x` and `y`:

```
$ cue export example.cue
{
    "x": 10,
    "y": 10
}
```

`cue vet` is an exception to the cycle rule since references are relative to the actual data that is being verified:

```
x: y + 1
y: x * 2
```

Will successfully validate the following JSON:

```json
{
    "x": -1,
    "y": -2
}
```

Note that despite the fact that these are the only valid values for `x` and `y`, `cue export` will error out since the cycle can't be broken off. CUE is not a general constraint solver.

### Hidden Fields, Definitions, and `let` Bindings

CUE has three mechanisms to factor out constraints without declaring a new field:

- Hidden fields: `_<identifier>: <constraint>`
- Definitions: `#<identifier>: <constraint>` or `_#<identifier>: <constraint>`
- `let` binding: `let <identifier> = <constraint>`

Identifiers must be referenced in the same way they were declared, including the `_`/`#`/`_#` prefix. They also respect block (`{}`) scope. Hidden fields and definitions, but not `let` bindings, can be duplicated and have circular references according to the normal rules for fields:

```
version: {
    build: #build
    #build: uint
    #build: <1
}
requires: [
    {
        package: name
        let name = "json"
        #build: 101
        build: #build
    }
]
```

Is exported to:

```
$ cue export example.cue
{
    "version": {
        "build": 0
    },
    "requires": [
        {
            "package": "json",
            "build": 101
        }
    ]
}
```

There doesn't seem to be any difference between definitions declared with `#` and `_#`. And according to the official documentation, the only difference between definitions and hidden fields is that definitions are always closed while hidden fields are open by default.

## Cheatsheet

For the authoritative reference, see: https://cuelang.org/docs/references/spec/.

### Values

- `null`
- `true`
- `false`

### Types

- `null`
- `bool`
- `int`, `int8`, `int16`, `int32`, `int64`, `int128`, `uint`, `uint8`, `uint16`, `uint32`, `uint64`, `uint128`
- `float`, `float32`, `float64`
- `number`: `int | float`
- `string`
- `bytes`

### Operators

Unary operators on values:

- `!`: applies to `bool`
- `-`: applies to `number`

Binary operators on values:

- `+`: applies to `number`, `string`, `bytes`, and `list` (sum/append)
- `-`: applies to `number` (difference)
- `*`: applies to `number`. Also applies to `string`, `bytes`, and `list` as the left operand (multiplication/repeat `n` times)
- `/`: applies to `float` (division)
- `div`: applies to `int` (division)
- `mod`: applies to `int` (modulo)
- `quo`: applies to `int` (quotient)
- `rem`: applies to `int` (remainder)
- `&&`: applies to `bool` (conjunction)
- `||`: applies to `bool` (disjunction)
- `==`: applies to any value (equal to)
- `!=`: applies to any value (not equal to)
- `>`: applies to any value (greater than)
- `>=`: applies to any value (greater or equal than)
- `<`: applies to any value (less than)
- `<=`: applies to any value (less or equal than)
- `=~`: applies to `string` (matches regex)
- `!~`: applies to `string` (does not match regex)

Unary operators that create constraints:

- `!=`: applies to any value (not equal to)
- `>`: applies to any value (greater than)
- `>=`: applies to any value (greater or equal than)
- `<`: applies to any value (less than)
- `<=`: applies to any value (less or equal than)
- `=~`: applies to `string` (matches regex)
- `!~`: applies to `string` (does not match regex)

Binary operators on constraints:

- `&`: applies to any constraint (unification)
- `|`: applies to any constraint (disjunction)
