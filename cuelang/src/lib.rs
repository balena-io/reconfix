//! Rusty bindings to [cuelang](https://cuelang.org/).

use crate::ast::File;
use cuelang_sys::{
    cue_Instance, cue_Runtime, cue_Value, ffiString, GoAny, GoMap, GoPtr,
    GoSlice,
};
use lazy_static::lazy_static;
use serde::{
    ser::{
        self, SerializeMap, SerializeSeq, SerializeStruct,
        SerializeStructVariant, SerializeTuple, SerializeTupleStruct,
        SerializeTupleVariant, Serializer,
    },
    Serialize,
};
use std::{convert::TryFrom, fmt::Debug, fmt::Display, result, sync::Mutex};
use thiserror::Error;

pub mod ast;

#[cfg(test)]
mod test;

lazy_static! {
    // Compilation-related methods in CUE access shared resources without
    // proper locking so we need a global lock for those
    static ref COMPILE_LOCK: Mutex<()> = Mutex::new(());
}

/// This crate's error type.
#[derive(Debug, Error)]
pub enum Error {
    /// Compilation of a CUE fragment failed.
    #[error("compilation failed: {0}")]
    Compile(String),

    /// Go does not have sum types, thus there is no unambiguous way to send
    /// those to go
    #[error("cannot send enums to go")]
    EnumAsGoValue,

    /// Parsing of a CUE fragment failed.
    #[error("parsing failed: {0}")]
    Parse(String),

    /// A [`Value`] is invalid.
    #[error("invalid value: {0}")]
    InvalidValue(String),

    /// Serialization into `GoAny` failed.
    #[error("serialization into GoAny failed: {0}")]
    SerializingGoAny(String),

    /// Failed to unify a value with an instance.
    #[error("failed to unify a value with an instance: {0}")]
    UnifyValue(String),
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::SerializingGoAny(msg.to_string())
    }
}

/// This crate's result type.
pub type Result<T> = result::Result<T, Error>;

/// A cuelang runtime.
pub struct Runtime {
    inner: GoPtr<cue_Runtime>,
}

impl Runtime {
    /// Create a new runtime.
    pub fn new() -> Self {
        Self::default()
    }

    /// Compile a single CUE file.
    pub fn compile(&self, name: &str, source: &str) -> Result<Instance> {
        let _lock = COMPILE_LOCK.lock().unwrap();

        try_value(
            self.inner.Compile(name, &GoAny::from_str(source)),
            Error::Compile,
        )
        .map(Instance::from)
    }

    /// Compile a previously parsed CUE file or expression.
    pub fn compile_ast(&self, ast: &File) -> Result<Instance> {
        let _lock = COMPILE_LOCK.lock().unwrap();

        try_value(self.inner.CompileFile(ast.inner()), Error::Compile)
            .map(Instance::from)
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self {
            inner: GoPtr::new(cue_Runtime::default()),
        }
    }
}

/// A collection of CUE files compiled into a single object.
pub struct Instance {
    inner: GoPtr<cue_Instance>,
}

impl Instance {
    /// Get a value at the specified path, if it exists and is valid.
    pub fn get<P, I>(&self, path: P) -> Result<Value>
    where
        P: IntoIterator<Item = I>,
        I: AsRef<str>,
    {
        let value = self.inner.Lookup(&to_go_str_slice(path));

        Value::try_from(value)
    }

    /// Compute a new [`Instance`] where the specified path is unified with the
    /// given value.
    pub fn unify<P, I, V>(&self, path: P, value: V) -> Result<Self>
    where
        P: IntoIterator<Item = I>,
        I: AsRef<str>,
        V: Serialize,
    {
        let _lock = COMPILE_LOCK.lock().unwrap();

        try_value(
            self.inner.Fill(&to_go_any(value)?, &to_go_str_slice(path)),
            Error::UnifyValue,
        )
        .map(Instance::from)
    }
}

impl From<GoPtr<cue_Instance>> for Instance {
    fn from(inner: GoPtr<cue_Instance>) -> Self {
        Self { inner }
    }
}

/// A single CUE value.
pub struct Value {
    inner: cue_Value,
}

impl Value {
    /// Return `self` as a bool if the underlying value is a bool.
    pub fn as_bool(&self) -> Option<bool> {
        try_value_option(self.inner.Bool())
    }

    /// Return `self` as a float if the underlying value is a float that can
    /// fit in 64 bits.
    pub fn as_float(&self) -> Option<f64> {
        try_value_option(self.inner.Float64())
    }

    /// Return `self` as an int if the underlying value is an int that can fit
    /// in 64 bits.
    pub fn as_int(&self) -> Option<i64> {
        try_value_option(self.inner.Int64())
    }

    /// Return `self` as a string if the underlying value is a string.
    pub fn as_string(&self) -> Option<ffiString> {
        try_value_option(self.inner.String())
    }

