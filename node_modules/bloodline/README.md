# Bloodline
[![npm](https://img.shields.io/npm/v/bloodline.svg?style=flat-square)](https://npmjs.com/package/bloodline)
[![npm license](https://img.shields.io/npm/l/bloodline.svg?style=flat-square)](https://npmjs.com/package/bloodline)
[![npm downloads](https://img.shields.io/npm/dm/bloodline.svg?style=flat-square)](https://npmjs.com/package/bloodline)
[![build status](https://img.shields.io/travis/jhermsmeier/node-bloodline.svg?style=flat-square)](https://travis-ci.org/jhermsmeier/node-bloodline)

Proper inheritance in JavaScript

## Install via [npm](https://npmjs.com)

```sh
$ npm install --save bloodline
```

## Usage

```js
var inherit = require( 'bloodline' )
```

```js
inherit( Constructor, SuperConstructor )
```

## Notes

### **ES6 / ES2015 / ESNext**

ES6 classes cannot be subclassed by ES5 classes (ES5 classes cannot inherit from ES6 classes);
there will be a `TypeError: Class constructor SuperConstructor cannot be invoked without 'new'`.
This is not a limitation of this module, but of the ECMAScript specification.

The other way around also has issues; ES6 classes don't preserve the prototype chain
when extending ES5 classes (for an example, see https://gist.github.com/jhermsmeier/e6fb16fab193c0aa4220c98de64fe546).

So much for backwards compatibility.

### util.inherits

Usage of Node core's of `util.inherits()` is discouraged (see [nodejs.org/api/util#util_inherits](https://nodejs.org/api/util.html#util_util_inherits_constructor_superconstructor)), because of its [semantic incompatibility](https://github.com/nodejs/node/issues/4179).
**NOTE:** This module *does not* exhibit these semantic incompatibilities.
