# cyclic-32
[![npm](https://img.shields.io/npm/v/cyclic-32.svg?style=flat-square)](https://npmjs.com/package/cyclic-32)
[![npm license](https://img.shields.io/npm/l/cyclic-32.svg?style=flat-square)](https://npmjs.com/package/cyclic-32)
[![npm downloads](https://img.shields.io/npm/dm/cyclic-32.svg?style=flat-square)](https://npmjs.com/package/cyclic-32)
[![build status](https://img.shields.io/travis/jhermsmeier/node-cyclic-32.svg?style=flat-square)](https://travis-ci.org/jhermsmeier/node-cyclic-32)

A tiny, streaming, seedable [CRC32] library, compatible with Node's [crypto.Hash API].
In less than 100 LOC.

[CRC32]: https://en.wikipedia.org/wiki/Cyclic_redundancy_check
[crypto.Hash API]: https://nodejs.org/api/crypto.html#crypto_class_hash

## Install via [npm](https://npmjs.com)

```sh
$ npm install --save cyclic-32
```

## Features

- **Speed:** ~300 MB/s on a 2012 Macbook Air
- **Size:** It's 96 lines of code (plus 22 lines of comments, and 17 blank)
- **Streams:** .pipe().pipe().pipe()
- **Seedable:** If you have more complex things in mind
- **CLI:** Possibly useful for npm run scripts

## Usage

```js
var crc32 = require( 'cyclic-32' )
```

Calculate the CRC32 checksum of a Buffer:

```js
var buffer = new Buffer( 'I shall be summed', 'ascii' )
console.log( crc32( buffer ) ) // -476443423
```

Pass a seed value:

```js
var buffer = new Buffer( 'I shall be summoned', 'ascii' )
console.log( crc32( buffer, 666 ) ) // -823676065
```

Calculate the CRC32 checksum over a Stream:

```js
fs.createReadStream( filename )
  .pipe( crc32.createHash() )
  .on( 'data', function( buffer ) {
    console.log( 'CRC32:', buffer.toString( 'hex' ) )
  })
```

If you want to pass an `encoding`, or `seed`:

```js
fs.createReadStream( filename )
  .pipe( new crc32.Hash({ encoding: 'hex', seed: -12345678 }) )
  .on( 'data', function( checksum ) {
    console.log( 'CRC32:', checksum )
  })
```

Or, if you'd rather stick to the `crypto.Hash` API:

```js
var hash = crc32.createHash()

hash.update( 'I shall' )
  .update( 'be summed' )

console.log( 'CRC32:', hash.digest( 'hex' ) )
```

## CLI Usage

Calculate the checksum of a file:

```sh
$ crc32 filename
0a0ca5aa
```

Pipe stuff into it via stdin:

```sh
$ cat filename | crc32
ffe6bbc0
```
