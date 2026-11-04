// tests/siliconflow_test.rs
use anyhow::Result;

use siliconflow_t2t::utils::setting::{build_params_from_config, load_chat_config_sync};
use siliconflow_t2t::utils::siliconflow::{build_api_url, parse_chat_response, siliconflow_chat};

// ============ 真实 API 调用（需运行目录的 database/info.db 与网络） ============

#[test]
#[ignore = "需要运行目录的 database/info.db 与 SiliconFlow 网络"]
fn test_siliconflow_chat() -> Result<()> {
    let cfg = load_chat_config_sync()?;
    let params = build_params_from_config(&cfg, "你好，请介绍一下 Rust 语言。".to_string());
    let response = siliconflow_chat(&cfg.api_key, &cfg.base_url, &params)?;

    println!("AI 回复：{}", response.content);
    println!("响应 id：{:?}", response.id);
    assert!(!response.content.is_empty(), "AI 返回的内容不能为空");
    assert!(response.id.is_some(), "响应应包含 id");
    assert_eq!(response.model.as_deref(), Some(cfg.model.as_str()));

    Ok(())
}

// ============ URL 拼接（纯函数，不需要网络） ============

#[test]
fn test_build_api_url_with_base() {
    assert_eq!(
        build_api_url("https://api.siliconflow.cn/v1"),
        "https://api.siliconflow.cn/v1/messages"
    );
}

#[test]
fn test_build_api_url_with_trailing_slash() {
    assert_eq!(
        build_api_url("https://api.siliconflow.cn/v1/"),
        "https://api.siliconflow.cn/v1/messages"
    );
}

#[test]
fn test_build_api_url_already_full() {
    assert_eq!(
        build_api_url("https://api.siliconflow.cn/v1/messages"),
        "https://api.siliconflow.cn/v1/messages"
    );
}

#[test]
fn test_build_api_url_empty() {
    assert_eq!(build_api_url(""), "/messages");
}

// ============ 响应解析（纯函数） ============

#[test]
fn test_parse_chat_response_with_id() {
    let body = r#"{
        "id": "msg_123",
        "type": "message",
        "role": "assistant",
        "model": "deepseek-ai/DeepSeek-V4-Flash",
        "content": [
            {"type": "thinking", "thinking": "先想一想", "signature": "s"},
            {"type": "text", "text": "你好"}
        ],
        "stop_reason": "end_turn",
        "stop_sequence": null,
        "usage": {"input_tokens": 6, "output_tokens": 2}
    }"#;

    let result = parse_chat_response(body).expect("解析失败");
    assert_eq!(result.content, "你好");
    assert_eq!(result.id.as_deref(), Some("msg_123"));
    assert_eq!(
        result.model.as_deref(),
        Some("deepseek-ai/DeepSeek-V4-Flash")
    );
    assert_eq!(result.stop_reason.as_deref(), Some("end_turn"));
    assert!(result.usage.is_some());
}

#[test]
fn test_parse_chat_response_without_id() {
    let body = r#"{
        "content": [{"type": "text", "text": "ok"}]
    }"#;

    let result = parse_chat_response(body).expect("解析失败");
    assert_eq!(result.content, "ok");
    assert!(result.id.is_none());
    assert!(result.usage.is_none());
}

#[test]
fn test_parse_chat_response_empty_text_fails() {
    let body = r#"{
        "id": "msg_empty",
        "content": [{"type": "thinking", "thinking": "只有思考"}]
    }"#;

    assert!(parse_chat_response(body).is_err());
}
