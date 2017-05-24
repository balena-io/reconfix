
use nom::{IResult, space, alphanumeric, multispace};

use super::{Adaptor, Config, AdaptorError};

use std::collections::{HashMap, hash_map};
use std::fmt::Debug;
use std::io::{Read, Write};
use std::iter::FromIterator;
use std::str;

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
    /// Deserialize the INI data into the `Config` AST
    fn deserialize<R>(&self, mut reader: R) -> Result<Config, AdaptorError> 
        where R: Read 
    {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer);

        // parse the basic INI structure
        let output = match sections(&buffer) {
            IResult::Done(_, o) => o,
            _ => return Err("unable to parse INI data"),
        };

        let mut combined = HashMap::new();

        // Here we convert the INI into our configuration AST,
        // performing section and key de-duplication as necessary
        for (name, pairs) in output {
            // fetch existing entry or create a new one, deduplicating sections
            let mut entry = combined.entry(name.into()).or_insert_with(|| HashMap::new());
            // later, we will need schema data in order to encode type information into the AST
            // for now, just assume everything is a string
            let converted = pairs.iter().map(|&(key,value)| (key.to_string(), Config::Text(value.to_string())));
            insert_all(entry, converted);
        }

        // wrap it all up in an object
        let objects = combined.into_iter().map(|(key, value)| (key, Config::Object(value))).collect();
        Ok(Config::Object(objects))
    }

    /// Serialize the `Config` AST into INI format
    fn serialize<W>(&self, config: Config, mut writer: W) -> Result<(), AdaptorError> 
        where W: Write 
    {
        let ini_model = try!(convert_model(config));

        for (header, props) in ini_model {
            writeln!(writer, "[{}]", header);
            for (key, value) in props {
                writeln!(writer, "{} = {}", key, value);
            }
            writeln!(writer, "");
        }

        Ok(())
    }
}

#[test]
fn deserialize_ini_section() {
    let adaptor = IniAdaptor::new();
    let mut ini = b"[section]
key = value";

    let config = adaptor.deserialize(&ini[..]).unwrap();
    let mut pairs = HashMap::new();
    pairs.insert("key".to_string(), Config::Text("value".to_string()));
    let mut sections = HashMap::new();
    sections.insert("section".to_string(), Config::Object(pairs));
    assert_eq!(config, Config::Object(sections));

}

#[test]
fn deserialize_ini_duplicate_keys() {
    let adaptor = IniAdaptor::new();
    let mut ini = b"[section]
key = value1
key = value2";

    let config = adaptor.deserialize(&ini[..]).unwrap();
    let mut pairs = HashMap::new();
    pairs.insert("key".to_string(), Config::Array(
        vec![
            Config::Text("value1".to_string()),
            Config::Text("value2".to_string()),
        ]
    ));
    let mut sections = HashMap::new();
    sections.insert("section".to_string(), Config::Object(pairs));
    assert_eq!(config, Config::Object(sections));

}

#[test]
fn serialize_ini_section() {
    let adaptor = IniAdaptor::new();

    let mut pairs = HashMap::new();
    pairs.insert("key".to_string(), Config::Text("value".to_string()));
    let mut section = HashMap::new();
    section.insert("section".to_string(), Config::Object(pairs));

    let mut buffer = Vec::new();
    adaptor.serialize(Config::Object(section), &mut buffer);

    let expected = &b"[section]\nkey = value\n\n"[..];

    assert_eq!(buffer, expected);
}

#[test]
fn serialize_ini_array_value() {
    let adaptor = IniAdaptor::new();

    let mut pairs = HashMap::new();
    pairs.insert("key".to_string(), Config::Array(vec![
        Config::Text("value1".to_string()),
        Config::Text("value2".to_string()),
    ]));

    let mut section = HashMap::new();
    section.insert("section".to_string(), Config::Object(pairs));

    let mut buffer = Vec::new();
    adaptor.serialize(Config::Object(section), &mut buffer);

    let expected = &b"[section]
key = value1
key = value2\n\n"[..];

    assert_eq!(buffer, expected);
}

