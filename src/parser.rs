use crate::{
    error::{error, ParserError},
    grammar::{declaration, expression, statement},
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

    fn parse_declaration(&mut self) -> Result<declaration::Declaration, ParserError> {
        let ret = if self.consume_if_is_type(vec![TokenType::Var]) {
            self.parse_var_declaration()
        } else {
            self.parse_statement_declaration()
        };

        if ret.is_err() {
            self.synchronize();
        }

        ret
    }

    fn parse_var_declaration(&mut self) -> Result<declaration::Declaration, ParserError> {
        match self.get_current().token_type {
            TokenType::Identifier(name) => {
                self.advance();
                if self.consume_if_is_type(vec![TokenType::Semicolon]) {
                    Ok(declaration::Declaration::initialization(name))
                } else if self.consume_if_is_type(vec![TokenType::Equal]) {
                    let definition = self.parse_expression()?;
                    if self.consume_if_is_type(vec![TokenType::Semicolon]) {
                        Ok(declaration::Declaration::instantiation(name, definition))
                    } else {
                        Err(ParserError::new(
                            "variable initializations must end with a semicolon".to_string(),
                            self.get_current(),
                        ))
                    }
                } else {
                    Err(ParserError::new(
                        "invalid variable initialization".to_string(),
                        self.get_current(),
                    ))
                }
            }
            _ => Err(ParserError::new(
                "variable names must be valid identifiers".to_string(),
                self.get_current(),
            )),
        }
    }

    fn parse_statement_declaration(&mut self) -> Result<declaration::Declaration, ParserError> {
        let ret = self.parse_statement()?;
        Ok(declaration::Declaration::statement(ret))
    }

    fn parse_statement(&mut self) -> Result<statement::Statement, ParserError> {
        let ret = if self.consume_if_is_type(vec![TokenType::Print]) {
            self.parse_print_statement()
        } else {
            self.parse_expression_statement()
        }?;

        if self.consume_if_is_type(vec![TokenType::Semicolon]) {
            Ok(ret)
        } else {
            Err(ParserError::new(
                "statements must end with a semicolon".to_string(),
                self.get_current(),
            ))
        }
    }

    fn parse_expression_statement(&mut self) -> Result<statement::Statement, ParserError> {
        let expr = self.parse_expression()?;
        Ok(statement::Statement::expression(expr))
    }

    fn parse_print_statement(&mut self) -> Result<statement::Statement, ParserError> {
        let expr = self.parse_expression()?;
        Ok(statement::Statement::print(expr))
    }

    fn parse_expression(&mut self) -> Result<expression::Expression, ParserError> {
        self.parse_comma()
    }

    fn parse_comma(&mut self) -> Result<expression::Expression, ParserError> {
        let mut expr = self.parse_ternary()?;

        while self.consume_if_is_type(vec![TokenType::Comma]) {
            let operator = expression::BinaryOperator::from(self.get_previous().token_type);
            let right = self.parse_ternary()?;
            expr = expression::Expression::binary(operator, expr, right)
        }

        Ok(expr)
    }

    fn parse_ternary(&mut self) -> Result<expression::Expression, ParserError> {
        let mut expr = self.parse_equality()?;

        if self.consume_if_is_type(vec![TokenType::QuestionMark]) {
            let inner = self.parse_expression()?; // precedence is ignored on the inner branch of a ternary

            if self.consume_if_is_type(vec![TokenType::Colon]) {
                let outer = self.parse_equality()?;
                expr = expression::Expression::ternary(expr, inner, outer);
            } else {
                return Err(ParserError::new(
                    "unterminated ternary operator".to_string(),
                    self.get_current(),
                ));
            }
        }

        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<expression::Expression, ParserError> {
        let mut expr = self.parse_comparison()?;

        while self.consume_if_is_type(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = expression::BinaryOperator::from(self.get_previous().token_type);
            let right = self.parse_comparison()?;
            expr = expression::Expression::binary(operator, expr, right)
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<expression::Expression, ParserError> {
        let mut expr = self.parse_term()?;

        while self.consume_if_is_type(vec![
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
        ]) {
            let operator = expression::BinaryOperator::from(self.get_previous().token_type);
            let right = self.parse_term()?;
            expr = expression::Expression::binary(operator, expr, right);
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<expression::Expression, ParserError> {
        let mut expr = self.parse_factor()?;

        while self.consume_if_is_type(vec![TokenType::Plus, TokenType::Minus]) {
            let operator = expression::BinaryOperator::from(self.get_previous().token_type);
            let right = self.parse_factor()?;
            expr = expression::Expression::binary(operator, expr, right);
        }

        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<expression::Expression, ParserError> {
        let mut expr = self.parse_unary()?;

        while self.consume_if_is_type(vec![TokenType::Star, TokenType::Slash]) {
            let operator = expression::BinaryOperator::from(self.get_previous().token_type);
            let right = self.parse_unary()?;
            expr = expression::Expression::binary(operator, expr, right);
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<expression::Expression, ParserError> {
        if self.consume_if_is_type(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = expression::UnaryOperator::from(self.get_previous().token_type);
            let expr = self.parse_primary()?;
            return Ok(expression::Expression::unary(operator, expr));
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<expression::Expression, ParserError> {
        if self.consume_if(|t| match t.token_type {
            TokenType::String(_)
            | TokenType::Number(_)
            | TokenType::True
            | TokenType::False
            | TokenType::Nil => true,
            _ => false,
        }) {
            return Ok(expression::Expression::literal(expression::Literal::from(
                self.get_previous().token_type,
            )));
        }

        if let TokenType::Identifier(name) = self.get_current().token_type {
            self.advance();
            return Ok(expression::Expression::variable(name));
        }

        if self.consume_if_is_type(vec![TokenType::LeftParen]) {
            let expr = self.parse_expression()?;
            if !self.consume_if_is_type(vec![TokenType::RightParen]) {
                let message = String::from("missing closing parenthesis");
                self.error(message.as_str());
                return Err(ParserError::new(message, self.get_current()));
            }

            return Ok(expression::Expression::grouping(expr));
        }

        Err(ParserError::new(
            String::from("unable to parse primary expression"),
            self.get_current(),
        ))
    }

    pub fn parse(&mut self) -> Vec<Result<declaration::Declaration, ParserError>> {
        let mut ret: Vec<Result<declaration::Declaration, ParserError>> = Vec::new();
        while !self.consume_if_is_type(vec![TokenType::EOF]) {
            ret.push(self.parse_declaration());
        }
        ret
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
