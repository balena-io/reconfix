use balena_temen::ast::IdentifierValue;
use std::fmt;

#[derive(Clone, Debug, Default)]
pub struct Path {
    identifiers: Vec<IdentifierValue>,
}

// Till: Implement Display for ast::Identifier
// https://github.com/balena-io-modules/balena-temen/issues/51
impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.identifiers.is_empty() {
            return write!(f, "$");
        }

        for (idx, value) in self.identifiers.iter().enumerate() {
            match value {
                IdentifierValue::Name(name) => {
                    if idx > 0 {
                        write!(f, ".")?;
                    }
                    write!(f, "{}", name)?;
                }
                IdentifierValue::Index(index) => write!(f, "[{}]", index)?,
                _ => unreachable!(),
            };
        }

        Ok(())
    }
}

impl Path {
    pub fn new() -> Path {
        Path::default()
    }

    pub fn push_array_index(&mut self, index: isize) {
        self.identifiers.push(IdentifierValue::Index(index));
    }

    pub fn push_property_name<S>(&mut self, name: S)
    where
        S: Into<String>,
    {
        self.identifiers.push(IdentifierValue::Name(name.into()));
    }
}
