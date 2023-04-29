// --- Expressions ---

use std::fmt::Display;

use crate::{
    token::TokenType,
    visitors::visitor::{Visitable, Visitor},
};

// definitions

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    True,
    False,
    Nil,
}

#[derive(Debug, Clone)]
pub struct Grouping {
    pub expr: Box<Expression>,
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Not,
    Minus,
}

#[derive(Debug, Clone)]
pub struct Unary {
    pub operator: UnaryOperator,
    pub expr: Box<Expression>,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Binary {
    pub operator: BinaryOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct Ternary {
    pub condition: Box<Expression>,
    pub left_branch: Box<Expression>,
    pub right_branch: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Grouping(Grouping),
    Unary(Unary),
    Binary(Binary),
    Ternary(Ternary),
    Variable(Variable),
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
            left: Box::new(left_expr),
            right: Box::new(right_expr),
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

    pub fn variable(name: String) -> Self {
        Expression::Variable(Variable { name })
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

impl From<TokenType> for Variable {
    fn from(typ: TokenType) -> Self {
        match typ {
            TokenType::Identifier(name) => Variable { name },

            _ => panic!("unable to convert {:?} to Variable", typ),
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
            Expression::Variable(v) => v.fmt(f),
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
        write!(f, "({})", self.expr)
    }
}

impl Display for Unary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
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
            "{} {} {}",
            self.left,
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
            self.right,
        )
    }
}

impl Display for Ternary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ? {} : {}",
            self.condition, self.left_branch, self.right_branch,
        )
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

// impls

impl<R> Visitable<R> for Expression {
    fn accept(&mut self, v: &mut dyn Visitor<R>) -> R {
        match self {
            Expression::Literal(x) => x.accept(v),
            Expression::Grouping(x) => x.accept(v),
            Expression::Unary(x) => x.accept(v),
            Expression::Binary(x) => x.accept(v),
            Expression::Ternary(x) => x.accept(v),
            Expression::Variable(x) => x.accept(v),
        }
    }
}

impl<R> Visitable<R> for Literal {
    fn accept(&mut self, v: &mut dyn Visitor<R>) -> R {
        v.visit_literal_expr(self)
    }
}

impl<R> Visitable<R> for Grouping {
    fn accept(&mut self, v: &mut dyn Visitor<R>) -> R {
        v.visit_grouping_expr(self)
    }
}

impl<R> Visitable<R> for Unary {
    fn accept(&mut self, v: &mut dyn Visitor<R>) -> R {
        v.visit_unary_expr(self)
    }
}

impl<R> Visitable<R> for Binary {
    fn accept(&mut self, v: &mut dyn Visitor<R>) -> R {
        v.visit_binary_expr(self)
    }
}

impl<R> Visitable<R> for Ternary {
    fn accept(&mut self, v: &mut dyn Visitor<R>) -> R {
        v.visit_ternary_expr(self)
    }
}

impl<R> Visitable<R> for Variable {
    fn accept(&mut self, v: &mut dyn Visitor<R>) -> R {
        v.visit_var_expr(self)
    }
}
