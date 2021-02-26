JSONTemplate
============

> Bidirectional JSON-based templating engine

JSONTemplate is a templating engine that operates on JSON objects, providing
the unique benefit of allowing bidirectional transformations, that is, from
template and data to a result, and from a template and result to the original
data.

For example, consider the following template:

```json
{
  "foo": "My name is {{name}}"
}
```

Notice the use of double curly braces to denote string interpolation.

In order to compile this template, we need a `name` value. This is an example
data object that can be used to compile the above template:

```json
{
  "name": "John Doe"
}
```

The compilation result looks like this:

```json
{
  "foo": "My name is John Doe"
}
```

Now consider that we have the compilation result and the template, and we want
to be able to determine what was the original data used to compile it.

JSONTemplate will realise `"My name is John Doe"` was compiled from `"My name
is {{name}}"`, and therefore that `name` equals `John Doe`. Using this
information, JSONTemplate will "decompile" the template and return back the
following object to the user, which unsurprisingly equals the "data" object:

```json
{
  "name": "John Doe"
}
```

The example objects contain one key and a single interpolation, but on real
templates, there can be complex nesting levels and multiple interpolations
(even many per property).
