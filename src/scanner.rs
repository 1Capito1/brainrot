#![allow(dead_code)]

use crate::token::{Literal, Token, TokenType};
use anyhow::{Error, Result};
use phf;
use phf_macros::phf_map;
use thiserror::Error;

const KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "and" => TokenType::And,
    "class" => TokenType::Class,
    "else" => TokenType::Else,
    "false" => TokenType::False,
    "for" => TokenType::For,
    "fun" => TokenType::Fun,
    "if" => TokenType::If,
    "nil" => TokenType::Nil,
    "or" => TokenType::Or,
    "print" => TokenType::Print,
    "return" => TokenType::Return,
    "super" => TokenType::Super,
    "this" => TokenType::This,
    "true" => TokenType::True,
    "var" => TokenType::Var,
    "while" => TokenType::While,
};

pub struct Scanner {
    source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    errors: Vec<Error>,
}

#[derive(Error, Debug)]
pub enum LexicalError {
    #[error("[{1}, {2}]: Invalid Character Detected: {0}")]
    InvalidCharacter(char, usize, usize),

    #[error("[{0}, {1}]: Unterminated String")]
    UnterminatedString(usize, usize),

    #[error("[{0}]: Unterminated Block Comment")]
    UnterminatedBlockComment(usize, String),
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            errors: vec![],
        }
    }
    pub fn default() -> Self {
        Self {
            source: "".to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            errors: vec![],
        }
    }

    fn add_error(&mut self, e: Error) {
        self.errors.push(e);
    }

    pub fn get_errors(&mut self) -> &Vec<Error> {
        &self.errors
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            let res = self.scan_token();
            if let Err(e) = res {
                self.add_error(e);
            }
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            Some(Literal::Nil),
            self.line,
        ));
    }

    fn advance(&mut self) -> char {
        let c = self.source.as_bytes()[self.current] as char;
        self.current += 1;
        c
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    pub fn scan_token(&mut self) -> Result<()> {
        let c = self.advance();
        let mut is_ok = Ok(());
        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::SemiColon, None),
            '*' => self.add_token(TokenType::Star, None),

            '!' => {
                let is_next_char_equals = self.is_next_char('=');
                self.add_token(
                    if is_next_char_equals {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    },
                    None,
                )
            }
            '=' => {
                let is_next_char_equals = self.is_next_char('=');
                self.add_token(
                    if is_next_char_equals {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    },
                    None,
                )
            }
            '<' => {
                let is_next_char_equals = self.is_next_char('=');
                self.add_token(
                    if is_next_char_equals {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    },
                    None,
                )
            }
            '>' => {
                let is_next_char_equals = self.is_next_char('=');
                self.add_token(
                    if is_next_char_equals {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    },
                    None,
                )
            }
            '/' => {
                if self.is_next_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.is_next_char('*') {
                    self.advance();
                    let mut depth = 1;
                    while depth > 0 && !self.is_at_end() {
                        if self.peek() == '/' && self.peek_next() == '*' {
                            depth += 1;
                            self.advance();
                            self.advance();
                        }
                        if self.peek() == '*' && self.peek_next() == '/' {
                            depth -= 1;
                            self.advance();
                            self.advance();
                        } else {
                            self.advance();
                        }
                    }

                    if depth > 0 {
                        let e =
                            LexicalError::UnterminatedBlockComment(self.line, self.source.clone());
                        is_ok = Err(e.into());
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            }

            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => self.line += 1,

            '"' => is_ok = self.string(),
            _ if Self::is_digit(c) => is_ok = self.number(),
            _ if Self::is_alpha(c) => self.identifier(),
            _ => return Err(LexicalError::InvalidCharacter(c, self.line, self.current).into()),
        }

        is_ok
    }

    fn is_next_char(&mut self, c: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != c {
            return false;
        }

        self.current += 1;
        true
    }
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source.chars().nth(self.current).unwrap();
    }
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn is_digit(c: char) -> bool {
        return c >= '0' && c <= '9';
    }
    fn is_alpha(c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
    }
    fn is_alpha_numeric(c: char) -> bool {
        Self::is_alpha(c) || Self::is_digit(c)
    }

    fn string(&mut self) -> Result<()> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }
        if self.is_at_end() {
            return Err(LexicalError::UnterminatedString(self.line, self.current).into());
        }

        self.advance();

        let value = &self.source[self.start + 1..self.current];

        self.add_token(TokenType::String, Some(Literal::String(value.to_string())));

        Ok(())
    }
    fn number(&mut self) -> Result<()> {
        while Self::is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && Self::is_digit(self.peek_next()) {
            self.advance();
        }

        while Self::is_digit(self.peek()) {
            self.advance();
        }

        let parsed_number = self.source[self.start..self.current].parse()?;
        self.add_token(TokenType::Number, Some(Literal::Number(parsed_number)));
        Ok(())
    }
    fn identifier(&mut self) {
        while Self::is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];

        let mut token_type = TokenType::Identifier;
        let t = KEYWORDS.get(text);

        if let Some(id) = t {
            token_type = *id;
        }
        self.add_token(token_type, None);
    }
}
