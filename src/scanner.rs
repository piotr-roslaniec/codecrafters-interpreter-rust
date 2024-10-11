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

    /// Scan all tokens from `self.source`.
    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::Eof, "", None, self.line))
    }

    /// Scan a single token from `self.source`.
    fn scan_token(&mut self) {
        use crate::lexer::TokenType::*;

        let char = self.advance();
        match char {
            '(' => self.add_char_token(LeftParen),
            ')' => self.add_char_token(RightParen),
            '{' => self.add_char_token(LeftBrace),
            '}' => self.add_char_token(RightBrace),
            ',' => self.add_char_token(Comma),
            '.' => self.add_char_token(Dot),
            '-' => self.add_char_token(Minus),
            '+' => self.add_char_token(Plus),
            ';' => self.add_char_token(Semicolon),
            '*' => self.add_char_token(Star),
            '!' => {
                let token = if self.match_char('=') { BangEqual } else { Bang };
                self.add_char_token(token);
            }
            '=' => {
                let token = if self.match_char('=') { EqualEqual } else { Equal };
                self.add_char_token(token);
            }
            '<' => {
                let token = if self.match_char('=') { LessEqual } else { Less };
                self.add_char_token(token);
            }
            '>' => {
                let token = if self.match_char('=') { GreaterEqual } else { Greater };
                self.add_char_token(token);
            }
            '/' => {
                if self.match_char('/') {
                    // It's a comment - We skip until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_char_token(Slash);
                }
            }
            '\n' => self.line += 1,
            // Ignore whitespace.
            ' ' => {}
            '\r' => {}
            '\t' => {}
            _ => self.reporter.error(self.line, &format!("Unexpected character: {char}")),
        }
    }

    /// Only consume a character in `self.source` if it matches the `expected` character.
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            false
        } else if self.source.chars().nth(self.current).unwrap() != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    /// Get the next character in `self.source` without consuming it.
    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// Consume the next character in `self.source`.
    fn advance(&mut self) -> char {
        let char = self.source.chars().nth(self.current).unwrap_or('\0');
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
    fn scans_single_character_token() {
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
    fn scans_double_character_token() {
        let source = ">=".to_string();
        let tokens = scan(&source);
        assert_eq!(
            tokens,
            vec![
                Token::new(TokenType::GreaterEqual, ">=", None, 1),
                Token::new(TokenType::Eof, "", None, 1)
            ]
        );
    }

    #[test]
    fn scans_comments() {
        let source = "// this is a comment ()".to_string();
        let tokens = scan(&source);
        assert_eq!(
            tokens,
            vec![
                Token::new(TokenType::Eof, "", None, 1)
            ]
        );
    }
    #[test]
    fn scans_operators() {
        let source = "!*+-/=<> <=\n== // should be ignored: >=".to_string();
        let tokens = scan(&source);
        assert_eq!(
            tokens,
            vec![
                Token::new(TokenType::Bang, "!", None, 1),
                Token::new(TokenType::Star, "*", None, 1),
                Token::new(TokenType::Plus, "+", None, 1),
                Token::new(TokenType::Minus, "-", None, 1),
                Token::new(TokenType::Slash, "/", None, 1),
                Token::new(TokenType::Equal, "=", None, 1),
                Token::new(TokenType::Less, "<", None, 1),
                Token::new(TokenType::Greater, ">", None, 1),
                Token::new(TokenType::LessEqual, "<=", None, 1),
                Token::new(TokenType::EqualEqual, "==", None, 2),
                Token::new(TokenType::Eof, "", None, 2)
            ]
        );
    }

    #[test]
    fn ignores_unicode_chars() {
        let source = "///Unicode:£§᯽☺♣".to_string();
        let tokens = scan(&source);
        assert_eq!(
            tokens,
            vec![
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
