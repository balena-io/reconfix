#!/usr/bin/env bash

set -e

source "${HOME}/.cargo/env"
source "${HOME}/.nvm/nvm.sh"
nvm use

# TODO Uncomment to enable crate publishing once done
#
# echo "Authenticating to crates.io..."
# cargo login "${CARGO_API_TOKEN}"
# echo "Publishing Rust crate..."
# cargo publish

echo "Authenticating to npmjs.org registry..."
echo "//registry.npmjs.org/:_authToken=${NPM_TOKEN}" > ~/.npmrc

ci/build-wasm.sh

echo "Publishing NPM package..."
npm publish --access public target/npm/pkg
