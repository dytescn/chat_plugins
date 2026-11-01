// src/utils/setting.rs
// info.db setting 表的只读访问。
use std::collections::{HashMap, HashSet};

use anyhow::Result;

use crate::utils::db::db_query;

fn esc(val: &str) -> String {
    format!("'{}'", val.replace('\'', "''"))
}

/// 读取单个 setting；不存在时返回 None。
pub fn get_setting(key: &str) -> Result<Option<String>> {
    let sql = format!("SELECT value FROM setting WHERE key = {} LIMIT 1", esc(key));
    let rows = db_query(&sql)?;
    Ok(rows.first().and_then(|row| {
        row.get("value")
            .and_then(|v| v.as_str())
            .map(str::to_string)
    }))
}

/// 批量读取 setting，返回 key -> value。
pub fn get_settings(keys: &[&str]) -> Result<HashMap<String, String>> {
    if keys.is_empty() {
        return Ok(HashMap::new());
    }

    let mut seen: HashSet<&str> = HashSet::with_capacity(keys.len());
    let mut list = String::with_capacity(keys.len() * 24);
    let mut first = true;
    for key in keys {
        if !seen.insert(*key) {
            continue;
        }
        if !first {
            list.push_str(", ");
        }
        list.push_str(&esc(key));
        first = false;
    }

    let sql = format!("SELECT key, value FROM setting WHERE key IN ({list})");
    let rows = db_query(&sql)?;

    let mut map = HashMap::with_capacity(rows.len());
    for row in &rows {
        if let (Some(key), Some(value)) = (
            row.get("key").and_then(|v| v.as_str()),
            row.get("value").and_then(|v| v.as_str()),
        ) {
            map.insert(key.to_string(), value.to_string());
        }
    }
    Ok(map)
}
