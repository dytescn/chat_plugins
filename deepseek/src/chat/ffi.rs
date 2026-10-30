// src/chat/ffi.rs
use std::os::raw::{c_char, c_int};

use serde::Deserialize;
use serde_json::json;

use crate::utils::cjson::{parse_json, write_error_json, write_success_json};
use crate::utils::deepseek::{deepseek_chat, ChatParams, Message, Thinking};
use crate::utils::errcode::{
    self, ERR_CHAT_JSON_PARSE, ERR_CHAT_PROMPT_REQUIRED, ERR_CHAT_REQUEST_FAILED,
};
use crate::utils::setting::{build_params_from_config, load_chat_config_sync};

#[derive(Debug, Deserialize)]
struct ChatReq {
    prompt: Option<String>,
    messages: Option<Vec<Message>>,
    /// system 提示词（只来自请求，不从 setting 读）
    system: Option<String>,
    model: Option<String>,
    max_tokens: Option<u32>,
    temperature: Option<f64>,
    thinking: Option<Thinking>,
    reasoning_effort: Option<String>,
}

macro_rules! step {
    ($tag:expr, $($arg:tt)*) => {
        eprintln!("[post_chat][{}] {}", $tag, format!($($arg)*));
    };
}

#[no_mangle]
pub extern "C" fn post_chat(
    json_in: *const c_char,
    json_out: *mut c_char,
) -> c_int {
    step!("enter", "post_chat 被调用");

    // ===== 1. 解析 =====
    let req: ChatReq = match parse_json::<ChatReq>(json_in) {
        Ok(v) => v,
        Err(e) => {
            step!("step1", "解析失败: {:?}", e);
            let code = ERR_CHAT_JSON_PARSE;
            return write_error_json(json_out, code, errcode::default_message(code));
        }
    };

    // ===== 2. 读 setting =====
    let cfg = match load_chat_config_sync() {
        Ok(c) => {
            step!("step2", "读取成功: model={}, max_tokens={}", c.model, c.max_tokens);
            c
        }
        Err(e) => {
            step!("step2", "读取失败: {:?}", e);
            let msg = format!("{}: {}", errcode::default_message(ERR_CHAT_REQUEST_FAILED), e);
            return write_error_json(json_out, ERR_CHAT_REQUEST_FAILED, &msg);
        }
    };

    // ===== 3. 组装 ChatParams =====
    let params: ChatParams = if let Some(mut messages) = req.messages {
        // 多轮：如果请求带 system，插最前
        if let Some(sys) = req.system {
            if !sys.is_empty() {
                messages.insert(0, Message::system(sys));
            }
        }
        if messages.is_empty() {
            let code = ERR_CHAT_PROMPT_REQUIRED;
            return write_error_json(json_out, code, "messages 不能为空");
        }
        step!("step3", "多轮 messages，共 {} 条", messages.len());

        ChatParams {
            model: req.model.unwrap_or_else(|| cfg.model.clone()),
            messages,
            max_tokens: req.max_tokens.unwrap_or(cfg.max_tokens),
            temperature: req.temperature.unwrap_or(cfg.temperature),
            stream: false,
            stop_sequences: None,
            thinking: req.thinking.unwrap_or_else(|| cfg.thinking.clone()),
            reasoning_effort: req.reasoning_effort.or_else(|| cfg.reasoning_effort.clone()),
        }
    } else if let Some(prompt) = req.prompt {
        if prompt.is_empty() {
            let code = ERR_CHAT_PROMPT_REQUIRED;
            return write_error_json(json_out, code, "prompt 不能为空");
        }
        step!("step3", "单轮 prompt，长度 {}", prompt.len());

        // system 来自请求
        let mut p = build_params_from_config(&cfg, prompt, req.system);

        // 其余覆盖
        if let Some(m) = req.model { p.model = m; }
        if let Some(mt) = req.max_tokens { p.max_tokens = mt; }
        if let Some(t) = req.temperature { p.temperature = t; }
        if let Some(th) = req.thinking { p.thinking = th; }
        if let Some(re) = req.reasoning_effort { p.reasoning_effort = Some(re); }
        p
    } else {
        let code = ERR_CHAT_PROMPT_REQUIRED;
        return write_error_json(json_out, code, "prompt 或 messages 至少提供一个");
    };

    // ===== 4. 调 API =====
    step!("step4", "调用 DeepSeek API");
    match deepseek_chat(&cfg.api_key, &cfg.base_url, &params) {
        Ok(content) => {
            step!("step4", "成功，回复长度 {}", content.len());
            write_success_json(json_out, "success", json!({ "content": content }))
        }
        Err(e) => {
            step!("step4", "失败: {:?}", e);
            let msg = format!("{}: {}", errcode::default_message(ERR_CHAT_REQUEST_FAILED), e);
            write_error_json(json_out, ERR_CHAT_REQUEST_FAILED, &msg)
        }
    }
}
