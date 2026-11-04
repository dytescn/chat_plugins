// src/utils/errcode.rs
// 全局错误码定义
// 约定：
//   0        成功
//   4xxxxx   客户端错误（输入、参数）
//   5xxxxx   服务端错误（内部、依赖）
//   6xxxxx   预留（业务自定义）

use serde_json::json;

// ============ 成功 ============
pub const OK: i32 = 0;

// ============ 通用错误（400xxx） ============
pub const ERR_JSON_IN_NULL: i32 = 400001;
pub const ERR_JSON_PARSE: i32 = 400002;
pub const ERR_JSON_UTF8: i32 = 400003;
pub const ERR_PARAM_MISSING: i32 = 400004;
pub const ERR_PARAM_INVALID: i32 = 400005;

// ============ chat 模块（4001xx / 5001xx） ============
pub const ERR_CHAT_JSON_PARSE: i32 = 400101;
pub const ERR_CHAT_PROMPT_REQUIRED: i32 = 400102;
pub const ERR_CHAT_RUNTIME: i32 = 500100;
pub const ERR_CHAT_REQUEST_FAILED: i32 = 500101;
pub const ERR_CHAT_API_ERROR: i32 = 500102;
pub const ERR_CHAT_RESPONSE_PARSE: i32 = 500103;
pub const ERR_CHAT_EMPTY_RESPONSE: i32 = 500104;

// ============ setting 模块（4002xx / 5002xx） ============
pub const ERR_SETTING_DB_REQUEST: i32 = 500200;
pub const ERR_SETTING_DB_RESPONSE: i32 = 500201;
pub const ERR_SETTING_KEY_MISSING: i32 = 500202;
pub const ERR_SETTING_VALUE_INVALID: i32 = 500203;

// ============ 数据库模块（4003xx / 5003xx） ============
pub const ERR_DB_COLLECTION_REQUIRED: i32 = 400301;
pub const ERR_DB_OPEN_FAILED: i32 = 500300;
pub const ERR_DB_QUERY_FAILED: i32 = 500301;
pub const ERR_DB_INSERT_FAILED: i32 = 500302;
pub const ERR_DB_DELETE_FAILED: i32 = 500303;

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
        ERR_PARAM_MISSING => "required parameter is missing",
        ERR_PARAM_INVALID => "parameter is invalid",

        ERR_CHAT_JSON_PARSE => "chat request json parse failed",
        ERR_CHAT_PROMPT_REQUIRED => "prompt is required",
        ERR_CHAT_RUNTIME => "tokio runtime create failed",
        ERR_CHAT_REQUEST_FAILED => "chat request failed",
        ERR_CHAT_API_ERROR => "chat api returned error",
        ERR_CHAT_RESPONSE_PARSE => "chat response parse failed",
        ERR_CHAT_EMPTY_RESPONSE => "chat response is empty",

        ERR_SETTING_DB_REQUEST => "setting db request failed",
        ERR_SETTING_DB_RESPONSE => "setting db response error",
        ERR_SETTING_KEY_MISSING => "setting key is missing",
        ERR_SETTING_VALUE_INVALID => "setting value is invalid",

        ERR_DB_COLLECTION_REQUIRED => "collection is required",
        ERR_DB_OPEN_FAILED => "db open failed",
        ERR_DB_QUERY_FAILED => "db query failed",
        ERR_DB_INSERT_FAILED => "db insert failed",
        ERR_DB_DELETE_FAILED => "db delete failed",

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
