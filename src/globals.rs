use names;
use types::Type;
use env::TypeEnv;

use std::rc::Rc;

pub fn global_env() -> Rc<TypeEnv<'static>> {
    let unary_int = Rc::new(Type::Function(integer(), integer()));
    let binary_int = Rc::new(Type::Function(integer(), Rc::clone(&unary_int)));
    TypeEnv::with_bindings(
        &TypeEnv::from_types(&[
            (names::ident("bool"), bool()),
            (names::ident("int"), integer()),
        ]),
        &[
            (names::ident("add"), Rc::clone(&binary_int)),
            (names::ident("sub"), Rc::clone(&binary_int)),
            (names::ident("mul"), Rc::clone(&binary_int)),
            (names::ident("dec"), Rc::clone(&unary_int)),
            (
                names::ident("iszero"),
                Rc::new(Type::Function(integer(), bool())),
            ),
            (names::ident("not"), Rc::new(Type::Function(bool(), bool()))),
        ],
    )
}

pub fn bool() -> Rc<Type<'static>> {
    Rc::new(Type::Primitive(names::ident("bool")))
}

pub fn integer() -> Rc<Type<'static>> {
    Rc::new(Type::Primitive(names::ident("int")))
}
