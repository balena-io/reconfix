# Contributing

Thanks for your interest in contributing to this project! This document aims to serve as a friendly
guide for making your first contribution.

## Table of Contents

* [Development](#development)
* [Coding Style](#coding-style)

## Development

### Rust

Reconfix is aiming for the stable [Rust 2018 edition](https://blog.rust-lang.org/2018/03/12/roadmap.html).
It will be released on [2018-10-25](https://internals.rust-lang.org/t/rust-2018-the-home-stretch/7810).
Then the toolchain will be switched from `nightly` to `beta` and then to `stable`.

Don't know what the edition is? Visit [The Rust Edition Guide](https://rust-lang-nursery.github.io/edition-guide/2018/index.html).

#### Installation

```bash
curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain=nightly
rustup component add rustfmt-preview
rustup component add clippy-preview
```

You can always consult [.travis.yml](.travis.yml) if these steps doesn't work for you.

#### Cargo watch

It's recommended to also install [cargo watch](https://github.com/passcod/cargo-watch).

> Cargo Watch watches over your project's source for changes, and runs Cargo commands when they
> occur.

Check [Usage examples](https://github.com/passcod/cargo-watch#usage). You can watch over your
project's source code for changes and run tests automatically with `cargo watch -x test` command.

## Coding Style

### Editor Config

Install [Editor Config](https://editorconfig.org/) plugin for your favorite IDE. If it's not
available, you should obey rules from the [.editorconfig](.editorconfig) file.

### Line Length

80 characters wide terminals era is over. Rust language can have pretty long lines, Markdown
documents as well (especially with links). Line length limit for this project is 100 characters.

### Markdown

**Enforced by CI: No.**

All documents must pass [markdownlint](https://github.com/DavidAnson/markdownlint) checks.
Configuration is available in the [.markdownlint.json](.markdownlint.json) file.

Markdown lines can be longer than 100 characters if they contain link.

### Source Code

#### Compiler Warnings

**Enforced by CI: No.**

Treat all warnings as errors. Rust warnings are pretty talkative (which is good), but it's hard to
find errors if the output is very long. Pull requests with warnings will not be accepted.

#### Format

**Enforced by CI: Yes.**

Setup pre commit hook or run `cargo fmt` manually. Configuration is available in the
[rustfmt.toml](rustfmt.toml) file.

#### Clippy

**Enforced by CI: Yes.**

You can run Clippy manually with `cargo clippy` command. It's recommended to always clean the
`reconfix` project, because subsequent Clippy calls do not produce warnings, errors and
recommendations.

```bash
cargo clean -p reconfix
cargo clippy
```

Try to fix all Clippy recommendations. If they do not make sense to you,
[disable specific lints in the code](https://github.com/rust-lang-nursery/rust-clippy#allowingdenying-lints)
and **ADD** appropriate comment why.

Do **NOT** disable lints for the whole file / crate.
