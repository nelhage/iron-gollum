use pest;
use pest::iterators;
use pest::Parser;

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("gollum.pest");

#[derive(Parser)]
#[grammar = "gollum.pest"]
struct Gollum;

pub fn parse(input: &str) ->  Result<iterators::Pairs<Rule>, pest::Error<Rule>> {
    Gollum::parse(Rule::program, input)
}
