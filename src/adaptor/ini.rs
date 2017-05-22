
use nom::{IResult, space, alphanumeric, multispace};

use super::{Adaptor, Config};

use std::fmt::Debug;
use std::io::{Read, Write};
use std::str;

// pub struct IniAdaptor {
    
// }

// impl Adaptor for IniAdaptor {
//     fn read<R>(reader: R) -> Result<Config> {

//     }
// }

named!(section_name<&str>, map_res!(
    delimited!(
        char!('['),
        is_not!("]"),
        char!(']')
    ),
    str::from_utf8
));

named!(comment, preceded!(char!('#'), take_until!("\n")));

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
    assert_eq!(res, IResult::Done(&b"# a helpful comment"[..], ("parameter", "value")));
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
