// --- Statements ---

use std::fmt::Display;

use crate::grammar::expression;
use crate::visitors::visitor::{Visitable, Visitor};

// definitions

#[derive(Debug, Clone)]
pub struct Expression {
    pub expr: expression::Expression,
}

#[derive(Debug, Clone)]
pub struct Print {
    pub expr: expression::Expression,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression),
    Print(Print),
}

// constructors

impl Statement {
    pub fn expression(expr: expression::Expression) -> Self {
        Statement::Expression(Expression { expr })
    }
    pub fn print(expr: expression::Expression) -> Self {
        Statement::Print(Print { expr })
    }
}

// impl Display

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Expression(expr) => expr.fmt(f),
            Statement::Print(expr) => expr.fmt(f),
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{};", self.expr)
    }
}

impl Display for Print {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "print {};", self.expr)
    }
}

// impl Visitable

impl<R> Visitable<R> for Statement {
    fn accept(&mut self, v: &mut dyn Visitor<R>) -> R {
        match self {
            Statement::Expression(expr) => expr.accept(v),
            Statement::Print(expr) => expr.accept(v),
        }
    }
}

impl<R> Visitable<R> for Expression {
    fn accept(&mut self, v: &mut dyn Visitor<R>) -> R {
        v.visit_expr_stmt(self)
    }
}

impl<R> Visitable<R> for Print {
    fn accept(&mut self, v: &mut dyn Visitor<R>) -> R {
        v.visit_print_stmt(self)
    }
}
