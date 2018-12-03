use std::ops::Deref;

pub trait OptionDeref<T: Deref> {
    /// Converts from `&Option<T>` to `Option<&T::Target>`
    ///
    /// Leaves the original Option in-place, creating a new
    /// one with a reference to the original one, additionally
    /// coercing the contents via `Deref`.
    ///
    /// Mimicks nightly `inner_deref` feature.
    fn as_deref(&self) -> Option<&T::Target>;
}

impl<T: Deref> OptionDeref<T> for Option<T> {
    fn as_deref(&self) -> Option<&T::Target> {
        self.as_ref().map(Deref::deref)
    }
}
