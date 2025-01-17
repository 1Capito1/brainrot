#![allow(dead_code)]
use crate::token::{Token, Literal};


//=== AST Structs ===

pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Binary {
    pub fn new(
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    ) -> Self {
        Self { left, operator, right }
    }
}

impl ToString for Binary {
    fn to_string(&self) -> String {
        format!("({} {} {})",
            self.operator,
            self.left,
            self.right)
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
}

impl ToString for LiteralExpr {
    fn to_string(&self) -> String {
        let fm = match &self.value {
            Literal::Number(x) => x.to_string(),
            Literal::String(s) => s.to_string(),
            Literal::Nil => "Nil".to_string()
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
