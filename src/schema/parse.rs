use super::types::{Case, ObjectSchema, Schema, TypeKind};
use error::*;
use json::Pointer;

use std::fmt;
use std::io::Read;
use std::marker;
use std::result;
use std::str::FromStr;

use serde::de::{
    self, Deserialize, Deserializer, IntoDeserializer, MapAccess, SeqAccess, Unexpected, Visitor,
};
use serde::ser::{Serialize, SerializeTuple, Serializer};
use serde_json::{self, Value};

pub fn from_reader<R>(rdr: R) -> Result<Schema>
where
    R: Read,
{
    serde_json::from_reader::<R, Schema>(rdr).chain_err(|| "unable to parse schema")
}

pub fn from_value(value: Value) -> Result<Schema> {
    serde_json::from_value(value).chain_err(|| "unable to parse schema")
}

pub fn deserialize_some<'de, T, D>(deserializer: D) -> result::Result<Option<T>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Deserialize::deserialize(deserializer).map(Some)
}

struct SchemaVisitor;

impl<'de> Visitor<'de> for SchemaVisitor {
    type Value = Schema;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a JSON Schema value")
    }

    fn visit_bool<E>(self, v: bool) -> result::Result<Schema, E>
    where
        E: de::Error,
    {
        Ok(Schema::Boolean(v))
    }

    fn visit_map<V>(self, map: V) -> result::Result<Schema, V::Error>
    where
        V: MapAccess<'de>,
    {
        let inner: ObjectSchema =
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))?;
        Ok(Schema::Object(Box::new(inner)))
    }
}

impl<'de> Deserialize<'de> for super::types::Schema {
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(SchemaVisitor)
    }
}

impl Serialize for super::types::Schema {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Schema::Boolean(ref b) => b.serialize(serializer),
            Schema::Object(ref o) => o.serialize(serializer),
        }
    }
}

struct TypeKindVisitor<T> {
    _marker: marker::PhantomData<T>,
}

impl<T> Default for TypeKindVisitor<T> {
    fn default() -> Self {
        TypeKindVisitor {
            _marker: Default::default(),
        }
    }
}

impl<'de, T> Visitor<'de> for TypeKindVisitor<T>
where
    T: Deserialize<'de>,
{
    type Value = TypeKind<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "single value or list of values")
    }

    fn visit_bool<E>(self, v: bool) -> result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(TypeKind::Single(Deserialize::deserialize(
            v.into_deserializer(),
        )?))
    }

    fn visit_i64<E>(self, v: i64) -> result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(TypeKind::Single(Deserialize::deserialize(
            v.into_deserializer(),
        )?))
    }

    fn visit_u64<E>(self, v: u64) -> result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(TypeKind::Single(Deserialize::deserialize(
            v.into_deserializer(),
        )?))
    }

    fn visit_f64<E>(self, v: f64) -> result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(TypeKind::Single(Deserialize::deserialize(
            v.into_deserializer(),
        )?))
    }

    fn visit_str<E>(self, v: &str) -> result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(TypeKind::Single(Deserialize::deserialize(
            v.into_deserializer(),
        )?))
    }

    fn visit_unit<E>(self) -> result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(TypeKind::Single(Deserialize::deserialize(
            ().into_deserializer(),
        )?))
    }

    fn visit_map<V>(self, map: V) -> result::Result<Self::Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        Ok(TypeKind::Single(Deserialize::deserialize(
            de::value::MapAccessDeserializer::new(map),
        )?))
    }

    fn visit_seq<V>(self, seq: V) -> result::Result<Self::Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        Ok(TypeKind::Set(Deserialize::deserialize(
            de::value::SeqAccessDeserializer::new(seq),
        )?))
    }
}

impl<'de, T> Deserialize<'de> for super::types::TypeKind<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let visitor: TypeKindVisitor<T> = Default::default();
        deserializer.deserialize_any(visitor)
    }
}

impl<T> Serialize for super::types::TypeKind<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            TypeKind::Single(ref x) => x.serialize(serializer),
            TypeKind::Set(ref x) => x.serialize(serializer),
        }
    }
}

struct CaseVisitor;

