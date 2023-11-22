use super::{literal::Literal, token::Token};

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expr: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Literal(Literal),
}

impl Expr {
    pub fn accept<T: ExprVisitor>(&self, visitor: &T) -> T::ReturnType {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary(left, operator, right),
            Expr::Grouping { expr } => visitor.visit_grouping(expr),
            Expr::Unary { operator, right } => visitor.visit_unary(operator, right),
            Expr::Literal(literal) => visitor.visit_literal(literal),
        }
    }
}

pub trait ExprVisitor {
    type ReturnType;

    fn visit_binary(
        &self,
        left: &Box<Expr>,
        operator: &Token,
        right: &Box<Expr>,
    ) -> Self::ReturnType;

    fn visit_grouping(&self, expr: &Box<Expr>) -> Self::ReturnType;

    fn visit_unary(&self, operator: &Token, right: &Box<Expr>) -> Self::ReturnType;

    fn visit_literal(&self, literal: &Literal) -> Self::ReturnType;
}
