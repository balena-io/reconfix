# Style

## Editor Config

It's recommended to install [Editor Config](https://editorconfig.org/) plugin
for your favorite IDE.

Configuration is available in the [.editorconfig](../.editorconfig) file.

## Markdown

Enforced by CI: No.

All documents must pass [markdownlint](https://github.com/DavidAnson/markdownlint)
checks. Configuration is available in the [.markdownlint.json](../.markdownlint.json)
file.

### Disabled Lints

* `MD013` - Line length
  * You should keep line length below 80 characters
  * This lint is disabled because some lines are longer (mainly lines with
    links)
* `MD024` - No duplicate header / No duplicate heading
  * This lint is disabled because of the [schema.md](schema.md) document
  * Will be enabled in the future, have to check if it's really necessary

## Rust

Read [Installation](rust.md#Installation) to learn how to install `rustfmt` and
`clippy`. It's assumed that your default toochain is `nightly`. If not, you
have to add `+nightly` to all following `cargo` commands. Example:

* `cargo fmt` -> `cargo +nightly fmt`

### Compiler Warnings

Enforced by CI: No.

Treat all warnings as errors. Rust warnings are pretty talkative (which is
good), but it's hard to find errors if the output is very long. Pull requests
with warnings will not be accepted.

### Format

Enforced by CI: Yes.

Setup pre commit hook or run `cargo fmt` manually. Configuration is available
in the [rustfmt.toml](../rustfmt.toml) file.

### Lints

Enforced by CI: Yes.

You can run Clippy manually with `cargo clippy` command. It's recommended to
always clean the `reconfix` project, because subsequent Clippy calls do not
produce warnings, errors and recommendations.

```bash
cargo clean -p reconfix
cargo clippy
```

Try to fix all Clippy recommendations. If they do not make sense to you, [disable
specific lints in the code](https://github.com/rust-lang-nursery/rust-clippy#allowingdenying-lints)
and **ADD** appropriate comment why.

Do **NOT** disable lints for the whole file / crate.