impl<'de> Visitor<'de> for CaseVisitor {
    type Value = Case;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "'identity' or tuple")
    }

    fn visit_str<E>(self, v: &str) -> result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        match v {
            "identity" => Ok(Case::Identity),
            "stringify" => Ok(Case::Stringify),
            x => Err(de::Error::invalid_value(Unexpected::Str(x), &"identity")),
        }
    }

    fn visit_seq<V>(self, mut seq: V) -> result::Result<Self::Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let left: Value = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &"exactly 2 items"))?;
        let right: Schema = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(1, &"exactly 2 items"))?;

        Ok(Case::Tuple(left, right))
    }
}

impl<'de> Deserialize<'de> for super::types::Case {
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CaseVisitor)
    }
}

impl Serialize for super::types::Case {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Case::Identity => "identity".serialize(serializer),
            Case::Stringify => "stringify".serialize(serializer),
            Case::Tuple(ref l, ref r) => {
                let mut tuple = serializer.serialize_tuple(2)?;
                tuple.serialize_element(l)?;
                tuple.serialize_element(r)?;
                tuple.end()
            },
        }
    }
}

impl<'de> Deserialize<'de> for Pointer {
    fn deserialize<D>(deserialzier: D) -> result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserialzier)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

impl<'de> Serialize for Pointer {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

#[cfg(test)]
mod tests {

    use super::super::types::*;
    use super::*;

    macro_rules! testfile {
        ($name:ident) => {
            include_str!(concat!(
                "../../tests/testdata/schemas/",
                stringify!($name),
                ".json"
            ))
        };
    }

    fn parse_schema(data: &str) -> Schema {
        from_reader(data.as_bytes()).expect("unable to parse test file")
    }

    fn parse_object(data: &str) -> ObjectSchema {
        match parse_schema(data) {
            Schema::Object(o) => *o,
            _ => panic!("expected object, found boolean"),
        }
    }

    #[test]
    fn parse_boolean() {
        let schema = parse_schema(testfile!(boolean));

        assert_eq!(schema, Schema::Boolean(true));
    }

    #[test]
    fn parse_empty_object() {
        let schema = parse_schema(testfile!(empty_object));
        let expected = Box::new(ObjectSchema::default());

        assert_eq!(schema, Schema::Object(expected));
    }

    #[test]
    fn parse_const() {
        let schema = parse_object(testfile!(constant));
        let expected = Some(json!({ "foo": "bar" }));
        assert_eq!(schema.const_, expected);
    }

    #[test]
    fn parse_ref() {
        let schema = parse_object(testfile!(reference));
        let expected = Some(String::from("#"));
        assert_eq!(schema.ref_, expected);
    }

    #[test]
    fn parse_single_type() {
        let schema = parse_object(testfile!(single_type));
        let expected = Some(TypeKind::Single(Type::String));
        assert_eq!(schema.type_, expected);
    }

    #[test]
    fn parse_multi_type() {
        let schema = parse_object(testfile!(multi_type));
        let expected = Some(TypeKind::Set(vec![Type::String, Type::Number]));
        assert_eq!(schema.type_, expected);
    }

    #[test]
    fn parse_reconfix() {
        let schema = parse_object(testfile!(reconfix));
        let expected = Some(Default::default());
        assert_eq!(schema.reconfix, expected);
    }

    #[test]
    fn parse_reconfix_target_disk() {
        let schema = parse_object(testfile!(reconfix_target_disk));
        let mut expected = Map::new();
        expected.insert(
            "disk_file".into(),
            Target::File {
                format: Format::Json,
                location: Location::Disk {
                    partition: Partition::String("boot".into()),
                    path: "/foo/bar".into(),
                },
            },
        );
        let result = schema.reconfix.expect("no reconfix object found");
        assert_eq!(result.targets, expected);
    }

    #[test]
    fn parse_reconfix_target_nested() {
        let schema = parse_object(testfile!(reconfix_target_nested));
        let mut expected = Map::new();
        expected.insert(
            "nested_file".into(),
            Target::File {
                format: Format::Ini,
                location: Location::Nested {
                    file: "#/another/file".into(),
                    path: "/foo/bar".into(),
                },
            },
        );
        let result = schema.reconfix.expect("no reconfix object found");
        assert_eq!(result.targets, expected);
    }

    #[test]
    fn parse_reconfix_target_network_manager() {
        let schema = parse_object(testfile!(reconfix_target_network_manager));
        let mut expected = Map::new();
        expected.insert("nm".into(), Target::NetworkManager);
        let result = schema.reconfix.expect("no reconfix object found");
        assert_eq!(result.targets, expected);
    }
}
