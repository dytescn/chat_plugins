// src/chat/ffi.rs
// 测试用聊天插件：不访问网络、不读写数据库，按请求确定性地返回内容。
use std::os::raw::{c_char, c_int};

use serde::Deserialize;
use serde_json::{json, Map, Value};

use crate::chat::call;
use crate::chat::supplier::{self, Resolution, SupplierSpec};
use crate::utils::cjson::{parse_json, write_error_json, write_success_json};
use crate::utils::errcode::{
    self, ERR_CHAT_AI_CONFIG, ERR_CHAT_JSON_PARSE, ERR_CHAT_PROMPT_REQUIRED,
    ERR_CHAT_SUPPLIER_CALL, ERR_CHAT_SUPPLIER_EMPTY,
};
use crate::utils::setting;

#[derive(Debug, Deserialize, Default, serde::Serialize)]
struct Message {
    #[serde(default)]
    role: String,
    #[serde(default)]
    content: String,
}

#[derive(Debug, Deserialize, Default)]
struct ChatReq {
    /// 单轮输入
    prompt: Option<String>,
    /// 多轮输入
    messages: Option<Vec<Message>>,
    /// 回显用，不参与生成
    system: Option<String>,
    /// 回显用；缺省为 do_test_chat
    model: Option<String>,
    /// 回显用
    session_id: Option<String>,
    /// 指定固定回复，便于断言
    reply: Option<String>,
    /// 模拟失败：非 0 时直接返回该错误码
    fail_code: Option<i32>,
    fail_msg: Option<String>,
    /// 临时指定供应商；缺省读取 info.db 的 ai_supplier，未配置时默认 deepseek
    ai_supplier: Option<String>,
    /// 测试用：跳过 AI 配置解析，直接返回确定性内容
    #[serde(default)]
    no_ai: bool,
}

/// 取用于回显的文本：prompt 优先，否则取最后一条有内容的 user 消息，
/// 再退回到最后一条有内容的消息。
fn pick_input(req: &ChatReq) -> Option<String> {
    if let Some(prompt) = req.prompt.as_ref() {
        if !prompt.is_empty() {
            return Some(prompt.clone());
        }
    }
    let messages = req.messages.as_ref()?;
    if let Some(message) = messages
        .iter()
        .rev()
        .find(|m| m.role == "user" && !m.content.is_empty())
    {
        return Some(message.content.clone());
    }
    messages
        .iter()
        .rev()
        .find(|m| !m.content.is_empty())
        .map(|m| m.content.clone())
}

/// 写入业务错误响应并返回 0。
/// cyctron 把 FFI 返回值非 0 视为插件调用失败（返回 404），
/// 因此业务错误只放在响应体的 `code` 中。
fn respond_error(json_out: *mut c_char, code: i32, msg: &str) -> c_int {
    let _ = write_error_json(json_out, code, msg);
    errcode::OK
}

macro_rules! step {
    ($tag:expr, $($arg:tt)*) => {
        eprintln!("[post_chat][{}] {}", $tag, format!($($arg)*));
    };
}

