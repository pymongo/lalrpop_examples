lalrpop_mod!(json_grammar, "/json/json_grammar.rs");

mod ast;

#[test]
fn test_parse_json() {
    let p = json_grammar::JsonRootParser::new();
    dbg!(p.parse("null").unwrap());

    // bug: should parse error
    dbg!(p.parse("[1]").unwrap());
    // dbg!(p.parse(r#"{"key": 1,}"#).unwrap());
}
