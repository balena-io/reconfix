# Reconfix Transformations

This document describes the algorithms used in the early Reconfix prototype.

## Conventions

* `$result`: the resulting value of an algorithm

## Dry -> Wet

* For each `file` in the schema, processing files with independent locations first
  * For each `property` in `properties` where `when` matches
    * Process mapping
  * Encode result

## Wet -> Dry

* For each `file` in the schema, processing files with independent locations first
  * Decode source
  * For each `property` in `properties`
    * Process mapping against source
    * If `property.when` exists and the mapping is sucessful
      * Assume the values from `property.when` are true in the dry object
      * If this assumption means there is a conflict with previous assumptions
        * Mark the whole file as tainted and continue to the next file
    * If no mapping is successful
      * Mark the whole file as tainted and continue to the next file

## Dry -> Wet mapping

* For each `mapping` in `property.mapping`, where `mapping` is an object
  * If for every type in `property.type`, the dry value doesn't match, fail
  * If `mapping.value` doesn't match the dry value
    * Continue
  * Else
    * Ignore wildcard properties
    * Merge `mapping.template` with `$result`
    * If the merge overwrites different values in `$result`
      * There is a mismatch. Mark the whole file as tainted and continue
        with the next file

* For each `mapping` in `property.mapping`, where `mapping` is an array
  * If for every type in `property.type`, the dry value doesn't match, fail
  * If `$result[mapping]` is already set and doesn't match the dry value
    * Mark the whole file as tainted and continue to the next file
  * Else
    * Set `$result[mapping]` to the passed dry value

## Wet -> Dry mapping

* For each `mapping` in `property.mapping`
  * If they are all objects (e.g. template mappings)
    * Evaluate the "matching degree" of every `mapping.template` against the
      wet object
    * Pick the object with the highest matching degree
    * If there is a tie, or there is no match
      * Mark the whole file as tainted and continue with the next file
  * If they are all arrays (e.g. direct mappings)
    * Get the `mapping` property of the wet object
    * If the value doesn't match `mapping.type`
      * Mark the whole file as tainted and continue with the next file
    * Assign `$result[name of property including the mapping]` to the
      obtained wet value
    * If the `$result` property is already set to a different value
      * Mark the whole file as tainted and continue with the next file

## Encode wet object

* If a handler for `file.type` doesn't exist, fail
* If `file.fileset` is `false`
  * Call the encode function of the type handler on the wet object
  * If `file.location.path`
    * Set `$result[file.location.path]` to the encoded string
  * Else if `file.location.parent`
    * Set `$result[file.location.parent][file.location.property]` to the encoded string
* If `file.fileset` is `true`
  * For each `[ key, value ]` in the wet object
    * Call the encode function of the type handler on `value`
    * If `file.location.path`
      * Set `$result[file.location.path][key]` to the encoded value
    * If `file.location.parent`
      * Set `$result[file.location.parent][file.location.property][key]` to the encoded string
