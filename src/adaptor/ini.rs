
use std::fmt::Debug;
use std::io::{Read, Write};
use std::iter::FromIterator;
use std::str;

use super::{Adaptor, AResult, AdaptorError};

use nom::{IResult, space, alphanumeric, multispace};

use regex::Regex;

use serde_json::{Value, Map, Number};
use serde_json::map::Entry;

type Pair = (String, String);
type Section = (String, Vec<Pair>);

/// The adaptor struct for INI files
/// Later, this might contain parameters for the myriad INI quirks
pub struct IniAdaptor {
    
}

impl IniAdaptor {
    /// Constructs a new `IniAdaptor`
    pub fn new() -> IniAdaptor {
        IniAdaptor { }
    }
}

impl<'a> Adaptor<'a> for IniAdaptor {
    /// Deserialize the INI data into the `Value` AST
    fn deserialize<R>(&self, mut reader: R) -> AResult<Value> 
        where R: Read 
    {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer);

        // parse the basic INI structure
        let (no_section, sections) = match ini_file(&buffer) {
            IResult::Done(_, o) => o,
            _ => return Err("unable to parse INI data"),
        };

        let mut section_map = Map::new();

        // Here we convert the INI into our Valueuration AST,
        // performing section and key de-duplication as necessary
        for (name, pairs) in sections {
            // fetch existing entry or create a new one, deduplicating sections
            let mut entry = section_map.entry(name.to_string()).or_insert_with(|| Value::Object(Map::new()));

            let mut object = match *entry {
                Value::Object(ref mut o) => o,
                _ => return Err("section name collision"),
            };

            // later, we will need schema data in order to encode type information into the AST
            // for now, just assume everything is a string
            let converted = pairs.iter().map(|&(key,value)| (key.to_string(), infer_type(value)));
            insert_all(&mut object, converted);
        }

        let mut full_map = Map::new();

        // Insert the key-value pairs with no section header as top level properties
        let converted = no_section.iter().map(|&(key, value)| (key.to_string(), infer_type(value)));
        insert_all(&mut full_map, converted);

        // insert the sections
        for (section, pairs) in section_map {
            if let Some(_) = full_map.insert(section, pairs) {
                return Err("section name collision");
            }
        }

        // Return the combined object
        Ok(Value::Object(full_map))
    }

    /// Serialize the `Value` AST into INI format
    fn serialize<W>(&self, value: Value, mut writer: W) -> AResult<()> 
        where W: Write 
    {
        let (pairs, sections) = try!(convert_model(value));
        
        let extra_line = !pairs.is_empty();

        for (key, value) in pairs {
            writeln!(writer, "{} = {}", key, value);
        }

        if extra_line {
            writeln!(writer, "");
        }

        for (header, props) in sections {
            writeln!(writer, "[{}]", header);
            for (key, value) in props {
                writeln!(writer, "{} = {}", key, value);
            }
            writeln!(writer, "");
        }

        Ok(())
    }
}

/// Use heurisitics to determine a value's type
fn infer_type(value: &str) -> Value {
    value.parse::<u64>().map(|x| Value::Number(x.into()))
        .or_else(|_| value.parse::<i64>().map(|x| Value::Number(x.into())))
        // will fix this unwrap once we add error_chain, for now not very dangerous
        .or_else(|_| value.parse::<f64>().map(|x| Value::Number(Number::from_f64(x.into()).unwrap())))
        .or_else(|_| value.parse::<bool>().map(|x| Value::Bool(x.into())))
        .unwrap_or_else(|_| Value::String(value.into()))
}

/// Determine if a string is likely a number
fn is_number(value: &str) -> bool {
    let regex = Regex::new(r"^[+-]?\d*\.?\d+$").unwrap();
    regex.is_match(value)
}

