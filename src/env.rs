use names::Name;
use types::Type;

use std::rc::Rc;
use std::collections::HashMap;

#[derive(Debug)]
pub struct TypeEnv<'a> {
    binds: HashMap<Name<'a>, Rc<Type<'a>>>,
    parent: Option<Rc<TypeEnv<'a>>>,
}

impl<'a> TypeEnv<'a> {
    pub fn empty() -> Rc<TypeEnv<'a>> {
        Rc::new(TypeEnv {
            binds: HashMap::new(),
            parent: None,
        })
    }

    pub fn from_bindings<'b>(binds: &'b [(Name<'a>, Rc<Type<'a>>)]) -> Rc<TypeEnv<'a>> {
        let mut env = Rc::try_unwrap(TypeEnv::empty()).unwrap();
        for bind in binds {
            env.binds.insert(bind.0.clone(), Rc::clone(&bind.1));
        }
        Rc::new(env)
    }

    pub fn lookup(&self, var: &Name) -> Option<Rc<Type<'a>>> {
        if let ok @ Some(_) = self.binds.get(var) {
            ok.cloned()
        } else if let Some(ref env) = self.parent {
            env.lookup(var)
        } else {
            None
        }
    }

    pub fn with_bindings<'b>(
        parent: &Rc<TypeEnv<'a>>,
        binds: &'b [(Name<'a>, Rc<Type<'a>>)],
    ) -> Rc<TypeEnv<'a>> {
        let mut env = TypeEnv {
            binds: HashMap::new(),
            parent: Some(Rc::clone(parent)),
        };
        for bind in binds {
            env.binds.insert(bind.0.clone(), Rc::clone(&bind.1));
        }
        Rc::new(env)
    }
}
