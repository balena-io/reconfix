

use std::error;
use std::io;
use std::result;

pub mod host;

use common::FileNode;

/// Trait representing an IO implementation
// pub trait Content: io::Read + io::Write {}

// impl<'a, T> io::Read for Has<'a, T>
//     where T: Content
// {
//     fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
//         match self {
//             &mut Has::Borrowed(ref mut b) => b.read(buf),
//             &mut Has::Owned(ref mut o) => o.read(buf),
//         }
//     }
// }

// impl<'a, T> io::Write for Has<'a, T>
//     where T: Content
// {
//     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
//         match self {
//             &mut Has::Borrowed(ref mut b) => b.write(buf),
//             &mut Has::Owned(ref mut o) => o.write(buf),
//         }
//     }

//     fn flush(&mut self) -> io::Result<()> {
//         match self {
//             &mut Has::Borrowed(ref mut b) => b.flush(),
//             &mut Has::Owned(ref mut o) => o.flush(),
//         }
//     }
// }

// impl<'a, T> Content for Has<'a, T>
//     where T: Content
// {
// }

/// Trait that Reconfix plugins must adhere to
pub trait Plugin
{
    /// Type representing the readable contents of a file
    type Read: io::Read;
    /// Type representint the writable contents of a file
    type Write: io::Write;
    /// Given a `FileNode` provide an `Read` implementation for reading the contents.
    fn read(self, &FileNode) -> result::Result<Self::Read, Box<error::Error + Send + Sync>>;
    /// Given a `FileNode` provide an `Write` implementation for reading the contents.
    fn write(self, &FileNode) -> result::Result<Self::Write, Box<error::Error + Send + Sync>>;
}

// impl<'b, 'r, P, T> Plugin<'b, T> for &'r mut P
//     where P: Plugin<'b, T> + 'r, T: Content
// {
//     fn open(&mut self, node: &FileNode) -> result::Result<Has<'b, T>, Box<error::Error + Send + Sync>> {
//         self.open(node)
//     }
// }
