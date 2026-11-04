use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use do_test_chat::chat::ffi::post_chat;
use serde_json::Value;

const BUF_SIZE: usize = 65536;

fn call(body: &str) -> Value {
    let input = CString::new(body).expect("body has no NUL");
    let mut out = vec![0u8; BUF_SIZE];
    let _ret = post_chat(input.as_ptr(), out.as_mut_ptr() as *mut c_char);
    let text = unsafe { CStr::from_ptr(out.as_ptr() as *const c_char) }
        .to_str()
        .expect("output is utf-8");
    serde_json::from_str(text).expect("output is json")
}

#[test]
fn echoes_prompt_with_tag_and_time() {
    let value = call(r#"{"prompt":"你好"}"#);
    assert_eq!(value["code"], 0);
    let content = value["data"]["content"].as_str().unwrap();
    let time = value["data"]["time"].as_u64().unwrap();
    assert!(content.starts_with("你好do_chat_test"), "content={content}");
    assert!(content.ends_with(&time.to_string()));
    assert_eq!(value["data"]["input"], "你好");
    assert_eq!(value["data"]["tag"], "do_chat_test");
}

#[test]
fn uses_last_user_message() {
    let value = call(
        r#"{"messages":[{"role":"system","content":"sys"},{"role":"user","content":"第一句"},{"role":"user","content":"第二句"}]}"#,
    );
    assert_eq!(value["code"], 0);
    assert_eq!(value["data"]["input"], "第二句");
    assert!(value["data"]["content"]
        .as_str()
        .unwrap()
        .starts_with("第二句do_chat_test"));
}

#[test]
fn empty_input_is_rejected() {
    let value = call(r#"{"prompt":""}"#);
    assert_eq!(value["code"], 400102);
    assert_eq!(value["data"], serde_json::json!({}));
}

#[test]
fn invalid_json_is_rejected() {
    let value = call("{not json");
    assert_eq!(value["code"], 400101);
}
