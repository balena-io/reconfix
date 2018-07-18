use std::fs::File;
use std::io::{Read, Write};

use crate::common::FileNode;
use crate::io::Plugin;

/// The default Reconfix plugin
#[derive(Clone)]
pub struct HostFile {}

impl HostFile {
    pub fn new() -> Self {
        HostFile {}
    }
}

impl<'a> Plugin for &'a mut HostFile {
    fn read(
        self,
        node: FileNode,
    ) -> ::std::result::Result<Vec<u8>, Box<::std::error::Error + Send>> {
        let path = node.path.join("/");
        File::open(path)
            .and_then(|mut f| {
                let mut buffer = Vec::new();
                f.read_to_end(&mut buffer)?;
                Ok(buffer)
            })
            .map_err(|e| Box::new(e) as Box<::std::error::Error + Send>)
    }

    fn write(
        self,
        node: FileNode,
        buf: Vec<u8>,
    ) -> ::std::result::Result<(), Box<::std::error::Error + Send>> {
        let path = node.path.join("/");
        File::open(path)
            .and_then(|mut f| f.write_all(&buf))
            .map_err(|e| Box::new(e) as Box<::std::error::Error + Send>)
    }
}
