// tests/siliconflow_test.rs
use anyhow::Result;

use siliconflow_t2t::utils::setting::load_chat_config_sync;
use siliconflow_t2t::utils::siliconflow::{build_api_url, siliconflow_chat};

// ============ 真实 API 调用（需数据库已配置 key） ============

#[test]   // ← 不再是 #[tokio::test]
fn test_siliconflow_chat() -> Result<()> {
    let cfg = load_chat_config_sync("你好，请介绍一下 Rust 语言。".to_string())?;
    let response = siliconflow_chat(&cfg.api_key, &cfg.base_url, &cfg.params)?;

    println!("AI 回复：{}", response);
    assert!(!response.is_empty(), "AI 返回的内容不能为空");

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

