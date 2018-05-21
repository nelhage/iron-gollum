#![allow(dead_code)]

extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod names;
pub mod parser;
pub mod ast;
pub mod types;
pub mod env;
pub mod globals;
pub mod typecheck;

const VERSION: &'static str = "0.0.1";
