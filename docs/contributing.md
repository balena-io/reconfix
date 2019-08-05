# Toolchain

## Rust version

Supported: 1.36.0

You can't update to the newer Rust version on your own, because we have to keep
same Rust version as the internal
[balena CI image](https://github.com/balena-io/ci-images/blob/master/pipelines/rust/docker/ubuntu-rust-wasm.yml)
is using. Consult [Rust 1.36.0 PR](https://github.com/balena-io/ci-images/pull/29) to check
what needs to be updated.

### Installation

Docker image excerpt:

```dockerfile
RUN wget -q -O - https://dl.google.com/linux/linux_signing_key.pub | apt-key add -
RUN echo "deb http://dl.google.com/linux/chrome/deb/ stable main" >> /etc/apt/sources.list.d/google.list
RUN apt-get update && apt-get install -y google-chrome-stable firefox
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain 1.36.0
ENV PATH=$PATH:/root/.cargo/bin/
RUN rustup component add clippy
RUN rustup component add rustfmt
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh -s -- -f
```

## Node

Supported: 10.4.0

You can't update to the newer Node version on your own, because we have to keep
same Node version as the internal
[balena CI image](https://github.com/balena-io/ci-images/blob/master/pipelines/rust/docker/ubuntu-rust-wasm.yml)
is using.

### Installation

Docker image excerpt:

```dockerfile
RUN apt-get update && apt-get install -y google-chrome-stable firefox
ENV NODE_VERSION 10.4.0
ENV ARCH x64
RUN echo $NODE_VERSION
RUN echo $ARCH
RUN set -x && curl -SLO "https://nodejs.org/dist/v$NODE_VERSION/node-v$NODE_VERSION-linux-$ARCH.tar.xz" \
  && curl -SLO --compressed "https://nodejs.org/dist/v$NODE_VERSION/SHASUMS256.txt.asc" \
  && gpg --batch --decrypt --output SHASUMS256.txt SHASUMS256.txt.asc \
  && grep " node-v$NODE_VERSION-linux-$ARCH.tar.xz\$" SHASUMS256.txt | sha256sum -c - \
  && tar -xJf "node-v$NODE_VERSION-linux-$ARCH.tar.xz" -C /usr/local --strip-components=1 --no-same-owner \
  && rm "node-v$NODE_VERSION-linux-$ARCH.tar.xz" SHASUMS256.txt.asc SHASUMS256.txt \
  && ln -s /usr/local/bin/node /usr/local/bin/nodejs
```

# Tests

## Rust

### Formatting

Code MUST be properly formatted with `rustfmt`:

```sh
cargo fmt -- --check
```

### Clippy 

Code MUST pass `clippy` checks:

```sh
cargo clippy --all-targets --all-features -- -D warnings
```

If you disagree with `clippy`, not a rare case, you can
[disable specific checks](https://github.com/rust-lang/rust-clippy#allowingdenying-lints)
in the code.

### Rust tests

Code MUST pass all tests:

```sh
cargo test
```

### Packaging

Code MUST pass packaging:

```sh
cargo package --allow-dirty
```

## WASM

### WASM pack

Code MUST pass WASM pack tests: 

```sh
wasm-pack test --chrome --firefox --headless
```

### Node tests

Code MUST pass Node tests:

```sh
pushd node/tests
npm install
npm test
popd
```
