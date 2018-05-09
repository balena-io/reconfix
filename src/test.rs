use std::io::{self, BufRead, BufReader};
use std::cmp;

use serde_json;
use serde_json::{from_str, Value};

use super::error::*;

/// Represents a section of data in a test file.
pub enum Section {
    Json(Value),
    Error,
    Comment(String),
}

impl Section {
    /// Return a `&Value` if this is a line of JSON. Otherwise, return `None`.
    pub fn as_value(&self) -> Option<&Value> {
        match self {
            &Section::Json(ref v) => Some(v),
            _ => None,
        }
    }

    /// Return a `&str` if this is a comment. Otherwise, return `None`.
    pub fn as_comment(&self) -> Option<&str> {
        match self {
            &Section::Comment(ref c) => Some(c),
            _ => None,
        }
    }
}

/// Parses a file for `Section` elements. This can handle `# comments`, single-line and
/// multi-line JSON as well as the special value `#error` to indicate a test that should
/// fail.
pub fn read_sections<R>(r: R) -> Vec<Section>
where
    R: io::Read,
{
    let mut reader = RebufReader::new(r);
    let mut items = Vec::new();

    let title = read_section(&mut reader)
        .expect("unable to parse title")
        .expect("unexpected end of file")
        .as_comment()
        .expect("first line must be a comment")
        .to_string();

    items.push(Section::Comment(title));

    loop {
        match read_section(&mut reader) {
            Ok(Some(Section::Comment(_))) => (),
            Ok(Some(x)) => items.push(x),
            Ok(None) => break,
            Err(e) => panic!("parse error: {:?}", e),
        }
    }

    items
}

/// Helper function for parsing the multi-line test files. Assumes they follow the format:
///
/// 1. Test name in a `# comment`
/// 2. Schema JSON
/// 3. Input JSON
/// 4. Expected output JSON
///
/// A callback is provided for parsing the schema into a different structure before returning
/// values to the calling test.
pub fn parse_test_data<T, F>(data: &str, convert: F) -> (String, T, Value, Option<Value>)
where
    F: FnOnce(&Value) -> T,
{
    let _ = ::env_logger::init();
    let lines = read_sections(data.as_bytes());
    let title = lines[0]
        .as_comment()
        .expect("Invalid title on line 1!")
        .to_string();
    let schema = lines[1].as_value().expect("Invalid JSON on line 2!");
    let tree = lines[2]
        .as_value()
        .map(|x| x.clone())
        .expect("Invalid JSON on line 3!");
    let value = lines[3].as_value().map(|x| x.clone());

    let parsed = convert(schema);
    (title, parsed, tree, value)
}

/// Read a new section, first attempting to read single-line data, then multi-line.
fn read_section<R>(mut r: &mut RebufReader<R>) -> Result<Option<Section>>
where
    R: io::Read,
{
    read_text(&mut r).or_else(|_| read_json(&mut r))
}

/// Attempt to read a multi-line JSON section.
fn read_json<R>(mut r: &mut RebufReader<R>) -> Result<Option<Section>>
where
    R: io::Read,
{
    r.reset();
    let (line, pos) = {
        let de = serde_json::Deserializer::from_reader(&mut r);
        let mut stream = de.into_iter::<Value>();
        let line = match stream.next() {
            Some(Ok(j)) => Some(Section::Json(j)),
            Some(Err(e)) => return Err(Error::with_chain(e, "multi-line json parse error")),
            None => None,
        };

        (line, stream.byte_offset())
    };

    r.consume(pos);

    Ok(line)
}

/// Attempt to read a single line section of either a `Comment`, `Error`, or single-line `Json`.
fn read_text<R>(mut r: &mut RebufReader<R>) -> Result<Option<Section>>
where
    R: io::Read,
{
    r.reset();
    let line = {
        let mut buf_reader = BufReader::new(&mut r);
        let mut line = String::new();
        let count = buf_reader.read_line(&mut line).chain_err(|| "io error")?;

        if count == 0 {
            return Ok(None);
        }

        line
    };

    let pattern = |c: char| c.is_whitespace() || c == '#';
    let value = if line.starts_with(&pattern) {
        match line.trim_matches(&pattern) {
            "error" => Section::Error,
            x => Section::Comment(x.into()),
        }
    } else {
        from_str::<Value>(&line)
            .map(Section::Json)
            .chain_err(|| "single-line json parse error")?
    };

    r.consume(line.len());

    Ok(Some(value))
}

#[derive(Debug)]
/// A wrapper for `Read` which buffers all read data in order to
/// retry with a different parser upon failure.
struct RebufReader<R>
where
    R: io::Read,
{
    inner: R,
    pos: usize,
    buf: Vec<u8>,
}

impl<R> RebufReader<R>
where
    R: io::Read,
{
    fn new(read: R) -> RebufReader<R> {
        RebufReader {
            inner: read,
            pos: 0,
            buf: Vec::new(),
        }
    }

    fn consume(&mut self, count: usize) {
        let clamped = cmp::min(count, self.buf.len());
        self.buf.drain(..clamped);
        self.pos -= clamped;

        assert!(self.pos <= self.buf.len());
    }

    fn reset(&mut self) {
        self.pos = 0;
    }
}

impl<R> io::Read for RebufReader<R>
where
    R: io::Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let count = if self.pos < self.buf.len() {
            // if there is unread data in the buffer, read it out first
            let count = cmp::min(self.buf.len() - self.pos, buf.len());
            buf[..count].copy_from_slice(&self.buf[self.pos..self.pos + count]);
            count
        } else {
            // otherwise, read data from the inner Read and extend the buffer
            let count = self.inner.read(buf)?;
            self.buf.extend_from_slice(&buf[..count]);
            count
        };

        self.pos += count;

        assert!(self.pos <= self.buf.len());

        Ok(count)
    }
}
