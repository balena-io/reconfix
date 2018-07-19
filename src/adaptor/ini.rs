use std::cmp::Ordering;
use std::io::{Read, Write};
use std::str;

use crate::adaptor::Adaptor;
use crate::error::*;

use nom::{self, multispace, space, IResult, Needed};

use serde_json::map::Entry;
use serde_json::{Map, Number, Value};

/// The `Property` enum is used to represent a heirarchichal data
/// structure. It is required to properly sort the data for
/// serialization.
#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
enum Property {
    Value(String),
    Section(Vec<Pair>),
}

/// The `Pair` struct is used to represent a key and either a section or value.
#[derive(Eq, PartialEq, Debug)]
struct Pair(String, Property);

/// This ordering is used to correctly sort sections before serialization.
impl Ord for Pair {
    fn cmp(&self, other: &Pair) -> Ordering {
        match (&self.1, &other.1) {
            (&Property::Value(_), &Property::Section(_)) => Ordering::Less,
            (&Property::Section(_), &Property::Value(_)) => Ordering::Greater,
            _ => self.0.cmp(&other.0),
        }
    }
}

/// A simple passthrough implementation.
impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Pair) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// The adaptor struct for INI files
/// Later, this might contain parameters for the myriad INI quirks
#[derive(Default)]
pub struct IniAdaptor {}

impl IniAdaptor {
    /// Constructs a new `IniAdaptor`
    pub fn new() -> IniAdaptor {
        IniAdaptor {}
    }
}

impl<'a> Adaptor<'a> for IniAdaptor {
    /// Deserialize the INI data into the `Value` AST
    fn deserialize<R>(&self, mut reader: R) -> Result<Value>
    where
        R: Read,
    {
        let mut buffer = Vec::new();
        reader
            .read_to_end(&mut buffer)
            .map_err(|e| ErrorKind::Io(e.into()))?;

        // parse the basic INI structure
        let (no_section, sections) = match ini_file(&buffer) {
            IResult::Done(_, o) => o,
            _ => return Err("unable to parse INI data".into()),
        };

        let mut full_map = Map::new();

        // insert headerless values
        insert_all(&mut full_map, no_section);

        // insert the sections
        for (name, pairs) in sections {
            insert_section(&mut full_map, name, pairs);
        }

        Ok(Value::Object(full_map))
    }

    /// Serialize the `Value` AST into INI format
    fn serialize<W>(&self, value: Value, mut writer: W) -> Result<()>
    where
        W: Write,
    {
        // extract the root object, else error
        let top_level = match value {
            Value::Object(o) => o,
            _ => return Err("invalid root element".into()),
        };

        let properties = convert_model(top_level)?;

        write_section(None, properties, &mut writer)
    }
}

/// Use heurisitics to determine a value's type
fn infer_type(value: &str) -> Value {
    value.parse::<u64>().map(|x| Value::Number(x.into()))
        .or_else(|_| value.parse::<i64>().map(|x| Value::Number(x.into())))
        // will fix this unwrap once we add error_chain, for now not very dangerous
        .or_else(|_| value.parse::<f64>().map(|x| {
            Value::Number(Number::from_f64(x).unwrap())
        }))
        .or_else(|_| value.parse::<bool>().map(Value::Bool))
        .unwrap_or_else(|_| Value::String(value.into()))
}

/// Iterate through all key value pairs, and insert them into the map
fn insert_all(map: &mut Map<String, Value>, values: Vec<(&str, &str)>) {
    for (key, value) in values {
        insert_or_expand(map, key.to_string(), infer_type(value));
    }
}

/// Insert a new value or create an array if there are duplicates
fn insert_or_expand(map: &mut Map<String, Value>, key: String, value: Value) {
    match map.entry(key) {
        Entry::Vacant(e) => {
            e.insert(value);
        },
        Entry::Occupied(mut e) => {
            // we use a dummy value here so we can replace it with
            // the modified value later. If we remove the value,
            // we lose ownership of the `Entry`.
            let current = e.insert(Value::Bool(false));
            let modified = match current {
                Value::Array(mut a) => {
                    a.push(value);
                    a
                },
                x => vec![x, value],
            };

            // add back the modified vector, droping the dummy value
            e.insert(Value::Array(modified));
        },
    }
}

