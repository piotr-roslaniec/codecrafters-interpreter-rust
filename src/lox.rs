use crate::ast::AstPrinter;
use crate::lexer::Token;
use crate::parser::Parser;
use crate::reporter::Reporter;
use crate::scanner::Scanner;

pub struct Lox {
    pub reporter: Reporter,
    pub tokens: Vec<Token>,
}

impl Lox {
    pub fn new(source: &str) -> Self {
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();
        Self { reporter: scanner.reporter, tokens: scanner.tokens }
    }

    pub fn run(&self) -> Option<String> {
        let mut parser = Parser::new(&self.tokens);
        let expr = parser.parse();
        if parser.had_error() {
            None
        } else {
            let printer = AstPrinter::new();
            Some(printer.print(&expr))
        }
    }

    pub fn had_error(&self) -> bool {
        !self.reporter.errors.is_empty()
    }
}
