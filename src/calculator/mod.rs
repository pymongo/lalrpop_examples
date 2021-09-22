lalrpop_mod!(calculator, "/calculator/calculator.rs");

#[test]
fn test_expr_precedence1() {
    let p = calculator::ExprPrecedence1Parser::new();
    dbg!(p.parse("1 + (2 * 3 - 4) / 5").unwrap());
}
