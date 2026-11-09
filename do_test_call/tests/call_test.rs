use do_test_call::chat::ffi::post_chat;
use serde_json::Value;
use std::ffi::{CStr, CString};

fn call(input: &str) -> Value {
    let input = CString::new(input).unwrap();
    let mut out = vec![0u8; 65536];
    let ret = post_chat(input.as_ptr(), out.as_mut_ptr() as *mut _);
    assert_eq!(ret, 0, "post_chat 应写入响应");
    let text = unsafe { CStr::from_ptr(out.as_ptr() as *const _) }
        .to_str()
        .unwrap();
    serde_json::from_str(text).unwrap()
}

#[test]
fn invalid_json_returns_400102() {
    let response = call("{not-json");
    assert_eq!(response["code"], 400102);
    assert!(response["msg"].as_str().unwrap().contains("invalid json"));
}

#[test]
fn call_chat_shape_is_accepted() {
    // 没有 info_lib 时走加载失败分支，但四字段请求本身应能通过解析。
    let response = call(
        r#"{"plugins_uid":"p-1","category":"chat_plugin","prompts":"查询","context":{}}"#,
    );
    assert_eq!(response["code"], 500100);
    assert!(response["msg"].as_str().unwrap().contains("info_list"));
}
