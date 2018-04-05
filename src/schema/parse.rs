
use super::types::{Schema, ObjectSchema, TypeKind};
use ::error::*;

use std::fmt;
use std::io::Read;
use std::marker;
use std::result;

use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess, SeqAccess, IntoDeserializer};
use serde_json;

pub fn from_reader<R>(rdr: R) -> Result<Schema> 
    where R: Read
{
    serde_json::from_reader::<R, Schema>(rdr)
        .chain_err(|| "unable to parse schema")
}

struct SchemaVisitor;

impl<'de> Visitor<'de> for SchemaVisitor {
    type Value = Schema;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a JSON Schema value")
    }

    fn visit_bool<E>(self, v: bool) -> result::Result<Schema, E> 
        where E: de::Error
    {
        Ok(Schema::Boolean(v))
    }

    fn visit_map<V>(self, map: V) -> result::Result<Schema, V::Error>
        where V: MapAccess<'de>
    {
        let inner: ObjectSchema = Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))?;
        Ok(Schema::Object(Box::new(inner)))
    }
}

impl<'de> Deserialize<'de> for super::types::Schema {
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_any(SchemaVisitor)
    }
}

struct TypeKindVisitor<T> {
    _marker: marker::PhantomData<T>
}

impl<T> Default for TypeKindVisitor<T> {
    fn default() -> Self {
        TypeKindVisitor {
            _marker: Default::default()
        }
    }
}

impl<'de, T> Visitor<'de> for TypeKindVisitor<T> 
    where T: Deserialize<'de>
{
    type Value = TypeKind<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "single value or list of values")
    }

    fn visit_bool<E>(self, v: bool) -> result::Result<Self::Value, E> 
        where E: de::Error
    {
        Ok(TypeKind::Single(Deserialize::deserialize(v.into_deserializer())?))
    }

    fn visit_i64<E>(self, v: i64) -> result::Result<Self::Value, E> 
        where E: de::Error
    {
        Ok(TypeKind::Single(Deserialize::deserialize(v.into_deserializer())?))
    }

    fn visit_u64<E>(self, v: u64) -> result::Result<Self::Value, E> 
        where E: de::Error
    {
        Ok(TypeKind::Single(Deserialize::deserialize(v.into_deserializer())?))
    }

    fn visit_f64<E>(self, v: f64) -> result::Result<Self::Value, E> 
        where E: de::Error
    {
        Ok(TypeKind::Single(Deserialize::deserialize(v.into_deserializer())?))
    }

    fn visit_str<E>(self, v: &str) -> result::Result<Self::Value, E> 
        where E: de::Error
    {
        Ok(TypeKind::Single(Deserialize::deserialize(v.into_deserializer())?))
    }

    fn visit_unit<E>(self) -> result::Result<Self::Value, E>
        where E: de::Error
    {
        Ok(TypeKind::Single(Deserialize::deserialize(().into_deserializer())?))
    }

    fn visit_map<V>(self, map: V) -> result::Result<Self::Value, V::Error>
        where V: MapAccess<'de>
    {
        Ok(TypeKind::Single(Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))?))
    }

    fn visit_seq<V>(self, seq: V) -> result::Result<Self::Value, V::Error>
        where V: SeqAccess<'de>
    {
        Ok(TypeKind::Set(Deserialize::deserialize(de::value::SeqAccessDeserializer::new(seq))?))
    }
}

impl<'de, T> Deserialize<'de> for super::types::TypeKind<T> 
    where T: Deserialize<'de>
{
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let visitor: TypeKindVisitor<T> = Default::default();
        deserializer.deserialize_any(visitor)
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use super::super::types::*;

    macro_rules! testfile {
        ( $name:ident ) => {
            include_str!(concat!("../../tests/testdata/schemas/", stringify!($name), ".json"))
        }
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
        expected.insert("disk_file".into(), Target::File { 
            format: Format::Json,
            location: Location::Disk {
                partition: Partition::String("boot".into()),
                path: "/foo/bar".into(),
            },
        });
        let result = schema.reconfix.expect("no reconfix object found");
        assert_eq!(result.targets, expected);
    }

    #[test]
    fn parse_reconfix_target_nested() {
        let schema = parse_object(testfile!(reconfix_target_nested));
        let mut expected = Map::new();
        expected.insert("nested_file".into(), Target::File { 
            format: Format::Ini,
            location: Location::Nested {
                file: "#/another/file".into(),
                path: "/foo/bar".into(),
            },
        });
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