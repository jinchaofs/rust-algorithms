use std::cell::Cell;

/// 递归下降解析器
use super::expr::{Expr, ExprVisitor};

use super::literal::Literal;
use super::token::{Token, TokenType};

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: Cell<usize>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: Cell::new(0),
        }
    }

    fn evaluate(&self, expr: &Box<Expr>) -> Literal {
        expr.accept(self)
    }
    // 表达式
    fn expression() {
        
    }

    fn advance(&self) -> &Token {
        self.current.set(self.current.get() + 1);
        &self.tokens[self.current.get() - 1]
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current.get()]
    }

    fn is_at_end(&self) -> bool {
        self.tokens.len() == self.current.get() + 1
    }
}

impl ExprVisitor for Parser {
    type ReturnType = Literal;
    fn visit_binary(
        &self,
        left: &Box<Expr>,
        operator: &Token,
        right: &Box<Expr>,
    ) -> Self::ReturnType {
        let token_type = operator.token_type;
        let left = self.evaluate(left);
        let right = self.evaluate(right);
        match token_type {
            TokenType::Plus => left + right,
            TokenType::Minus => left - right,
            TokenType::Star => left * right,
            TokenType::Slash => left / right,

            TokenType::BangEqual => Literal::Bool(left != right),
            TokenType::EqualEqual => Literal::Bool(left == right),
            TokenType::Greater => Literal::Bool(left > right),
            TokenType::GreaterEqual => Literal::Bool(left >= right),
            TokenType::Less => Literal::Bool(left < right),
            TokenType::LessEqual => Literal::Bool(left <= right),

            _ => panic!("Expression error."),
        };
        Literal::None
    }

    fn visit_grouping(&self, expr: &Box<Expr>) -> Self::ReturnType {
        self.evaluate(expr)
    }

    fn visit_literal(&self, literal: &Literal) -> Self::ReturnType {
        literal.clone()
    }

    fn visit_unary(&self, operator: &Token, right: &Box<Expr>) -> Self::ReturnType {
        let token_type = operator.token_type;
        let literal = self.evaluate(right);
        match token_type {
            TokenType::Bang => !literal,
            TokenType::Minus => -literal,
            _ => panic!("Does not conform to unary operations"),
        }
    }
}
