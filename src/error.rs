//! Error handling
//!
//! A module allowing us to create more detailed, context aware, errors.
//!
//! # Examples
//!
//! Result chaining example.
//!
//! ```rust
//! use reconfix::error::*;
//!
//! fn eval_as_number() -> Result<()> {
//!     Err(Error::with_message("unable to evaluate as a number")
//!         .context("value", "some value")
//!         .context("expected", "number"))
//! }
//!
//! fn eval_math() -> Result<()> {
//!     Ok(eval_as_number()
//!         .frame_with(|| "eval_math".into())
//!         .context_with(|| ("rhs".to_string(), "`23`".to_string()))?)
//! }
//!
//! fn eval() -> Result<()> {
//!     Ok(eval_math().frame_with_name("eval").context("expression", "1 = `23`")?)
//! }
//!
//! eprintln!("{}", eval().err().unwrap());
//! ```
//!
//! Printed error:
//!
//! ```text
//! temen: unable to evaluate as a number
//!  ├ frame[0]
//!  |  └ context:
//!  |     ├ value: some value
//!  |     └ expected: number
//!  ├ frame[1]: eval_math
//!  |  └ context:
//!  |     └ rhs: `23`
//!  └ frame[2]: eval
//!     └ context:
//!        └ expression: 1 = `23`
//! ```
use std::borrow::Cow;
use std::error;
use std::fmt;
use std::result;

/// Standard library result wrapper
pub type Result<T> = result::Result<T, Error>;

type Display = Cow<'static, str>;

/// Result extension
pub trait ResultExt<T> {
    /// Appends key, value pair to context of the last frame
    ///
    /// # Arguments
    ///
    /// * `k` - A key
    /// * `v` - A value
    fn context<K, V>(self, k: K, v: V) -> Result<T>
    where
        K: Into<Display>,
        V: Into<Display>;

    /// Appends key, value pair to context of the last frame
    ///
    /// # Arguments
    ///
    /// * `f` - A function which must return tuple (key, value)
    fn context_with<F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> (String, String);

    /// Appends new, anonymous, frame
    ///
    /// Anonymous means that the frame does not have a name.
    fn frame(self) -> Result<T>;

    /// Appends new frame
    ///
    /// # Arguments
    ///
    /// * `f` - A function which must return frame name
    fn frame_with<F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> String;

    /// Appends new frame
    ///
    /// # Arguments
    ///
    /// * `name` - A frame name
    fn frame_with_name<N>(self, name: N) -> Result<T>
    where
        N: Into<Display>;
}

impl<T> ResultExt<T> for Result<T> {
    fn context<K, V>(self, k: K, v: V) -> Result<T>
    where
        K: Into<Display>,
        V: Into<Display>,
    {
        self.map_err(|e| e.context(k, v))
    }

    fn context_with<F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> (String, String),
    {
        self.map_err(|e| {
            let (k, v) = f();
            e.context(k, v)
        })
    }

    fn frame(self) -> Result<T> {
        self.map_err(|e| e.frame())
    }

    fn frame_with<F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|e| e.frame_with_name(f()))
    }

    fn frame_with_name<N>(self, name: N) -> Result<T>
    where
        N: Into<Display>,
    {
        self.map_err(|e| e.frame_with_name(name))
    }
}

/// Error type
pub struct Error {
    // Box is not really required here, but we'd like to keep
    // Result as small as possible. Inner can be very huge
    // sometimes.
    inner: Box<Inner>,
}

impl Error {
    /// Creates new error with message
    ///
    /// # Arguments
    ///
    /// * `message` - An error message
    pub fn with_message<M>(message: M) -> Error
    where
        M: Into<Display>,
    {
        let inner = Inner::new(message);
        Error { inner: Box::new(inner) }
    }

    /// Appends key, value pair to context of the last frame
    ///
    /// # Arguments
    ///
    /// * `k` - A key
    /// * `v` - A value
    pub fn context<K, V>(mut self, k: K, v: V) -> Error
    where
        K: Into<Display>,
        V: Into<Display>,
    {
        self.inner
            .frames
            .last_mut()
            .expect("Inner must contain at least one frame")
            .push(k, v);
        self
    }

    /// Appends new, anonymous, frame
    ///
    /// Anonymous means that the frame does not have a name.
    pub fn frame(mut self) -> Error {
        let frame = Frame::new();
        self.inner.frames.push(frame);
        self
    }

    /// Appends new frame
    ///
    /// # Arguments
    ///
    /// * `name` - A frame name
    pub fn frame_with_name<N>(mut self, name: N) -> Error
    where
        N: Into<Display>,
    {
        let frame = Frame::with_name(name);
        self.inner.frames.push(frame);
        self
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "temen: {}", self.inner.message)?;

        if self.inner.frames.is_empty() {
            return Ok(());
        }

        let last_frame_idx = self.inner.frames.len() - 1;
        for (frame_idx, frame) in self.inner.frames.iter().enumerate() {
            let context_indent: &str;
            let frame_indent: &str;

            if last_frame_idx == frame_idx {
                frame_indent = " └";
                context_indent = "   ";
            } else {
                frame_indent = " ├";
                context_indent = " | ";
            }

            write!(f, "{} frame[{}]", frame_indent, frame_idx)?;
            if frame.name.is_some() {
                writeln!(f, ": {}", frame.name.as_ref().unwrap())?;
            } else {
                writeln!(f)?;
            }

            if !frame.context.is_empty() {
                writeln!(f, "{} └ context:", context_indent)?;
                let last_index = frame.context().len() - 1;
                for (idx, (k, v)) in frame.context().iter().enumerate() {
                    if idx == last_index {
                        writeln!(f, "{}    └ {}: {}", context_indent, k, v)?;
                    } else {
                        writeln!(f, "{}    ├ {}: {}", context_indent, k, v)?;
                    }
                }
            }
        }
        Ok(())
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(error::Error + 'static)> {
        None
    }
}

struct Inner {
    message: Display,
    frames: Vec<Frame>,
}

impl Inner {
    fn new<M>(message: M) -> Inner
    where
        M: Into<Display>,
    {
        Inner {
            message: message.into(),
            frames: vec![Frame::new()],
        }
    }
}

struct Frame {
    name: Option<Display>,
    context: Vec<(Display, Display)>,
}

impl Frame {
    fn new() -> Frame {
        Frame {
            name: None,
            context: vec![],
        }
    }

    fn with_name<N>(name: N) -> Frame
    where
        N: Into<Display>,
    {
        Frame {
            name: Some(name.into()),
            context: vec![],
        }
    }

    fn push<K, V>(&mut self, k: K, v: V)
    where
        K: Into<Display>,
        V: Into<Display>,
    {
        self.context.push((k.into(), v.into()))
    }

    fn context(&self) -> &[(Display, Display)] {
        self.context.as_ref()
    }
}
