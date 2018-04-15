use names::Name;

use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum Type<'a> {
    Primitive(Name<'a>),
    //    Variable(Name<'a>),
    Function(Rc<Type<'a>>, Rc<Type<'a>>),
}
