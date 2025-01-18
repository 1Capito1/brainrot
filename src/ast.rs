#![allow(dead_code)]
use crate::token::{Literal, Token};

//=== AST Structs ===

pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Binary {
    pub fn new(left: Box<Expr>, operator: Token, right: Box<Expr>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }

    pub fn make_expr(left: Expr, operator: Token, right: Expr) -> Expr {
        Expr::Binary(Box::new(Self::new(
            Box::new(left),
            operator,
            Box::new(right),
        )))
    }
}

impl ToString for Binary {
    fn to_string(&self) -> String {
        format!("({} {} {})", self.operator, self.left, self.right)
    }
}

//------------------------------------------

pub struct Unary {
    pub operator: Token,
    pub next: Box<Expr>,
}

impl Unary {
    pub fn new(operator: Token, next: Box<Expr>) -> Self {
        Self { operator, next }
    }

    pub fn make_expr(operator: Token, next: Expr) -> Expr {
        Expr::Unary(Box::new(Unary::new(operator, Box::new(next))))
    }
}

impl ToString for Unary {
    fn to_string(&self) -> String {
        format!("({} {})", self.operator, self.next)
    }
}

//------------------------------------------

pub struct LiteralExpr {
    pub value: Literal,
}

impl LiteralExpr {
    pub fn new(value: Literal) -> Self {
        Self { value }
    }
    pub fn make_expr(value: Literal) -> Expr {
        Expr::Literal(Self::new(value))
    }
}

impl ToString for LiteralExpr {
    fn to_string(&self) -> String {
        let fm = match &self.value {
            Literal::Number(x) => x.to_string(),
            Literal::String(s) => s.to_string(),
            Literal::Boolean(b) => b.to_string(),
            Literal::Nil => "Nil".to_string(),
        };

        format!("({fm})")
    }
}

//------------------------------------------

pub struct Grouping {
    pub expression: Box<Expr>,
}

impl Grouping {
    pub fn new(expression: Box<Expr>) -> Self {
        Self { expression }
    }

    pub fn make_expr(expression: Expr) -> Expr {
        Expr::Grouping(Box::new(Self::new(Box::new(expression))))
    }
}

impl ToString for Grouping {
    fn to_string(&self) -> String {
        format!("(group {})", self.expression.pretty_print())
    }
}

//------------------------------------------

pub enum Expr {
    Binary(Box<Binary>),
    Unary(Box<Unary>),
    Literal(LiteralExpr),
    Grouping(Box<Grouping>),
}

impl Expr {
    pub fn pretty_print(&self) -> String {
        match self {
            Expr::Binary(b) => b.to_string(),
            Expr::Unary(u) => u.to_string(),
            Expr::Literal(l) => l.to_string(),
            Expr::Grouping(g) => g.to_string(),
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pretty_print())
    }
}
