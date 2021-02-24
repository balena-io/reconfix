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
        let mut has_xsave = false;
        let mut has_ysave = false;
        for declaration in ast.declarations() {
            if PackageDeclaration::try_from(&declaration).is_ok() {
                found_package = true;
            } else if let Ok(field) = FieldDeclaration::try_from(&declaration) {
                match &*field.name() {
                    "X" => found_x = true,
                    "XSAVE" => has_xsave = true,
                    "Y" => found_y = true,
                    "YSAVE" => has_ysave = true,
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
        if has_xsave && has_ysave {
            return Err(Error::InvalidLens(
                source.to_owned(),
                anyhow!("a single lens cannot declare both XSAVE and YSAVE at the same time"),
            ));
        }

        let instance = RUNTIME
            .compile_ast(&ast)
            .map_err(|err| Error::InvalidLens(source.to_owned(), err.into()))?;

        Ok(Self {
            shared: Arc::new(LensShared {
                instance,
                has_xsave,
                has_ysave,
            }),
            invert_xy: false,
        })
    }

    /// Return `true` if this lens is stateful. That is, it declares either an
    /// `XSAVE` or an `YSAVE` field.
    pub fn is_stateful(&self) -> bool {
        self.shared.has_xsave || self.shared.has_ysave
    }

    /// Return `true` if this lens declares an `XSAVE` field.
    pub fn has_xsave(&self) -> bool {
        self.shared.has_xsave
    }

    /// Return `true` if this lens declares an `YSAVE` field.
    pub fn has_ysave(&self) -> bool {
        self.shared.has_ysave
    }

    /// Evaluate `Y` and `YSAVE` (if present) with `X`, and optionally `XSAVE`,
    /// set to the given values.
    pub fn apply_x<X, XS>(
        &self,
        x: X,
        xsave: Option<XS>,
    ) -> Result<(Value, Option<Value>)>
    where
        X: Serialize,
        XS: Serialize,
    {
        if self.invert_xy {
            self.apply(x, xsave, Field::Y)
        } else {
            self.apply(x, xsave, Field::X)
        }
    }

    /// Evaluate `X` and `XSAVE` (if present) with `Y`, and optionally `YSAVE`,
    /// set to the given values.
    pub fn apply_y<Y, YS>(
        &self,
        y: Y,
        ysave: Option<YS>,
    ) -> Result<(Value, Option<Value>)>
    where
        Y: Serialize,
        YS: Serialize,
    {
        if self.invert_xy {
            self.apply(y, ysave, Field::X)
        } else {
            self.apply(y, ysave, Field::Y)
        }
    }

    fn apply<V, S>(
        &self,
        value: V,
        save: Option<S>,
        apply_to: Field,
    ) -> Result<(Value, Option<Value>)>
    where
        V: Serialize,
        S: Serialize,
    {
        if save.is_some() && !self.field_is_saved(apply_to) {
            return Err(Error::EvalError(
                "lens has no appropriate SAVE field but one was passed to the apply call".to_string()
            ));
        }
        if !save.is_some() && self.field_is_saved(apply_to) {
            return Err(Error::EvalError(
                "lens has a SAVE field but its value was not passed to the apply call".to_string()
            ));
        }

        let mut unfied = self
            .shared
            .instance
            .unify(&[apply_to.as_str()], value)
            .map_err(Error::InvalidValue)?;
        if let Some(save) = save {
            unfied = unfied
                .unify(&[apply_to.as_save_str()], save)
                .map_err(Error::InvalidValue)?;
        }

        let output_field = apply_to.inverse();
        let output = unfied.get(&[output_field.as_str()]).map_err(|err| {
            Error::EvalError(format!(
                "cannot evaluate '{}': {:?}",
                output_field.as_str(),
                err
            ))
        })?;
        let output_save = if self.field_is_saved(output_field) {
            Some(unfied.get(&[output_field.as_save_str()]).map_err(|err| {
                Error::EvalError(format!(
                    "cannot evaluate '{}': {:?}",
                    output_field.as_save_str(),
                    err
                ))
            })?)
        } else {
            None
        };

        Ok((output, output_save))
    }

    fn field_is_saved(&self, field: Field) -> bool {
        match field {
            Field::X => self.shared.has_xsave,
            Field::Y => self.shared.has_ysave,
        }
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
    has_xsave: bool,
    has_ysave: bool,
}

#[derive(Clone, Copy)]
enum Field {
    X,
    Y,
}

impl Field {
    fn as_str(self) -> &'static str {
        match self {
            Self::X => "X",
            Self::Y => "Y",
        }
    }

    fn as_save_str(self) -> &'static str {
        match self {
            Self::X => "XSAVE",
            Self::Y => "YSAVE",
        }
    }

    fn inverse(self) -> Self {
        match self {
            Self::X => Self::Y,
            Self::Y => Self::X,
        }
    }
}
