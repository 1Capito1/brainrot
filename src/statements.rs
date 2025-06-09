use crate::ast::Expr;
use crate::token::Token;

pub enum Statement {
    Expression(Expr),
    Print(Expr),
    Var(Token, Expr)
}
