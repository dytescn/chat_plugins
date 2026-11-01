// src/chat/ffi.rs
// 对话内容处理：从文本中提取 IOC（IP / 域名 / 文件哈希 / URL）。
// 参考：other_code/Anthropic-Cybersecurity-Skills/skills/analyzing-indicators-of-compromise
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::os::raw::{c_char, c_int};

use crate::utils::cjson::{parse_json, write_error_json, write_success_json};
use crate::utils::errcode::{self, ERR_CHAT_JSON_PARSE, ERR_CHAT_PROMPT_REQUIRED};

#[derive(Debug, Deserialize, Default)]
struct Message {
    #[serde(default)]
    role: String,
    #[serde(default)]
    content: String,
}

#[derive(Debug, Deserialize, Default)]
struct ChatReq {
    prompt: Option<String>,
    messages: Option<Vec<Message>>,
}

fn pick_input(req: &ChatReq) -> Option<String> {
    if let Some(prompt) = req.prompt.as_ref() {
        if !prompt.trim().is_empty() {
            return Some(prompt.clone());
        }
    }
    let messages = req.messages.as_ref()?;
    if let Some(message) = messages
        .iter()
        .rev()
        .find(|message| message.role == "user" && !message.content.trim().is_empty())
    {
        return Some(message.content.clone());
    }
    messages
        .iter()
        .rev()
        .find(|message| !message.content.trim().is_empty())
        .map(|message| message.content.clone())
}

fn respond_error(json_out: *mut c_char, code: i32, msg: &str) -> c_int {
    let _ = write_error_json(json_out, code, msg);
    errcode::OK
}

/// 解析文本中的 IOC，返回 `{ips, domains, hashes, urls, count}`。
pub fn extract(text: &str) -> Value {
    let mut ips = BTreeSet::new();
    let mut domains = BTreeSet::new();
    let mut hashes = BTreeSet::new();
    let mut urls = BTreeSet::new();

    for token in text.split(|c: char| {
        c.is_whitespace()
            || matches!(
                c,
                ',' | ';' | '(' | ')' | '[' | ']' | '{' | '}' | '"' | '\'' | '<' | '>' | '`'
            )
            || "，。；：、！？（）【】".contains(c)
    }) {
        let token = token.trim_matches(|c: char| !c.is_ascii_alphanumeric() && c != '/' && c != '.');
        if token.is_empty() {
            continue;
        }
        if is_url(token) {
            urls.insert(token.to_string());
        } else if is_ip(token) {
            ips.insert(token.to_string());
        } else if is_hash(token) {
            hashes.insert(token.to_string());
        } else if is_domain(token) {
            domains.insert(token.to_string());
        }
    }

    let count = ips.len() + domains.len() + hashes.len() + urls.len();
    json!({
        "ips": ips.into_iter().collect::<Vec<_>>(),
        "domains": domains.into_iter().collect::<Vec<_>>(),
        "hashes": hashes.into_iter().collect::<Vec<_>>(),
        "urls": urls.into_iter().collect::<Vec<_>>(),
        "count": count,
    })
}

fn is_url(token: &str) -> bool {
    (token.starts_with("http://") || token.starts_with("https://")) && token.len() > 8
}

fn is_ip(token: &str) -> bool {
    let parts: Vec<&str> = token.split('.').collect();
    parts.len() == 4
        && parts
            .iter()
            .all(|part| !part.is_empty() && part.parse::<u16>().map(|n| n <= 255).unwrap_or(false))
}

fn is_hash(token: &str) -> bool {
    matches!(token.len(), 32 | 40 | 64) && token.chars().all(|c| c.is_ascii_hexdigit())
}

fn is_domain(token: &str) -> bool {
    if is_ip(token) || is_url(token) {
        return false;
    }
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() < 2 {
        return false;
    }
    let tld = parts.last().copied().unwrap_or("");
    (2..=24).contains(&tld.len())
        && tld.chars().all(|c| c.is_ascii_alphabetic())
        && parts
            .iter()
            .all(|part| !part.is_empty() && part.chars().all(|c| c.is_ascii_alphanumeric() || c == '-'))
}

#[no_mangle]
pub extern "C" fn post_chat(json_in: *const c_char, json_out: *mut c_char) -> c_int {
    let req: ChatReq = match parse_json(json_in) {
        Ok(value) => value,
        Err(_) => {
            let code = ERR_CHAT_JSON_PARSE;
            return respond_error(json_out, code, errcode::default_message(code));
        }
    };

    let input = match pick_input(&req) {
        Some(value) => value,
        None => {
            let code = ERR_CHAT_PROMPT_REQUIRED;
            return respond_error(json_out, code, errcode::default_message(code));
        }
    };

    let iocs = extract(&input);
    let count = iocs.get("count").and_then(Value::as_i64).unwrap_or(0);
    let content = if count == 0 {
        "未在对话内容中发现 IOC（IP / 域名 / 哈希 / URL）。".to_string()
    } else {
        format!(
            "从对话内容中提取到 {count} 个 IOC：IP {} 个、域名 {} 个、哈希 {} 个、URL {} 个。",
            iocs["ips"].as_array().map(Vec::len).unwrap_or(0),
            iocs["domains"].as_array().map(Vec::len).unwrap_or(0),
            iocs["hashes"].as_array().map(Vec::len).unwrap_or(0),
            iocs["urls"].as_array().map(Vec::len).unwrap_or(0),
        )
    };

    write_success_json(
        json_out,
        "success",
        json!({ "content": content, "iocs": iocs, "model": "chat_ioc" }),
    )
}
