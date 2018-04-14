use types;

use std::rc::Rc;

pub fn global_env() -> Rc<types::TypeEnv<'static>> {
    types::TypeEnv::empty()
}
