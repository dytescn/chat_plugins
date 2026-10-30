// src/utils/deepseek.rs
// DeepSeek API 调用（Anthropic 兼容格式）
// 文档: https://api-docs.deepseek.com

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::Duration;

// ============ 消息 ============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,       // "system" | "user" | "assistant"
    pub content: String,
}

impl Message {
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".into(),
            content: content.into(),
        }
    }
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".into(),
            content: content.into(),
        }
    }
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: "assistant".into(),
            content: content.into(),
        }
    }
}

// ============ 思考模式 ============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thinking {
    #[serde(rename = "type")]
    pub kind: String,       // "enabled" | "disabled"
}

impl Thinking {
    pub fn enabled() -> Self {
        Self {
            kind: "enabled".into(),
        }
    }
    pub fn disabled() -> Self {
        Self {
            kind: "disabled".into(),
        }
    }
}

// ============ 请求参数 ============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatParams {
    pub model: String,
    pub messages: Vec<Message>,
    pub max_tokens: u32,
    pub temperature: f64,
    pub stream: bool,
    /// 停止序列，最多 4 个
    pub stop_sequences: Option<Vec<String>>,
    pub thinking: Thinking,
    /// 仅支持 "high" | "max"
    pub reasoning_effort: Option<String>,
}

// ============ URL 拼接 ============

/// DeepSeek Anthropic base_url 默认 https://api.deepseek.com/anthropic
/// 最终请求: {base_url}/v1/messages
pub fn build_api_url(base_url: &str) -> String {
    if base_url.ends_with("/v1/messages") || base_url.ends_with("/messages") {
        base_url.to_string()
    } else {
        format!("{}/v1/messages", base_url.trim_end_matches('/'))
    }
}

// ============ 调用 ============

/// 调用 DeepSeek Chat API（Anthropic 兼容格式）
pub fn deepseek_chat(
    api_key: &str,
    base_url: &str,
    params: &ChatParams,
) -> Result<String> {
    if api_key.is_empty() {
        bail!("api_key 为空");
    }
    if params.model.is_empty() {
        bail!("model 为空");
    }
    if params.messages.is_empty() {
        bail!("messages 为空");
    }

    let api_url = build_api_url(base_url);

    // ============ 组装请求体 ============
    let mut request_body = json!({
        "model": params.model,
        "messages": params.messages,
        "max_tokens": params.max_tokens,
        "temperature": params.temperature,
        "stream": params.stream,
        "thinking": { "type": params.thinking.kind }
    });

    // 可选：stop_sequences
    if let Some(stops) = &params.stop_sequences {
        if !stops.is_empty() {
            request_body["stop_sequences"] = json!(stops);
        }
    }

    // 可选：reasoning_effort（放进 output_config）
    if let Some(effort) = &params.reasoning_effort {
        if !effort.is_empty() {
            request_body["output_config"] = json!({ "effort": effort });
        }
    }

    // ============ 发送请求 ============
    let resp = ureq::post(&api_url)
        .set("x-api-key", api_key)
        .set("anthropic-version", "2023-06-01")
        .set("Content-Type", "application/json")
        .timeout(Duration::from_secs(120))
        .send_json(&request_body);

    let response = match resp {
        Ok(r) => r,
        Err(ureq::Error::Status(code, r)) => {
            let body = r.into_string().unwrap_or_default();
            bail!("API 请求失败\n状态码: {}\n响应: {}", code, body);
        }
        Err(e) => {
            bail!("API 请求发送失败: {}", e);
        }
    };

    let body = response.into_string().context("读取 API 响应失败")?;
    let result: Value = serde_json::from_str(&body).context("JSON 解析失败")?;

    // ============ 解析响应（Anthropic 格式） ============
    // {
    //   "content": [
    //     { "type": "text", "text": "..." }
    //   ]
    // }
    let content = result
        .get("content")
        .and_then(|c| c.as_array())
        .context("API 响应中缺少 content 数组")?;

    let mut output = String::new();
    for item in content {
        if item["type"].as_str() == Some("text") {
            if let Some(text) = item["text"].as_str() {
                output.push_str(text);
            }
        }
    }

    if output.is_empty() {
        bail!("模型未返回文本内容: {}", serde_json::to_string(&result)?);
    }

    Ok(output)
}

// ============ 测试 ============

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_message_helpers() {
        let s = Message::system("sys");
        assert_eq!(s.role, "system");
        assert_eq!(s.content, "sys");

        let u = Message::user("hi");
        assert_eq!(u.role, "user");

        let a = Message::assistant("ok");
        assert_eq!(a.role, "assistant");
    }

    #[test]
    fn test_thinking_helpers() {
        assert_eq!(Thinking::enabled().kind, "enabled");
        assert_eq!(Thinking::disabled().kind, "disabled");
    }
}
