use crate::lexer::{Literal, Token, TokenType};
use crate::reporter::SharedReporter;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::str::FromStr;
use std::string::ToString;

const NULL_C: char = '\0';
static KEYWORDS: Lazy<HashMap<&'static str, TokenType>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("and", TokenType::And);
    m.insert("class", TokenType::Class);
    m.insert("else", TokenType::Else);
    m.insert("false", TokenType::False);
    m.insert("for", TokenType::For);
    m.insert("fun", TokenType::Fun);
    m.insert("if", TokenType::If);
    m.insert("nil", TokenType::Nil);
    m.insert("or", TokenType::Or);
    m.insert("print", TokenType::Print);
    m.insert("return", TokenType::Return);
    m.insert("super", TokenType::Super);
    m.insert("this", TokenType::This);
    m.insert("true", TokenType::True);
    m.insert("var", TokenType::Var);
    m.insert("while", TokenType::While);
    m
});

pub struct Scanner {
    source: String,
    pub reporter: SharedReporter,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str, reporter: SharedReporter) -> Self {
        Self {
            source: source.to_string(),
            reporter,
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

        let current = self.current;
        let char = self.advance();
        if char.is_none() {
            self.reporter.borrow_mut().report(
                self.line,
                "",
                &format!("Invalid UTF-8 codepoint at: {}", current),
            );
            return;
        }

        let char = char.unwrap();
        let token_type = match char {
            '(' => Some(LeftParen),
            ')' => Some(RightParen),
            '{' => Some(LeftBrace),
            '}' => Some(RightBrace),
            ',' => Some(Comma),
            '.' => Some(Dot),
            '-' => Some(Minus),
            '+' => Some(Plus),
            ';' => Some(Semicolon),
            '*' => Some(Star),
            '!' => Some(if self.match_char('=') { BangEqual } else { Bang }),
            '=' => Some(if self.match_char('=') { EqualEqual } else { Equal }),
            '<' => Some(if self.match_char('=') { LessEqual } else { Less }),
            '>' => Some(if self.match_char('=') { GreaterEqual } else { Greater }),
            '/' => {
                if self.match_char('/') {
                    // Skip until the end of the line for comments
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    None
                } else {
                    Some(Slash)
                }
            },
            '\n' => {
                self.line += 1;
                None
            },
            ' ' | '\r' | '\t' => None, // Ignore whitespace
            '"' => {
                self.string();
                None
            },
            _ => {
                if self.is_digit(char) {
                    self.number();
                    None
                } else if self.is_alpha(char) {
                    self.identifier();
                    None
                } else {
                    self.reporter.borrow_mut().report(
                        self.line,
                        "",
                        &format!("Unexpected character: {char}"),
                    );
                    None
                }
            },
        };

        if let Some(token) = token_type {
            self.add_char_token(token);
        }
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    fn is_alphanumeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = *KEYWORDS.get(text).unwrap_or(&TokenType::Identifier);
        self.add_token(token_type, None);
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // Consume the "."
            self.advance();
        }

        // Read the rest of the digits
        while self.is_digit(self.peek()) {
            self.advance();
        }
        let as_num = f64::from_str(&self.source[self.start..self.current])
            .expect("Failed to parse number from source");
        self.add_token(TokenType::Number, Some(Literal::Number(as_num)));
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            // Our string are multiline
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.reporter.borrow_mut().report(self.line, "", "Unterminated string.");
            return;
        }

        // The closing ".
        self.advance();

        // Trim the surrounding quotes.
        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token(TokenType::String, Some(Literal::String(value.to_string())));
    }

    /// Only consume a character in `self.source` if it matches the `expected` character.
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.nth(self.current) != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    /// Get the next character in `self.source` without consuming it.
    fn peek(&self) -> char {
        if self.is_at_end() {
            NULL_C
        } else {
            self.nth(self.current)
        }
    }

    /// Get the character after the next character in `self.source` without consuming it.
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            NULL_C
        } else {
            self.nth(self.current + 1)
        }
    }

    fn nth(&self, n: usize) -> char {
        char::from(self.source.as_bytes()[n])
    }

    fn is_digit(&self, c: char) -> bool {
        c.is_numeric()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// Consume the next character in `self.source`.
    fn advance(&mut self) -> Option<char> {
        // The next value in `self.source` may or may not be a valid Unicode codepoint
        // See: https://users.rust-lang.org/t/should-you-really-use-chars-for-characters-in-a-string/71459/3
        let char = self.source.chars().nth(self.current);
        self.current += 1;
        char
    }

    fn add_char_token(&mut self, t: TokenType) {
        self.add_token(t, None)
    }

    fn add_token(&mut self, t: TokenType, literal: Option<Literal>) {
        let text = self.source[self.start..self.current].to_string();
        let token = Token::new(t, &text, literal, self.line);
        self.tokens.push(token);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::reporter::Reporter;

    fn scan(source: &str) -> Vec<Token> {
        let reporter = Reporter::shared();
        let mut scanner = Scanner::new(source, reporter);
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
        assert_eq!(tokens, vec![Token::new(TokenType::Eof, "", None, 1)]);
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
        assert_eq!(tokens, vec![Token::new(TokenType::Eof, "", None, 1)]);
    }

    #[test]
    fn scans_multiline_string() {
        let source = "\"hello\nworld\"".to_string();
        let tokens = scan(&source);
        assert_eq!(
            tokens,
            vec![
                Token::new(
                    TokenType::String,
                    "\"hello\nworld\"",
                    Some(Literal::String("hello\nworld".to_string())),
                    2
                ),
                Token::new(TokenType::Eof, "", None, 2)
            ]
        );
    }

    #[test]
    fn scans_numbers() {
        let source = "1\n2.0\n03\n.0".to_string();
        let tokens = scan(&source);
        assert_eq!(
            tokens,
            vec![
                Token::new(TokenType::Number, "1", Some(Literal::Number(1.0)), 1),
                Token::new(TokenType::Number, "2.0", Some(Literal::Number(2.0)), 2),
                Token::new(TokenType::Number, "03", Some(Literal::Number(3.0)), 3),
                Token::new(TokenType::Dot, ".", None, 4),
                Token::new(TokenType::Number, "0", Some(Literal::Number(0.0)), 4),
                Token::new(TokenType::Eof, "", None, 4)
            ]
        );
    }

    #[test]
    fn scans_identifiers() {
        let source = "foo bar _hello".to_string();
        let tokens = scan(&source);
        assert_eq!(
            tokens,
            vec![
                Token::new(TokenType::Identifier, "foo", None, 1),
                Token::new(TokenType::Identifier, "bar", None, 1),
                Token::new(TokenType::Identifier, "_hello", None, 1),
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
        let reporter = Reporter::shared();
        let mut scanner = Scanner::new(source, reporter);
        scanner.scan_tokens();
        assert_eq!(
            scanner.reporter.borrow().errors,
            vec![
                "[line 1] Error: Unexpected character: $",
                "[line 1] Error: Unexpected character: #"
            ]
        );
    }
}