/// Inserts all the values of a section into the configuration heirarchy
/// Performs recursive lookups on the `Map` until the correct object is found
fn insert_section(root: &mut Map<String, Value>, section_name: &str, pairs: Vec<(&str, &str)>) {
    // recursively query the object using the split section name
    let mut insert_map = section_name.split('.').fold(root, |map, key| {
        let entry = map
            .entry(key.trim())
            .or_insert_with(|| Value::Object(Map::new()));
        match *entry {
            Value::Object(ref mut sub) => sub,
            _ => panic!("name collision"),
        }
    });

    // insert the values
    insert_all(&mut insert_map, pairs);
}

/// Convert a property value into a representative `String`
/// Returns an `Error` if the value is not representable as a `String`
fn flatten_value(value: &Value) -> Result<String> {
    let string = match *value {
        Value::Bool(ref x) => x.to_string(),
        Value::String(ref x) => x.to_string(),
        Value::Number(ref x) => x.to_string(),
        _ => return Err("invalid element".into()),
    };

    Ok(string)
}

/// Emits a vector of key-value pairs representing the specified
/// property and value. This will stringify primitive values and
/// convert an array into multiple key-value pairs. Object values
/// are not allowed.
fn emit_values(key: &str, value: &Value) -> Result<Vec<Pair>> {
    let result = match (flatten_value(value), value) {
        (Ok(x), _) => Ok(vec![x]),
        (_, &Value::Array(ref elems)) => {
            elems.iter().map(flatten_value).collect::<Result<Vec<_>>>()
        },
        _ => Err("invalid value".into()),
    };

    let tuples = result?
        .into_iter()
        .map(|x| Pair(key.to_string(), Property::Value(x)))
        .collect::<Vec<_>>();

    Ok(tuples)
}

/// Convert a root `Value` object into a list of named
/// sections with key-value pairs in each. This converts
/// the internal configuration model to the INI data model.
fn convert_model(object: Map<String, Value>) -> Result<Vec<Pair>> {
    // filter out top-level key-value pairs and only use sections
    let section_map = object.into_iter().map(|(key, value)| {
        match value {
            Value::Object(o) => {
                convert_model(o).map(|props| vec![Pair(key, Property::Section(props))])
            },
            x => emit_values(&key, &x),
        }
    });

    // perform some flattening
    let flat_result = section_map.collect::<Result<Vec<_>>>();
    flat_result.map(|x| {
        x.into_iter()
            .flat_map(|y| y.into_iter())
            .collect::<Vec<_>>()
    })
}

/// Recursively serialize section data
fn write_section<W>(name: Option<&str>, mut data: Vec<Pair>, writer: &mut W) -> Result<()>
where
    W: Write,
{
    data.sort();

    // don't output a header for top-level values
    if let Some(txt) = name {
        writeln!(writer, "[{}]", txt).unwrap();
    }

    // chain names together for subsections
    let parent_name = name
        .map(|x| x.to_string() + ".")
        .unwrap_or_else(|| "".to_string());

    for Pair(mut key, value) in data {
        match value {
            Property::Value(v) => {
                writeln!(writer, "{} = {}", key, v).unwrap();
            },
            Property::Section(s) => {
                writeln!(writer).unwrap();
                key.insert_str(0, &parent_name);
                write_section(Some(&key), s, writer)?;
            },
        };
    }

    Ok(())
}

/// Parses the section name from the `[header]`
named!(
    section_name<&str>,
    map_res!(
        delimited!(char!('['), is_not!("]"), char!(']')),
        str::from_utf8
    )
);

/// Parses a `# comment` value
named!(
    comment,
    delimited!(
        tag!(b"#"),
        take_while!(call!(|c| c != b'\n')),
        opt!(complete!(tag!("\n")))
    )
);

/// Parses and swallows any whitespace or comments
named_attr!(
    #[allow(unreachable_pub)],
    blanks,
    map!(many0!(alt!(comment | multispace)), |_| &b""[..])
);

/// Parses a key element terminated by whitespace or an `=` character.
fn key(i: &[u8]) -> IResult<&[u8], &[u8]> {
    // Find the longest valid utf-8 string in the current slice.
    let text = match str::from_utf8(i) {
        Ok(s) => s,
        Err(e) => {
            let boundary = e.valid_up_to();
            if boundary > 0 {
                // Rust guarantees that boundary is safe
                unsafe { str::from_utf8_unchecked(&i[..boundary]) }
            } else {
                return IResult::Incomplete(Needed::Unknown);
            }
        },
    };

    // Search for the byte index of a terminating character.
    let terminator = text
        .char_indices()
        .find(|&(_, elem)| elem.is_whitespace() || elem == '=')
        .map(|(index, _)| index);

    match terminator {
        // We don't allow zero length keys
        Some(0) => IResult::Error(nom::ErrorKind::Custom(0)),
        Some(index) => IResult::Done(&i[index..], &i[..index]),
        None => IResult::Incomplete(Needed::Unknown),
    }
}

