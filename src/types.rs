use names::Name;

use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum Type<'a> {
    Primitive(Name<'a>),
    Variable(Name<'a>),
    // ForAll(Vec<Rc<Type<'a>>>, Rc<Type<'a>>),
    Function(Rc<Type<'a>>, Rc<Type<'a>>),
}

pub fn map_vars<'a, F>(ty: &Rc<Type<'a>>, map: &mut F) -> Rc<Type<'a>>
where
    F: FnMut(Rc<Type<'a>>) -> Rc<Type<'a>>,
{
    match &**ty {
        &Type::Primitive(_) => Rc::clone(ty),
        &Type::Variable(_) => map(Rc::clone(ty)),
        &Type::Function(ref dom, ref range) => {
            Rc::new(Type::Function(map_vars(&dom, map), map_vars(&range, map)))
        }
    }
}
