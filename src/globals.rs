use types::{Name, Type, TypeEnv};

use std::rc::Rc;

pub fn global_env() -> Rc<TypeEnv<'static>> {
    TypeEnv::from_types(&[
        (Name { name: "bool" }, bool()),
        (Name { name: "int" }, integer()),
    ])
}

pub fn bool() -> Rc<Type<'static>> {
    Rc::new(Type::Primitive(Name { name: "bool" }))
}

pub fn integer() -> Rc<Type<'static>> {
    Rc::new(Type::Primitive(Name { name: "int" }))
}
