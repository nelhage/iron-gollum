#[derive(Clone, Debug)]
pub struct Loc {
    pub file: String,
    pub begin: u32,
    pub end: u32,
}

#[derive(Clone, Debug)]
pub enum AST {
    Variable(Loc, String),
    Integer(Loc, i64),
    Boolean(Loc, bool),
    Application(Loc, Box<AST>, Vec<Box<AST>>),
    Abstraction(Loc, Vec<Box<AST>>, Box<AST>),
    Ascription(Loc, Box<AST>, Box<AST>),

    TyName(Loc, String),
    TyFn(Loc, Box<AST>, Box<AST>),
}


impl AST {
    pub fn loc(&self) -> Loc {
        match *self {
            AST::Variable(ref loc, _) => loc,
            AST::Integer(ref loc, _) => loc,
            AST::Boolean(ref loc, _) => loc,
            AST::Application(ref loc, _, _) => loc,
            AST::Abstraction(ref loc, _, _) => loc,
            AST::Ascription(ref loc, _, _) => loc,
            AST::TyName(ref loc, _) => loc,
            AST::TyFn(ref loc, _,_) => loc,
        }.clone()
    }
}