/// 解析 ai_supplier：请求字段优先，其次 info.db 的 setting。
/// 返回要合并进 data 的状态字段。
fn status_fields(requested: Option<&str>) -> (Map<String, Value>, Option<&'static SupplierSpec>) {
    let mut fields = Map::new();

    let stored = match setting::get_setting("ai_supplier") {
        Ok(value) => value,
        Err(error) => {
            fields.insert("status".into(), json!("config_error"));
            fields.insert(
                "prompt".into(),
                json!(format!("读取 info.db 配置失败：{error}")),
            );
            fields.insert("suppliers".into(), supplier::suppliers_json());
            return (fields, None);
        }
    };

    let stored = stored
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    let selected = requested
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .or(stored)
        .or_else(|| Some(supplier::default_supplier().to_string()));

    let values = if let Some(spec) = selected.as_deref().and_then(supplier::lookup) {
        let mut keys = vec!["ai_supplier"];
        keys.extend(spec.required.iter().map(|field| field.key));
        keys.extend(spec.optional.iter().map(|field| field.key));
        match setting::get_settings(&keys) {
            Ok(values) => values,
            Err(error) => {
                fields.insert("status".into(), json!("config_error"));
                fields.insert(
                    "prompt".into(),
                    json!(format!("读取供应商配置失败：{error}")),
                );
                fields.insert("suppliers".into(), supplier::suppliers_json());
                return (fields, None);
            }
        }
    } else {
        std::collections::HashMap::new()
    };

    let ready_spec = match supplier::resolve(selected.as_deref(), &values) {
        Resolution::NeedSupplier => {
            fields.insert("status".into(), json!("need_supplier"));
            fields.insert("suppliers".into(), supplier::suppliers_json());
            fields.insert(
                "prompt".into(),
                json!(format!(
                    "请选择聊天供应商：{}",
                    supplier::supplier_names().join(" / ")
                )),
            );
            None
        }
        Resolution::InvalidSupplier(name) => {
            fields.insert("status".into(), json!("invalid_supplier"));
            fields.insert("supplier".into(), json!(name));
            fields.insert("suppliers".into(), supplier::suppliers_json());
            fields.insert(
                "prompt".into(),
                json!(format!(
                    "未知的聊天供应商：{name}；可选：{}",
                    supplier::supplier_names().join(" / ")
                )),
            );
            None
        }
        Resolution::NeedConfig {
            spec,
            missing,
            config,
        } => {
            fields.insert("status".into(), json!("need_config"));
            fields.insert("supplier".into(), json!(spec.value));
            fields.insert("endpoint".into(), json!(spec.endpoint));
            fields.insert("missing".into(), supplier::missing_json(&missing));
            fields.insert("config".into(), config);
            let keys = missing
                .iter()
                .map(|field| field.key)
                .collect::<Vec<_>>()
                .join(", ");
            fields.insert(
                "prompt".into(),
                json!(format!("已选择 {}，请配置：{keys}", spec.label)),
            );
            None
        }
        Resolution::Ready { spec, config } => {
            fields.insert("status".into(), json!("ready"));
            fields.insert("supplier".into(), json!(spec.value));
            fields.insert("endpoint".into(), json!(spec.endpoint));
            fields.insert("config".into(), config);
            fields.insert(
                "prompt".into(),
                json!(format!("已选择 {}，可以开始对话", spec.label)),
            );
            Some(spec)
        }
    };

    (fields, ready_spec)
}

/// 把非 ready 的 AI 配置状态转换为错误码与可读消息。
fn config_error(status: &Map<String, Value>) -> (i32, String) {
    let kind = status.get("status").and_then(Value::as_str).unwrap_or("");
    match kind {
        "need_config" => {
            let supplier = status
                .get("supplier")
                .and_then(Value::as_str)
                .unwrap_or("AI");
            let missing = status
                .get("missing")
                .and_then(Value::as_array)
                .map(|items| {
                    items
                        .iter()
                        .filter_map(|item| item.get("key").and_then(Value::as_str))
                        .collect::<Vec<_>>()
                        .join(", ")
                })
                .unwrap_or_default();
            (
                ERR_CHAT_AI_CONFIG,
                format!("未配置 {supplier} token：{missing}"),
            )
        }
        "config_error" => (
            ERR_CHAT_SUPPLIER_CALL,
            status
                .get("prompt")
                .and_then(Value::as_str)
                .unwrap_or("读取 AI 配置失败")
                .to_string(),
        ),
        "invalid_supplier" => (
            ERR_CHAT_AI_CONFIG,
            status
                .get("prompt")
                .and_then(Value::as_str)
                .unwrap_or("未知的 AI 供应商")
                .to_string(),
        ),
        _ => (
            ERR_CHAT_AI_CONFIG,
            status
                .get("prompt")
                .and_then(Value::as_str)
                .unwrap_or("AI 配置不完整")
                .to_string(),
        ),
    }
}

