#!/usr/bin/env bash

set -e
set -o pipefail

source "${HOME}/.cargo/env"

echo "Checking Rust crate formatting..."
cargo fmt -- --check

echo "Linting Rust crate..."
cargo clippy --all-targets --all-features -- -D warnings

echo "Testing Rust crate..."
cargo test

echo "Trying to package Rust crate..."

CARGO_PACKAGE_ARGS=''
if ! [ "$CI" = true ]; then
    # Allow to test uncommitted changes locally
    CARGO_PACKAGE_ARGS='--allow-dirty'
fi
cargo package ${CARGO_PACKAGE_ARGS}


ci/build-wasm.sh

source "${HOME}/.nvm/nvm.sh"
nvm use
echo "NodeJS version $(node --version)"

echo "Testing browser NPM package..."
wasm-pack test --chrome --firefox --headless

if [ -d "node/tests" ]; then
    echo "Testing NodeJS NPM package..."
    DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"
    cd node/tests
    npm install
    npm test
    cd "${DIR}"
else
    echo "Skipping NodeJS NPM package tests, folder node/tests not found"
fi
