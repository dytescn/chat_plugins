// src/utils/cjson.rs
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

use crate::utils::errcode::{self, ERR_JSON_OUT_NULL, ERR_JSON_OUT_TOO_LARGE, OK};

/// 调用方分配的 JSON 输出缓冲区大小
pub const JSON_OUT_BUF_SIZE: usize = 65536;

/// 把 JSON 写进调用方缓冲区
fn write_json_out(json_out: *mut c_char, value: &Value) -> c_int {
    if json_out.is_null() {
        return ERR_JSON_OUT_NULL;
    }
    let out_str = value.to_string();
    let bytes = out_str.as_bytes();
    if bytes.len() + 1 > JSON_OUT_BUF_SIZE {
        let fallback = errcode::error_json(ERR_JSON_OUT_TOO_LARGE, None).to_string();
        let fb = fallback.as_bytes();
        if fb.len() + 1 > JSON_OUT_BUF_SIZE {
            return ERR_JSON_OUT_TOO_LARGE;
        }
        unsafe {
            std::ptr::copy_nonoverlapping(fb.as_ptr(), json_out as *mut u8, fb.len());
            *json_out.add(fb.len()) = 0;
        }
        return 0;
    }
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), json_out as *mut u8, bytes.len());
        *json_out.add(bytes.len()) = 0;
    }
    0
}

/// 错误 JSON -> json_out，返回值 = 错误码
pub fn write_error_json(json_out: *mut c_char, code: i32, msg: &str) -> c_int {
    let _ = write_json_out(json_out, &errcode::error_json(code, Some(msg)));
    code
}

/// 成功 JSON -> json_out，返回值 = OK（0）
pub fn write_success_json(json_out: *mut c_char, msg: &str, data: Value) -> c_int {
    let _ = write_json_out(json_out, &errcode::success_json(data, Some(msg)));
    OK
}

/// 从 C 字符串解析 JSON
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn parse_json<T>(json_in: *const c_char) -> Result<T, i32>
where
    T: DeserializeOwned,
{
    if json_in.is_null() {
        return Err(errcode::ERR_JSON_IN_NULL);
    }
    let body = match unsafe { CStr::from_ptr(json_in) }.to_str() {
        Ok(s) => s,
        Err(_) => return Err(errcode::ERR_JSON_UTF8),
    };
    serde_json::from_str(body).map_err(|_| errcode::ERR_JSON_PARSE)
}

// ============ 测试辅助 ============

pub fn json_to_c_string(value: &Value) -> *mut c_char {
    let s = CString::new(value.to_string())
        .unwrap_or_else(|_| CString::new(r#"{"code":500,"msg":"json contains null"}"#).unwrap());
    s.into_raw()
}

/// # Safety
/// `ptr` 必须来自 `json_to_c_string` 且未释放
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_json_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            drop(CString::from_raw(ptr));
        }
    }
}