    /// Return `self` as a JSON map if the underlying value is a map.
    pub fn as_map(&self) -> Option<serde_json::Map<String, serde_json::Value>> {
        let mut options = GoSlice::make(cuelang_sys::Final(), 1);
        options.push(cuelang_sys::Final());
        let iter = try_value_option(self.inner.Fields(&options))?;
        let mut map = serde_json::Map::new();
        while iter.Next() {
            let value = if let Ok(Some(value)) =
                Value::try_from(iter.Value()).map(|x| x.as_json_value())
            {
                value
            } else {
                return None;
            };
            map.insert(iter.Label().to_string(), value);
        }

        Some(map)
    }

    /// Return `self` as a JSON value if the underlying value can be
    /// interpreted as a JSON value.
    pub fn as_json_value(&self) -> Option<serde_json::Value> {
        if let Some(x) = self.as_bool() {
            Some(x.into())
        } else if let Some(x) = self.as_int() {
            // Make sure we test `as_int` before `as_float`, as the latter will
            // return a floatified int if it can
            Some(x.into())
        } else if let Some(x) = self.as_float() {
            Some(x.into())
        } else if let Some(x) = self.as_string() {
            Some((*x).into())
        } else if let Some(x) = self.as_map() {
            Some(x.into())
        } else {
            None
        }
    }
}

impl TryFrom<cue_Value> for Value {
    type Error = Error;

    fn try_from(inner: cue_Value) -> Result<Self> {
        let mut slice = GoSlice::make(cuelang_sys::All(), 0);
        slice.push(cuelang_sys::Concrete(true));
        let err = inner.Validate(&slice);
        if !err.is_empty() {
            return Err(Error::InvalidValue(err.to_string()));
        }

        Ok(Self { inner })
    }
}

enum GoAnySerializer {
    Any(GoAny),
    Map {
        len_hint: usize,
        items: Vec<(Self, Self)>,
        key: Option<Box<Self>>,
    },
    Null,
    Slice {
        len_hint: usize,
        items: Vec<Self>,
    },
    String(String),
}

impl GoAnySerializer {
    fn new() -> Self {
        Self::Null
    }

    fn into_go_any(self) -> GoAny {
        // TODO: need to specialize `GoSlice`s and `GoMap`s
        match self {
            Self::Any(go_any) => go_any,
            Self::Map {
                len_hint, items, ..
            } => {
                let mut map = GoMap::<String, GoAny>::with_capacity(len_hint);
                for (key, value) in items {
                    map.insert(
                        key.into_string()
                            .expect("map keys can only be strings"),
                        value.into_go_any(),
                    );
                }

                map.into()
            }
            Self::Null => GoAny::nil_interface(),
            Self::Slice { len_hint, items } => {
                let mut slice = GoSlice::with_capacity(len_hint);
                for item in items {
                    slice.push(item.into_go_any());
                }

                slice.into()
            }
            Self::String(string) => string.into(),
        }
    }

    fn into_string(self) -> Option<String> {
        match self {
            Self::String(string) => Some(string),
            _ => None,
        }
    }
}

impl Serializer for GoAnySerializer {
    type Ok = Self;
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, value: bool) -> Result<Self> {
        Ok(Self::Any(GoAny::from(value)))
    }

    fn serialize_i8(self, value: i8) -> Result<Self> {
        Ok(Self::Any(GoAny::from(value)))
    }

    fn serialize_i16(self, value: i16) -> Result<Self> {
        Ok(Self::Any(GoAny::from(value)))
    }

    fn serialize_i32(self, value: i32) -> Result<Self> {
        Ok(Self::Any(GoAny::from(value)))
    }

    fn serialize_i64(self, value: i64) -> Result<Self> {
        Ok(Self::Any(GoAny::from(value)))
    }

    fn serialize_u8(self, value: u8) -> Result<Self> {
        Ok(Self::Any(GoAny::from(value)))
    }

    fn serialize_u16(self, value: u16) -> Result<Self> {
        Ok(Self::Any(GoAny::from(value)))
    }

    fn serialize_u32(self, value: u32) -> Result<Self> {
        Ok(Self::Any(GoAny::from(value)))
    }

    fn serialize_u64(self, value: u64) -> Result<Self> {
        Ok(Self::Any(GoAny::from(value)))
    }

    fn serialize_f32(self, value: f32) -> Result<Self> {
        Ok(Self::Any(GoAny::from(value)))
    }

    fn serialize_f64(self, value: f64) -> Result<Self> {
        Ok(Self::Any(GoAny::from(value)))
    }

    fn serialize_char(self, _value: char) -> Result<Self> {
        unimplemented!()
    }

    fn serialize_str(self, value: &str) -> Result<Self> {
        Ok(Self::String(value.to_owned()))
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self> {
        // TODO: copy the slice and build a `GoSlice` instead
        Ok(Self::Slice {
            len_hint: value.len(),
            items: value
                .iter()
                .copied()
                .map(|x| Self::Any(GoAny::from(x)))
                .collect(),
        })
    }

    fn serialize_none(self) -> Result<Self> {
        Ok(Self::Null)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self> {
        Ok(Self::Null)
    }

    fn serialize_unit_struct(self, _: &str) -> Result<Self> {
        Ok(Self::Null)
    }

    fn serialize_unit_variant(self, _: &str, _: u32, _: &str) -> Result<Self> {
        Err(Error::EnumAsGoValue)
    }

    fn serialize_newtype_struct<T>(self, _: &str, value: &T) -> Result<Self>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _: &str,
        _: u32,
        _: &str,
        _: &T,
    ) -> Result<Self>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::EnumAsGoValue)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        let len_hint = len.unwrap_or(0);

        Ok(Self::Slice {
            len_hint,
            items: Vec::with_capacity(len_hint),
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        Ok(Self::Slice {
            len_hint: len,
            items: Vec::with_capacity(len),
        })
    }

    fn serialize_tuple_struct(
        self,
        _: &str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(Self::Slice {
            len_hint: len,
            items: Vec::with_capacity(len),
        })
    }

    fn serialize_tuple_variant(
        self,
        _: &str,
        _: u32,
        _: &str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::EnumAsGoValue)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        let len_hint = len.unwrap_or(0);

        Ok(Self::Map {
            len_hint,
            items: Vec::with_capacity(len_hint),
            key: None,
        })
    }

    fn serialize_struct(
        self,
        _: &str,
        len: usize,
    ) -> Result<Self::SerializeStruct> {
        Ok(Self::Map {
            len_hint: len,
            items: Vec::with_capacity(len),
            key: None,
        })
    }

    fn serialize_struct_variant(
        self,
        _: &str,
        _: u32,
        _: &str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::EnumAsGoValue)
    }
}

