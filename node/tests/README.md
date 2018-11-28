# Node integration tests

The main purpose of these tests is to test if every exported function
successfully return correct value and if it throws in case of an error.
We do not need to test actual functionality (parsing, evaluation, etc.),
because these things are tested in the Rust crate itself.

## Run tests

* Build isomorphic NPM package with `ci/build-wasm.sh` script
* Install dependencies with `npm install`
* Run tests with `npm test`
