use crate::lexer::{Literal, Token};

trait Visitor<T> {
    fn visit(&self, expr: &Expression) -> T;
}

type Operator = Token;
type ObjectValue = Literal;

pub enum Expression {
    Binary(Box<Expression>, Operator, Box<Expression>),
    Grouping(Box<Expression>),
    Literal(Option<ObjectValue>),
    Unary(Operator, Box<Expression>),
}

impl Expression {
    fn accept(&self, visitor: &impl Visitor<String>) -> String {
        visitor.visit(self)
    }
}

pub struct AstPrinter {}

impl AstPrinter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parenthesize(&self, name: &str, exprs: Vec<&Expression>) -> String {
        let mut s = format!("({}", name);
        for expr in exprs {
            s.push(' ');
            s.push_str(&expr.accept(self));
        }
        s.push(')');
        s
    }

    pub fn print(&self, expr: &Expression) -> String {
        self.visit(expr)
    }
}

impl Visitor<String> for AstPrinter {
    fn visit(&self, expr: &Expression) -> String {
        match expr {
            Expression::Binary(left, operator, right) => {
                self.parenthesize(&operator.lexeme, vec![left.as_ref(), right.as_ref()])
            },
            Expression::Grouping(expr) => self.parenthesize("group", vec![expr.as_ref()]),
            Expression::Literal(expr) => expr.as_ref().unwrap_or(&Literal::Null).to_string(),
            Expression::Unary(operator, expr) => {
                self.parenthesize(&operator.lexeme, vec![expr.as_ref()])
            },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::ast::{AstPrinter, Expression, Visitor};
    use crate::lexer::{Literal, Token, TokenType};

    #[test]
    fn prints_ast() {
        let printer = AstPrinter::new();
        let one = Expression::Literal(Some(Literal::Number(1.0)));
        let plus = Token::new(TokenType::Number, "+", None, 1);
        let two = Expression::Literal(Some(Literal::Number(2.0)));
        let expr = Expression::Binary(Box::new(one), plus, Box::new(two));
        assert_eq!(printer.visit(&expr), "(+ 1.0 2.0)".to_string())
    }
}
