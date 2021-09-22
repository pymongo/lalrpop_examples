lalrpop_mod!(calculator, "/calculator/calculator.rs");

fn test_regex(re: &str, text: &str) {
    assert!(regex::Regex::new(re)
        .unwrap()
        .captures(text)
        .unwrap()
        .get(0)
        .is_some());
}

#[test]
fn test_num_parser() {
    let p = calculator::NumParser::new();
    dbg!(p.parse("123").unwrap());

    test_regex("[0-9]+", "123");
    test_regex("[[0-9]]+", "123");
    test_regex(r"[0-9]+", "123");
    test_regex(r"[[0-9]]+", "123");
}

#[test]
fn test_expr_precedence1() {
    let p = calculator::ExprPrecedence1Parser::new();
    dbg!(p.parse("1 + (2 * 3 - 4) / 5").unwrap());
}
