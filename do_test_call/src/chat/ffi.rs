// src/chat/ffi.rs
//
// 测试插件：回调 manager_baseinfo 查询信息数量。
//
// manager_baseinfo 是 MCP 资源包，实际执行的是 `info_lib` 的 `info_list`。
// 插件运行在 cyctron 的同步主线程中，不能再用 HTTP 回调同一个 cyctron
// （主线程会等待自己，形成死锁），因此这里直接 dlopen `./libary/info_lib.*`
// 并调用 `post_info_list`，等价于 manager_baseinfo 的底层调用。
use libloading::Library;
use serde::Deserialize;
use serde_json::{json, Value};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::path::PathBuf;
use std::ptr;

const OUT_BUF_SIZE: usize = 65536;
const TAG: &str = "do_test_call";
const INFO_LIST_SYMBOL: &[u8] = b"post_info_list\0";

type FfiFn = unsafe extern "C" fn(*const c_char, *mut c_char) -> c_int;

#[derive(Debug, Deserialize, Default)]
struct Message {
    #[serde(default)]
    role: String,
    #[serde(default)]
    content: String,
}

#[derive(Debug, Deserialize, Default)]
struct CallReq {
    /// call_chat 透传：对话 id
    #[serde(default)]
    plugins_uid: Option<String>,
    /// call_chat 透传：kv 分类
    #[serde(default)]
    category: Option<String>,
    /// call_chat 透传：提示词 / 检索词
    #[serde(default)]
    prompts: Option<String>,
    /// call_chat 透传：调用方上下文，可带 category / keyword 过滤
    #[serde(default)]
    context: Value,
    /// 兼容普通 chat 请求
    #[serde(default)]
    prompt: Option<String>,
    #[serde(default)]
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

/// 从 `context` 读取可选的 `category` / `keyword`，用于收窄 info_list 的统计范围。
fn filters(req: &CallReq) -> (Option<String>, Option<String>) {
    let context = req.context.as_object();
    let pick = |key: &str| {
        context
            .and_then(|map| map.get(key))
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_string)
    };
    (pick("category"), pick("keyword"))
}

/// 回显本次调用的输入：`prompts` 优先，其次 `prompt`，最后取最后一条非空消息。
fn pick_input(req: &CallReq) -> Option<String> {
    if let Some(prompts) = req
        .prompts
        .as_ref()
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
    {
        return Some(prompts.to_string());
    }
    if let Some(prompt) = req
        .prompt
        .as_ref()
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
    {
        return Some(prompt.to_string());
    }
    let messages = req.messages.as_ref()?;
    messages
        .iter()
        .rev()
        .find(|message| message.role == "user" && !message.content.trim().is_empty())
        .or_else(|| {
            messages
                .iter()
                .rev()
                .find(|message| !message.content.trim().is_empty())
        })
        .map(|message| message.content.clone())
}

/// 构造 `info_list` 请求。`page_size = 1` 只为拿到 `total`，避免无谓的大列表。
fn info_list_request(category: Option<&str>, keyword: Option<&str>) -> Value {
    let mut request = json!({ "page": 1, "page_size": 1 });
    if let Some(category) = category {
        request["category"] = json!(category);
    }
    if let Some(keyword) = keyword {
        request["keyword"] = json!(keyword);
    }
    request
}

/// 解析 `{ "code": 200, "data": { "total": N } }`，返回信息条数。
fn parse_total(response: &Value) -> Result<u64, String> {
    let code = response.get("code").and_then(Value::as_i64).unwrap_or(-1);
    if code != 200 && code != 0 {
        let msg = response
            .get("msg")
            .and_then(Value::as_str)
            .unwrap_or("info_list 调用失败");
        return Err(format!("code={code}: {msg}"));
    }
    response
        .pointer("/data/total")
        .and_then(Value::as_u64)
        .ok_or_else(|| "响应缺少 data.total".to_string())
}

fn info_lib_path() -> Option<PathBuf> {
    let file = if cfg!(target_os = "windows") {
        "info_lib.dll"
    } else if cfg!(target_os = "macos") {
        "info_lib.dylib"
    } else {
        "info_lib.so"
    };
    [
        PathBuf::from("./libary").join(file),
        PathBuf::from("./plugins/manager_baseinfo").join(file),
    ]
    .into_iter()
    .find(|path| path.exists())
}

