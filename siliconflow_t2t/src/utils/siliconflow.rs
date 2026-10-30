// src/utils/siliconflow.rs
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
        Self { role: "system".into(), content: content.into() }
    }
    pub fn user(content: impl Into<String>) -> Self {
        Self { role: "user".into(), content: content.into() }
    }
    pub fn assistant(content: impl Into<String>) -> Self {
        Self { role: "assistant".into(), content: content.into() }
    }
}

// ============ 请求参数 ============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatParams {
    pub model: String,
    pub messages: Vec<Message>,
    pub max_tokens: u32,
    pub temperature: f64,
    pub top_p: f64,
    pub top_k: u32,
    pub stream: bool,
    pub thinking: Thinking,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thinking {
    #[serde(rename = "type")]
    pub kind: String,
}

// ============ 响应 ============

/// Anthropic Messages 兼容响应中，本项目关心的字段。
///
/// `id` 是供应商返回的唯一标识；chatllm 会把它写入 `dialogue.session_id`。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResult {
    pub content: String,
    pub id: Option<String>,
    pub model: Option<String>,
    pub stop_reason: Option<String>,
    pub usage: Option<Value>,
}

// ============ URL 拼接 ============

pub fn build_api_url(base_url: &str) -> String {
    if base_url.ends_with("/messages") {
        base_url.to_string()
    } else {
        format!("{}/messages", base_url.trim_end_matches('/'))
    }
}

// ============ 响应解析（便于单元测试） ============

pub fn parse_chat_response(body: &str) -> Result<ChatResult> {
    let result: Value = serde_json::from_str(body).context("JSON 解析失败")?;

    let content = result["content"]
        .as_array()
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

    Ok(ChatResult {
        content: output,
        id: result.get("id").and_then(|v| v.as_str()).map(str::to_string),
        model: result.get("model").and_then(|v| v.as_str()).map(str::to_string),
        stop_reason: result
            .get("stop_reason")
            .and_then(|v| v.as_str())
            .map(str::to_string),
        usage: result.get("usage").cloned(),
    })
}

// ============ 调用 ============

pub fn siliconflow_chat(
    api_key: &str,
    base_url: &str,
    params: &ChatParams,
) -> Result<ChatResult> {
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

    let request_body = json!({
        "model": params.model,
        "messages": params.messages,
        "max_tokens": params.max_tokens,
        "temperature": params.temperature,
        "top_p": params.top_p,
        "top_k": params.top_k,
        "stream": params.stream,
        "thinking": { "type": params.thinking.kind }
    });

    let resp = ureq::post(&api_url)
        .set("Authorization", &format!("Bearer {}", api_key))
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
    parse_chat_response(&body)
}
