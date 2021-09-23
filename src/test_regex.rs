fn test_regex(re: &str, text: &str) {
    let res = regex::Regex::new(re)
    .unwrap()
    .captures(text)
    .unwrap().get(0).unwrap();
    dbg!(res);
}

#[test]
fn test_regex_parse_sql_case_insensitive_keywords() {
    test_regex(r"(?i)select", "select");
    test_regex(r#"(?i)select"#, "SELECT");
    test_regex("(?i)select", "sEleCt");
}

#[test]
fn test_num_parser() {
    // 在 lalrpop 中必须加上正则字符串前缀 r 才能启用正则，在 regex 中并不需要
    test_regex("[0-9]+", "123");
    test_regex("[[0-9]]+", "123");

    test_regex(r"[0-9]+", "123");
    test_regex(r"[[0-9]]+", "123");
}

#[test]
fn test_f64_parser() {
    let re = r"[-+]?([0-9]*[.])?[0-9]+([eE][-+]?\d+)?";
    test_regex(re, "1.6e-9");
    test_regex(re, "1.3E8");
    test_regex(re, "1.3");
    test_regex(re, ".31");
}

#[test]
fn test_string_parser() {
    let re = r#"(?:[^"\\]|\\.)*"#;
    test_regex(re, "apple");
    test_regex(re, "😅");
    test_regex(re, "\u{F602}");
    test_regex(re, "hello \"name\"");
}
