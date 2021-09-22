mod ast;

lalrpop_mod!(calculator_ast, "/calculator_ast/calculator_ast.rs");

#[test]
fn test_calculator_eval() {
    let p = calculator_ast::Expr2Parser::new();
    println!("{:?}", p.parse("1 + (2 * 3 - 4) / 5").unwrap());
    let expr: ast::Expr = *p.parse("1 + (2 * 3 - 4) / 5").unwrap();
    dbg!(&expr);
    assert_eq!(expr.eval(), 1);
}
