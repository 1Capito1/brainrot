#![allow(dead_code)]
use crate::token::*;
use crate::ast::*;
use anyhow::Result;

// TODO:
#[derive(thiserror::Error, Debug)]
pub enum ParserError {
     
}
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
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
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0
        }
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

        while self.match_tokens(Self::FACTOR_TOKENS) {
            let operator = self.previous();
            let right = self.unary();
            expr = Binary::make_expr(expr, operator, right);
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_tokens(Self::UNARY_TOKENS) {
            let operator = self.previous();
            let right = self.primary();
            return Unary::make_expr(operator, right);
        }

        return self.primary()
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
            return LiteralExpr::make_expr(
                self.previous().get_literal().clone()
            )
        }

        if self.match_tokens(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ) after expression").unwrap();
            return Grouping::make_expr(expr);
        }
        unreachable!("Temporary, assume perfect code");
    }

    // TODO:
    fn consume(&mut self, token_type: TokenType, err_str: &str) -> Result<Token> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        todo!();
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
        if self.is_at_end() { return false }
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
        return self.tokens[self.current - 1].clone()
    }
    fn peek(&self) -> &Token {
        return &self.tokens[self.current]
    }
}
