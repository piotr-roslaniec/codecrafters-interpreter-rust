use crate::lexer::{Token, TokenType};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Reporter {
    pub errors: Vec<String>,
}

impl Default for Reporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Reporter {
    fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn shared() -> SharedReporter {
        Rc::new(RefCell::new(Reporter::new()))
    }

    pub fn error(&mut self, token: Token, message: &str) {
        if token.token_type == TokenType::Eof {
            self.report(token.line, " at the end", message)
        } else {
            self.report(token.line, &format!(" at '{}'", token.lexeme), message)
        }
    }

    pub fn report(&mut self, line: usize, location: &str, message: &str) {
        let error = format!("[line {line}] Error{location}: {message}");
        eprintln!("{}", error);
        self.errors.push(error)
    }
}

pub type SharedReporter = Rc<RefCell<Reporter>>;
