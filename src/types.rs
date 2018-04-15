use names::Name;

use std::rc::Rc;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Type<'a> {
    Primitive(Name<'a>),
    //    Variable(Name<'a>),
    Function(Rc<Type<'a>>, Rc<Type<'a>>),
}

#[derive(Debug)]
pub struct TypeEnv<'a> {
    vars: HashMap<Name<'a>, Rc<Type<'a>>>,
    types: HashMap<Name<'a>, Rc<Type<'a>>>,
    parent: Option<Rc<TypeEnv<'a>>>,
}

impl<'a> TypeEnv<'a> {
    pub fn empty() -> Rc<TypeEnv<'a>> {
        Rc::new(TypeEnv {
            vars: HashMap::new(),
            types: HashMap::new(),
            parent: None,
        })
    }

    pub fn from_types<'b>(binds: &'b [(Name<'a>, Rc<Type<'a>>)]) -> Rc<TypeEnv<'a>> {
        let mut env = TypeEnv {
            vars: HashMap::new(),
            types: HashMap::new(),
            parent: None,
        };
        for bind in binds {
            env.types.insert(bind.0.clone(), Rc::clone(&bind.1));
        }
        Rc::new(env)
    }

    pub fn lookup(&self, var: &Name) -> Option<Rc<Type<'a>>> {
        if let ok @ Some(_) = self.vars.get(var) {
            ok.cloned()
        } else if let Some(ref env) = self.parent {
            env.lookup(var)
        } else {
            None
        }
    }

    pub fn lookup_type(&self, var: &Name) -> Option<Rc<Type<'a>>> {
        if let ok @ Some(_) = self.types.get(var) {
            ok.cloned()
        } else if let Some(ref env) = self.parent {
            env.lookup_type(var)
        } else {
            None
        }
    }

    pub fn with_bindings<'b>(
        parent: &Rc<TypeEnv<'a>>,
        binds: &'b [(Name<'a>, Rc<Type<'a>>)],
    ) -> Rc<TypeEnv<'a>> {
        let mut env = TypeEnv {
            vars: HashMap::new(),
            types: HashMap::new(),
            parent: Some(Rc::clone(parent)),
        };
        for bind in binds {
            env.vars.insert(bind.0.clone(), Rc::clone(&bind.1));
        }
        Rc::new(env)
    }
}
