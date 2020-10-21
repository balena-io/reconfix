//! (Re)Configuration toolkit.
//!
//! Reconfix is a generic synchronization framework operating on structured
//! data. Reconfix maintains a database of *lenses* (bidirectional transforms)
//! that can be assembled into a transformation graph. This graph can connect
//! (parts of) files, memory buffers, local and remote services, or any other
//! kind of data repository.
//!
//! Reconfix relies on [CUE](https://cuelang.org/) to define lenses, detect
//! conflicts, and run the actual transformations.
//!
//! # Lenses
//!
//! A lens is a bidirectional transformation defined in CUE. Each lens is a
//! self-contained file that defines two mutually-recursive fields: `X` and
//! `Y`. `X` is defined as the transformation one way assuming `Y` is a
//! concrete value, and vice versa.
//!
//! For example, a simple lens that increases/decreases a number by 1 could be
//! defined as:
//!
//! ```cue
//! X: Y + 1
//! Y: X - 1
//! ```
//!
//! For more information see the [`lens`] module docs.
//!
//! # External Data
//!
//! Configuration files, JSON documents, REST APIs and all manner of data
//! repositories are handled in reconfix through objects implementing the
//! [`ExternalData`] trait. For more information see the [`external_data`]
//! module.
//!
//! # Transformation Graph
//!
//! A transformation graph is an directed, possibly cyclic graph where every
//! node is either a [`Lens`] or an [`ExternalData`] instance. Transformation
//! graphs are run asynchronously, synchronizing [`ExternalData`] nodes with
//! new values as they are pushed into the graph by those same nodes.
//!
//! Transformation graphs are run through [`Orchestrator::run`]. See also the
//! [`orchestrator`] module;
//!
//! # Example
//!
//! This example keeps an int and a string synchronized through two lenses:
//! increase/decrease a number by 1, and conversion between ints and strings in
//! base 10.
//!
//! ```rust
//! use reconfix::{
//!     external_data::InMemoryExternalData,
//!     Lens,
//!     Orchestrator,
//! };
//! use tokio::task;
//!
//! # #[tokio::main(flavor = "current_thread")]
//! # async fn main() -> reconfix::Result<()> {
//! // Set up a couple of `ExternalData` objects using the handy
//! // `InMemoryExternalData` struct
//! let int_data = InMemoryExternalData::new(&1)?;
//! let string_data = InMemoryExternalData::new(&"0".to_owned())?;
//!
//! let mut orchestrator = Orchestrator::new();
//!
//! // Add all nodes
//! let int_node = orchestrator.add_node(int_data.clone());
//! let inc_node = orchestrator.add_node(Lens::new(
//!     r#"
//!     X: Y + 1
//!     Y: X - 1
//!     "#
//! )?);
//! let int_str_node = orchestrator.add_node(Lens::new(
//!     r#"
//!     import "strconv"
//!
//!     X: strconv.Atoi(Y)
//!     Y: strconv.FormatInt(X, 10)
//!     "#
//! )?);
//! let string_node = orchestrator.add_node(string_data.clone());
//!
//! // Connect the nodes. Note that order of arguments does matter since the
//! // transformation graph is directed
//! orchestrator
//!     .add_edge(int_node, inc_node)
//!     .add_edge(inc_node, int_str_node)
//!     .add_edge(int_str_node, string_node);
//!
//! // Run everything asynchronously
//! tokio::spawn(async move {
//!     orchestrator.run().await.unwrap();
//! });
//!
//! // Wait for the orchestrator to call `listen()` on both `int_data` and
//! // `string_data`
//! while !int_data.is_listening().await {
//!     task::yield_now().await;
//! }
//! while !string_data.is_listening().await {
//!     task::yield_now().await;
//! }
//!
//! // Modify `int_data` and then check that `string_data` has been updated
//! // accordingly
//! int_data.set_cloned(&10).await?;
//! assert_eq!(string_data.get_cloned().await?, "9");
//!
//! // We can do the same the other way
//! string_data.set_cloned(&"50".to_owned()).await?;
//! assert_eq!(int_data.get_cloned().await?, 51);
//! # Ok(())
//! # }
//! ```

use external_data::ApplyError;
use json_patch::Patch;
use serde_json::Value;
use std::{result, sync::Arc};
use thiserror::Error;

pub mod external_data;
pub mod lens;
pub mod orchestrator;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

pub use crate::{
    external_data::ExternalData, lens::Lens, orchestrator::Orchestrator,
};

/// This crate's error type.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// [`Patcher::apply`](crate::external_data::Patcher::apply) failed.
    #[error("`Patcher::apply()` failed: {0}")]
    ApplyError(#[from] ApplyError),

    /// Found a conflict while propagating a patch in a transformation graph.
    #[error("conflict between nodes {0} and {2}. Node {0} is:\n\n{1}\n\nwhile node {2} is:\n\n{3}")]
    Conflict(usize, Arc<Value>, usize, Arc<Value>),

    /// Error evaluating a lens.
    #[error("error evaluating lens: {0}")]
    EvalError(String),

    /// A CUE lens is invalid.
    #[error("invalid lens:\n\n{0}\n\ndue to: {1}")]
    InvalidLens(String, #[source] anyhow::Error),

    /// The initial patch sent by an [`ExternalData`] node is invalid.
    #[error("the initial patch sent by the external data node {0} is invalid:\n{1:#?}")]
    InvalidInitialPatch(usize, Patch),

    /// A CUE value is invalid.
    #[error("invalid value: {0}")]
    InvalidValue(#[source] cuelang::Error),

    /// JSON deserialization failed.
    #[error("JSON deserialization failed: {0}")]
    JsonDeserializationError(#[source] serde_json::Error),

    /// JSON serialization failed.
    #[error("JSON serialization failed: {0}")]
    JsonSerializationError(#[source] serde_json::Error),

    /// A call to [`ExternalData::listen`] failed.
    #[error("listening on external data node {0} failed: {1}")]
    ListenError(usize, #[source] anyhow::Error),

    /// Failed to convert a CUE [`Value`](cuelang::Value) into a JSON
    /// [`Value`](serde_json::Value).
    #[error("failed to convert from a CUE value into a JSON value")]
    ValueConversionError,
}

/// This crate's result type.
pub type Result<T> = result::Result<T, Error>;
