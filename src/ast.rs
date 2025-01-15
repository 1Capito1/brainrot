#![allow(dead_code)]
use crate::token::Token;
use crate::token::Literal;

#[derive(Debug)]
pub enum ExprTrees {
    Binary(Box<Binary>),
    Unary(Box<Unary>),
    Literal(Literal),
    Grouping(Box<Grouping>),
}

pub trait ExprVisitor {
    type Output;
    fn visit_binary(&mut self, expr: &Binary) -> Self::Output;
    fn visit_literal(&mut self, expr: &LiteralExpr) -> Self::Output;
    fn visit_unary(&mut self, expr: &Unary) -> Self::Output;
}

pub trait Expr {
    fn accept<T: ExprVisitor>(&self, visitor: &mut dyn ExprVisitor) -> T;
}

// Macro to define nodes
macro_rules! define_ast {
    ($($name:ident { $($field_name:ident : $field_type:ty),* $(,)? }),* $(,)?) => {
        $(
            #[derive(Debug)]
            pub struct $name {
                $(pub $field_name: $field_type,)*
            }

            impl $name {
                pub fn new($($field_name: $field_type),*) -> Self {
                    Self { $($field_name,)* }
                }
            }
        )*
    };
}

// Define the AST nodes
define_ast!(
    Binary {
        left: Box<dyn Expr>,
        operator: Token,
        right: Box<dyn Expr>,
    },
    Unary {
        operator: Token,
        right: Box<dyn Expr>,
    },
    LiteralExpr {
        value: Literal,
    },
    Grouping {
        expression: Box<dyn Expr>,
    },
);

impl Expr for Binary {
    fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        visitor.visit_binary(self)
    }
}
impl Expr for Unary {
    fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        visitor.visit_unary(self)
    }
}
impl Expr for LiteralExpr {
    fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        visitor.visit_literal(self)
    }
}

//struct PrintVisitor;
//
//impl ExprVisitor<String> for PrintVisitor {
//    fn visit_binary(&mut self, expr: &Binary) -> String {
//        format!(
//            "({} {} {})",
//            expr.operator,
//            expr.left.accept(self),
//            expr.right.accept(self)
//            )
//    }
//}
