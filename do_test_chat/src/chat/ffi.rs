// src/chat/ffi.rs
// 测试聊天插件：不访问网络、不读写数据库。
// 输入什么就返回：输入 + do_chat_test + 时间（Unix 毫秒）。
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Deserialize;
use serde_json::{json, Value};

const OUT_BUF_SIZE: usize = 65536;
const TAG: &str = "do_chat_test";

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

fn write_json_out(json_out: *mut c_char, value: Value) -> c_int {
    if json_out.is_null() {
        return -1;
    }
    let text = value.to_string();
    let bytes = text.as_bytes();
    if bytes.len() >= OUT_BUF_SIZE {
        return -2;
    }
    unsafe {
        ptr::copy_nonoverlapping(bytes.as_ptr(), json_out as *mut u8, bytes.len());
        *json_out.add(bytes.len()) = 0;
    }
    0
}

fn respond_error(json_out: *mut c_char, code: i32, msg: &str) -> c_int {
    write_json_out(json_out, json!({ "code": code, "data": {}, "msg": msg }))
}

/// prompt 优先；否则取最后一条有内容的 user 消息；再退到最后一条有内容的消息。
fn pick_input(req: &ChatReq) -> Option<String> {
    if let Some(prompt) = req.prompt.as_ref().filter(|p| !p.trim().is_empty()) {
        return Some(prompt.clone());
    }
    let messages = req.messages.as_ref()?;
    messages
        .iter()
        .rev()
        .find(|m| m.role == "user" && !m.content.trim().is_empty())
        .or_else(|| messages.iter().rev().find(|m| !m.content.trim().is_empty()))
        .map(|m| m.content.clone())
}

#[unsafe(no_mangle)]
pub extern "C" fn post_chat(json_in: *const c_char, json_out: *mut c_char) -> c_int {
    if json_in.is_null() {
        return respond_error(json_out, 400100, "json_in is null");
    }
    let body = match unsafe { CStr::from_ptr(json_in) }.to_str() {
        Ok(body) => body,
        Err(_) => return respond_error(json_out, 400101, "invalid utf-8"),
    };
    let req: ChatReq = match serde_json::from_str(body) {
        Ok(req) => req,
        Err(error) => {
            return respond_error(json_out, 400101, &format!("invalid json: {error}"));
        }
    };
    let input = match pick_input(&req) {
        Some(input) => input,
        None => {
            return respond_error(json_out, 400102, "prompt 或 messages 至少提供一个有效内容");
        }
    };

    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);
    let content = format!("{input}{TAG}{time}");

    write_json_out(
        json_out,
        json!({
            "code": 0,
            "data": {
                "content": content,
                "input": input,
                "tag": TAG,
                "time": time
            },
            "msg": "success"
        }),
    )
}
