// src/utils/errcode.rs
// 错误码约定：
//   0        成功
//   4xxxxx   客户端错误（输入、参数）
//   5xxxxx   服务端错误（内部、依赖）

use serde_json::json;

// ============ 成功 ============
pub const OK: i32 = 0;

// ============ 通用错误（400xxx） ============
pub const ERR_JSON_IN_NULL: i32 = 400001;
pub const ERR_JSON_PARSE: i32 = 400002;
pub const ERR_JSON_UTF8: i32 = 400003;

// ============ chat 模块（4001xx / 5001xx） ============
pub const ERR_CHAT_JSON_PARSE: i32 = 400101;
pub const ERR_CHAT_PROMPT_REQUIRED: i32 = 400102;
pub const ERR_CHAT_RUNTIME: i32 = 500100;
pub const ERR_CHAT_SUPPLIER_CALL: i32 = 500101;
pub const ERR_CHAT_AI_CONFIG: i32 = 500102;
pub const ERR_CHAT_SUPPLIER_EMPTY: i32 = 500104;

// ============ 输出缓冲（5009xx） ============
pub const ERR_JSON_OUT_NULL: i32 = 500900;
pub const ERR_JSON_OUT_TOO_LARGE: i32 = 500901;

// ============ 错误码 -> 默认消息 ============
pub fn default_message(code: i32) -> &'static str {
    match code {
        OK => "success",

        ERR_JSON_IN_NULL => "json_in is null",
        ERR_JSON_PARSE => "json parse failed",
        ERR_JSON_UTF8 => "json input is not valid utf-8",

        ERR_CHAT_JSON_PARSE => "chat request json parse failed",
        ERR_CHAT_PROMPT_REQUIRED => "prompt or messages is required",
        ERR_CHAT_RUNTIME => "chat runtime error",
        ERR_CHAT_SUPPLIER_CALL => "supplier chat call failed",
        ERR_CHAT_AI_CONFIG => "AI supplier is not configured",
        ERR_CHAT_SUPPLIER_EMPTY => "supplier returned empty content",

        ERR_JSON_OUT_NULL => "json_out is null",
        ERR_JSON_OUT_TOO_LARGE => "json output exceeds buffer size",

        _ => "unknown error",
    }
}

// ============ 便捷构造：错误 JSON ============
pub fn error_json(code: i32, msg: Option<&str>) -> serde_json::Value {
    json!({
        "code": code,
        "msg": msg.unwrap_or_else(|| default_message(code)),
        "data": {}
    })
}

// ============ 便捷构造：成功 JSON ============
pub fn success_json(data: serde_json::Value, msg: Option<&str>) -> serde_json::Value {
    json!({
        "code": OK,
        "msg": msg.unwrap_or("success"),
        "data": data
    })
}
