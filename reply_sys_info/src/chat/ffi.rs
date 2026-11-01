// src/chat/ffi.rs
// 系统提示插件：把错误码 / AI 配置状态 / think 结果转换成面向用户的语义提示。
use serde::Deserialize;
use serde_json::{json, Map, Value};
use std::os::raw::{c_char, c_int};

use crate::chat::reply;
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
    /// 单轮输入
    prompt: Option<String>,
    /// 多轮输入
    messages: Option<Vec<Message>>,
    /// 系统错误码；非 0 时转换成语义提示
    code: Option<i32>,
    /// 错误详情
    detail: Option<String>,
    /// 当前选择的 AI 供应商
    supplier: Option<String>,
    /// 供应商 token
    token: Option<String>,
    /// think 阶段的结果
    think: Option<String>,
}

fn pick_input(req: &ChatReq) -> Option<String> {
    if let Some(prompt) = req.prompt.as_ref() {
        if !prompt.is_empty() {
            return Some(prompt.clone());
        }
    }
    let messages = req.messages.as_ref()?;
    if let Some(message) = messages
        .iter()
        .rev()
        .find(|message| message.role == "user" && !message.content.is_empty())
    {
        return Some(message.content.clone());
    }
    messages
        .iter()
        .rev()
        .find(|message| !message.content.is_empty())
        .map(|message| message.content.clone())
}

fn respond_error(json_out: *mut c_char, code: i32, msg: &str) -> c_int {
    let _ = write_error_json(json_out, code, msg);
    errcode::OK
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

    // 1. 系统错误码 → 语义提示
    if let Some(code) = req.code.filter(|code| *code != 0) {
        let content = reply::describe_error(code, req.detail.as_deref().unwrap_or(""));
        return success(json_out, content, "error", req);
    }

    // 2. AI 供应商选择 / token 绑定状态
    if req.supplier.is_some() || req.token.is_some() {
        let content = reply::supplier_hint(req.supplier.as_deref(), req.token.as_deref());
        return success(json_out, content, "supplier", req);
    }

    // 3. think 阶段的结果
    if let Some(think) = req.think.clone().filter(|text| !text.trim().is_empty()) {
        return success(json_out, think, "think", req);
    }

    // 4. 兜底：回显输入
    match pick_input(&req) {
        Some(input) => success(json_out, format!("我收到你的信息了：{input}"), "echo", req),
        None => {
            let code = ERR_CHAT_PROMPT_REQUIRED;
            respond_error(json_out, code, errcode::default_message(code))
        }
    }
}

fn success(json_out: *mut c_char, content: String, kind: &str, req: ChatReq) -> c_int {
    let mut data = Map::new();
    data.insert("content".into(), json!(content));
    data.insert("kind".into(), json!(kind));
    data.insert("model".into(), json!("reply_sys_info"));
    data.insert("think".into(), json!(req.think));
    data.insert("code".into(), json!(req.code));
    data.insert("supplier".into(), json!(req.supplier));
    write_success_json(json_out, "success", Value::Object(data))
}
