use std::rc::Rc;

use ast;
use types;

pub fn typecheck<'a>(ast: &ast::AST<'a>, globals: Rc<types::TypeEnv<'a>>) -> Result<types::Type<'a>, types::TypeError<'a>> {
    return Err(types::TypeError{loc: ast.loc(), err: "Unimplemented"})
}
