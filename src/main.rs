extern crate pest;
#[macro_use]
extern crate pest_derive;

mod parser;
mod ast;

fn main() {
    let mut args = std::env::args();
    args.next();

    let arg = args.next().unwrap_or("a1".to_string());
    let ast = parser::parse("-e", &arg).unwrap_or_else(|e| panic!("parse err: {}", e));

    println!("{:?}", ast);
}
