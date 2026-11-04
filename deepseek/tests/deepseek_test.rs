// tests/deepseek_test.rs —— DeepSeek URL 拼接与响应解析测试
use anyhow::Result;

use deepseek::utils::deepseek::{build_api_url, parse_chat_response};

// ============ URL 拼接（纯函数，不需要网络） ============

#[test]
fn test_build_api_url_with_base() {
    assert_eq!(
        build_api_url("https://api.deepseek.com/anthropic"),
        "https://api.deepseek.com/anthropic/v1/messages"
    );
}

#[test]
fn test_build_api_url_with_trailing_slash() {
    assert_eq!(
        build_api_url("https://api.deepseek.com/anthropic/"),
        "https://api.deepseek.com/anthropic/v1/messages"
    );
}

#[test]
fn test_build_api_url_already_full() {
    assert_eq!(
        build_api_url("https://api.deepseek.com/anthropic/v1/messages"),
        "https://api.deepseek.com/anthropic/v1/messages"
    );
}

#[test]
fn test_build_api_url_short() {
    assert_eq!(
        build_api_url("https://api.deepseek.com/anthropic/messages"),
        "https://api.deepseek.com/anthropic/messages"
    );
}

// ============ 响应解析（纯函数） ============

#[test]
fn test_parse_chat_response_with_metadata() -> Result<()> {
    let body = r#"{
        "id": "msg_123",
        "type": "message",
        "role": "assistant",
        "model": "deepseek-flash",
        "content": [
            {"type": "thinking", "thinking": "先想一想"},
            {"type": "text", "text": "你好"}
        ],
        "stop_reason": "end_turn",
        "usage": {"input_tokens": 6, "output_tokens": 2}
    }"#;

    let result = parse_chat_response(body)?;
    assert_eq!(result.content, "你好");
    assert_eq!(result.id.as_deref(), Some("msg_123"));
    assert_eq!(result.model.as_deref(), Some("deepseek-flash"));
    assert_eq!(result.stop_reason.as_deref(), Some("end_turn"));
    assert!(result.usage.is_some());
    Ok(())
}

#[test]
fn test_parse_chat_response_without_metadata() -> Result<()> {
    let body = r#"{ "content": [{"type": "text", "text": "ok"}] }"#;
    let result = parse_chat_response(body)?;
    assert_eq!(result.content, "ok");
    assert!(result.id.is_none());
    assert!(result.model.is_none());
    assert!(result.usage.is_none());
    Ok(())
}

#[test]
fn test_parse_chat_response_empty_text_fails() {
    let body = r#"{
        "id": "msg_empty",
        "content": [{"type": "thinking", "thinking": "只有思考"}]
    }"#;
    assert!(parse_chat_response(body).is_err());
}