impl SerializeSeq for GoAnySerializer {
    type Ok = Self;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match self {
            Self::Slice { items, .. } => {
                items.push(value.serialize(GoAnySerializer::new())?);
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    fn end(self) -> Result<Self> {
        Ok(self)
    }
}

impl SerializeTuple for GoAnySerializer {
    type Ok = Self;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        <Self as SerializeSeq>::serialize_element(self, value)
    }

    fn end(self) -> Result<Self> {
        <Self as SerializeSeq>::end(self)
    }
}

impl SerializeTupleStruct for GoAnySerializer {
    type Ok = Self;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        <Self as SerializeSeq>::serialize_element(self, value)
    }

    fn end(self) -> Result<Self> {
        <Self as SerializeSeq>::end(self)
    }
}

impl SerializeTupleVariant for GoAnySerializer {
    type Ok = Self;
    type Error = Error;

    fn serialize_field<T>(&mut self, _: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Self> {
        unreachable!()
    }
}

impl SerializeMap for GoAnySerializer {
    type Ok = Self;
    type Error = Error;

    fn serialize_key<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match self {
            Self::Map { key, .. } => {
                *key = Some(Box::new(value.serialize(GoAnySerializer::new())?));
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match self {
            Self::Map { items, key, .. } => {
                items.push((
                    *key.take().unwrap(),
                    value.serialize(GoAnySerializer::new())?,
                ));
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    fn end(self) -> Result<Self> {
        Ok(self)
    }
}

impl SerializeStruct for GoAnySerializer {
    type Ok = Self;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        <Self as SerializeMap>::serialize_entry(self, key, value)
    }

    fn end(self) -> Result<Self> {
        Ok(self)
    }
}

impl SerializeStructVariant for GoAnySerializer {
    type Ok = Self;
    type Error = Error;

    fn serialize_field<T>(&mut self, _: &str, _: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Self> {
        unreachable!()
    }
}

fn to_go_any<V>(value: V) -> Result<GoAny>
where
    V: Serialize,
{
    Ok(value.serialize(GoAnySerializer::new())?.into_go_any())
}

fn to_go_str_slice<Iter, Item>(iter: Iter) -> GoSlice<String>
where
    Iter: IntoIterator<Item = Item>,
    Item: AsRef<str>,
{
    let mut slice = GoSlice::new();
    for item in iter {
        slice.push_str(item.as_ref());
    }

    slice
}

fn try_value<T, E>(x: (T, ffiString), error_constructor: E) -> Result<T>
where
    E: FnOnce(String) -> Error,
{
    let (x, err) = x;

    if err.is_empty() {
        Ok(x)
    } else {
        Err(error_constructor(err.to_string()))
    }
}

fn try_error<E>(err: ffiString, error_constructor: E) -> Result<()>
where
    E: FnOnce(String) -> Error,
{
    if err.is_empty() {
        Ok(())
    } else {
        Err(error_constructor(err.to_string()))
    }
}

fn try_value_option<T>(x: (T, ffiString)) -> Option<T> {
    let (x, err) = x;

    if err.is_empty() {
        Some(x)
    } else {
        None
    }
}