/// Iterate through all key value pairs, and insert them into the map
fn insert_all<I>(map: &mut HashMap<String, Config>, values: I) 
    where I: IntoIterator<Item=(String, Config)> 
{
    for (key, value) in values.into_iter() {
        insert_or_expand(map, key, value);
    }
}

/// Insert a new value or create an array if there are duplicates
fn insert_or_expand(map: &mut HashMap<String, Config>, key: String, value: Config) {
    match map.entry(key) {
        hash_map::Entry::Vacant(e) => {
            e.insert(value);
        },
        hash_map::Entry::Occupied(mut e) => {
            // we use a dummy value here so we can replace it with
            // the modified value later. If we remove the value,
            // we lose ownership of the Entry.
            let mut current = e.insert(Config::Bool(false));
            let modified = match current {
                Config::Array(mut a) => {
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
            e.insert(Config::Array(modified));
        }
    }
}

/// Convert a property value into a representative `String`
/// Returns an `Error` if the value is not representable as a `String`
fn flatten_value(value: &Config) -> Result<String, AdaptorError> {
    let string = match *value {
        Config::Bool(ref x) => x.to_string(),
        Config::Text(ref x) => x.to_string(),
        Config::Number(ref x) => x.to_string(),
        _ => return Err("invalid element"),
    };

    Ok(string)
}

/// Emits a vector of key-value pairs representing the specified
/// property and value. This will stringify primitive values and
/// convert an array into multiple key-value pairs. Object values
/// are not allowed.
fn emit_values(key: &str, value: &Config) -> Result<Vec<(String, String)>, AdaptorError> {
    let result = match (flatten_value(value), value)  {
        (Ok(x), _)                      => Ok(vec![x]),
        (_, &Config::Array(ref elems))   => {
            elems.iter()
                .map(flatten_value)
                .collect::<Result<Vec<_>, AdaptorError>>()
        },
        _                               => Err("invalid value"),
    };

    let tuples = try!(result).into_iter()
        .map(|x| (key.to_string(), x))
        .collect::<Vec<_>>();

    Ok(tuples)
}

/// Convert a root configuration object into a list of named
/// sections with key-value pairs in each. This converts
/// the internal configuration model to the INI data model.
fn convert_model(model: Config) -> Result<Vec<(String, Vec<(String, String)>)>, AdaptorError> {
    // extract the root object, else error
    let section_map = match model {
        Config::Object(o) => o,
        _ => return Err("invalid root element"),
    };

    // convert each section, collecting the results
    let converted_map = section_map.iter().map(|(key, config)| {
        // extract the section object
        let pairs = match *config {
            Config::Object(ref o) => o,
            _ => return Err("invalid section element"),
        };

         // get one or more key-value pairs for each property
         // then flatten them into the section
        let flattened = pairs.iter()
            .map(|(key, value)| emit_values(key, value))
            // converts a list of results into a result with a list
            .collect::<Result<Vec<_>, AdaptorError>>() 
            // flatten the list of lists
            .map(|pairs| pairs.into_iter().flat_map(|x| x).collect::<Vec<_>>());

        // tuplize with section name
        flattened.map(|result| (key.to_string(), result))
    });
    
    //convert list of results to result with a list
    converted_map.collect::<Result<Vec<_>, _>>()
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

/// Parses multiple sections
named!(sections<&[u8], Vec<(&str, Vec<(&str, &str)>)>>, 
    many0!(section)
);

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

    let res = sections(ini);
    print_output(&res);
    let mut expected = Vec::new();
    expected.push(("section1", vec![("param1", "val1")]));
    expected.push(("section2", vec![("param2", "val2")]));
    assert_eq!(res, IResult::Done(&b""[..], expected));
}
