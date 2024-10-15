use crate::lexer::{Token, TokenType};

pub struct Reporter {
    pub errors: Vec<String>,
}

impl Default for Reporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Reporter {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
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

    pub fn merge(&self, other: &Self) -> Self {
        Self { errors: [self.errors.clone(), other.errors.clone()].concat() }
    }
}
