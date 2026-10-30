// src/utils/setting.rs
// Setting 配置表读取（通过 database.so 的 FFI）+ 对话参数组装

use std::collections::{HashMap, HashSet};

use anyhow::Result;

use crate::utils::db::db_query;
use crate::utils::siliconflow::{ChatParams, Message, Thinking};

const DEFAULT_API_URL: &str = "https://api.siliconflow.cn/v1";
const DEFAULT_MODEL: &str = "Qwen/Qwen2.5-7B-Instruct";

// ============ SQL 转义 ============

pub(crate) fn esc(val: &str) -> String {
    format!("'{}'", val.replace('\'', "''"))
}

// ============ 从 database.so 读 setting ============

/// 一次查询多个 key，返回 key -> value 的 map
pub fn get_settings(keys: &[&str]) -> Result<HashMap<String, String>> {
    if keys.is_empty() {
        return Ok(HashMap::new());
    }

    // 去重 + 拼 IN 列表
    let mut seen: HashSet<&str> = HashSet::with_capacity(keys.len());
    let mut key_list = String::with_capacity(keys.len() * 16);
    let mut first = true;
    for k in keys {
        if !seen.insert(*k) {
            continue;
        }
        if !first {
            key_list.push_str(", ");
        }
        key_list.push_str(&esc(k));
        first = false;
    }

    let sql = format!(
        "SELECT key, value FROM setting WHERE key IN ({})",
        key_list
    );

    let rows = db_query(&sql)?;
    let mut map = HashMap::with_capacity(rows.len());

    for row in &rows {
        if let Some(obj) = row.as_object() {
            let key = obj.get("key").and_then(|v| v.as_str());
            let value = obj.get("value").and_then(|v| v.as_str());
            if let (Some(k), Some(v)) = (key, value) {
                map.insert(k.to_string(), v.to_string());
            }
        }
    }

    Ok(map)
}

/// 读单个配置值
pub fn get_setting(key: &str) -> Result<Option<String>> {
    let sql = format!(
        "SELECT value FROM setting WHERE key = {} LIMIT 1",
        esc(key)
    );
    let rows = db_query(&sql)?;

    if let Some(row) = rows.first() {
        Ok(row
            .get("value")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()))
    } else {
        Ok(None)
    }
}

pub fn get_setting_or(key: &str, default: &str) -> String {
    match get_setting(key) {
        Ok(Some(v)) if !v.is_empty() => v,
        _ => default.to_string(),
    }
}

pub fn get_setting_number(key: &str, default: f64) -> f64 {
    get_setting(key)
        .ok()
        .flatten()
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(default)
}

pub fn get_setting_bool(key: &str, default: bool) -> bool {
    get_setting(key)
        .ok()
        .flatten()
        .map(|v| v == "true" || v == "1")
        .unwrap_or(default)
}

// ============ 纯函数：默认值解析 ============

pub fn resolve_or(value: Option<String>, default: &str) -> String {
    match value {
        Some(v) if !v.is_empty() => v,
        _ => default.to_string(),
    }
}

pub fn resolve_number(value: Option<String>, default: f64) -> f64 {
    value
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(default)
}

pub fn resolve_bool(value: Option<String>, default: bool) -> bool {
    value
        .map(|v| v == "true" || v == "1")
        .unwrap_or(default)
}

// ============ 纯函数：从 map 取值 ============

pub fn map_or(map: &HashMap<String, String>, key: &str, default: &str) -> String {
    match map.get(key) {
        Some(v) if !v.is_empty() => v.clone(),
        _ => default.to_string(),
    }
}

pub fn map_number(map: &HashMap<String, String>, key: &str, default: f64) -> f64 {
    map.get(key)
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(default)
}

pub fn map_bool(map: &HashMap<String, String>, key: &str, default: bool) -> bool {
    map.get(key)
        .map(|v| v == "true" || v == "1")
        .unwrap_or(default)
}

// ============ 对话配置 ============

pub const CHAT_SETTING_KEYS: &[&str] = &[
    "silicon_app_key",
    "silicon_app_base_url",
    "silicon_app_model",
    "silicon_app_max_tokens",
    "silicon_app_temperature",
    "silicon_app_top_p",
    "silicon_app_topk",
    "silicon_app_system_prompt", // ← 新增
];

/// 从 setting 读出的对话配置（不含 messages）
#[derive(Debug, Clone)]
pub struct ChatConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    pub max_tokens: u32,
    pub temperature: f64,
    pub top_p: f64,
    pub top_k: u32,
    pub system_prompt: Option<String>,
}

/// 从 setting 读配置，组装成 ChatConfig
pub fn load_chat_config_sync() -> Result<ChatConfig> {
    let map = get_settings(CHAT_SETTING_KEYS)?;
    build_chat_config(&map)
}

/// 纯函数：从 map 组装 ChatConfig（便于测试）
pub fn build_chat_config(map: &HashMap<String, String>) -> Result<ChatConfig> {
    let api_key = map
        .get("silicon_app_key")
        .filter(|s| !s.is_empty())
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("silicon_app_key 未配置"))?;

    let base_url = map_or(map, "silicon_app_base_url", DEFAULT_API_URL);
    let model = map_or(map, "silicon_app_model", DEFAULT_MODEL);
    let max_tokens = map_number(map, "silicon_app_max_tokens", 4096.0) as u32;
    let temperature = map_number(map, "silicon_app_temperature", 0.7);
    let top_p = map_number(map, "silicon_app_top_p", 0.7);
    let top_k = map_number(map, "silicon_app_topk", 50.0) as u32;

    let system_prompt = map
        .get("silicon_app_system_prompt")
        .filter(|s| !s.is_empty())
        .cloned();

    Ok(ChatConfig {
        api_key,
        base_url,
        model,
        max_tokens,
        temperature,
        top_p,
        top_k,
        system_prompt,
    })
}

/// 便捷：根据 ChatConfig + 用户 prompt 组装 ChatParams
pub fn build_params_from_config(cfg: &ChatConfig, prompt: String) -> ChatParams {
    let mut messages = Vec::new();

    if let Some(sys) = &cfg.system_prompt {
        messages.push(Message::system(sys.clone()));
    }
    messages.push(Message::user(prompt));

    ChatParams {
        model: cfg.model.clone(),
        messages,
        max_tokens: cfg.max_tokens,
        temperature: cfg.temperature,
        top_p: cfg.top_p,
        top_k: cfg.top_k,
        stream: false,
        thinking: Thinking {
            kind: "disabled".to_string(),
        },
    }
}
