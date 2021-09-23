lalrpop_mod!(json_grammar, "/json/json_grammar.rs");

mod ast;

#[test]
fn test_parse_json() {
    let p = json_grammar::tempParser::new();
    dbg!(p.parse("true1").unwrap());
    dbg!(p.parse("true").unwrap());
}
