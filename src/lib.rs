//! # `reconfix-core`
//!
//! This crate implements the core of reconfix schema handling and bidirectional transformation.

#![deny(missing_docs)]
#![recursion_limit = "1024"]

mod adaptor;
mod schema;
mod template;
mod transform;

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate nom;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate maplit;

mod errors {
    error_chain! {
        errors {
            /// Indicates a parsing error
            Parse
        }
    }
}

pub use errors::*;
pub use transform::*;