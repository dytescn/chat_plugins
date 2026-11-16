// src/chat/ffi.rs
// 插件阶段（chat_plugins）的基础能力：接收单个字符串输入，返回带插件标记的内容。
// 当前为确定性测试实现：content = "插件" + 输入，并把输入回传为 data.input。
use serde::Deserialize;
use serde_json::{json, Value};
use std::os::raw::{c_char, c_int};

use crate::utils::cjson::{parse_json, write_error_json, write_success_json};
use crate::utils::errcode::{self, ERR_CHAT_JSON_PARSE, ERR_CHAT_PROMPT_REQUIRED};

#[derive(Debug, Deserialize, Default)]
pub struct ChatReq {
    /// 显式输入字段。
    pub input: Option<String>,
    /// chatllm 插件阶段实际下发的输入字段。
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

/// 测试实现：返回插件阶段内容，并把输入作为下一步 input。
pub fn run(input: &str) -> Value {
    json!({
        "content": format!("插件{}", input),
        "input": input,
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

    write_success_json(json_out, "success", run(&input))
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
    fn returns_plugin_marker_and_next_input() {
        let data = run("你好");
        assert_eq!(data["content"], json!("插件你好"));
        assert_eq!(data["input"], json!("你好"));
    }
}
