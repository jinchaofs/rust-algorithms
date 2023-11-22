use std::ops::{Add, Div, Mul, Neg, Not, Sub};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
    None,
}

macro_rules! literal_math_op {
    ($left:expr, $operator:tt, $right:expr) => {
        match ($left, $right) {
            (Literal::Integer(left), Literal::Integer(right)) => Literal::Integer(left $operator right),
            (Literal::Float(left), Literal::Float(right)) => Literal::Float(left $operator right),
            (Literal::Integer(left), Literal::Float(right)) => Literal::Float(left as f64 $operator right),
            (Literal::Float(left), Literal::Integer(right)) => Literal::Float(left $operator right as f64),
            _ => panic!("Unsupported operation for given types."),
        }
    };
}

impl Add for Literal {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Literal::Integer(left), Literal::Integer(right)) => Literal::Integer(left + right),
            (Literal::Float(left), Literal::Float(right)) => Literal::Float(left + right),
            (Literal::Integer(left), Literal::Float(right)) => Literal::Float(left as f64 + right),
            (Literal::Float(left), Literal::Integer(right)) => Literal::Float(left + right as f64),
            (Literal::String(left), Literal::String(right)) => {
                Literal::String(format!("{}{}", left, right))
            }
            _ => panic!("Unsupported operation for given types."),
        }
    }
}

impl Sub for Literal {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        literal_math_op!(self, -, rhs)
    }
}

impl Mul for Literal {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        literal_math_op!(self, *, rhs)
    }
}

impl Div for Literal {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        literal_math_op!(self, /, rhs)
    }
}

impl Not for Literal {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Literal::Bool(val) => Literal::Bool(!val),
            Literal::Float(_) => Literal::Bool(false),
            Literal::Integer(val) => Literal::Bool(val == 0),
            Literal::String(val) => Literal::Bool(val.is_empty()),
            Literal::None => Literal::Bool(true),
        }
    }
}

impl Neg for Literal {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Literal::Integer(val) => Literal::Integer(-val),
            Literal::Float(val) => Literal::Float(-val),
            _ => panic!("Negative signs can only be applied before numbers."),
        }
    }
}
