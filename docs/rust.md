# Rust

Reconfix is aiming for the [Rust 2018 edition](https://blog.rust-lang.org/2018/03/12/roadmap.html).
It will be released on [2018-10-25](https://internals.rust-lang.org/t/rust-2018-the-home-stretch/7810).
Then the toolchain will be switched from `nightly` to `stable`.

## Installation

```bash
curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain=nightly
rustup component add rustfmt-preview
rustup component add clippy-preview
```

You can always consult [.travis.yml](https://github.com/resin-io/reconfix/blob/rust/.travis.yml)
if these steps doesn't work for you.
