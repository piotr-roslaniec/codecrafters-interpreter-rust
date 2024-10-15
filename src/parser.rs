use crate::ast::Expression;
use crate::lexer::{Literal, Token, TokenType};
use crate::reporter::Reporter;
use crate::Result;
use anyhow::{anyhow, Error};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    reporter: Reporter,
}

impl Parser {
    pub fn new(tokens: &[Token]) -> Self {
        Self { tokens: tokens.to_vec(), current: 0, reporter: Reporter::new() }
    }

    pub fn had_error(&self) -> bool {
        !self.reporter.errors.is_empty()
    }

    pub fn parse(&mut self) -> Expression {
        self.expression().unwrap_or(Expression::Literal(None))
    }

    fn expression(&mut self) -> Result<Expression> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expression> {
        let mut expr = self.comparison()?;
        while self.matches(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expression::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expression> {
        let mut expr = self.term()?;
        while self.matches(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expression::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expression> {
        let mut expr = self.factor()?;
        while self.matches(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expression::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expression> {
        let mut expr = self.unary()?;
        while self.matches(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expression::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expression> {
        if self.matches(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expression::Unary(operator, Box::new(right)));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expression> {
        if self.matches(vec![TokenType::False]) {
            return Ok(Expression::Literal(Some(Literal::Boolean(false))));
        }
        if self.matches(vec![TokenType::True]) {
            return Ok(Expression::Literal(Some(Literal::Boolean(true))));
        }
        if self.matches(vec![TokenType::Nil]) {
            return Ok(Expression::Literal(Some(Literal::String("nil".to_string()))));
        }

        if self.matches(vec![TokenType::Number, TokenType::String]) {
            return Ok(Expression::Literal(self.previous().literal));
        }

        if self.matches(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")
                .map_err(|_| self.synchronize())
                .unwrap();
            return Ok(Expression::Grouping(Box::new(expr)));
        }

        Err(self.error(self.peek(), "Expect expression."))
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<()> {
        if self.check(token_type) {
            self.advance();
            return Ok(());
        }
        Err(self.error(self.peek(), message))
    }

    fn error(&mut self, token: Token, message: &str) -> Error {
        self.reporter.error(token, message);
        anyhow!("Parser error")
    }

    /// Discard tokens until your each the next statement.
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class => {},
                TokenType::Fun => {},
                TokenType::Var => {},
                TokenType::For => {},
                TokenType::If => {},
                TokenType::While => {},
                TokenType::Print => {},
                TokenType::Return => {},
                _ => {
                    self.advance();
                },
            }
        }
    }

    /// Check whether the current token matches any of `token_types`.
    fn matches(&mut self, token_types: Vec<TokenType>) -> bool {
        for tt in token_types {
            if self.check(tt) {
                self.advance();
                return true;
            }
        }
        false
    }

    /// Checks whether the `self.current` token matches `token_type`.
    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    /// Advances `self.current` and consumes the `self.current` Token.
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    /// Returns the `self.current` token, yet to be consumed.
    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    /// Returns most recently consumed `Token`.
    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}
