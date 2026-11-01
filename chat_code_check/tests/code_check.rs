// tests/code_check.rs
use chat_code_check::chat::ffi::check;

#[test]
fn finds_common_issues() {
    let code = "fn main() {\n    // TODO: 补充逻辑\n    println!(\"debug\");\n    let secret = \"abc\";\n}\n";
    let issues = check(code);
    let kinds: Vec<&str> = issues
        .iter()
        .filter_map(|issue| issue["kind"].as_str())
        .collect();
    assert!(kinds.contains(&"todo"));
    assert!(kinds.contains(&"debug"));
    assert!(kinds.contains(&"secret"));
}

#[test]
fn clean_code_has_no_issues() {
    assert!(check("let answer = 42;\n").is_empty());
}
