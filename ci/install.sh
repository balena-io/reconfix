#!/usr/bin/env bash

set -e
set -o pipefail

echo "Installing Rust toolchain..."
curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain $(cat rust-toolchain)
source "${HOME}/.cargo/env"
rustup component add clippy
rustup component add rustfmt

echo "Updating Rust toolchain..."
(test -x "${HOME}/.cargo/bin/cargo-install-update" || cargo install cargo-update)
cargo install-update -a

echo "Installing NVM & NodeJS..."
curl -o- https://raw.githubusercontent.com/creationix/nvm/v0.33.11/install.sh | bash
source "${HOME}/.nvm/nvm.sh"
nvm install
nvm use
echo "NodeJS version $(node --version)"

echo "Installing wasm-pack..."
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh -s -- -f
