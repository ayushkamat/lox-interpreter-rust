use std::{error::Error, panic::catch_unwind};

use crate::{
    error::{error, ParserError},
    grammar::{BinaryOperator, Expression, Literal, UnaryOperator},
    token::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current_pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current_pos: 0,
        }
    }

    fn error(&self, message: &str) {
        if self.done() {
            error(0, message)
        }
        error(self.get_current().line, message);
    }

    fn advance(&mut self) {
        self.current_pos += 1;
    }

    fn has_previous(&self) -> bool {
        self.current_pos > 0
    }

    fn get_previous(&self) -> Token {
        (*self.tokens.get(self.current_pos - 1).unwrap()).clone()
    }

    fn get_current(&self) -> Token {
        (*self.tokens.get(self.current_pos).unwrap()).clone()
    }

    fn done(&self) -> bool {
        self.current_pos >= self.tokens.len()
    }

    fn consume_if<F>(&mut self, condition: F) -> bool
    where
        F: Fn(Token) -> bool,
    {
        if self.done() || !condition(self.get_current()) {
            return false;
        }

        self.advance();
        true
    }

    fn consume_if_is_type(&mut self, types: Vec<TokenType>) -> bool {
        return self.consume_if(|t| types.iter().any(|x| *x == t.token_type));
    }

    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        self.parse_comma()
    }

    fn parse_comma(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.parse_ternary()?;

        while self.consume_if_is_type(vec![TokenType::Comma]) {
            let operator = BinaryOperator::from(self.get_previous().token_type);
            let right = self.parse_ternary()?;
            expr = Expression::binary(operator, expr, right)
        }

        Ok(expr)
    }

    fn parse_ternary(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.parse_equality()?;

        if self.consume_if_is_type(vec![TokenType::QuestionMark]) {
            let inner = self.parse_expression()?; // precedence is ignored on the inner branch of a ternary

            if self.consume_if_is_type(vec![TokenType::Colon]) {
                let outer = self.parse_equality()?;
                expr = Expression::ternary(expr, inner, outer);
            } else {
                return Err(ParserError::new(
                    "unterminated ternary operator".to_string(),
                ));
            }
        }

        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.parse_comparison()?;

        while self.consume_if_is_type(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = BinaryOperator::from(self.get_previous().token_type);
            let right = self.parse_comparison()?;
            expr = Expression::binary(operator, expr, right)
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.parse_term()?;

        while self.consume_if_is_type(vec![
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
        ]) {
            let operator = BinaryOperator::from(self.get_previous().token_type);
            let right = self.parse_term()?;
            expr = Expression::binary(operator, expr, right);
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.parse_factor()?;

        while self.consume_if_is_type(vec![TokenType::Plus, TokenType::Minus]) {
            let operator = BinaryOperator::from(self.get_previous().token_type);
            let right = self.parse_factor()?;
            expr = Expression::binary(operator, expr, right);
        }

        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.parse_unary()?;

        while self.consume_if_is_type(vec![TokenType::Star, TokenType::Slash]) {
            let operator = BinaryOperator::from(self.get_previous().token_type);
            let right = self.parse_unary()?;
            expr = Expression::binary(operator, expr, right);
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expression, ParserError> {
        if self.consume_if_is_type(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = UnaryOperator::from(self.get_previous().token_type);
            let expr = self.parse_primary()?;
            return Ok(Expression::unary(operator, expr));
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expression, ParserError> {
        if self.consume_if(|t| match t.token_type {
            TokenType::String(_)
            | TokenType::Number(_)
            | TokenType::True
            | TokenType::False
            | TokenType::Nil => true,
            _ => false,
        }) {
            return Ok(Expression::literal(Literal::from(
                self.get_previous().token_type,
            )));
        }

        if self.consume_if_is_type(vec![TokenType::LeftParen]) {
            let expr = self.parse_expression()?;
            if !self.consume_if_is_type(vec![TokenType::RightParen]) {
                let message = String::from("missing closing parenthesis");
                self.error(message.as_str());
                return Err(ParserError::new(message));
            }

            return Ok(Expression::grouping(expr));
        }

        Err(ParserError {
            message: String::from("unable to parse primary expression"),
        })
    }

    pub fn parse(&mut self) -> Result<Expression, ParserError> {
        self.parse_expression()
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.done() {
            if match self.get_previous().token_type {
                TokenType::Semicolon
                | TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => true,
                _ => false,
            } {
                return;
            }
            self.advance();
        }
    }
}
