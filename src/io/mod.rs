use std::error;

pub mod host;

use common::FileNode;

/// Trait that Reconfix plugins must adhere to
pub trait Plugin {
    /// Given a `FileNode` provide an `Read` implementation for reading the
    /// contents.
    fn read(self, FileNode) -> Result<Vec<u8>, Box<error::Error + Send>>;
    /// Given a `FileNode` provide an `Write` implementation for reading the
    /// contents.
    fn write(self, FileNode, Vec<u8>) -> Result<(), Box<error::Error + Send>>;
}
