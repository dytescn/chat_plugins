// src/chat/ffi.rs
// 对话内容处理：检查代码片段中的待办标记、调试输出、疑似密钥与超长行。
// 参考：other_code/claude-plugins-official/plugins/code-review、code-simplifier
use serde::Deserialize;
use serde_json::{json, Value};
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

/// 检查文本，返回问题列表：`[{line, kind, message}]`。
pub fn check(text: &str) -> Vec<Value> {
    let mut issues = Vec::new();

    for (index, line) in text.lines().enumerate() {
        let number = index + 1;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if ["TODO", "FIXME", "XXX"]
            .iter()
            .any(|mark| trimmed.contains(mark))
        {
            issues.push(json!({ "line": number, "kind": "todo", "message": "存在待办标记" }));
        }

        if ["console.log", "dbg!", "println!(", "print("]
            .iter()
            .any(|mark| trimmed.contains(mark))
        {
            issues.push(json!({ "line": number, "kind": "debug", "message": "存在调试输出" }));
        }

        let lower = trimmed.to_ascii_lowercase();
        let looks_secret = (lower.contains("api_key")
            || lower.contains("secret")
            || lower.contains("token"))
            && trimmed.contains('=')
            && !lower.contains("env");
        if looks_secret {
            issues.push(json!({ "line": number, "kind": "secret", "message": "疑似硬编码密钥" }));
        }

        if trimmed.chars().count() > 120 {
            issues.push(json!({ "line": number, "kind": "long_line", "message": "行超过 120 字符" }));
        }
    }

    issues
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

    let issues = check(&input);
    let content = if issues.is_empty() {
        "代码检查完成：未发现待办标记、调试输出、疑似密钥或超长行。".to_string()
    } else {
        let kinds = issues
            .iter()
            .filter_map(|issue| issue.get("kind").and_then(Value::as_str))
            .collect::<Vec<_>>()
            .join("、");
        format!("代码检查完成：发现 {} 个问题（{kinds}）。", issues.len())
    };

    write_success_json(
        json_out,
        "success",
        json!({ "content": content, "issues": issues, "model": "chat_code_check" }),
    )
}
