use names;
use types::{Type, TypeEnv};

use std::rc::Rc;

pub fn global_env() -> Rc<TypeEnv<'static>> {
    TypeEnv::from_types(&[
        (names::ident("bool"), bool()),
        (names::ident("int"), integer()),
    ])
}

pub fn bool() -> Rc<Type<'static>> {
    Rc::new(Type::Primitive(names::ident("bool")))
}

pub fn integer() -> Rc<Type<'static>> {
    Rc::new(Type::Primitive(names::ident("int")))
}
