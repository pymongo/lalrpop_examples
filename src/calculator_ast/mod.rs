mod ast;

lalrpop_mod!(calculator_ast, "/calculator_ast/calculator_ast.rs");

#[test]
fn feature() {
    let p = calculator_ast::Expr2Parser::new();
    dbg!(p.parse("1 + (2 * 3 - 4) / 5").unwrap());
}
