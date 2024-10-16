mod ast;
mod interpreter;
mod lexer;
pub mod lox;
mod parser;
mod reporter;
mod scanner;

pub type Result<T> = anyhow::Result<T>;
