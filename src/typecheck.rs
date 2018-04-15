use std::rc::Rc;

use ast;
use ast::Name;

use types;
use types::Type;

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

fn ast_to_type<'a>(ast: &ast::AST<'a>, env: Rc<types::TypeEnv<'a>>) -> TCResult<'a> {
    match *ast {
        ast::AST::TyName(_, ref tyvar) => {
            if let Some(ty) = env.lookup_type(tyvar) {
                Ok(ty)
            } else {
                Err(TypeError::UnboundType(ast.loc(), tyvar.clone()))
            }
        }
        ast::AST::TyFn(_, ref dom, ref range) => {
            let dom_ty = ast_to_type(dom, Rc::clone(&env))?;
            let range_ty = ast_to_type(range, Rc::clone(&env))?;
            Ok(Rc::new(Type::Function(dom_ty, range_ty)))
        }
        _ => Err(TypeError::BadType(ast.loc())),
    }
}

pub fn typecheck<'a>(ast: &ast::AST<'a>, env: Rc<types::TypeEnv<'a>>) -> TCResult<'a> {
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
            let func_ty = typecheck(func, Rc::clone(&env))?;
            let arg_ty = typecheck(arg, Rc::clone(&env))?;
            if let types::Type::Function(ref dom, ref range) = *func_ty {
                tc_apply(&ast.loc(), Rc::clone(dom), Rc::clone(range), arg_ty)
            } else {
                Err(TypeError::BadFunction(func.loc(), Rc::clone(&func_ty)))
            }
        }
        ast::AST::Abstraction(_, ref arg, ref body) => {
            if let ast::AST::Ascription(_, ref vbox, ref ty) = **arg {
                let ty = ast_to_type(ty, Rc::clone(&env))?;

                if let ast::AST::Variable(_, ref var) = **vbox {
                    let frame =
                        types::TypeEnv::with_bindings(&env, &[(var.clone(), Rc::clone(&ty))]);
                    let result_ty = typecheck(body, frame)?;
                    return Ok(Rc::new(types::Type::Function(Rc::clone(&ty), result_ty)));
                }
            }

            Err(TypeError::BadDecl(arg.loc()))
        }
        _ => Err(TypeError::Generic(ast.loc(), "Unimplemented")),
    }
}
