use std::fmt;

#[derive(Debug, Clone)]
pub enum Component {
    Property(String),
    Index(usize),
}

#[derive(Debug, Clone)]
pub struct PathBuf {
    components: Vec<Component>,
}

impl PathBuf {
    pub fn new() -> PathBuf {
        PathBuf { components: vec![] }
    }

    pub fn push_index(&mut self, index: usize) {
        self.components.push(Component::Index(index));
    }

    pub fn push_property<S>(&mut self, property: S)
    where
        S: Into<String>,
    {
        self.components.push(Component::Property(property.into()))
    }
}

impl fmt::Display for PathBuf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (idx, component) in self.components.iter().enumerate() {
            match component {
                Component::Property(s) if idx > 0 => write!(f, ".{}", s)?,
                Component::Property(s) => write!(f, "{}", s)?,
                Component::Index(idx) => write!(f, "[{}]", idx)?,
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_buf() {
        assert_eq!(&PathBuf::new().to_string(), "");
    }
}
