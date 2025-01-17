mod test {
    use crate::{ast::*, token::*};

    #[test]
    fn ast_pretty_printer() {
        // Build:  Binary( (Unary(Minus, "4")), Token::new(...), Grouping(45) )
        // e.g. something that looks like:  -4 + (45)
        // But you can interpret your own desired structure.

        let minus_token = Token::new(
            TokenType::Minus,
            "-".to_string(),
            None,
            1,
        );
        let number_token = Token::new(
            TokenType::Number,
            "4".to_string(),
            Some(Literal::Number(4.0)),
            1,
        );

        // Build a unary node: - 4
        let unary_expr = Expr::Unary(Box::new(
            Unary::new(
                minus_token,
                Box::new(Expr::Literal(LiteralExpr::new(Literal::Number(4.0)))),
            )
        ));

        // Build a grouping node: (45)
        let group_expr = Expr::Grouping(Box::new(
            Grouping::new(
                Box::new(Expr::Literal(LiteralExpr::new(Literal::Number(45.0)))),
            )
        ));

        // Build a "Binary" node combining them:
        // For example, let's interpret it as: (-4) [operator= number_token? not typical, but let's do it] group_expr
        // In a real interpreter, you'd typically have the operator be something like "+" token. 
        let binary_expr = Expr::Binary(Box::new(
            Binary::new(
                Box::new(unary_expr),
                number_token,
                Box::new(group_expr),
            )
        ));

        // Now let's pretty-print it:
        let mut visitor = AstVisitor;
        let pretty_result = binary_expr.accept(&mut visitor);

        println!("Pretty result: {}", pretty_result);

        // Optionally, we can assert some string if we want:
        // assert_eq!("(Bin Number (Un Minus 4) (Grp 45))", pretty_result);
        assert_eq!(pretty_result, "(Bin Number (Un Minus 4) (Grp 45))")
    }
}
