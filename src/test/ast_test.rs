mod test {
    #[allow(unused_imports)] // for some reason rust thinks i don't need these
    use crate::{ast::*, token::*};
    #[test]
    fn ast_pretty_printer() {
        let unary_t = Unary::new(
            Token::new(TokenType::Minus, "-".to_string(), None, 1),
            Box::new(Expr::Literal(LiteralExpr {
                value: Literal::Number(123.0),
            })),
        );
        let group_t = Grouping::new(Box::new(Expr::Literal(LiteralExpr {
            value: Literal::Number(45.67),
        })));
        let head = Binary::new(
            Box::new(Expr::Unary(Box::new(unary_t))),
            Token::new(TokenType::Star, "*".to_string(), None, 1),
            Box::new(Expr::Grouping(Box::new(group_t))),
        );
        let pretty_result = Expr::Binary(Box::new(head)).pretty_print();

        println!("Pretty result: {}", pretty_result);

        // Optionally, we can assert some string if we want:
        // assert_eq!("(Bin Number (Un Minus 4) (Grp 45))", pretty_result);
        assert_eq!(pretty_result, "(* (- (123)) (group (123)))")
    }
}
