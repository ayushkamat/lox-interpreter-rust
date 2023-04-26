use std::fmt::Display;

use crate::{
    token::TokenType,
    visitor::{self},
};

// definitions

pub enum Literal {
    String(String),
    Number(f64),
    True,
    False,
    Nil,
}

pub struct Grouping {
    expr: Box<Expression>,
}

pub enum UnaryOperator {
    Not,
    Minus,
}

pub struct Unary {
    operator: UnaryOperator,
    expr: Box<Expression>,
}

pub enum BinaryOperator {
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    Plus,
    Minus,
    Multiply,
    Divide,

    Comma,
}

pub struct Binary {
    operator: BinaryOperator,
    left_expr: Box<Expression>,
    right_expr: Box<Expression>,
}

pub struct Ternary {
    condition: Box<Expression>,
    left_branch: Box<Expression>,
    right_branch: Box<Expression>,
}

pub enum Expression {
    Literal(Literal),
    Grouping(Grouping),
    Unary(Unary),
    Binary(Binary),
    Ternary(Ternary),
}

// Expression constructors

impl Expression {
    pub fn literal(l: Literal) -> Self {
        Expression::Literal(l)
    }

    pub fn grouping(expr: Expression) -> Self {
        Expression::Grouping(Grouping {
            expr: Box::new(expr),
        })
    }

    pub fn unary(operator: UnaryOperator, expr: Expression) -> Self {
        Expression::Unary(Unary {
            operator,
            expr: Box::new(expr),
        })
    }

    pub fn binary(operator: BinaryOperator, left_expr: Expression, right_expr: Expression) -> Self {
        Expression::Binary(Binary {
            operator,
            left_expr: Box::new(left_expr),
            right_expr: Box::new(right_expr),
        })
    }

    pub fn ternary(
        condition: Expression,
        left_branch: Expression,
        right_branch: Expression,
    ) -> Self {
        Expression::Ternary(Ternary {
            condition: Box::new(condition),
            left_branch: Box::new(left_branch),
            right_branch: Box::new(right_branch),
        })
    }
}

// impl From

impl From<TokenType> for UnaryOperator {
    fn from(typ: TokenType) -> Self {
        match typ {
            TokenType::Bang => Self::Not,
            TokenType::Minus => Self::Minus,

            _ => panic!("unable to convert {:?} to UnaryOperator", typ),
        }
    }
}

impl From<TokenType> for BinaryOperator {
    fn from(typ: TokenType) -> Self {
        match typ {
            TokenType::EqualEqual => Self::Equal,
            TokenType::BangEqual => Self::NotEqual,
            TokenType::Less => Self::Less,
            TokenType::LessEqual => Self::LessEqual,
            TokenType::Greater => Self::Greater,
            TokenType::GreaterEqual => Self::GreaterEqual,

            TokenType::Plus => Self::Plus,
            TokenType::Minus => Self::Minus,
            TokenType::Star => Self::Multiply,
            TokenType::Slash => Self::Divide,

            TokenType::Comma => Self::Comma,

            _ => panic!("unable to convert {:?} to BinaryOperator", typ),
        }
    }
}

impl From<TokenType> for Literal {
    fn from(typ: TokenType) -> Self {
        match typ {
            TokenType::String(s) => Literal::String(s),
            TokenType::Number(n) => Literal::Number(n),

            TokenType::True => Literal::True,
            TokenType::False => Literal::False,
            TokenType::Nil => Literal::Nil,

            _ => panic!("unable to convert {:?} to Literal", typ),
        }
    }
}

// impl Display

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Literal(l) => l.fmt(f),
            Expression::Grouping(g) => g.fmt(f),
            Expression::Unary(u) => u.fmt(f),
            Expression::Binary(b) => b.fmt(f),
            Expression::Ternary(t) => t.fmt(f),
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Literal::String(s) => format!("\"{}\"", s),
                Literal::Number(n) => format!("{}", n),
                Literal::True => String::from("true"),
                Literal::False => String::from("false"),
                Literal::Nil => String::from("nil"),
            }
        )
    }
}

impl Display for Grouping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(group {})", self.expr)
    }
}

impl Display for Unary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({} {})",
            match self.operator {
                UnaryOperator::Minus => "-",
                UnaryOperator::Not => "!",
            },
            self.expr,
        )
    }
}

impl Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({} {} {})",
            match self.operator {
                BinaryOperator::Equal => "==",
                BinaryOperator::NotEqual => "!=",
                BinaryOperator::Less => "<",
                BinaryOperator::LessEqual => "<=",
                BinaryOperator::Greater => ">",
                BinaryOperator::GreaterEqual => ">=",
                BinaryOperator::Plus => "+",
                BinaryOperator::Minus => "-",
                BinaryOperator::Multiply => "*",
                BinaryOperator::Divide => "/",
                BinaryOperator::Comma => ",",
            },
            self.left_expr,
            self.right_expr,
        )
    }
}

impl Display for Ternary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({} ? {} : {})",
            self.condition, self.left_branch, self.right_branch,
        )
    }
}

// ExpressionTrait + impls

pub trait Visitable<R> {
    fn accept(&self, v: &dyn visitor::Visitor<R>) -> R;
}

impl<R> Visitable<R> for Expression {
    fn accept(&self, v: &dyn visitor::Visitor<R>) -> R {
        match self {
            Expression::Literal(l) => l.accept(v),
            Expression::Grouping(l) => l.accept(v),
            Expression::Unary(l) => l.accept(v),
            Expression::Binary(l) => l.accept(v),
            Expression::Ternary(l) => l.accept(v),
        }
    }
}

impl<R> Visitable<R> for Literal {
    fn accept(&self, v: &dyn visitor::Visitor<R>) -> R {
        v.visit_literal_expr(self)
    }
}

impl<R> Visitable<R> for Grouping {
    fn accept(&self, v: &dyn visitor::Visitor<R>) -> R {
        v.visit_grouping_expr(self)
    }
}

impl<R> Visitable<R> for Unary {
    fn accept(&self, v: &dyn visitor::Visitor<R>) -> R {
        v.visit_unary_expr(self)
    }
}

impl<R> Visitable<R> for Binary {
    fn accept(&self, v: &dyn visitor::Visitor<R>) -> R {
        v.visit_binary_expr(self)
    }
}

impl<R> Visitable<R> for Ternary {
    fn accept(&self, v: &dyn visitor::Visitor<R>) -> R {
        v.visit_ternary_expr(self)
    }
}
