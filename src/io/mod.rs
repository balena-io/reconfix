

use std::error;
use std::io;
use std::result;

pub mod host;

use common::FileNode;

pub trait Content: io::Read + io::Write {}

/// Trait that Reconfix plugins must adhere to
pub trait Plugin {
    /// Given a `FileNode` provide an object that can be read from and writtent to.
    fn open(&self, &FileNode) -> result::Result<Box<Content>, Box<error::Error + Send + Sync>>;
}
