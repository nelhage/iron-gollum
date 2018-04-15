use std::rc::Rc;

use ast;
use names::Name;

use types;
use types::Type;

use env::TypeEnv;

use globals;

#[derive(Debug)]
pub enum TypeError<'a> {
    Generic(ast::Loc<'a>, &'a str),
    UnboundVariable(ast::Loc<'a>, Name<'a>),
    UnboundType(ast::Loc<'a>, Name<'a>),
    BadFunction(ast::Loc<'a>, Rc<Type<'a>>),
    Mismatch(ast::Loc<'a>, Rc<Type<'a>>, Rc<Type<'a>>),
    BadDecl(ast::Loc<'a>),
    BadType(ast::Loc<'a>),
}

pub type TCResult<'a> = Result<Rc<types::Type<'a>>, TypeError<'a>>;

fn tc_apply<'a>(
    loc: &ast::Loc<'a>,
    dom: Rc<types::Type<'a>>,
    range: Rc<types::Type<'a>>,
    arg: Rc<types::Type<'a>>,
) -> TCResult<'a> {
    if arg == dom {
        Ok(range)
    } else {
        Err(TypeError::Mismatch(loc.clone(), dom, arg))
    }
}

fn ast_to_type<'a>(env: &Rc<TypeEnv<'a>>, ast: &ast::AST<'a>) -> TCResult<'a> {
    match *ast {
        ast::AST::TyName(_, ref tyvar) => {
            if let Some(ty) = env.lookup_type(tyvar) {
                Ok(ty)
            } else {
                Err(TypeError::UnboundType(ast.loc(), tyvar.clone()))
            }
        }
        ast::AST::TyFn(_, ref dom, ref range) => {
            let dom_ty = ast_to_type(&env, dom)?;
            let range_ty = ast_to_type(&env, range)?;
            Ok(Rc::new(Type::Function(dom_ty, range_ty)))
        }
        _ => Err(TypeError::BadType(ast.loc())),
    }
}

pub fn typecheck<'a>(env: &Rc<TypeEnv<'a>>, ast: &ast::AST<'a>) -> TCResult<'a> {
    match *ast {
        ast::AST::Integer(..) => Ok(globals::integer()),
        ast::AST::Boolean(..) => Ok(globals::bool()),
        ast::AST::Variable(_, ref var) => {
            if let Some(ty) = env.lookup(var) {
                Ok(ty)
            } else {
                Err(TypeError::UnboundVariable(ast.loc(), var.clone()))
            }
        }
        ast::AST::Application(_, ref func, ref arg) => {
            let func_ty = typecheck(&env, func)?;
            let arg_ty = typecheck(&env, arg)?;
            if let types::Type::Function(ref dom, ref range) = *func_ty {
                tc_apply(&ast.loc(), Rc::clone(dom), Rc::clone(range), arg_ty)
            } else {
                Err(TypeError::BadFunction(func.loc(), Rc::clone(&func_ty)))
            }
        }
        ast::AST::Abstraction(_, ref arg, ref body) => {
            if let ast::AST::Ascription(_, ref vbox, ref ty) = **arg {
                let ty = ast_to_type(&env, ty)?;

                if let ast::AST::Variable(_, ref var) = **vbox {
                    let frame = TypeEnv::with_bindings(&env, &[(var.clone(), Rc::clone(&ty))]);
                    let result_ty = typecheck(&frame, body)?;
                    return Ok(Rc::new(types::Type::Function(Rc::clone(&ty), result_ty)));
                }
            }

            Err(TypeError::BadDecl(arg.loc()))
        }
        ast::AST::If(_, ref cond, ref cons, ref alt) => {
            let cond_ty = typecheck(&env, cond)?;
            let cons_ty = typecheck(&env, cons)?;
            let alt_ty = typecheck(&env, alt)?;

            if cond_ty != globals::bool() {
                return Err(TypeError::Mismatch(ast.loc(), globals::bool(), cond_ty));
            }
            if cons_ty != alt_ty {
                return Err(TypeError::Mismatch(ast.loc(), cons_ty, alt_ty));
            }
            Ok(cons_ty)
        }
        ast::AST::Ascription(_, ref val, ref ty) => {
            let got_ty = typecheck(&env, val)?;
            let exp_ty = ast_to_type(&env, ty)?;
            if got_ty != exp_ty {
                Err(TypeError::Mismatch(ast.loc(), exp_ty, got_ty))
            } else {
                Ok(got_ty)
            }
        }
        _ => Err(TypeError::Generic(ast.loc(), "Unimplemented")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parser;
    use globals;

    #[test]
    fn test_typecheck() {
        let tests = vec![("1", "int"), ("true", "bool")];
        for (src, expect) in tests {
            let path = &format!("test: {}", src);
            match (parser::parse(path, src), parser::parse_type(path, expect)) {
                (Ok(ast), Ok(ty_ast)) => {
                    let got = typecheck(&globals::global_env(), &ast);
                    let ty = ast_to_type(&globals::global_env(), &ty_ast);
                    assert!(got.is_ok(), format!("typecheck: {:?}", got));
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

}
