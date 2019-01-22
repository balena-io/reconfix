use std::fmt;

use super::path::Path;

#[derive(Debug)]
pub enum ValidationError {
    Missing(Path),
    Invalid(Path, String),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationError::Missing(path) => write!(f, "{}: missing", path),
            ValidationError::Invalid(path, msg) => write!(f, "{}: {}", path, msg),
        }
    }
}

impl std::error::Error for ValidationError {}
