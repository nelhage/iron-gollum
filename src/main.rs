extern crate pest;
#[macro_use]
extern crate pest_derive;

mod parser;

fn main() {
    let pairs = parser::parse("a1 b2").unwrap_or_else(|e| panic!("{}", e));

    println!("{:?}", pairs);
}
