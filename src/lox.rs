use crate::ast::{AstPrinter, ObjectValue};
use crate::interpreter::Interpreter;
use crate::lexer::Token;
use crate::parser::Parser;
use crate::reporter::{Reporter, SharedReporter};
use crate::scanner::Scanner;

pub struct Lox {
    pub reporter: SharedReporter,
    pub tokens: Vec<Token>,
}

impl Lox {
    pub fn new(source: &str) -> Self {
        let reporter = Reporter::shared();
        let mut scanner = Scanner::new(source, reporter);
        scanner.scan_tokens();
        Self { reporter: scanner.reporter, tokens: scanner.tokens }
    }

    pub fn run(&mut self) -> Option<String> {
        let mut parser = Parser::new(&self.tokens, &self.reporter);
        let expr = parser.parse();
        if self.had_error() {
            None
        } else {
            let printer = AstPrinter::new();
            Some(printer.print(&expr))
        }
    }

    pub fn evaluate(&mut self) -> Option<ObjectValue> {
        let mut parser = Parser::new(&self.tokens, &self.reporter);
        let expr = parser.parse();
        if !self.had_error() {
            let interpreter = Interpreter::new();
            return interpreter.evaluate(&expr);
        }
        None
    }

    pub fn had_error(&self) -> bool {
        !self.reporter.borrow().errors.is_empty()
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::{Literal, Token, TokenType};
    use crate::lox::Lox;

    #[test]
    fn lox_tokenizes() {
        let source = "true";
        let lox = Lox::new(source);
        assert_eq!(
            lox.tokens,
            vec![
                // Token::new(TokenType::True, "true", Some(Literal::Boolean(true)), 1), // TODO: Fix?
                Token::new(TokenType::True, "true", None, 1),
                Token::new(TokenType::Eof, "", None, 1)
            ]
        );
    }

    #[test]
    fn lox_evaluates_unary_boolean() {
        let source = "!true";
        let mut lox = Lox::new(source);
        let result = lox.evaluate();
        assert!(result.is_some());
        assert_eq!(result.unwrap(), Literal::Boolean(false));
    }

    #[test]
    fn lox_evaluates_binary_number() {
        let source = "2+3";
        let mut lox = Lox::new(source);
        let result = lox.evaluate();
        assert!(result.is_some());
        assert_eq!(result.unwrap(), Literal::Number(5.0));
    }

    #[test]
    fn lox_evaluates_nested() {
        let source = "(2/3)+(2*3)";
        let mut lox = Lox::new(source);
        let result = lox.evaluate();
        assert!(result.is_some());
        assert_eq!(result.unwrap(), Literal::Number(6.666666666666667));
    }

    #[test]
    fn lox_evaluates_strings() {
        let source = "\"hello\" + \" world\"";
        let mut lox = Lox::new(source);
        let result = lox.evaluate();
        assert!(result.is_some());
        assert_eq!(result.unwrap(), Literal::String("hello world".to_string()));
    }
}
