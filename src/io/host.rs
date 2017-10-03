

use std::fs::File;
use std::error;
use std::result;

use io::{Plugin, Content};
use common::FileNode;


/// The default Reconfix plugin
pub struct HostFile {}

impl Plugin for HostFile {
    fn open(
        &self,
        node: &FileNode,
    ) -> result::Result<Box<Content>, Box<error::Error + Send + Sync>> {
        let path = node.path.join("/");
        let file = File::open(path)?;

        Ok(Box::new(file))
    }
}

impl Content for File {}
