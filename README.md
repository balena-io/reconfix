# Reconfix

[![Build Status](https://travis-ci.org/balena-io/reconfix.svg?branch=master)](https://travis-ci.org/balena-io/reconfix)
[![Current Release](https://img.shields.io/github/tag/balena-io/reconfix.svg?style=flat-square)](https://github.com/balena-io/reconfix/tags)
[![License](https://img.shields.io/github/license/balena-io/reconfix.svg?style=flat-square)](https://github.com/balena-io/reconfix/blob/master/LICENSE)
[![Issues](https://img.shields.io/github/issues/balena-io/reconfix.svg?style=flat-square)](https://github.com/balena-io/reconfix/issues)

(Re)Configuration toolkit.

Reconfix is a generic synchronization framework operating on structured data. Reconfix maintains a database of *lenses* (bidirectional transforms) that can be assembled into a transformation graph. This graph can connect (parts of) files, memory buffers, local and remote services, or any other kind of data repository.

Reconfix relies on [CUE](https://cuelang.org/) to define lenses, detect conflicts, and run the actual transformations.

## Building

To build reconfix from source you will need a recent version of [Rust](https://www.rust-lang.org/tools/install) and [go](https://golang.org/doc/install).

Before building the project you must install CUE in your `GOPATH` (this will not be required in the future, see [goexport#1](https://github.com/balena-io-playground/goexport/issues/1)):

```
$ go get -u cuelang.org/go/cue
```

To build the project, run:

```
$ cargo build
```

Or

```
$ cargo build --release
```

The standalone `reconfix-cli` executable will be available in `target/debug/` or `target/release/`, depending on how the project was built.

### Portability and Cross Compilation

Reconfix depends on the [CUE](https://pkg.go.dev/cuelang.org/go) library, which is written in go. Rust bindings are generated through [goexport](https://github.com/balena-io-playground/goexport) into the `cuelang-sys` crate. Thus, portability is limited to what both rust and go support.

Currently `goexport` does not support cross-compilation *except* for the `wasm32-unknown-unknown` target: https://github.com/balena-io-playground/goexport/issues/3.

### WASM

While it is possible to build reconfix for WASM using a plain `cargo build`, `wasm-bindgen` has to post-process the generated `.wasm` file before it can be used. For that reason it is recommended to compile through `wasm-pack`, which can be installed globally with `cargo`:

```
$ cargo install wasm-pack
```

The `wasm-pack` binary will be available inside `$CARGO_HOME/bin/` (`$CARGO_HOME` defaults to `$HOME/.cargo/`). Add `$CARGO_HOME/bin/` to your `PATH` and run:

```
$ wasm-pack build reconfix
```

Or

```
$ wasm-pack build --release reconfix
```

*Almost* all necessary files will be available inside `reconfix/pkg/`. To use the generated WASM library, see the "How to Run/WASM Library" subsection.

It is also possible to use the reconfix executable inside a WASM environment. In that case, compile the `reconfix-cli` crate using `cargo build` (**not** `wasm-pack`) and see the "How to Run/WASM Library" subsection.

## How to Run

Reconfix can be used as a library or as a command-line tool. **TODO**

### WASM

https://github.com/balena-io-playground/goexport/issues/2

## How to Use

**TODO**

## License

`reconfix` is open source software, and may be redistributed under the terms specified in the [license].

[balena.io]: https://www.balena.io/
[contact us]: https://forums.balena.io/
[raise an issue]: https://github.com/balena-io/reconfix/issues/new
[license]: https://github.com/balena-io/reconfix/blob/master/LICENSE
