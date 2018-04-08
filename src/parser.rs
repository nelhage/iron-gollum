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

fn build_vec(path: &str, pair: pest::iterators::Pair<Rule>) -> Vec<Box<ast::AST>> {
    let pairs = pair.into_inner();
    pairs.map(|pair| build(path, pair)).collect()
}

fn build(path: &str, pair: pest::iterators::Pair<Rule>) -> Box<ast::AST> {
    let span = pair.clone().into_span();
    let loc = ast::Loc {
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
                None => *expr,
                Some(argv) => ast::AST::Application(loc, expr, build_vec(path, argv)),
            }
        }
        Rule::abstraction => {
            let mut inner = pair.into_inner();
            let vars = build_vec(path, inner.next().unwrap());
            let body = build(path, inner.next().unwrap());
            ast::AST::Abstraction(loc, vars, body)
        }
        Rule::boolean => ast::AST::Boolean(loc, parse_bool(pair.as_str())),
        Rule::variable => ast::AST::Variable(loc, pair.as_str().to_string()),
        Rule::int => ast::AST::Integer(loc, parse_int(pair.as_str())),
        _ => panic!("should not have generated a token: {:?}", pair.as_rule()),
    };

    Box::new(ast)
}

pub fn parse<'a, 'b>(
    path: &'a str,
    input: &'b str,
) -> Result<Box<ast::AST>, pest::Error<'b, Rule>> {
    let mut pairs = Gollum::parse(Rule::program, input)?;

    Ok(build(path, pairs.next().unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let tests = vec![
            "1",
            "false",
            "hello",
            "f(x)",
            "fn(x) { y }",
            "(x)",
            "f(x, y, z)",
            "fn(x, y) { z }",
        ];
        for test in tests {
            let res = parse(&format!("test: {}", test), test);
            assert!(res.is_ok(), "parse({}): {:?}", test, res)
        }
    }
}
