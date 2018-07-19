use std::error;

pub mod host;

use crate::common::FileNode;

/// Trait that Reconfix plugins must adhere to
pub trait Plugin {
    /// Given a `FileNode` provide an `Read` implementation for reading the
    /// contents.
    fn read(self, node: FileNode) -> Result<Vec<u8>, Box<dyn error::Error + Send>>;
    /// Given a `FileNode` provide an `Write` implementation for reading the
    /// contents.
    fn write(self, node: FileNode, buf: Vec<u8>) -> Result<(), Box<dyn error::Error + Send>>;
}
