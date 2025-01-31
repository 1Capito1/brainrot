#![allow(dead_code)]
use crate::ast::*;
use crate::token::*;
use anyhow::Error;
use anyhow::Result;

// TODO:
#[derive(thiserror::Error, Debug)]
pub enum ParserError {
    #[error("[{0}]: {1}")]
    GenericMessage(usize, String),
    #[error("[{0}]: Invalid Syntax")]
    InvalidSyntax(usize),
}
#[derive(Default)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    errors: Vec<Error>,
}

impl Parser {
    const EQUALITY_TOKENS: &[TokenType] = &[TokenType::BangEqual, TokenType::EqualEqual];
    const COMPARISON_TOKENS: &[TokenType] = &[
        TokenType::Greater,
        TokenType::GreaterEqual,
        TokenType::Less,
        TokenType::LessEqual,
    ];
    const TERM_TOKENS: &[TokenType] = &[TokenType::Plus, TokenType::Minus];
    const FACTOR_TOKENS: &[TokenType] = &[TokenType::Star, TokenType::Slash];
    const UNARY_TOKENS: &[TokenType] = &[TokenType::Bang, TokenType::Minus];
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            errors: Vec::new(),
        }
    }

    pub fn get_errors(&self) -> &Vec<Error> {
        &self.errors
    }

    pub fn parse(&mut self) -> Expr {
        self.expression()
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        //
        // != ==
        while self.match_tokens(Self::EQUALITY_TOKENS) {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Binary::make_expr(expr, operator.clone(), right);
        }
        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        // > >= < <=
        while self.match_tokens(Self::COMPARISON_TOKENS) {
            let operator = self.previous();
            let right = self.term();
            expr = Binary::make_expr(expr, operator, right);
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        // + -
        while self.match_tokens(Self::TERM_TOKENS) {
            let operator = self.previous();
            let right = self.factor();
            expr = Binary::make_expr(expr, operator, right)
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        // * /
        while self.match_tokens(Self::FACTOR_TOKENS) {
            let operator = self.previous();
            let right = self.unary();
            expr = Binary::make_expr(expr, operator, right);
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        // ! -
        if self.match_tokens(Self::UNARY_TOKENS) {
            let operator = self.previous();
            let right = self.primary();
            return Unary::make_expr(operator, right);
        }

        return self.primary();
    }

    fn primary(&mut self) -> Expr {
        if self.match_tokens(&[TokenType::False]) {
            return LiteralExpr::make_expr(Literal::Boolean(false));
        }
        if self.match_tokens(&[TokenType::True]) {
            return LiteralExpr::make_expr(Literal::Boolean(true));
        }
        if self.match_tokens(&[TokenType::Nil]) {
            return LiteralExpr::make_expr(Literal::Nil);
        }

        if self.match_tokens(&[TokenType::Number, TokenType::String]) {
            return LiteralExpr::make_expr(self.previous().get_literal().clone());
        }

        if self.match_tokens(&[TokenType::LeftParen]) {
            let expr = self.expression();
            let res = self.consume(TokenType::RightParen, "Expect ) after expression");
            if let Err(e) = res {
                self.errors.push(e.into());
            }
            return Grouping::make_expr(expr);
        }
        let get_token_loc = self.peek().clone().get_line();
        let error = ParserError::InvalidSyntax(get_token_loc);
        self.errors.push(error.into());

        LiteralExpr::make_expr(Literal::Nil)
    }

    // TODO:
    fn consume(&mut self, token_type: TokenType, err_str: &str) -> Result<Token, ParserError> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        let get_token_loc = self.peek().clone().get_line();
        Err(ParserError::GenericMessage(get_token_loc, err_str.to_string()).into())
    }

    fn match_tokens(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(*token_type) {
                self.advance();
                return true;
            }
        }
        false
    }
    fn check(&self, t: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return *self.peek().get_type() == t;
    }
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        return *self.peek().get_type() == TokenType::EOF;
    }
    fn previous(&self) -> Token {
        return self.tokens[self.current - 1].clone();
    }
    fn peek(&self) -> &Token {
        return &self.tokens[self.current];
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            match self.peek().get_type() {
                TokenType::Class => return,
                TokenType::Fun => return,
                TokenType::Var => return,
                TokenType::For => return,
                TokenType::If => return,
                TokenType::While => return,
                TokenType::Print => return,
                TokenType::Return => return,
                _ => (),
            }
            self.advance();
        }
    }
}
