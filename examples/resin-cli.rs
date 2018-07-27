// Rust 2018 edition opt-in
#![feature(rust_2018_preview, use_extern_macros)]
#![warn(rust_2018_compatibility)]
#![warn(rust_2018_idioms)]

use std::env;
use std::fs;

use clap::{App, Arg};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let command = args.get(1).expect("command required");
    let mut reconfix = ::reconfix::Reconfix::new();
    match command.as_ref() {
        "test" => {
            let path = args.get(2).expect("file arg required");
            let file = fs::File::open(path).expect("file not found");
            reconfix.load_schema(file).expect("unable to load schema");
        },
        _ => unimplemented!(),
    }
    let matches = App::new("resin-cli")
        .arg(
            Arg::with_name("SCHEMA")
                .required(true)
                .takes_value(true)
                .short("s")
                .long("schema")
                .help("Resin JSON schema path"),
        )
        .get_matches();

    let schema_path = matches.value_of("SCHEMA").unwrap();
    let schema_file = fs::File::open(schema_path).expect("Schema file not found");

    let mut rfx = reconfix::Reconfix::new();

    println!("{:?}", rfx.load_schema(schema_file));
}
