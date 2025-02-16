#![allow(dead_code)]
#![allow(unused_variables)]
use crate::ast::Expr;
use crate::ast::{Binary, Grouping, LiteralExpr, Unary};
use crate::token::Literal;
use crate::token::TokenType;
use crate::statements::Statement;
use anyhow::Result;
use anyhow::Error;

#[derive(thiserror::Error, Debug)]
enum InterpretError {
    #[error("Error, incorrect type passed to expression")]
    IncorrectType,
}

// handles the interpretation of Expr's, it does not hold the expression state
// only the error state
pub struct Interpreter {
    pub errors: Vec<Error>
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            errors: vec![]
        }
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) -> Result<()> {
        for statement in statements {
            self.execute(statement)?;
        }
        Ok(())
    }

    pub fn interpret_expression(&mut self, expr: Expr) -> Result<Literal> {
        self.evaluate(expr)
    }

    fn execute(&mut self, statement: Statement) -> Result<()> {
        match statement {
            Statement::Print(e) => {
                let literal = self.evaluate(e)?;
                println!("{}", literal.to_string());
            },
            Statement::Expression(e) => {
                let literal = self.evaluate(e)?;
            },
        }
        Ok(())
    }

    fn evaluate(&mut self, expr: Expr) -> Result<Literal> {
        match expr {
            Expr::Binary(b) => Ok(self.interpret_binary(*b)?),
            Expr::Unary(u) => Ok(self.interpret_unary(*u)?),
            Expr::Literal(l) => Ok(self.interpret_literal(l)),
            Expr::Grouping(g) => Ok(self.interpret_grouping(*g)),
        }
    }

    //----------------------------BINARY EXPRESSIONS---------------------------
    fn interpret_binary(&mut self, expr: Binary) -> Result<Literal> {
        let token_type = expr.operator.get_type();
        let left_expr = *expr.left;
        let right_expr = *expr.right;

        match token_type {
            TokenType::BangEqual => self.not_equal(left_expr, right_expr),            
            TokenType::EqualEqual => self.equal(left_expr, right_expr),            
            TokenType::Equal => Ok(Literal::Number(0.0)),            

            TokenType::Less => self.less(left_expr, right_expr),            
            TokenType::LessEqual => self.less_equal(left_expr, right_expr),            

            TokenType::Plus => self.plus(left_expr, right_expr),            
            TokenType::Minus => self.minus(left_expr, right_expr),            

            TokenType::Star => self.mult(left_expr, right_expr),            
            TokenType::Slash => self.div(left_expr, right_expr),            
            _ => Err(InterpretError::IncorrectType.into())
        }
    }

    fn type_checkable(&mut self, left: &Literal, right: &Literal) -> bool {
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

    fn plus(&mut self, left: Expr, right: Expr) -> Result<Literal> {
        let left = self.interpret_expression(left)?;
        let right = self.interpret_expression(right)?;

        return match (left, right) {
            (Literal::Number(a), Literal::Number(b)) => Ok(Literal::Number(a + b)),
            (Literal::String(a), Literal::String(b)) => Ok(Literal::String(a + &b)),
            (Literal::Number(a), Literal::String(b)) => Ok(Literal::String(a.to_string() + &b)),
            (Literal::String(a), Literal::Number(b)) => Ok(Literal::String(a + &b.to_string())),
            _ => Err(InterpretError::IncorrectType.into())
        }
    }

    fn minus(&mut self, left: Expr, right: Expr) -> Result<Literal> {
        let left = self.interpret_expression(left)?;
        let right = self.interpret_expression(right)?;

        return match (left, right) {
            (Literal::Number(a), Literal::Number(b)) => Ok(Literal::Number(a - b)),
            _ => Err(InterpretError::IncorrectType.into())
        }
    }

    fn mult(&mut self, left: Expr, right: Expr) -> Result<Literal> {
        let left = self.interpret_expression(left)?;
        let right = self.interpret_expression(right)?;

        return match (left, right) {
            (Literal::Number(a), Literal::Number(b)) => Ok(Literal::Number(a * b)),
            (Literal::String(a), Literal::Number(b)) => Ok(Literal::String(a.repeat(b as usize))),
            (Literal::Number(a), Literal::String(b)) => Ok(Literal::String(b.repeat(a as usize))),
            _ => Err(InterpretError::IncorrectType.into())
        }
    }

    fn div(&mut self, left: Expr, right: Expr) -> Result<Literal> {
        let left = self.interpret_expression(left)?;
        let right = self.interpret_expression(right)?;

        return match (left, right) {
            (Literal::Number(a), Literal::Number(b)) => Ok(Literal::Number(a / b)),
            _ => Err(InterpretError::IncorrectType.into())
        }
    }

    fn less(&mut self, left: Expr, right: Expr) -> Result<Literal> {
        let left = self.interpret_expression(left)?;
        let right = self.interpret_expression(right)?;
        return match (left, right) {
            (Literal::Number(a), Literal::Number(b)) => Ok(Literal::Boolean(a < b)),
            _ => Err(InterpretError::IncorrectType.into())
        }

    }

    fn less_equal(&mut self, left: Expr, right: Expr) -> Result<Literal> {
        let left = self.interpret_expression(left)?;
        let right = self.interpret_expression(right)?;
        return match (left, right) {
            (Literal::Number(a), Literal::Number(b)) => Ok(Literal::Boolean(a <= b)),
            _ => Err(InterpretError::IncorrectType.into())
        }


    }

    fn not_equal(&mut self, left: Expr, right: Expr) -> Result<Literal> {
        let left = self.interpret_expression(left)?;
        let right = self.interpret_expression(right)?;
        if self.type_checkable(&left, &right) {
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

    fn equal(&mut self, left: Expr, right: Expr) -> Result<Literal> {
        let left = self.interpret_expression(left)?;
        let right = self.interpret_expression(right)?;
        if self.type_checkable(&left, &right) {
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
    fn interpret_unary(&mut self, expr: Unary) -> Result<Literal> {
        let token_type = expr.operator.get_type();
        let next_expr = *expr.next;
        match token_type {
            TokenType::Bang => self.not(next_expr),
            TokenType::Minus => self.negate(next_expr),
            _ => Err(InterpretError::IncorrectType.into())
        }
    }

    fn not(&mut self, next: Expr) -> Result<Literal> {
        let literal = self.interpret_expression(next)?;
        match literal {
            Literal::Boolean(b) => Ok(Literal::Boolean(!b)),
            _ => Err(InterpretError::IncorrectType.into())
        }
    }

    fn negate(&mut self, next: Expr) -> Result<Literal> {
        let literal = self.interpret_expression(next)?;
        match literal {
            Literal::Number(n) => Ok(Literal::Number(-n)),
            _ => Err(InterpretError::IncorrectType.into())
        }
    }

    //---------------------------LITERAL EXPRESSIONS---------------------------
    fn interpret_literal(&mut self, expr: LiteralExpr) -> Literal {
        expr.value
    }
    //---------------------------GROUPING EXPRESSIONS--------------------------
    fn interpret_grouping(&mut self, expr: Grouping) -> Literal {
        self.interpret_expression(*expr.expression).unwrap()
    }
}
