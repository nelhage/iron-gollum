use ast;

use pest;
use pest::Parser;

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("gollum.pest");

#[derive(Parser)]
#[grammar = "gollum.pest"]
struct Gollum;

fn parse_bool(val: &str) -> bool {
    match val {
        "true" => true,
        "false" => true,
        _ => panic!("bad bool"),
    }
}

fn parse_int(val: &str) -> i64 {
    val.parse::<i64>().unwrap()
}


fn build(path: &str, pair : pest::iterators::Pair<Rule>) -> ast::AST {
    let span = pair.clone().into_span();
    let loc = ast::Loc{
        file: path.to_string(),
        begin: span.start() as u32,
        end: span.end() as u32,
    };
    let ast = match pair.as_rule() {
        Rule::expression_body => {
            let mut inner = pair.into_inner();
            let expr = build(path, inner.next().unwrap());
            let args = inner.next();
            match args {
                None => expr,
                Some(argp) => ast::AST::Application(
                    loc, Box::new(expr), Box::new(build(path, argp)),
                ),
            }
        },
        Rule::abstraction => {
            let mut inner = pair.into_inner();
            let var = build(path, inner.next().unwrap());
            let body = build(path, inner.next().unwrap());
            ast::AST::Abstraction(loc, Box::new(var), Box::new(body))
        },
        Rule::boolean => ast::AST::Boolean(loc, parse_bool(pair.as_str())),
        Rule::variable => ast::AST::Variable(loc, pair.as_str().to_string()),
        Rule::int => ast::AST::Integer(loc, parse_int(pair.as_str())),
        _ => panic!("should not have generated a token: {:?}", pair.as_rule())
    };

    ast
}

pub fn parse<'a, 'b>(path: &'a str, input: &'b str) ->  Result<ast::AST, pest::Error<'b, Rule>> {
    let mut pairs = Gollum::parse(Rule::program, input)?;

    Ok(build(path, pairs.next().unwrap()))
}