/// Iterate through all key value pairs, and insert them into the map
fn insert_all<I>(map: &mut Map<String, Value>, values: I) 
    where I: IntoIterator<Item=(String, Value)> 
{
    for (key, value) in values.into_iter() {
        insert_or_expand(map, key, value);
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
            // we lose ownership of the Entry.
            let current = e.insert(Value::Bool(false));
            let modified = match current {
                Value::Array(mut a) => {
                    a.push(value);
                    a
                },
                x @ _ => {
                    let mut array = Vec::new();
                    array.push(x);
                    array.push(value);
                    array
                }
            };

            // add back the modified vector, droping the dummy value
            e.insert(Value::Array(modified));
        }
    }
}

/// Convert a property value into a representative `String`
/// Returns an `Error` if the value is not representable as a `String`
fn flatten_value(value: &Value) -> AResult<String> {
    let string = match *value {
        Value::Bool(ref x) => x.to_string(),
        Value::String(ref x) => x.to_string(),
        Value::Number(ref x) => x.to_string(),
        _ => return Err("invalid element"),
    };

    Ok(string)
}

/// Emits a vector of key-value pairs representing the specified
/// property and value. This will stringify primitive values and
/// convert an array into multiple key-value pairs. Object values
/// are not allowed.
fn emit_values(key: &str, value: &Value) -> AResult<Vec<Pair>> {
    let result = match (flatten_value(value), value)  {
        (Ok(x), _)                      => Ok(vec![x]),
        (_, &Value::Array(ref elems))   => {
            elems.iter()
                .map(flatten_value)
                .collect::<AResult<Vec<_>>>()
        },
        _                               => Err("invalid value"),
    };

    let tuples = try!(result).into_iter()
        .map(|x| (key.to_string(), x))
        .collect::<Vec<_>>();

    Ok(tuples)
}

/// Convert a root Valueuration object into a list of named
/// sections with key-value pairs in each. This converts
/// the internal Valueuration model to the INI data model.
fn convert_model(model: Value) -> AResult<(Vec<Pair>, Vec<Section>)> {
    // extract the root object, else error
    let top_level = match model {
        Value::Object(o) => o,
        _ => return Err("invalid root element"),
    };
    
    // filter out top-level key-value pairs and only use sections
    let section_map = top_level.iter()
        .filter_map(|(key, value)| match *value {
            Value::Object(ref o) => Some((key, o)),
            _ => None,
        });

    // convert each section, collecting the results
    let converted_map = section_map.map(|(key, pairs)| {
         // get one or more key-value pairs for each property
         // then flatten them into the section
        let flattened = pairs.iter()
            .map(|(key, value)| emit_values(key, value))
            // converts a list of results into a result with a list
            .collect::<AResult<Vec<_>>>() 
            // flatten the list of lists
            .map(|pairs| pairs.into_iter().flat_map(|x| x).collect::<Vec<_>>());

        // tuplize with section name
        flattened.map(|result| (key.to_string(), result))
    });

    let pairs = top_level.iter()
        .filter_map(|(key, value)| emit_values(key, value).ok())
        .flat_map(|x| x)
        .collect::<Vec<_>>();
    
    //convert list of results to result with a list
    let result = converted_map.collect::<AResult<Vec<_>>>();

    result.map(|sections| (pairs, sections))
}

/// Parses the section name from the `[header]`
named!(section_name<&str>, map_res!(
    delimited!(
        char!('['),
        is_not!("]"),
        char!(']')
    ),
    str::from_utf8
));

/// Parses a `# comment` value
named!(comment, delimited!(
        tag!(b"#"),
        take_while!(call!(|c| c != '\n' as u8)),
        opt!(complete!(tag!("\n")))
    )
);

/// Parses and swallows any whitespace or comments
named!(blanks, 
    map!(
        many0!(alt!(comment | multispace)),
        |_| { &b""[..] }
    )
);

