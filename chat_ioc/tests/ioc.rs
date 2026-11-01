// tests/ioc.rs
use chat_ioc::chat::ffi::extract;

#[test]
fn extracts_common_iocs() {
    let text = "访问 https://evil.example.com/login，C2 为 10.0.0.5，\
                样本哈希 d41d8cd98f00b204e9800998ecf8427e，备用域 evil.test";
    let iocs = extract(text);
    assert_eq!(iocs["urls"][0], "https://evil.example.com/login");
    assert!(iocs["ips"].as_array().unwrap().contains(&serde_json::json!("10.0.0.5")));
    assert!(iocs["hashes"]
        .as_array()
        .unwrap()
        .contains(&serde_json::json!("d41d8cd98f00b204e9800998ecf8427e")));
    assert!(iocs["domains"]
        .as_array()
        .unwrap()
        .contains(&serde_json::json!("evil.test")));
}

#[test]
fn ignores_plain_text() {
    let iocs = extract("这是一段没有 IOC 的普通对话。");
    assert_eq!(iocs["count"], 0);
}
