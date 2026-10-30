// src/chat/ffi.rs
use std::os::raw::{c_char, c_int};

use serde::Deserialize;
use serde_json::json;

use crate::utils::cjson::{parse_json, write_error_json, write_success_json};
use crate::utils::errcode::{
    self, ERR_CHAT_JSON_PARSE, ERR_CHAT_PROMPT_REQUIRED, ERR_CHAT_REQUEST_FAILED,
};
use crate::utils::setting::load_chat_config_sync;
use crate::utils::siliconflow::{siliconflow_chat, ChatParams, Message, Thinking};

#[derive(Debug, Deserialize, Default)]
struct ChatReq {
    /// 单轮输入（简单）
    prompt: Option<String>,
    /// 多轮输入（完整 messages）
    messages: Option<Vec<Message>>,
    /// 请求级 system；未传时回落到 setting.silicon_app_system_prompt
    system: Option<String>,
    /// 请求级模型与采样参数；未传时回落到 setting
    model: Option<String>,
    max_tokens: Option<u32>,
    temperature: Option<f64>,
    top_p: Option<f64>,
    top_k: Option<u32>,
    thinking: Option<Thinking>,
    /// 业务会话标识；当前只回传，不发送给上游（上游没有该入参）
    session_id: Option<String>,
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
    // 1. 解析
    let req: ChatReq = match parse_json::<ChatReq>(json_in) {
        Ok(v) => v,
        Err(e) => {
            step!("step1", "解析失败: {:?}", e);
            let code = ERR_CHAT_JSON_PARSE;
            return write_error_json(json_out, code, errcode::default_message(code));
        }
    };
    let session_id = req.session_id.clone();

    // 2. 读 setting
    let cfg = match load_chat_config_sync() {
        Ok(c) => c,
        Err(e) => {
            step!("step2", "读取 setting 失败: {:?}", e);
            let msg = format!("{}: {}", errcode::default_message(ERR_CHAT_REQUEST_FAILED), e);
            return write_error_json(json_out, ERR_CHAT_REQUEST_FAILED, &msg);
        }
    };

    // 3. 组装 messages（多轮优先；单轮拼 user）
    let mut messages: Vec<Message> = if let Some(messages) = req.messages {
        if messages.is_empty() {
            let code = ERR_CHAT_PROMPT_REQUIRED;
            return write_error_json(json_out, code, "messages 不能为空");
        }
        step!("step3", "使用多轮 messages，共 {} 条", messages.len());
        messages
    } else if let Some(prompt) = req.prompt {
        if prompt.is_empty() {
            let code = ERR_CHAT_PROMPT_REQUIRED;
            return write_error_json(json_out, code, "prompt 不能为空");
        }
        step!("step3", "使用单轮 prompt，长度 {}", prompt.len());
        vec![Message::user(prompt)]
    } else {
        let code = ERR_CHAT_PROMPT_REQUIRED;
        return write_error_json(json_out, code, "prompt 或 messages 至少提供一个");
    };

    // system 优先级：请求级 > setting；已有首条 system 时不重复插入
    let system = req
        .system
        .or_else(|| cfg.system_prompt.clone());
    if let Some(system) = system {
        if messages.first().map(|m| m.role.as_str()) != Some("system") {
            messages.insert(0, Message::system(system));
        }
    }

    // 4. 请求级参数覆盖 setting
    let params = ChatParams {
        model: req.model.unwrap_or(cfg.model),
        messages,
        max_tokens: req.max_tokens.unwrap_or(cfg.max_tokens),
        temperature: req.temperature.unwrap_or(cfg.temperature),
        top_p: req.top_p.unwrap_or(cfg.top_p),
        top_k: req.top_k.unwrap_or(cfg.top_k),
        stream: false,
        thinking: req.thinking.unwrap_or(Thinking {
            kind: "disabled".to_string(),
        }),
    };

    // 5. 调 API
    step!("step4", "调用 SiliconFlow API");
    match siliconflow_chat(&cfg.api_key, &cfg.base_url, &params) {
        Ok(result) => {
            step!("step4", "成功，回复长度 {}，id={:?}", result.content.len(), result.id);
            write_success_json(
                json_out,
                "success",
                json!({
                    "content": result.content,
                    "id": result.id,
                    "model": result.model,
                    "stop_reason": result.stop_reason,
                    "usage": result.usage,
                    "session_id": session_id,
                }),
            )
        }
        Err(e) => {
            step!("step4", "失败: {:?}", e);
            let msg = format!("{}: {}", errcode::default_message(ERR_CHAT_REQUEST_FAILED), e);
            write_error_json(json_out, ERR_CHAT_REQUEST_FAILED, &msg)
        }
    }
}
