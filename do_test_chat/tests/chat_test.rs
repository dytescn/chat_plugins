use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use do_test_chat::chat::ffi::post_chat;
use serde_json::Value;

const BUF_SIZE: usize = 65536;

fn call(body: &str) -> Value {
    let input = CString::new(body).expect("body has no NUL");
    let mut out = vec![0u8; BUF_SIZE];
    // 失败时返回值是业务错误码，响应体仍会写入缓冲区
    let _ret = post_chat(input.as_ptr(), out.as_mut_ptr() as *mut c_char);
    let text = unsafe { CStr::from_ptr(out.as_ptr() as *const c_char) }
        .to_str()
        .expect("output is utf-8");
    serde_json::from_str(text).expect("output is json")
}

#[test]
fn echoes_prompt() {
    let value = call(r#"{"prompt":"你好","session_id":"s1","no_ai":true}"#);
    assert_eq!(value["code"], 0);
    assert_eq!(value["data"]["content"], "[do_test_chat] 你好");
    assert_eq!(value["data"]["messages_count"], 1);
    assert_eq!(value["data"]["session_id"], "s1");
}

#[test]
fn uses_last_user_message() {
    let value = call(
        r#"{"messages":[{"role":"system","content":"sys"},{"role":"user","content":"第一句"},{"role":"user","content":"第二句"}],"no_ai":true}"#,
    );
    assert_eq!(value["code"], 0);
    assert_eq!(value["data"]["content"], "[do_test_chat] 第二句");
    assert_eq!(value["data"]["messages_count"], 3);
}

#[test]
fn reply_override_is_returned_as_is() {
    let value = call(r#"{"prompt":"ignored","reply":"固定回复","model":"demo","no_ai":true}"#);
    assert_eq!(value["code"], 0);
    assert_eq!(value["data"]["content"], "固定回复");
    assert_eq!(value["data"]["model"], "demo");
}

#[test]
fn empty_input_is_rejected() {
    let value = call(r#"{"prompt":""}"#);
    assert_eq!(value["code"], 400102);
    assert_eq!(value["data"], serde_json::json!({}));
}

#[test]
fn fail_code_is_simulated() {
    let value = call(r#"{"prompt":"hi","fail_code":500101,"fail_msg":"upstream down"}"#);
    assert_eq!(value["code"], 500101);
    assert_eq!(value["msg"], "upstream down");
}

#[test]
fn invalid_json_is_rejected() {
    let value = call("{not json");
    assert_eq!(value["code"], 400101);
}

#[test]
fn missing_ai_config_is_reported() {
    // 未显式指定 ai_supplier：默认 deepseek；测试环境没有 info.db / token，
    // 应返回 AI 配置错误，而不是确定性回显。
    let value = call(r#"{"prompt":"hi"}"#);
    let code = value["code"].as_i64().unwrap_or(0);
    assert!(code == 500101 || code == 500102, "unexpected code: {code}");
    assert!(
        value["msg"].as_str().unwrap_or("").contains("AI 配置错误"),
        "unexpected msg: {}",
        value["msg"]
    );
}
