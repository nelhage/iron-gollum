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

fn pair_loc(path: &str, pair: &pest::iterators::Pair<Rule>) -> ast::Loc {
    let span = pair.clone().into_span();
    ast::Loc {
        file: path.to_string(),
        begin: span.start() as u32,
        end: span.end() as u32,
    }
}

fn build_vec(path: &str, pair: pest::iterators::Pair<Rule>) -> Vec<Box<ast::AST>> {
    let pairs = pair.into_inner();
    pairs.map(|pair| build(path, pair)).collect()
}

fn build_type(path: &str, pair: pest::iterators::Pair<Rule>) -> Box<ast::AST> {
    let loc = pair_loc(path, &pair);
    let ast = match pair.as_rule() {
        Rule::typ => {
            let mut inner = pair.into_inner();
            let mut stk: Vec<Box<ast::AST>> = inner.map(|pair| build_type(path, pair)).collect();
            let ty = stk.pop().unwrap();
            stk.reverse();
            stk.into_iter().fold(ty, |rv, arg| {
                let floc = ast::Loc {
                    file: path.to_string(),
                    begin: arg.loc().begin,
                    end: loc.end,
                };
                Box::new(ast::AST::TyFn(floc, arg, rv))
            })
        }
        Rule::variable => Box::new(ast::AST::TyName(loc, pair.as_str().to_string())),
        _ => panic!("should not have generated a token: {:?}", pair.as_rule()),
    };
    ast
}

fn build(path: &str, pair: pest::iterators::Pair<Rule>) -> Box<ast::AST> {
    let loc = pair_loc(path, &pair);
    let ast = match pair.as_rule() {
        Rule::typ => *build_type(path, pair),
        Rule::expression_body => {
            let mut inner = pair.into_inner();
            let mut expr = *build(path, inner.next().unwrap());
            let mut args = inner.next();
            if let Some(argv) = args.clone() {
                if argv.as_rule() == Rule::func_args {
                    expr = ast::AST::Application(
                        loc.clone(),
                        Box::new(expr),
                        build_vec(path, argv.into_inner().next().unwrap()),
                    );
                    args = inner.next();
                }
            }
            if let Some(ascribe) = args {
                if ascribe.as_rule() != Rule::ascription {
                    panic!("unexpected: {:?}", ascribe)
                }
                expr = ast::AST::Ascription(
                    loc,
                    Box::new(expr),
                    build(path, ascribe.into_inner().next().unwrap()),
                )
            }

            expr
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
            "x : int",
            "1 ",
            "x : x -> y -> z",
            "0 : x -> y",
            "f(0) : int -> int",
        ];
        for test in tests {
            let res = parse(&format!("test: {}", test), test);
            assert!(res.is_ok(), "parse({}): {:?}", test, res)
        }
    }

    #[test]
    fn test_bad() {
        let tests = vec!["-", "1.0", "'hi'", "\"hi\"", "(x", "fn(x) y"];
        for test in tests {
            let res = parse(&format!("test: {}", test), test);
            assert!(!res.is_ok(), "parse({}): {:?}", test, res)
        }
    }

}
