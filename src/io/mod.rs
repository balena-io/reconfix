

use std::error;
use std::io;
use std::result;

pub mod host;

use common::FileNode;

/// Trait representing an IO implementation
pub trait Content: io::Read + io::Write {}

// pub enum Has<'a, T> where T: Content + 'a {
//     Borrowed(&'a T),
//     Owned(T),
// }

/// Trait that Reconfix plugins must adhere to
pub trait Plugin<'a, 'b, C>
    where C: Content + 'b, 'a: 'b
{
    /// Given a `FileNode` provide an object that can be read from and writtent to.
    fn open(&'a mut self, &FileNode) -> result::Result<C, Box<error::Error + Send + Sync>>;
}

impl<'a, 'b, 'r, P, C> Plugin<'a, 'b, C> for &'r mut P
    where P: Plugin<'a, 'b, C>, C: Content + 'b, 'a: 'b
{
    fn open(&'a mut self, node: &FileNode) -> result::Result<C, Box<error::Error + Send + Sync>> {
        self.open(node)
    }
}
