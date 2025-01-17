use crate::token::{Token, TokenType, Literal};

pub trait ExprVisitor {
    type Output;
    fn visit_binary(&mut self, expr: &Binary) -> Self::Output;
    fn visit_unary(&mut self, expr: &Unary) -> Self::Output;
    fn visit_literal(&mut self, expr: &LiteralExpr) -> Self::Output;
    fn visit_grouping(&mut self, expr: &Grouping) -> Self::Output;
}

pub trait VisitableExpr {
    fn accept(&self, visitor: &mut dyn ExprVisitor<Output = String>) -> String;
}

//=== AST Structs ===

pub struct Binary {
    pub left: Box<dyn VisitableExpr>,
    pub operator: Token,
    pub right: Box<dyn VisitableExpr>,
}

impl Binary {
    pub fn new(
        left: Box<dyn VisitableExpr>,
        operator: Token,
        right: Box<dyn VisitableExpr>,
    ) -> Self {
        Self { left, operator, right }
    }
}

impl VisitableExpr for Binary {
    fn accept(&self, visitor: &mut dyn ExprVisitor<Output = String>) -> String {
        visitor.visit_binary(self)
    }
}

//------------------------------------------

pub struct Unary {
    pub operator: Token,
    pub next: Box<dyn VisitableExpr>,
}

impl Unary {
    pub fn new(operator: Token, next: Box<dyn VisitableExpr>) -> Self {
        Self { operator, next }
    }
}

impl VisitableExpr for Unary {
    fn accept(&self, visitor: &mut dyn ExprVisitor<Output = String>) -> String {
        visitor.visit_unary(self)
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

impl VisitableExpr for LiteralExpr {
    fn accept(&self, visitor: &mut dyn ExprVisitor<Output = String>) -> String {
        visitor.visit_literal(self)
    }
}

//------------------------------------------

pub struct Grouping {
    pub expression: Box<dyn VisitableExpr>,
}

impl Grouping {
    pub fn new(expression: Box<dyn VisitableExpr>) -> Self {
        Self { expression }
    }
}

impl VisitableExpr for Grouping {
    fn accept(&self, visitor: &mut dyn ExprVisitor<Output = String>) -> String {
        visitor.visit_grouping(self)
    }
}

//------------------------------------------

pub enum Expr {
    Binary(Box<Binary>),
    Unary(Box<Unary>),
    Literal(LiteralExpr),
    Grouping(Box<Grouping>),
}

// For convenience, implement VisitableExpr on the enum itself:
impl VisitableExpr for Expr {
    fn accept(&self, visitor: &mut dyn ExprVisitor<Output = String>) -> String {
        match self {
            Expr::Binary(b) => b.accept(visitor),
            Expr::Unary(u) => u.accept(visitor),
            Expr::Literal(l) => l.accept(visitor),
            Expr::Grouping(g) => g.accept(visitor),
        }
    }
}


pub struct AstVisitor;

impl ExprVisitor for AstVisitor {
    type Output = String;

    fn visit_binary(&mut self, expr: &Binary) -> String {
        // Recursively visit left & right to build a string
        let left_str = expr.left.accept(self);
        let right_str = expr.right.accept(self);
        let op_str = format!("{:?}", expr.operator.get_token());
        format!("(Bin {} {} {})", op_str, left_str, right_str)
    }

    fn visit_unary(&mut self, expr: &Unary) -> String {
        let next_str = expr.next.accept(self);
        let op_str = format!("{:?}", expr.operator.get_token());
        format!("(Un {} {})", op_str, next_str)
    }

    fn visit_literal(&mut self, expr: &LiteralExpr) -> String {
        match &expr.value {
            Literal::Number(n) => format!("{}", n),
            Literal::String(s) => s.to_string(),
            Literal::Nil => "Nil".to_string(),
        }
    }

    fn visit_grouping(&mut self, expr: &Grouping) -> String {
        let expr_str = expr.expression.accept(self);
        format!("(Grp {})", expr_str)
    }
}
