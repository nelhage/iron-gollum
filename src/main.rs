#![allow(dead_code)]

extern crate pest;
#[macro_use]
extern crate pest_derive;

extern crate clap;

mod names;
mod parser;
mod ast;
mod types;
mod globals;
mod typecheck;

use std::fs::File;
use std::io::Read;

const VERSION: &'static str = "0.0.1";

fn main() {
    let args = clap::App::new("Iron Gollum")
        .version(VERSION)
        .author("Nelson Elhage <nelhage@nelhage.com>")
        .arg(
            clap::Arg::with_name("eval")
                .short("e")
                .value_name("CODE")
                .help("Evaluate code from command-line"),
        )
        .arg(
            clap::Arg::with_name("print-ast")
                .long("print-ast")
                .help("Print the parsed AST"),
        )
        .arg(clap::Arg::with_name("input").help("Source file").index(1))
        .get_matches();

    let path: String;
    let mut src: String = "".to_string();

    match args.value_of("input") {
        Some(arg) => {
            path = arg.to_string();
            let mut file = File::open(&path).expect("open");
            file.read_to_string(&mut src).expect("read");
        }
        None => {
            path = "-e".to_string();
            src = args.value_of("eval")
                .expect("You must specify either a path or -e")
                .to_string();
        }
    }

    let ast = parser::parse(&path, &src).unwrap_or_else(|e| panic!("parse err: {}", e));

    if args.is_present("print-ast") {
        println!("ast: {:?}", ast);
    }

    match typecheck::typecheck(&ast, globals::global_env()) {
        Ok(ty) => println!("type: {:?}", ty),
        Err(e) => println!("typecheck: err: {:?}", e),
    }
}
