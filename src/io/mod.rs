
use common::FileNode;
use error::*;

use std::error;
use std::fs::File;
use std::io;
use std::path::Path;
use std::result;

pub trait Content: io::Read + io::Write { }

/// Trait that Reconfix plugins must adhere to
pub trait Plugin
{
    /// Given a `FileNode` provide an object that can be read from and writtent to.
    fn open(&self, &FileNode) -> result::Result<Box<Content>, Box<error::Error + Send + Sync>>;
}

/// The default Reconfix plugin
pub struct HostFile { }

impl Plugin for HostFile {
    fn open(&self, node: &FileNode) -> result::Result<Box<Content>, Box<error::Error + Send + Sync>> {
        let path = node.path.join("/");
        let file = File::open(path)?;
        
        Ok(Box::new(file))
    }
}

impl Content for File { }
