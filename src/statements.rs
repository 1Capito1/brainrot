use crate::ast::Expr;

pub enum Statement {
    Expression(Expr),
    Print(Expr),
}
