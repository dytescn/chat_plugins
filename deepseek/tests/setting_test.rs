#[test]
fn test_build_chat_config_full() {
    let mut m = HashMap::new();
    m.insert("silicon_app_key".to_string(), "sk-test".to_string());
    m.insert("silicon_app_system_prompt".to_string(), "你是助手".to_string());
    // ...

    let cfg = build_chat_config(&m).unwrap();
    assert_eq!(cfg.system_prompt, Some("你是助手".to_string()));
}

#[test]
fn test_build_params_with_system() {
    let cfg = ChatConfig {
        api_key: "k".into(),
        base_url: "u".into(),
        model: "m".into(),
        max_tokens: 100,
        temperature: 0.5,
        top_p: 0.9,
        top_k: 10,
        system_prompt: Some("你是助手".into()),
    };

    let p = build_params_from_config(&cfg, "你好".into());
    assert_eq!(p.messages.len(), 2);
    assert_eq!(p.messages[0].role, "system");
    assert_eq!(p.messages[1].role, "user");
}

#[test]
fn test_build_params_without_system() {
    let cfg = ChatConfig {
        api_key: "k".into(),
        base_url: "u".into(),
        model: "m".into(),
        max_tokens: 100,
        temperature: 0.5,
        top_p: 0.9,
        top_k: 10,
        system_prompt: None,
    };

    let p = build_params_from_config(&cfg, "你好".into());
    assert_eq!(p.messages.len(), 1);
    assert_eq!(p.messages[0].role, "user");
}
