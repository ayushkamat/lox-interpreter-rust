use std::fmt::Display;

use crate::{grammar::expression::Expression, token::Token};

pub fn error(line: usize, message: &str) {
    report(line, "", message);
}

pub fn report(line: usize, cause: &str, message: &str) {
    println!("[Line {line}] Error {cause}: {message}")
}

#[derive(Debug, Clone)]
pub struct ParserError {
    pub message: String,
    pub token: Token,
}

impl ParserError {
    pub fn new(message: String, token: Token) -> Self {
        Self { message, token }
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParserError at L{}: {}", self.token.line, self.message)
    }
}

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub message: String,
    pub expr: Expression,
}

impl RuntimeError {
    pub fn new(message: String, expr: Expression) -> Self {
        Self { message, expr }
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RuntimeError while evaluating `{}`: {}",
            self.expr, self.message
        )
    }
}