/// 直接调用 `info_lib.post_info_list`，返回 `data.total`。
fn query_info_count(category: Option<&str>, keyword: Option<&str>) -> Result<u64, String> {
    let path = info_lib_path()
        .ok_or_else(|| "找不到 ./libary/info_lib.{dll,so,dylib}".to_string())?;
    let library = unsafe { Library::new(&path) }
        .map_err(|error| format!("加载 info_lib 失败（{}）: {error}", path.display()))?;
    let function = unsafe { library.get::<FfiFn>(INFO_LIST_SYMBOL) }
        .map_err(|error| format!("解析 post_info_list 失败: {error}"))?;

    let request = info_list_request(category, keyword).to_string();
    let request = CString::new(request).map_err(|_| "info_list 请求含 \\0".to_string())?;
    let mut out = vec![0u8; OUT_BUF_SIZE];
    let ret = unsafe { function(request.as_ptr(), out.as_mut_ptr() as *mut c_char) };
    if ret != 0 {
        return Err(format!("post_info_list 返回 {ret}"));
    }

    let text = unsafe { CStr::from_ptr(out.as_ptr() as *const c_char) }
        .to_str()
        .map_err(|_| "info_list 响应不是 UTF-8".to_string())?;
    let response: Value =
        serde_json::from_str(text).map_err(|error| format!("解析 info_list 响应失败: {error}"))?;
    parse_total(&response)
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
    let req: CallReq = match serde_json::from_str(body) {
        Ok(req) => req,
        Err(error) => {
            return respond_error(json_out, 400102, &format!("invalid json: {error}"));
        }
    };

    let (category, keyword) = filters(&req);
    let total = match query_info_count(category.as_deref(), keyword.as_deref()) {
        Ok(total) => total,
        Err(error) => {
            return respond_error(
                json_out,
                500100,
                &format!("manager_baseinfo(info_list) 查询失败: {error}"),
            );
        }
    };

    let content = match (&category, &keyword) {
        (Some(category), Some(keyword)) => {
            format!("manager_baseinfo: category={category}, keyword={keyword} 共 {total} 条信息")
        }
        (Some(category), None) => {
            format!("manager_baseinfo: category={category} 共 {total} 条信息")
        }
        (None, Some(keyword)) => {
            format!("manager_baseinfo: keyword={keyword} 共 {total} 条信息")
        }
        (None, None) => format!("manager_baseinfo: 共 {total} 条信息"),
    };
    let input = pick_input(&req);

    write_json_out(
        json_out,
        json!({
            "code": 0,
            "data": {
                "content": content,
                "count": total,
                "total": total,
                "tag": TAG,
                "endpoint": "info_lib",
                "symbol": "info_list",
                "input": input,
                "plugins_uid": req.plugins_uid,
                "call_category": req.category,
                "category": category,
                "keyword": keyword,
                "prompts": req.prompts,
            },
            "msg": "success"
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_request_without_filters() {
        assert_eq!(
            info_list_request(None, None),
            json!({ "page": 1, "page_size": 1 })
        );
    }

    #[test]
    fn builds_request_with_filters() {
        assert_eq!(
            info_list_request(Some("base"), Some("语言")),
            json!({ "page": 1, "page_size": 1, "category": "base", "keyword": "语言" })
        );
    }

    #[test]
    fn filters_read_context_and_trim() {
        let req: CallReq = serde_json::from_value(json!({
            "context": { "category": " base ", "keyword": " 语言 " }
        }))
        .unwrap();
        assert_eq!(
            filters(&req),
            (Some("base".to_string()), Some("语言".to_string()))
        );
    }

    #[test]
    fn filters_ignore_empty_values() {
        let req: CallReq = serde_json::from_value(json!({
            "category": "chat_plugin",
            "prompts": "查询",
            "context": { "category": "   ", "keyword": "" }
        }))
        .unwrap();
        assert_eq!(filters(&req), (None, None));
    }

    #[test]
    fn parses_total_from_success_response() {
        let response = json!({ "code": 200, "data": { "total": 42 }, "msg": "success" });
        assert_eq!(parse_total(&response), Ok(42));
    }

    #[test]
    fn parses_total_accepts_code_zero() {
        let response = json!({ "code": 0, "data": { "total": 7 }, "msg": "success" });
        assert_eq!(parse_total(&response), Ok(7));
    }

    #[test]
    fn parse_total_reports_business_error() {
        let response = json!({ "code": 500100, "data": {}, "msg": "db failed" });
        let error = parse_total(&response).unwrap_err();
        assert!(error.contains("500100"));
        assert!(error.contains("db failed"));
    }

    #[test]
    fn parse_total_requires_total_field() {
        let response = json!({ "code": 200, "data": {} });
        assert!(parse_total(&response).unwrap_err().contains("total"));
    }
}
