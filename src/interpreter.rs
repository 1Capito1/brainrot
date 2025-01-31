#![allow(dead_code)]
#![allow(unused_variables)]
use crate::ast::Expr;
use crate::ast::{Binary, Grouping, LiteralExpr, Unary};
use crate::token::Literal;
use crate::token::TokenType;
use anyhow::Result;

#[derive(thiserror::Error, Debug)]
enum InterpretError {
    #[error("Error, incorrect type passed to expression")]
    IncorrectType,
}

impl Expr {
    pub fn interpret(expr: Self) -> Literal {
        return match expr {
            Expr::Binary(b) => Self::interpret_binary(*b).unwrap(),
            Expr::Unary(u) => Self::interpret_unary(*u).unwrap(),
            Expr::Literal(l) => Self::interpret_literal(l),
            Expr::Grouping(g) => Self::interpret_grouping(*g),
        };
    }

    //----------------------------BINARY EXPRESSIONS---------------------------
    fn interpret_binary(expr: Binary) -> Result<Literal> {
        let token_type = expr.operator.get_type();
        let left_expr = *expr.left;
        let right_expr = *expr.right;

        match token_type {
            TokenType::BangEqual => Self::not_equal(left_expr, right_expr),            
            TokenType::EqualEqual => Self::equal(left_expr, right_expr),            
            TokenType::Equal => Ok(Literal::Number(0.0)),            

            TokenType::Less => Self::less(left_expr, right_expr),            
            TokenType::LessEqual => Self::less_equal(left_expr, right_expr),            

            TokenType::Plus => Self::plus(left_expr, right_expr),            
            TokenType::Minus => Self::minus(left_expr, right_expr),            

            TokenType::Star => Self::mult(left_expr, right_expr),            
            TokenType::Slash => Self::div(left_expr, right_expr),            
            _ => Err(InterpretError::IncorrectType.into())
        }
    }

    fn type_checkable(left: &Literal, right: &Literal) -> bool {
        match (left, right) {
            (Literal::Number(_), Literal::Number(_)) |
            (Literal::String(_), Literal::String(_)) |
            (Literal::Boolean(_), Literal::Boolean(_)) => true,
            (Literal::Nil, Literal::Nil) => true,
            (Literal::Nil, _) |
            (_, Literal::Nil) => false,
            _ => false
        }
    }

    fn plus(left: Expr, right: Expr) -> Result<Literal> {
        let left = Self::interpret(left);
        let right = Self::interpret(right);

        return match (left, right) {
            (Literal::Number(a), Literal::Number(b)) => Ok(Literal::Number(a + b)),
            (Literal::String(a), Literal::String(b)) => Ok(Literal::String(a + &b)),
            (Literal::Number(a), Literal::String(b)) => Ok(Literal::String((a.to_string() + &b).to_string())),
            (Literal::String(a), Literal::Number(b)) => Ok(Literal::String((a + b.to_string().as_str()).to_string())),
            _ => Err(InterpretError::IncorrectType.into())
        }
    }

    fn minus(left: Expr, right: Expr) -> Result<Literal> {
        let left = Self::interpret(left);
        let right = Self::interpret(right);

        return match (left, right) {
            (Literal::Number(a), Literal::Number(b)) => Ok(Literal::Number(a - b)),
            _ => Err(InterpretError::IncorrectType.into())
        }
    }

    fn mult(left: Expr, right: Expr) -> Result<Literal> {
        let left = Self::interpret(left);
        let right = Self::interpret(right);

        return match (left, right) {
            (Literal::Number(a), Literal::Number(b)) => Ok(Literal::Number(a * b)),
            (Literal::String(a), Literal::Number(b)) => Ok(Literal::String(a.repeat(b as usize))),
            (Literal::Number(a), Literal::String(b)) => Ok(Literal::String(b.repeat(a as usize))),
            _ => Err(InterpretError::IncorrectType.into())
        }
    }

    fn div(left: Expr, right: Expr) -> Result<Literal> {
        let left = Self::interpret(left);
        let right = Self::interpret(right);

        return match (left, right) {
            (Literal::Number(a), Literal::Number(b)) => Ok(Literal::Number(a / b)),
            _ => Err(InterpretError::IncorrectType.into())
        }
    }

    fn less(left: Expr, right: Expr) -> Result<Literal> {
        let left = Self::interpret(left);
        let right = Self::interpret(right);
        return match (left, right) {
            (Literal::Number(a), Literal::Number(b)) => Ok(Literal::Boolean(a < b)),
            _ => Err(InterpretError::IncorrectType.into())
        }

    }

    fn less_equal(left: Expr, right: Expr) -> Result<Literal> {
        let left = Self::interpret(left);
        let right = Self::interpret(right);
        return match (left, right) {
            (Literal::Number(a), Literal::Number(b)) => Ok(Literal::Boolean(a <= b)),
            _ => Err(InterpretError::IncorrectType.into())
        }


    }

    fn not_equal(left: Expr, right: Expr) -> Result<Literal> {
        let left = Self::interpret(left);
        let right = Self::interpret(right);
        if Self::type_checkable(&left, &right) {
            // left is always of type right, therefore right doesn't need to be
            // type checked
            let result = left != right;
            return match left {
                Literal::Number(_) => Ok(Literal::Boolean(result)),
                Literal::Boolean(_) => Ok(Literal::Boolean(result)),
                Literal::String(_) => Ok(Literal::Boolean(result)),
                Literal::Nil => Ok(Literal::Boolean(result)),
            }
        };
        return Err(InterpretError::IncorrectType.into());
    }

    fn equal(left: Expr, right: Expr) -> Result<Literal> {
        let left = Self::interpret(left);
        let right = Self::interpret(right);
        if Self::type_checkable(&left, &right) {
            // left is always of type right, therefore right doesn't need to be
            // type checked
            let result = left == right;
            return match left {
                Literal::Number(_) => Ok(Literal::Boolean(result)),
                Literal::Boolean(_) => Ok(Literal::Boolean(result)),
                Literal::String(_) => Ok(Literal::Boolean(result)),
                Literal::Nil => unreachable!(),
            }
        };
        return Err(InterpretError::IncorrectType.into());
    }

    //----------------------------UNARY EXPRESSIONS----------------------------
    fn interpret_unary(expr: Unary) -> Result<Literal> {
        let token_type = expr.operator.get_type();
        let next_expr = *expr.next;
        match token_type {
            TokenType::Bang => Self::not(next_expr),
            TokenType::Minus => Self::negate(next_expr),
            _ => Err(InterpretError::IncorrectType.into())
        }
    }

    fn not(next: Expr) -> Result<Literal> {
        let literal = Self::interpret(next);
        match literal {
            Literal::Boolean(b) => Ok(Literal::Boolean(!b)),
            _ => Err(InterpretError::IncorrectType.into())
        }
    }

    fn negate(next: Expr) -> Result<Literal> {
        let literal = Self::interpret(next);
        match literal {
            Literal::Number(n) => Ok(Literal::Number(-n)),
            _ => Err(InterpretError::IncorrectType.into())
        }
    }

    //---------------------------LITERAL EXPRESSIONS---------------------------
    fn interpret_literal(expr: LiteralExpr) -> Literal {
        expr.value
    }
    //---------------------------GROUPING EXPRESSIONS--------------------------
    fn interpret_grouping(expr: Grouping) -> Literal {
        Self::interpret(*expr.expression)
    }
}
