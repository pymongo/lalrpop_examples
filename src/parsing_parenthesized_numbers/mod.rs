#[test]
fn feature() {
    lalrpop_mod!(parser, "/parsing_parenthesized_numbers/parser.rs");

    let p = parser::NumParser::new();
    dbg!(p.parse("12313").unwrap());
    assert!(p.parse("rust").is_err());
}
