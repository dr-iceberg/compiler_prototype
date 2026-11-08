#![deny(unused_must_use)]

pub mod analyzer;
pub mod ast;
pub mod cli;
pub mod codegen;
pub mod common;
pub mod diagnostics;
pub mod file;
pub mod grammar;
pub mod parser;
pub mod tokenize;
pub(crate) mod util;