/// Parses a `key = value` pair and returns a tuple
named!(key_value_pair <&[u8],(&str,&str)>,
    do_parse!(
        key: map_res!(key, str::from_utf8)
        >> opt!(space)
        >> char!('=')
        >> not!(char!('='))
        >> opt!(space)
        >> value: map_res!(
            // There may be more elegant parsers, but this is the only one
            // I've tested that doesn't choke on EOF. Needs more investigation.
            take_while!(call!(|c| c != b'\n' && c != b'#')),
            str::from_utf8
        )
        >> opt!(complete!(comment))
        >> (key, value)
    )
);

/// Parses a group of key value pairs
named!(key_value_group<&[u8], Vec<(&str, &str)>>,
    many0!(terminated!(key_value_pair, opt!(complete!(blanks))))
);

/// Parses a section header and all included key value pairs
named!(section<&[u8], (&str, Vec<(&str, &str)>)>,
    do_parse!(
        opt!(complete!(blanks))
        >> section: section_name
        >> opt!(complete!(blanks))
        >> pairs: key_value_group
        >> (section, pairs)
    )
);

/// Parses a full INI file
named!(ini_file<&[u8], (Vec<(&str, &str)>, Vec<(&str, Vec<(&str, &str)>)>)>,
    do_parse!(
        no_section: key_value_group
        >> section: many0!(section)
        >> (no_section, section)
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    use std::fmt::Debug;

    use nom::ErrorKind;

    #[test]
    fn infer_type_boolean_true() {
        assert_eq!(infer_type("true"), Value::Bool(true));
    }

    #[test]
    fn infer_type_boolean_false() {
        assert_eq!(infer_type("false"), Value::Bool(false));
    }

    #[test]
    fn infer_type_boolean_incorrect() {
        assert_eq!(infer_type("True"), Value::String("True".to_string()));
    }

    #[test]
    fn infer_type_number_integer() {
        assert_eq!(infer_type("1234"), Value::Number(1234.into()));
    }

    #[test]
    fn infer_type_number_decimal() {
        assert_eq!(
            infer_type("12.34"),
            Value::Number(Number::from_f64(12.34).unwrap())
        );
    }

    #[test]
    fn deserialize_ini_section() {
        let adaptor = IniAdaptor::new();
        let ini = b"[section]\nkey = value";

        let value = adaptor.deserialize(&ini[..]).unwrap();
        let mut pairs = Map::new();
        pairs.insert("key".to_string(), Value::String("value".to_string()));
        let mut sections = Map::new();
        sections.insert("section".to_string(), Value::Object(pairs));
        assert_eq!(value, Value::Object(sections));
    }

    #[test]
    fn deserialize_ini_duplicate_keys() {
        let adaptor = IniAdaptor::new();
        let ini = b"[section]\nkey = value1\nkey = value2";

        let value = adaptor.deserialize(&ini[..]).unwrap();
        let mut pairs = Map::new();
        pairs.insert(
            "key".to_string(),
            Value::Array(vec![
                Value::String("value1".to_string()),
                Value::String("value2".to_string()),
            ]),
        );
        let mut sections = Map::new();
        sections.insert("section".to_string(), Value::Object(pairs));
        assert_eq!(value, Value::Object(sections));
    }

    #[test]
    fn deserialize_ini_no_header() {
        let adaptor = IniAdaptor::new();
        let ini = b"key1 = value1\nkey2 = value2";

        let value = adaptor.deserialize(&ini[..]).unwrap();
        let mut pairs = Map::new();
        pairs.insert("key1".to_string(), Value::String("value1".to_string()));
        pairs.insert("key2".to_string(), Value::String("value2".to_string()));
        assert_eq!(value, Value::Object(pairs));
    }

    #[test]
    fn deserialize_ini_no_header_duplicate_keys() {
        let adaptor = IniAdaptor::new();
        let ini = b"key1 = value1\nkey2 = value2";

        let value = adaptor.deserialize(&ini[..]).unwrap();
        let mut pairs = Map::new();
        pairs.insert("key1".to_string(), Value::String("value1".to_string()));
        pairs.insert("key2".to_string(), Value::String("value2".to_string()));
        assert_eq!(value, Value::Object(pairs));
    }

    #[test]
    fn deserialize_ini_nested_sections() {
        let adaptor = IniAdaptor::new();
        let ini = b"[parent.child]\nkey1 = value1\nkey2 = value2";

        let value = adaptor.deserialize(&ini[..]).unwrap();
        let mut child = Map::new();
        child.insert("key1".to_string(), Value::String("value1".to_string()));
        child.insert("key2".to_string(), Value::String("value2".to_string()));
        let mut parent = Map::new();
        parent.insert("child".to_string(), Value::Object(child));
        let mut root = Map::new();
        root.insert("parent".to_string(), Value::Object(parent));
        assert_eq!(value, Value::Object(root));
    }

    #[test]
    fn deserialize_ini_section_name_whitespace() {
        let adaptor = IniAdaptor::new();
        let ini = b"[ foo bar . baz ]\nkey1 = value1\nkey2 = value2";

        let value = adaptor.deserialize(&ini[..]).unwrap();
        let mut child = Map::new();
        child.insert("key1".to_string(), Value::String("value1".to_string()));
        child.insert("key2".to_string(), Value::String("value2".to_string()));
        let mut parent = Map::new();
        parent.insert("baz".to_string(), Value::Object(child));
        let mut root = Map::new();
        root.insert("foo bar".to_string(), Value::Object(parent));
        assert_eq!(value, Value::Object(root));
    }

    #[test]
    fn serialize_ini_section() {
        let adaptor = IniAdaptor::new();

        let mut pairs = Map::new();
        pairs.insert("key".to_string(), Value::String("value".to_string()));
        let mut section = Map::new();
        section.insert("section".to_string(), Value::Object(pairs));

        let mut buffer = Vec::new();
        adaptor
            .serialize(Value::Object(section), &mut buffer)
            .unwrap();

        let expected = &b"\n[section]\nkey = value\n"[..];

        println!("{}", str::from_utf8(&buffer[..]).unwrap());

        assert_eq!(buffer, expected);
    }

    #[test]
    fn serialize_ini_array_value() {
        let adaptor = IniAdaptor::new();

        let mut pairs = Map::new();
        pairs.insert(
            "key".to_string(),
            Value::Array(vec![
                Value::String("value1".to_string()),
                Value::String("value2".to_string()),
            ]),
        );

        let mut section = Map::new();
        section.insert("section".to_string(), Value::Object(pairs));

        let mut buffer = Vec::new();
        adaptor
            .serialize(Value::Object(section), &mut buffer)
            .unwrap();

        let expected = &b"\n[section]\nkey = value1\nkey = value2\n"[..];

        println!("{}", str::from_utf8(&buffer[..]).unwrap());

        assert_eq!(buffer, expected);
    }

    #[test]
    fn serialize_ini_no_section_then_section() {
        let adaptor = IniAdaptor::new();

        let mut pairs = Map::new();
        pairs.insert("key1".to_string(), Value::String("value1".to_string()));
        pairs.insert("key2".to_string(), Value::String("value2".to_string()));

        let mut section_pairs = Map::new();
        section_pairs.insert("key3".to_string(), Value::String("value3".to_string()));
        section_pairs.insert("key4".to_string(), Value::String("value4".to_string()));
        pairs.insert("section".to_string(), Value::Object(section_pairs));

        let mut buffer = Vec::new();
        adaptor
            .serialize(Value::Object(pairs), &mut buffer)
            .unwrap();

        let expected = &b"key1 = value1
key2 = value2

[section]
key3 = value3
key4 = value4\n"[..];

        println!("{}", str::from_utf8(&buffer[..]).unwrap());

        assert_eq!(buffer, expected);
    }

    #[test]
    fn serialize_ini_no_section() {
        let adaptor = IniAdaptor::new();

        let mut pairs = Map::new();
        pairs.insert("key1".to_string(), Value::String("value1".to_string()));
        pairs.insert("key2".to_string(), Value::String("value2".to_string()));

        let mut buffer = Vec::new();
        adaptor
            .serialize(Value::Object(pairs), &mut buffer)
            .unwrap();

        let expected = &b"key1 = value1\nkey2 = value2\n"[..];

        println!("{}", str::from_utf8(&buffer[..]).unwrap());

        assert_eq!(buffer, expected);
    }

    #[test]
    fn serialize_ini_nested_section() {
        let adaptor = IniAdaptor::new();

        let mut child = Map::new();
        child.insert("key1".to_string(), Value::String("value1".to_string()));
        child.insert("key2".to_string(), Value::String("value2".to_string()));

        let mut parent = Map::new();
        parent.insert("child".to_string(), Value::Object(child));

        let mut root = Map::new();
        root.insert("parent".to_string(), Value::Object(parent));

        let mut buffer = Vec::new();
        adaptor.serialize(Value::Object(root), &mut buffer).unwrap();

        let expected = &b"\n[parent]

[parent.child]
key1 = value1
key2 = value2\n"[..];

        println!("{}", str::from_utf8(&buffer[..]).unwrap());

        assert_eq!(buffer, expected);
    }

    fn print_output<T: Debug>(res: &IResult<&[u8], T>) {
        match *res {
            IResult::Done(ref i, ref o) => println!("i: {:?} | o: {:?}", str::from_utf8(i), o),
            _ => println!("error"),
        }
    }

    #[test]
    fn parse_key_value_pair_test() {
        let pair = &b"parameter=value"[..];

        let res = key_value_pair(pair);
        print_output(&res);
        assert_eq!(res, IResult::Done(&b""[..], ("parameter", "value")));
    }

    #[test]
    fn parse_key_value_spaced_test() {
        let pair = &b"parameter = value"[..];

        let res = key_value_pair(pair);
        print_output(&res);
        assert_eq!(res, IResult::Done(&b""[..], ("parameter", "value")));
    }

    #[test]
    fn parse_key_value_pair_dash_test() {
        let pair = &b"test-param = value"[..];

        let res = key_value_pair(pair);
        print_output(&res);
        assert_eq!(res, IResult::Done(&b""[..], ("test-param", "value")));
    }

    #[test]
    fn parse_key_value_pair_duplicate_equals_test() {
        let pair = &b"parameter==value"[..];

        let res = key_value_pair(pair);
        print_output(&res);
        assert_eq!(res, IResult::Error(ErrorKind::Not));
    }

    #[test]
    fn parse_key_value_pair_no_key_test() {
        let pair = &b"=value"[..];

        let res = key_value_pair(pair);
        print_output(&res);
        assert_eq!(res, IResult::Error(ErrorKind::Custom(0)));
    }

    #[test]
    fn parse_key_value_pair_no_value_test() {
        let pair = &b"=value"[..];

        let res = key_value_pair(pair);
        print_output(&res);
        assert_eq!(res, IResult::Error(ErrorKind::Custom(0)));
    }

    #[test]
    fn parse_key_value_pair_leading_space_test() {
        let pair = &b"   key=value"[..];

        let res = key_value_pair(pair);
        print_output(&res);
        assert_eq!(res, IResult::Error(ErrorKind::Custom(0)));
    }

    #[test]
    fn parse_key_value_pair_unbalanced_space_test() {
        let pair = &b"key= value"[..];

        let res = key_value_pair(pair);
        print_output(&res);
        assert_eq!(res, IResult::Done(&b""[..], ("key", "value")));
    }

    #[test]
    fn parse_key_value_newline_test() {
        let pair = &b"parameter=value\n"[..];

        let res = key_value_pair(pair);
        print_output(&res);
        assert_eq!(res, IResult::Done(&b"\n"[..], ("parameter", "value")));
    }

    #[test]
    fn parse_key_value_comment_test() {
        let pair = &b"parameter=value# a helpful comment"[..];

        let res = key_value_pair(pair);
        print_output(&res);
        assert_eq!(res, IResult::Done(&b""[..], ("parameter", "value")));
    }

    #[test]
    fn parse_comment_test() {
        let ini = &b"# a comment"[..];
        let res = comment(ini);
        print_output(&res);
        assert_eq!(res, IResult::Done(&b""[..], &b" a comment"[..]));
    }

    #[test]
    fn parse_multi_key_value_test() {
        let ini = &b"param1 = value1# a helpful comment\n\nparam2 = value2"[..];

        let res = key_value_group(ini);
        print_output(&res);
        let expected = vec![("param1", "value1"), ("param2", "value2")];
        assert_eq!(res, IResult::Done(&b""[..], expected));
    }

    #[test]
    fn parse_duplicate_key_value_test() {
        let ini = &b"param1 = value1\nparam1 = value2"[..];

        let res = key_value_group(ini);
        print_output(&res);
        let expected = vec![("param1", "value1"), ("param1", "value2")];
        assert_eq!(res, IResult::Done(&b""[..], expected));
    }

    #[test]
    fn parse_section_test() {
        let ini = &b"[section_name]\nparam1 = value1\nparam2 = value2"[..];

        let res = section(ini);
        print_output(&res);
        let mut expected = Vec::new();
        expected.push(("param1", "value1"));
        expected.push(("param2", "value2"));
        let sec = ("section_name", expected);
        assert_eq!(res, IResult::Done(&b""[..], sec));
    }

    #[test]
    fn parse_section_newline_test() {
        let ini = &b"[section_name]\n\nparam1 = value1\n\n\nparam2 = value2\n\n"[..];

        let res = section(ini);
        print_output(&res);
        let mut expected = Vec::new();
        expected.push(("param1", "value1"));
        expected.push(("param2", "value2"));
        let sec = ("section_name", expected);
        assert_eq!(res, IResult::Done(&b""[..], sec));
    }

    #[test]
    fn parse_section_comment_test() {
        let ini = &b"[section_name]
param1 = value1
# a helpful comment
param2 = value2"[..];

        let res = section(ini);
        print_output(&res);
        let mut expected = Vec::new();
        expected.push(("param1", "value1"));
        expected.push(("param2", "value2"));
        let sec = ("section_name", expected);
        assert_eq!(res, IResult::Done(&b""[..], sec));
    }

    #[test]
    fn parse_section_comment_after_header_test() {
        let ini = &b"[section_name]
# a helpful comment
param1 = value1
param2 = value2"[..];

        let res = section(ini);
        print_output(&res);
        let mut expected = Vec::new();
        expected.push(("param1", "value1"));
        expected.push(("param2", "value2"));
        let sec = ("section_name", expected);
        assert_eq!(res, IResult::Done(&b""[..], sec));
    }

    #[test]
    fn parse_section_comment_before_header_test() {
        let ini = &b"# a helpful comment
[section_name]
param1 = value1
param2 = value2"[..];

        let res = section(ini);
        print_output(&res);
        let mut expected = Vec::new();
        expected.push(("param1", "value1"));
        expected.push(("param2", "value2"));
        let sec = ("section_name", expected);
        assert_eq!(res, IResult::Done(&b""[..], sec));
    }

    #[test]
    fn parse_section_no_values() {
        let ini = &b"[section_name]"[..];

        let res = section(ini);
        print_output(&res);
        let expected = Vec::new();
        let sec = ("section_name", expected);
        assert_eq!(res, IResult::Done(&b""[..], sec));
    }

    #[test]
    fn parse_section_blank_lines_prefix() {
        let ini = &b"\n\n[section_name]"[..];

        let res = section(ini);
        print_output(&res);
        let expected = Vec::new();
        let sec = ("section_name", expected);
        assert_eq!(res, IResult::Done(&b""[..], sec));
    }

    #[test]
    fn parse_multi_section() {
        let ini = &b"[section1]
param1 = val1

# some documentation
[section2]
param2 = val2"[..];

        let res = ini_file(ini);
        print_output(&res);
        let mut expected = Vec::new();
        expected.push(("section1", vec![("param1", "val1")]));
        expected.push(("section2", vec![("param2", "val2")]));
        assert_eq!(res, IResult::Done(&b""[..], (vec![], expected)));
    }

    #[test]
    fn parse_section_no_header() {
        let ini = &b"param1 = val1
# some documentation
param2 = val2"[..];

        let res = ini_file(ini);
        print_output(&res);
        let expected = vec![("param1", "val1"), ("param2", "val2")];
        assert_eq!(res, IResult::Done(&b""[..], (expected, vec![])));
    }

    #[test]
    fn parse_multi_section_no_header_and_header() {
        let ini = &b"param1 = val1
# some documentation
param2 = val2

[section]
param3 = val3
param4 = val4"[..];

        let res = ini_file(ini);
        print_output(&res);
        let expected = vec![("param1", "val1"), ("param2", "val2")];

        let sections = vec![("section", vec![("param3", "val3"), ("param4", "val4")])];

        assert_eq!(res, IResult::Done(&b""[..], (expected, sections)));
    }

    #[test]
    fn parse_nested_section() {
        let ini = &b"[parent.child]
param1 = val1
param2 = val2"[..];

        let res = ini_file(ini);
        print_output(&res);
        let expected = vec![("parent.child", vec![("param1", "val1"), ("param2", "val2")])];
        assert_eq!(res, IResult::Done(&b""[..], (vec![], expected)));
    }

}
