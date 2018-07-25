// Rust 2018 edition opt-in
#![feature(rust_2018_preview, use_extern_macros)]
#![warn(rust_2018_compatibility)]
#![warn(rust_2018_idioms)]

use std::env;
use std::fs;

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
}
