// src/chat/ffi.rs
// think 阶段基础调用：单个字符串输入，返回关键字（prompts）与内容（outputs）。
// 当前为确定性测试实现：固定返回 prompts["问候"]，outputs = "测试" + 输入。
use serde::Deserialize;
use serde_json::{json, Value};
use std::os::raw::{c_char, c_int};

use crate::utils::cjson::{parse_json, write_error_json, write_success_json};
use crate::utils::errcode::{self, ERR_CHAT_JSON_PARSE, ERR_CHAT_PROMPT_REQUIRED};

#[derive(Debug, Deserialize, Default)]
pub struct ChatReq {
    /// MCP / think 阶段的标准输入字段。
    pub input: Option<String>,
    /// chatllm 实际下发字段，作为 input 的兼容别名。
    pub prompt: Option<String>,
}

/// 取单个字符串输入：优先 `input`，其次 `prompt`。
pub fn pick_input(req: &ChatReq) -> Option<String> {
    [req.input.as_deref(), req.prompt.as_deref()]
        .into_iter()
        .flatten()
        .find(|value| !value.trim().is_empty())
        .map(str::to_string)
}

/// 测试实现：prompts 为关键字，outputs 为内容。
pub fn think(input: &str) -> Value {
    json!({
        "prompts": ["问候"],
        "outputs": [format!("测试{}", input)],
    })
}

fn respond_error(json_out: *mut c_char, code: i32) -> c_int {
    let _ = write_error_json(json_out, code, errcode::default_message(code));
    errcode::OK
}

#[no_mangle]
pub extern "C" fn post_chat(json_in: *const c_char, json_out: *mut c_char) -> c_int {
    let req: ChatReq = match parse_json(json_in) {
        Ok(value) => value,
        Err(_) => return respond_error(json_out, ERR_CHAT_JSON_PARSE),
    };

    let input = match pick_input(&req) {
        Some(value) => value,
        None => return respond_error(json_out, ERR_CHAT_PROMPT_REQUIRED),
    };

    write_success_json(json_out, "success", think(&input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_wins_over_prompt() {
        let req = ChatReq {
            input: Some("你好".into()),
            prompt: Some("旧字段".into()),
        };
        assert_eq!(pick_input(&req).as_deref(), Some("你好"));
    }

    #[test]
    fn blank_input_falls_back_to_prompt() {
        let req = ChatReq {
            input: Some("  ".into()),
            prompt: Some("你好".into()),
        };
        assert_eq!(pick_input(&req).as_deref(), Some("你好"));
    }

    #[test]
    fn empty_request_has_no_input() {
        let req = ChatReq {
            input: Some("".into()),
            prompt: None,
        };
        assert_eq!(pick_input(&req), None);
    }

    #[test]
    fn returns_greeting_and_test_content() {
        let data = think("你好");
        assert_eq!(data["prompts"], json!(["问候"]));
        assert_eq!(data["outputs"], json!(["测试你好"]));
    }
}
