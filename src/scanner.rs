use crate::error::error;
use crate::token::{Token, TokenType};
use crate::util::*;

use std::iter::Peekable;
use unicode_segmentation::{Graphemes, UnicodeSegmentation};

pub struct Scanner<'a> {
    pub source: Peekable<Graphemes<'a>>,
    pub tokens: Vec<Token>,

    pub line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            source: UnicodeSegmentation::graphemes(s, true).peekable(),
            line: 0,
            tokens: Vec::new(),
        }
    }

    fn error(&self, message: &str) {
        error(self.line, message);
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token {
            token_type,
            line: self.line,
        })
    }

    fn peek(&mut self) -> &str {
        let c = self.source.peek();

        if c.is_none() {
            EOF
        } else {
            *c.unwrap()
        }
    }

    fn consume(&mut self) {
        let c = self.source.next();

        if c.is_none() {
            error(self.line, "unexpected end of file");
        }
    }

    fn consume_and_return(&mut self) -> &str {
        let c = self.source.next();

        if c.is_none() {
            error(self.line, "unexpected end of file");
        }

        c.unwrap()
    }

    fn consume_if_next(&mut self, condition: fn(&str, &str) -> bool) -> bool {
        if self.peek() == EOF {
            false
        } else {
            let mut cloned = self.source.clone();
            let c1 = match cloned.next() {
                None => EOF,
                Some(s) => s,
            };

            let c2 = match cloned.next() {
                None => EOF,
                Some(s) => s,
            };

            if condition(c1, c2) {
                self.consume();
            }
            return condition(c1, c2);
        }
    }

    fn consume_and_return_if(&mut self, condition: fn(&str) -> bool) -> Option<&str> {
        let n = self.peek();
        if n == EOF || !condition(n) {
            None
        } else {
            Some(self.consume_and_return())
        }
    }

    fn consume_while(&mut self, condition: fn(&str) -> bool) {
        loop {
            let n = self.peek();
            if n == EOF || !condition(n) {
                break;
            }
            self.consume()
        }
    }

    fn scan_string(&mut self) {
        let mut out = String::new();
        let mut lines = 0;
        loop {
            if self.peek() == EOF {
                self.error("unterminated string literal");
                return;
            }
            let n = self.consume_and_return();

            if n == "\n" {
                lines = lines + 1;
            }
            if n == "\"" {
                break;
            }
            out += n;
        }

        self.add_token(TokenType::String(out))
    }

    fn scan_number(&mut self) {
        let mut out: f64 = 0.0;

        loop {
            let p = self.peek();
            if is_digit(p) {
                out = 10.0 * out + self.consume_and_return().parse::<f64>().unwrap();
            } else if self.consume_if_next(|c1, c2| c1 == "." && is_digit(c2)) {
                break;
            } else {
                self.add_token(TokenType::Number(out));
                return;
            }
        }

        let mut divisor: f64 = 10.0;

        loop {
            let p = self.peek();
            if is_digit(p) {
                out = out + self.consume_and_return().parse::<f64>().unwrap() / divisor;
                divisor *= 10.0;
            } else {
                self.add_token(TokenType::Number(out));
                return;
            }
        }
    }

    fn scan_identifier_or_keyword(&mut self) {
        let mut out = String::new();
        loop {
            let c = self.peek();
            if !is_alphanumeric(c) {
                break;
            }

            out += self.consume_and_return();
        }

        let kw_type = get_token_type_from_keyword(out.as_str());
        if let Some(kw_type) = kw_type {
            self.add_token(kw_type)
        } else {
            self.add_token(TokenType::Identifier(out))
        }
    }

    fn scan_token(&mut self) {
        let c = self.peek();
        if is_digit(c) {
            self.scan_number();
        } else if is_alphanumeric(c) {
            self.scan_identifier_or_keyword();
        }

        let token_type: TokenType = match self.consume_and_return() {
            "(" => TokenType::LeftParen,
            ")" => TokenType::RightParen,
            "{" => TokenType::LeftBrace,
            "}" => TokenType::RightBrace,
            "," => TokenType::Comma,
            "." => TokenType::Dot,
            "-" => TokenType::Minus,
            "+" => TokenType::Plus,
            ";" => TokenType::Semicolon,
            "*" => TokenType::Star,
            "?" => TokenType::QuestionMark,
            ":" => TokenType::Colon,

            "!" => {
                if self.consume_and_return_if(|c| c == "=").is_none() {
                    TokenType::Bang
                } else {
                    TokenType::BangEqual
                }
            }
            "=" => {
                if self.consume_and_return_if(|c| c == "=").is_none() {
                    TokenType::Equal
                } else {
                    TokenType::EqualEqual
                }
            }
            "<" => {
                if self.consume_and_return_if(|c| c == "=").is_none() {
                    TokenType::Less
                } else {
                    TokenType::LessEqual
                }
            }
            ">" => {
                if self.consume_and_return_if(|c| c == "=").is_none() {
                    TokenType::Greater
                } else {
                    TokenType::GreaterEqual
                }
            }

            "/" => {
                if self.consume_and_return_if(|c| c == "/").is_some() {
                    self.consume_while(|c| c != "\n");
                    return;
                } else {
                    TokenType::Slash
                }
            }

            " " | "\r" | "\t" => return,
            "\n" => {
                self.line += 1;
                return;
            }

            "\"" => {
                self.scan_string();
                return;
            }

            _ => {
                self.error("unknown character");
                return;
            }
        };

        self.add_token(token_type);
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        while self.peek() != EOF {
            self.scan_token();
        }
        self.add_token(TokenType::EOF);

        self.tokens.clone()
    }
}
