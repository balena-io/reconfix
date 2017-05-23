
use nom::{IResult, space, alphanumeric, multispace};

use super::{Adaptor, Config};

use std::collections::{HashMap, hash_map};
use std::fmt::Debug;
use std::io::{Read, Write};
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
    fn deserialize<R>(&self, mut reader: R) -> Result<Config, String> 
        where R: Read 
    {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer);

        // parse the basic INI structure
        let (_, output) = sections(&buffer).unwrap();

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
    fn serialize<W>(&self, config: Config, writer: W) -> Result<(), String> {
        unimplemented!();
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

named!(section_name<&str>, map_res!(
    delimited!(
        char!('['),
        is_not!("]"),
        char!(']')
    ),
    str::from_utf8
));

named!(comment, delimited!(
        tag!(b"#"),
        take_while!(call!(|c| c != '\n' as u8)),
        opt!(complete!(tag!("\n")))
    )
);

named!(blanks, 
    map!(
        many0!(alt!(comment | multispace)),
        |_| { &b""[..] }
    )
);

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

named!(key_value_group<&[u8], Vec<(&str, &str)>>,
    many0!(terminated!(key_value_pair, opt!(complete!(blanks))))
);

named!(section<&[u8], (&str, Vec<(&str, &str)>)>,
    do_parse!(
        opt!(complete!(blanks))
        >> section: section_name
        >> opt!(complete!(blanks))
        >> pairs: key_value_group
        >> (section, pairs) 
    )
);

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
