extern crate iron_golem;
extern crate glob;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use iron_golem::globals;
use iron_golem::typecheck;
use iron_golem::parser;

use glob::glob;

fn read_file(path: &Path) -> String {
    let mut f = File::open(path).expect("open to succeed");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("read to succeed");
    contents
}

#[test]
fn test_typecheck() {
    for entry in glob("test/testdata/type/*.gol").expect("glob failed") {
        let path = entry.expect("failed to glob path");
        println!("checking: {}...", path.display());
    }
    let tests = vec![
            ("1", "int"),
            ("true", "bool"),
            ("add(1, 1)", "int"),
            ("fn(x : int, y : int) { add(x,y) }", "int -> int -> int"),
            (
                "fn(fact: int -> int, x: int) {
                   if iszero(x) {
                     1
                   } else {
                     mul(x, fact(dec(x)))
                   }
                 }",
                "(int -> int) -> int -> int",
            ),
            (
                "fn(Y: ((int -> int) -> int -> int) -> int -> int, f: (int -> int) -> int -> int) {
                   Y(f)
                 }"
                ,"(((int -> int) -> int -> int) -> int -> int) -> ((int->int) -> int -> int) -> int -> int",
            )
        ];
    for (src, expect) in tests {
        let path = &format!("test: {}", src);
        match (parser::parse(path, src), parser::parse_type(path, expect)) {
            (Ok(ast), Ok(ty_ast)) => {
                println!("test: {}", src);
                let got = typecheck::typecheck(&globals::global_env(), &ast);
                let ty = typecheck::ast_to_type(&globals::global_env(), &ty_ast);
                assert!(got.is_ok(), format!("typecheck({}): {:?}", src, got));
                assert!(ty.is_ok(), format!("expect: {:?}", ty));
                assert!(
                    got.as_ref().unwrap() == ty.as_ref().unwrap(),
                    format!("tc({}) = {:?} != {:?}", src, got, ty)
                );
            }
            (Err(err), _) => assert!(false, format!("parse({}): {:?}", src, err)),
            (_, Err(err)) => assert!(false, format!("parse_type({}): {:?}", expect, err)),
        }
    }
}

#[test]
fn test_bad() {}
