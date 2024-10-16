use crate::ast::{Expression, ObjectValue, Visitor};
use crate::lexer::{Literal, Token, TokenType};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Visitor for Interpreter {
    type Output = Option<ObjectValue>;

    fn visit(&self, expr: &Expression) -> Self::Output {
        match expr {
            Expression::Literal(l) => l.clone(),
            Expression::Grouping(expr) => self.evaluate(expr),
            Expression::Unary(operator, expr) => self.evaluate_unary(operator, expr),
            Expression::Binary(left, operator, right) => {
                self.evaluate_binary(left, operator, right)
            },
        }
    }
}

impl Interpreter {
    pub fn interpret(&self, expr: &Expression) {
        let value = self.evaluate(expr).map(|v| v.to_string()).unwrap_or("".to_string());
        println!("{}", value);
    }
    pub fn evaluate(&self, expr: &Expression) -> Option<ObjectValue> {
        self.visit(expr)
    }

    fn evaluate_unary(&self, operator: &Token, expr: &Expression) -> Option<ObjectValue> {
        let right = self.evaluate(expr)?;
        match right {
            ObjectValue::Number(v) => match operator.token_type {
                TokenType::Minus => Some(ObjectValue::Number(-v)),
                _ => None,
            },
            ObjectValue::Boolean(v) => match operator.token_type {
                TokenType::Bang => Some(ObjectValue::Boolean(!v)),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn evaluate_binary(
        &self,
        left: &Expression,
        operator: &Token,
        right: &Expression,
    ) -> Option<ObjectValue> {
        let left_value = self.evaluate(left)?;
        let right_value = self.evaluate(right)?;

        if !Interpreter::are_compatible(operator, &left_value, &right_value) {
            eprintln!(
                "Incompatible types for operator {}: {:?}, {:?}",
                operator.token_type, left_value, right_value
            );
            return None;
        }

        match (operator.token_type, left_value, right_value) {
            (TokenType::Minus, ObjectValue::Number(left), ObjectValue::Number(right)) => {
                Some(ObjectValue::Number(left - right))
            },
            (TokenType::Slash, ObjectValue::Number(left), ObjectValue::Number(right)) => {
                Some(ObjectValue::Number(left / right))
            },
            (TokenType::Star, ObjectValue::Number(left), ObjectValue::Number(right)) => {
                Some(ObjectValue::Number(left * right))
            },
            (TokenType::Plus, ObjectValue::Number(left), ObjectValue::Number(right)) => {
                Some(ObjectValue::Number(left + right))
            },
            (TokenType::Plus, ObjectValue::String(left), ObjectValue::String(right)) => {
                Some(ObjectValue::String([left, right].concat()))
            },
            (TokenType::Greater, ObjectValue::Number(left), ObjectValue::Number(right)) => {
                Some(ObjectValue::Boolean(left > right))
            },
            (TokenType::GreaterEqual, ObjectValue::Number(left), ObjectValue::Number(right)) => {
                Some(ObjectValue::Boolean(left >= right))
            },
            (TokenType::Less, ObjectValue::Number(left), ObjectValue::Number(right)) => {
                Some(ObjectValue::Boolean(left < right))
            },
            (TokenType::LessEqual, ObjectValue::Number(left), ObjectValue::Number(right)) => {
                Some(ObjectValue::Boolean(left <= right))
            },
            (TokenType::BangEqual, left, right) => Some(ObjectValue::Boolean(left != right)),
            (TokenType::EqualEqual, left, right) => Some(ObjectValue::Boolean(left == right)),
            _ => None,
        }
    }

    fn are_compatible(operator: &Token, left: &Literal, right: &Literal) -> bool {
        match operator.token_type {
            TokenType::Plus => matches!(
                (left, right),
                (Literal::Number(_), Literal::Number(_)) | (Literal::String(_), Literal::String(_))
            ),
            TokenType::Minus | TokenType::Slash | TokenType::Star => {
                matches!((left, right), (Literal::Number(_), Literal::Number(_)))
            },
            TokenType::Greater
            | TokenType::GreaterEqual
            | TokenType::Less
            | TokenType::LessEqual => {
                matches!((left, right), (Literal::Number(_), Literal::Number(_)))
            },
            TokenType::BangEqual | TokenType::EqualEqual => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::ast::Expression;
    use crate::interpreter::Interpreter;
    use crate::lexer::{Literal, Token, TokenType};

    #[test]
    fn interprets_number_addition() {
        let one = Expression::Literal(Some(Literal::Number(1.0)));
        let plus = Token::new(TokenType::Plus, "+", None, 1);
        let two = Expression::Literal(Some(Literal::Number(2.0)));
        let expr = Expression::Binary(Box::new(one), plus, Box::new(two));

        let interpreter = Interpreter::new();
        let result = interpreter.evaluate(&expr);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), Literal::Number(3.0));
    }
}
