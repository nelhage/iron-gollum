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
}