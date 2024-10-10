use crate::lexer::{Token, TokenType};
use crate::reporter::Reporter;

pub struct Scanner {
    source: String,
    pub reporter: Reporter,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            reporter: Reporter::new(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            // Source, even if empty, starts at the first line.
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "", None, self.line))
    }
    fn scan_token(&mut self) {
        let char = self.advance();
        match char {
            '(' => self.add_char_token(TokenType::LeftParen),
            ')' => self.add_char_token(TokenType::RightParen),
            '{' => self.add_char_token(TokenType::LeftBrace),
            '}' => self.add_char_token(TokenType::RightBrace),
            ',' => self.add_char_token(TokenType::Comma),
            '.' => self.add_char_token(TokenType::Dot),
            '-' => self.add_char_token(TokenType::Minus),
            '+' => self.add_char_token(TokenType::Plus),
            ';' => self.add_char_token(TokenType::Semicolon),
            '*' => self.add_char_token(TokenType::Star),
            _ => self
                .reporter
                .error(self.line, &format!("Unexpected character: {char}")),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let char = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        char
    }

    fn add_char_token(&mut self, t: TokenType) {
        self.add_token(t, None)
    }

    fn add_token(&mut self, t: TokenType, literal: Option<String>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(t, &text, literal, self.line))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn scan(source: &str) -> Vec<Token> {
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();
        scanner.tokens
    }
    #[test]
    fn scans_empty_source() {
        let source = "".to_string();
        let tokens = scan(&source);
        assert_eq!(tokens, vec![Token::new(TokenType::Eof, "", None, 1)]);
    }

    #[test]
    fn scans_single_token() {
        let source = "(".to_string();
        let tokens = scan(&source);
        assert_eq!(
            tokens,
            vec![
                Token::new(TokenType::LeftParen, "(", None, 1),
                Token::new(TokenType::Eof, "", None, 1)
            ]
        );
    }

    #[test]
    fn scans_unbalanced_parens() {
        let source = "(()".to_string();
        let tokens = scan(&source);
        assert_eq!(
            tokens,
            vec![
                Token::new(TokenType::LeftParen, "(", None, 1),
                Token::new(TokenType::LeftParen, "(", None, 1),
                Token::new(TokenType::RightParen, ")", None, 1),
                Token::new(TokenType::Eof, "", None, 1)
            ]
        );
    }

    #[test]
    fn makes_errors_for_unexpected_characters() {
        let source = ",.$(#";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();
        assert_eq!(
            scanner.reporter.errors,
            vec![
                "[line 1] Error: Unexpected character: $",
                "[line 1] Error: Unexpected character: #"
            ]
        );
    }
}