#[no_mangle]
pub extern "C" fn post_chat(json_in: *const c_char, json_out: *mut c_char) -> c_int {
    // 1. 解析
    let req: ChatReq = match parse_json::<ChatReq>(json_in) {
        Ok(v) => v,
        Err(e) => {
            step!("step1", "解析失败: {:?}", e);
            let code = ERR_CHAT_JSON_PARSE;
            return respond_error(json_out, code, errcode::default_message(code));
        }
    };

    // 2. 可选的失败模拟
    if let Some(code) = req.fail_code {
        if code != 0 {
            let msg = req
                .fail_msg
                .as_deref()
                .unwrap_or_else(|| errcode::default_message(code));
            step!("step2", "模拟失败: code={} msg={}", code, msg);
            return respond_error(json_out, code, msg);
        }
    }

    // 3. 取输入文本
    let input = match pick_input(&req) {
        Some(value) => value,
        None => {
            let code = ERR_CHAT_PROMPT_REQUIRED;
            return respond_error(json_out, code, errcode::default_message(code));
        }
    };

    // 4. 生成回复：默认 deepseek；检查 token 配置，缺配置直接报错，配置完整则转发
    let messages_count = req.messages.as_ref().map(|m| m.len()).unwrap_or(1usize);
    let deterministic = req
        .reply
        .clone()
        .unwrap_or_else(|| format!("[do_test_chat] {input}"));
    let (status, ready_spec) = if req.no_ai {
        (Map::new(), None)
    } else {
        status_fields(req.ai_supplier.as_deref())
    };

    let (content, upstream) = if req.no_ai {
        (deterministic, None)
    } else if let Some(spec) = ready_spec {
        let request = json!({
            "prompt": req.prompt.clone(),
            "messages": req.messages,
            "system": req.system.clone(),
            "model": req.model.clone(),
            "session_id": req.session_id.clone(),
        });
        match call::forward(spec.value, &request) {
            Ok(data) => {
                let content = data
                    .get("content")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string();
                if content.trim().is_empty() {
                    step!("step4", "供应商 {} 返回空内容", spec.value);
                    return respond_error(
                        json_out,
                        ERR_CHAT_SUPPLIER_EMPTY,
                        errcode::default_message(ERR_CHAT_SUPPLIER_EMPTY),
                    );
                }
                (content, Some(data))
            }
            Err(error) => {
                step!("step4", "转发 {} 失败: {:#}", spec.value, error);
                return respond_error(
                    json_out,
                    ERR_CHAT_SUPPLIER_CALL,
                    &format!("调用 {} 失败：{error:#}", spec.label),
                );
            }
        }
    } else {
        let (code, message) = config_error(&status);
        step!("step4", "AI 配置错误: {}", message);
        return respond_error(json_out, code, &format!("AI 配置错误：{message}"));
    };

    let model = upstream
        .as_ref()
        .and_then(|data| data.get("model"))
        .and_then(Value::as_str)
        .map(str::to_string)
        .or_else(|| req.model.clone())
        .unwrap_or_else(|| "do_test_chat".to_string());

    let mut data = Map::new();
    data.insert("content".into(), json!(content));
    data.insert("model".into(), json!(model));
    data.insert("messages_count".into(), json!(messages_count));
    data.insert("system".into(), json!(req.system));
    data.insert("session_id".into(), json!(req.session_id));
    data.insert("echo".into(), json!(input));
    for (key, value) in status {
        data.insert(key, value);
    }
    if let Some(upstream) = upstream {
        data.insert("upstream".into(), upstream);
    }

    step!(
        "step3",
        "回复长度 {}，messages={}",
        content.len(),
        messages_count
    );
    write_success_json(json_out, "success", Value::Object(data))
}
