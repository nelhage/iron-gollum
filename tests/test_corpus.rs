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
    let mut i = 0;
    for entry in glob("tests/testdata/typecheck/good/*.gol").expect("glob failed") {
        i = i + 1;
        let path = entry.expect("failed to glob path");
        println!("checking: {}...", path.display());
        let src = read_file(&path);
        let mut expect_path = path.clone();
        expect_path.set_extension("expect");
        let expect_src = read_file(&expect_path);

        match (parser::parse(path.to_str().unwrap(), &src), parser::parse_type(expect_path.to_str().unwrap(), &expect_src)) {
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
            (_, Err(err)) => assert!(false, format!("parse_type({}): {:?}", expect_src, err)),
        }
    }
    assert!(i > 0, "found no examples!");
}

#[test]
fn test_bad() {
    let mut i = 0;
    for entry in glob("tests/testdata/typecheck/bad/*.gol").expect("glob failed") {
         i= i +1;
        let path = entry.expect("failed to glob path");
        println!("checking: {}...", path.display());
        let src = read_file(&path);
        let ast = parser::parse(path.to_str().unwrap(), &src).expect("parse ok");
        match typecheck::typecheck(&globals::global_env(), &ast) {
            Err(_) => {
                // OK
            },
            Ok(ty) => {
                assert!(false, format!("typecheck({}) = {:?}", path.display(), ty))
            }
        }
    }
    assert!(i > 0, "found no examples!");
}