/// Parses a `key = value` pair and returns a tuple
named!(key_value_pair <&[u8],(&str,&str)>,
    do_parse!(
        key: map_res!(alphanumeric, str::from_utf8)
        >> opt!(space)
        >> char!('=')
        >> opt!(space)
        >> value: map_res!(
            // There may be more elegant parsers, but this is the only one
            // I've tested that doesn't choke on EOF. Needs more investigation.
            take_while!(call!(|c| c != '\n' as u8 && c != '#' as u8)),
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
        assert_eq!(infer_type("12.34"), Value::Number(Number::from_f64(12.34).unwrap()));
    }

    #[test]
    fn is_number_integer() {
        assert_eq!(is_number("1234"), true);
    }

    #[test]
    fn is_number_decimal() {
        assert_eq!(is_number("12.34"), true);
    }

    #[test]
    fn is_number_leading_decimal() {
        assert_eq!(is_number(".1234"), true);
    }

    #[test]
    fn is_number_zero_leading_decimal() {
        assert_eq!(is_number("0.1234"), true);
    }

    #[test]
    fn is_number_leading_double_zero() {
        assert_eq!(is_number("00.1234"), true);
    }

    #[test]
    fn is_number_positive_integer() {
        assert_eq!(is_number("+1234"), true);
    }

    #[test]
    fn is_number_double_positive() {
        assert_eq!(is_number("++1234"), false);
    }  

    #[test]
    fn is_number_negative_integer() {
        assert_eq!(is_number("-1234"), true);
    }

    #[test]
    fn is_number_double_negative() {
        assert_eq!(is_number("--1234"), false);
    }

    #[test]
    fn is_number_alphanumeric() {
        assert_eq!(is_number("1a2b3c4d"), false);
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
        pairs.insert("key".to_string(), Value::Array(
            vec![
                Value::String("value1".to_string()),
                Value::String("value2".to_string()),
            ]
        ));
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
    fn serialize_ini_section() {
        let adaptor = IniAdaptor::new();

        let mut pairs = Map::new();
        pairs.insert("key".to_string(), Value::String("value".to_string()));
        let mut section = Map::new();
        section.insert("section".to_string(), Value::Object(pairs));

        let mut buffer = Vec::new();
        adaptor.serialize(Value::Object(section), &mut buffer).unwrap();

        let expected = &b"[section]\nkey = value\n\n"[..];

        assert_eq!(buffer, expected);
    }

    #[test]
    fn serialize_ini_array_value() {
        let adaptor = IniAdaptor::new();

        let mut pairs = Map::new();
        pairs.insert("key".to_string(), Value::Array(vec![
            Value::String("value1".to_string()),
            Value::String("value2".to_string()),
        ]));

        let mut section = Map::new();
        section.insert("section".to_string(), Value::Object(pairs));

        let mut buffer = Vec::new();
        adaptor.serialize(Value::Object(section), &mut buffer).unwrap();

        let expected = &b"[section]\nkey = value1\nkey = value2\n\n"[..];

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
        adaptor.serialize(Value::Object(pairs), &mut buffer).unwrap();

        let expected = &b"key1 = value1
key2 = value2

[section]
key3 = value3
key4 = value4\n\n"[..];

        assert_eq!(buffer, expected);
    }

    #[test]
    fn serialize_ini_no_section() {
        let adaptor = IniAdaptor::new();

        let mut pairs = Map::new();
        pairs.insert("key1".to_string(), Value::String("value1".to_string()));
        pairs.insert("key2".to_string(), Value::String("value2".to_string()));

        let mut buffer = Vec::new();
        adaptor.serialize(Value::Object(pairs), &mut buffer).unwrap();

        let expected = &b"key1 = value1\nkey2 = value2\n\n"[..];

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
        let mut expected = Vec::new();
        expected.push(("param1", "value1"));
        expected.push(("param2", "value2"));
        assert_eq!(res, IResult::Done(&b""[..], expected));
    }

    #[test]
    fn parse_duplicate_key_value_test() {
        let ini = &b"param1 = value1\nparam1 = value2"[..];

        let res = key_value_group(ini);
        print_output(&res);
        let mut expected = Vec::new();
        expected.push(("param1", "value1"));
        expected.push(("param1", "value2"));
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
        let expected = vec![
            ("param1", "val1"),
            ("param2", "val2")
        ];
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
        let expected = vec![
            ("param1", "val1"),
            ("param2", "val2")
        ];

        let sections = vec![
            ("section", vec![
                ("param3", "val3"),
                ("param4", "val4"),
            ])
        ];

        assert_eq!(res, IResult::Done(&b""[..], (expected, sections)));
    }

}

