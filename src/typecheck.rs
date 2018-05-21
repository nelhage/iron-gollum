use std::rc::Rc;
use std::collections::HashMap;

use ast;
use names;
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
    Occur(ast::Loc<'a>, Rc<Type<'a>>, Rc<Type<'a>>),
}

pub type TCResult<'a> = Result<Rc<types::Type<'a>>, TypeError<'a>>;

struct Typecheck<'a> {
    uniq: i32,
    soln: HashMap<Name<'a>, Rc<Type<'a>>>,
}

impl<'a, 'b> Typecheck<'a> {
    fn new() -> Typecheck<'a> {
        Typecheck {
            uniq: 0,
            soln: HashMap::new(),
        }
    }

    fn gensym(&'b mut self, base: Name<'a>) -> Name<'a> {
        self.uniq += 1;
        Name::Unique(Box::new(base.clone()), self.uniq)
    }

    fn genvar(&'b mut self, base: Name<'a>) -> Rc<types::Type<'a>> {
        Rc::new(types::Type::Variable(self.gensym(base)))
    }

    pub fn subst_type(&'b mut self, ty: &Rc<types::Type<'a>>) -> Rc<types::Type<'a>> {
        types::map_vars(ty, &mut |var| {
            println!("map?({:?})", var);

            match *var {
                types::Type::Variable(ref name) => match self.soln.get(name).cloned() {
                    Some(mapped) => {
                        let inner = self.subst_type(&mapped);
                        println!(" = {:?} = {:?}", mapped, &inner);
                        self.soln.insert(name.clone(), Rc::clone(&inner));
                        inner
                    }
                    None => Rc::clone(&var),
                },
                _ => unreachable!(),
            }
        })
    }

    fn occur(&'b self, _var: &Name<'a>, _ty: &Rc<types::Type<'a>>) -> bool {
        return false;
    }

    fn add_soln(&'b mut self, var: &Name<'a>, ty: &Rc<types::Type<'a>>) {
        println!("solve: {:?} = {:?}", var, ty);
        self.soln.insert(var.clone(), Rc::clone(ty));
    }

    fn unify(
        &'b mut self,
        node: &ast::AST<'a>,
        left: &Rc<types::Type<'a>>,
        right: &Rc<types::Type<'a>>,
    ) -> TCResult<'a> {
        println!("unify: {:?} =?= {:?}", left, right);

        let left = self.subst_type(left);
        let right = self.subst_type(right);

        println!("substituted: {:?} =?= {:?}", left, right);

        if Rc::ptr_eq(&left, &right) {
            return Ok(left);
        }

        match (&*left, &*right) {
            (&types::Type::Variable(ref lv), _) => {
                if self.occur(lv, &right) {
                    return Err(TypeError::Occur(
                        node.loc(),
                        Rc::clone(&left),
                        Rc::clone(&right),
                    ));
                }
                self.add_soln(lv, &right);
            }
            (_, &types::Type::Variable(ref rv)) => {
                if self.occur(rv, &left) {
                    return Err(TypeError::Occur(
                        node.loc(),
                        Rc::clone(&right),
                        Rc::clone(&left),
                    ));
                }
                self.add_soln(rv, &left);
            }
            (
                &types::Type::Function(ref ldom, ref lrange),
                &types::Type::Function(ref rdom, ref rrange),
            ) => {
                self.unify(node, ldom, rdom)?;
                self.unify(node, lrange, rrange)?;
            }
            (&types::Type::Primitive(ref lt), &types::Type::Primitive(ref rt)) if lt == rt => {}
            (_, _) => {
                return Err(TypeError::Mismatch(
                    node.loc(),
                    Rc::clone(&left),
                    Rc::clone(&right),
                ));
            }
        }

        Ok(self.subst_type(&left))
    }

    fn ast_to_type(&mut self, env: &Rc<TypeEnv<'a>>, ast: &ast::AST<'a>) -> TCResult<'a> {
        match *ast {
            ast::AST::TyName(_, ref tyvar) => {
                if let Some(ty) = env.lookup(tyvar) {
                    Ok(ty)
                } else {
                    Err(TypeError::UnboundType(ast.loc(), tyvar.clone()))
                }
            }
            ast::AST::TyFn(_, ref dom, ref range) => {
                let dom_ty = self.ast_to_type(&env, dom)?;
                let range_ty = self.ast_to_type(&env, range)?;
                Ok(Rc::new(Type::Function(dom_ty, range_ty)))
            }
            _ => Err(TypeError::BadType(ast.loc())),
        }
    }

    fn typecheck(&mut self, env: &Rc<TypeEnv<'a>>, ast: &ast::AST<'a>) -> TCResult<'a> {
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
                let func_ty = self.typecheck(&env, func)?;
                let arg_ty = self.typecheck(&env, arg)?;
                let rng = self.genvar(names::typ("rv"));
                self.unify(
                    ast,
                    &Rc::new(types::Type::Function(arg_ty, Rc::clone(&rng))),
                    &func_ty,
                )?;
                Ok(rng)
            }
            ast::AST::Abstraction(_, ref arg, ref body) => {
                if let ast::AST::Ascription(_, ref vbox, ref ty) = **arg {
                    let ty = self.ast_to_type(&env, ty)?;

                    if let ast::AST::Variable(_, ref var) = **vbox {
                        let frame = TypeEnv::with_bindings(&env, &[(var.clone(), Rc::clone(&ty))]);
                        let result_ty = self.typecheck(&frame, body)?;
                        return Ok(Rc::new(types::Type::Function(Rc::clone(&ty), result_ty)));
                    }
                }

                Err(TypeError::BadDecl(arg.loc()))
            }
            ast::AST::If(_, ref cond, ref cons, ref alt) => {
                let cond_ty = self.typecheck(&env, cond)?;
                let cons_ty = self.typecheck(&env, cons)?;
                let alt_ty = self.typecheck(&env, alt)?;

                self.unify(cond, &cond_ty, &globals::bool())?;
                self.unify(ast, &cons_ty, &alt_ty)?;

                Ok(cons_ty)
            }
            ast::AST::Ascription(_, ref val, ref ty) => {
                let got_ty = self.typecheck(&env, val)?;
                let exp_ty = self.ast_to_type(&env, ty)?;
                self.unify(ast, &got_ty, &exp_ty)?;
                Ok(got_ty)
            }
            _ => Err(TypeError::Generic(ast.loc(), "Unimplemented")),
        }
    }
}

pub fn typecheck<'a>(env: &Rc<TypeEnv<'a>>, ast: &ast::AST<'a>) -> TCResult<'a> {
    let mut tc = Typecheck::new();
    let ty = tc.typecheck(env, ast)?;
    Ok(tc.subst_type(&ty))
}

pub fn ast_to_type<'a>(env: &Rc<TypeEnv<'a>>, ast: &ast::AST<'a>) -> TCResult<'a> {
    Typecheck::new().ast_to_type(env, ast)
}
