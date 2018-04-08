extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("gollum.pest");

#[derive(Parser)]
#[grammar = "gollum.pest"]
pub struct Gollum;

// mod parser;

fn main() {
    let pairs = Gollum::parse(Rule::program, "a1 b2").unwrap_or_else(|e| panic!("{}", e));

    println!("{:?}", pairs);
}
