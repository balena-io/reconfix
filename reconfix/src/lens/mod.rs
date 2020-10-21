//! CUE lenses.
//!
//! A lens is a bidirectional transformation defined in CUE. Each lens is a
//! self-contained file that defines two mutually-recursive fields: `X` and
//! `Y`. `X` is defined as the transformation one way assuming `Y` is a
//! concrete value, and vice versa.
//!
//! For example, a simple lens that increases/decreases a number by 1 could be
//! defined as:
//!
//! ```cue
//! X: Y + 1
//! Y: X - 1
//! ```
//!
//! Lenses can also import and use any builtin
//! [package](https://pkg.go.dev/cuelang.org/go/pkg). Keep in mind that
//! reconfix assumes that `X` and `Y` are defined as pure functions that
//! directly or indirectly depend, at most, only on `Y` and `X`, respectively.
//! Breaking this assumption may lead to weird errors and silent
//! desynchronization. Consider using an implementation of
//! [`ExternalData`](crate::ExternalData) if this is too restrictive.
//!
//! # Templating
//!
//! **TODO**
//!
//! # Equivalence Classes
//!
//! **TODO**
//!
//! # Testing
//!
//! **TODO**

use crate::{Error, Result};
use anyhow::anyhow;
use cuelang::{
    ast::{FieldDeclaration, File, PackageDeclaration},
    Instance, Runtime, Value,
};
use lazy_static::lazy_static;
use maplit::hashmap;
use serde::Serialize;
use std::{collections::HashMap, convert::TryFrom, sync::Arc};

lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new();
    static ref COMBINE_SWITCH: HashMap<&'static str, HashMap<&'static str, &'static str>> = hashmap! {
        "a" => hashmap! {
            "X" => "X",
            "Y" => "b_X",
        },
        "b" => hashmap! {
            "X" => "a_Y",
            "Y" => "Y",
        },
    };
}

#[cfg(test)]
mod test;

/// A compiled lens that is ready to be used.
///
/// Cloning a [`Lens`] is cheap and preferable over recompiling another lens.
#[derive(Clone)]
pub struct Lens {
    shared: Arc<LensShared>,
    invert_xy: bool,
}

impl Lens {
    /// Create a new lens with the given CUE source.
    pub fn new(source: &str) -> Result<Self> {
        // Run a trim so that errors are correctly formatted
        let source = source.trim();

        // Parse the source into an AST we can introspect
        let ast = File::parse("", source)
            .map_err(|err| Error::InvalidLens(source.to_owned(), err.into()))?;

        // Run some lens-specific error checks
        let mut found_package = false;
        let mut found_x = false;
        let mut found_y = false;
        for declaration in ast.declarations() {
            if PackageDeclaration::try_from(&declaration).is_ok() {
                found_package = true;
            } else if let Ok(field) = FieldDeclaration::try_from(&declaration) {
                match &*field.name() {
                    "X" => found_x = true,
                    "Y" => found_y = true,
                    _ => (),
                }
            }
        }
        if found_package {
            return Err(Error::InvalidLens(
                source.to_owned(),
                anyhow!("found package declaration"),
            ));
        }
        if !found_x {
            return Err(Error::InvalidLens(
                source.to_owned(),
                anyhow!("missing field: X"),
            ));
        }
        if !found_y {
            return Err(Error::InvalidLens(
                source.to_owned(),
                anyhow!("missing field: Y"),
            ));
        }

        let instance = RUNTIME
            .compile_ast(&ast)
            .map_err(|err| Error::InvalidLens(source.to_owned(), err.into()))?;

        Ok(Self {
            shared: Arc::new(LensShared { instance }),
            invert_xy: false,
        })
    }

    /// Evaluate `Y` with `X` set to the given value.
    pub fn apply_x<T>(&self, x: T) -> Result<Value>
    where
        T: Serialize,
    {
        if self.invert_xy {
            self.apply(x, "Y", "X")
        } else {
            self.apply(x, "X", "Y")
        }
    }

    /// Evaluate `X` with `Y` set to the given value.
    pub fn apply_y<T>(&self, y: T) -> Result<Value>
    where
        T: Serialize,
    {
        if self.invert_xy {
            self.apply(y, "X", "Y")
        } else {
            self.apply(y, "Y", "X")
        }
    }

    fn apply<T>(&self, value: T, apply_to: &str, output: &str) -> Result<Value>
    where
        T: Serialize,
    {
        Ok(self
            .shared
            .instance
            .unify(&[apply_to], value)
            .map_err(Error::InvalidValue)?
            .get(&[output])
            .map_err(|err| {
                Error::EvalError(format!(
                    "missing field '{}': {:?}",
                    output, err
                ))
            })?)
    }

    /// Rename `X` to `Y` and `Y` to `X`.
    ///
    /// This is similar to [`inverted`](Lens::inverted) except it takes/returns
    /// the lens by reference.
    pub fn invert(&mut self) -> &mut Self {
        self.invert_xy = !self.invert_xy;

        self
    }

    /// Rename `X` to `Y` and `Y` to `X`.
    ///
    /// This is similar to [`invert`](Lens::invert) except it takes/returns the
    /// lens by value.
    pub fn inverted(mut self) -> Self {
        self.invert();

        self
    }
}

struct LensShared {
    instance: Instance,
}
