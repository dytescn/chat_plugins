// tests/setting_test.rs —— DeepSeek 配置与消息组装测试
use std::collections::HashMap;

use deepseek::utils::setting::{build_chat_config, build_params_from_config, ChatConfig};

#[test]
fn test_build_chat_config_full() {
    let mut m = HashMap::new();
    m.insert("deepseek_app_key".to_string(), "sk-test".to_string());
    m.insert(
        "deepseek_app_system_prompt".to_string(),
        "你是助手".to_string(),
    );
    m.insert(
        "deepseek_app_model".to_string(),
        "deepseek-flash".to_string(),
    );

    let cfg = build_chat_config(&m).expect("配置解析失败");
    assert_eq!(cfg.api_key, "sk-test");
    assert_eq!(cfg.system_prompt, Some("你是助手".to_string()));
    assert_eq!(cfg.model, "deepseek-flash");
}

#[test]
fn test_build_chat_config_requires_key() {
    let m = HashMap::new();
    assert!(build_chat_config(&m).is_err());
}

#[test]
fn test_build_params_with_system() {
    let cfg = ChatConfig {
        api_key: "k".into(),
        base_url: "u".into(),
        model: "m".into(),
        max_tokens: 100,
        temperature: 0.5,
        system_prompt: Some("你是助手".into()),
        thinking: deepseek::utils::deepseek::Thinking::disabled(),
        reasoning_effort: None,
    };

    let p = build_params_from_config(&cfg, "你好".into(), cfg.system_prompt.clone());
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
        system_prompt: None,
        thinking: deepseek::utils::deepseek::Thinking::disabled(),
        reasoning_effort: None,
    };

    let p = build_params_from_config(&cfg, "你好".into(), None);
    assert_eq!(p.messages.len(), 1);
    assert_eq!(p.messages[0].role, "user");
}
