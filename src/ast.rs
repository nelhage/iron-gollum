use names::Name;

#[derive(Clone, Debug)]
pub struct Loc<'a> {
    pub file: &'a str,
    pub begin: u32,
    pub end: u32,
}

#[derive(Clone, Debug)]
pub enum AST<'a> {
    Variable(Loc<'a>, Name<'a>),
    Integer(Loc<'a>, i64),
    Boolean(Loc<'a>, bool),

    Application(Loc<'a>, Box<AST<'a>>, Box<AST<'a>>),
    Abstraction(Loc<'a>, Box<AST<'a>>, Box<AST<'a>>),
    Ascription(Loc<'a>, Box<AST<'a>>, Box<AST<'a>>),

    If(Loc<'a>, Box<AST<'a>>, Box<AST<'a>>, Box<AST<'a>>),

    TyName(Loc<'a>, Name<'a>),
    TyFn(Loc<'a>, Box<AST<'a>>, Box<AST<'a>>),
}

impl<'a> AST<'a> {
    pub fn loc(&self) -> Loc<'a> {
        match *self {
            AST::Variable(ref loc, _) => loc,
            AST::Integer(ref loc, _) => loc,
            AST::Boolean(ref loc, _) => loc,
            AST::Application(ref loc, _, _) => loc,
            AST::Abstraction(ref loc, _, _) => loc,
            AST::Ascription(ref loc, _, _) => loc,
            AST::If(ref loc, _, _, _) => loc,
            AST::TyName(ref loc, _) => loc,
            AST::TyFn(ref loc, _, _) => loc,
        }.clone()
    }
}
