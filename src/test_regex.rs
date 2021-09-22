fn test_regex(re: &str, text: &str) {
    assert!(regex::Regex::new(re)
        .unwrap()
        .captures(text)
        .unwrap()
        .get(0)
        .is_some());
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
