use ast;

use std::fmt;
use std::rc::Rc;
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
pub struct Name<'a> {
    pub name: &'a str,
}

impl<'a> fmt::Debug for Name<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.name.fmt(f)
    }
}

#[derive(Debug)]
pub enum Type<'a> {
    Primitive(Name<'a>),
    //    Variable(Name<'a>),
    FunctionType(Rc<Type<'a>>, Rc<Type<'a>>),
}

pub struct TypeEnv<'a> {
    frame: HashMap<Name<'a>, Rc<Type<'a>>>,
    parent: Option<Rc<TypeEnv<'a>>>,
}

impl<'a> TypeEnv<'a> {
    pub fn empty() -> Rc<TypeEnv<'a>> {
        Rc::new(TypeEnv {
            frame: HashMap::new(),
            parent: None,
        })
    }
}

#[derive(Debug)]
pub struct TypeError<'a> {
    pub loc: ast::Loc<'a>,
    pub err: &'a str,
}
