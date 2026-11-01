// src/chat/supplier.rs
// ai_supplier 解析：候选供应商、必需配置与状态判定。
use std::collections::HashMap;

use serde_json::{json, Value};

#[derive(Debug, Clone, Copy)]
pub struct ConfigField {
    pub key: &'static str,
    pub title: &'static str,
    pub value_type: &'static str,
    pub description: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct SupplierSpec {
    pub value: &'static str,
    pub label: &'static str,
    pub endpoint: &'static str,
    pub required: &'static [ConfigField],
    pub optional: &'static [ConfigField],
}

const DEEPSEEK_REQUIRED: &[ConfigField] = &[ConfigField {
    key: "deepseek_app_key",
    title: "应用 Key",
    value_type: "secret",
    description: "DeepSeek API Key",
}];

const DEEPSEEK_OPTIONAL: &[ConfigField] = &[
    ConfigField {
        key: "deepseek_app_base_url",
        title: "API 地址",
        value_type: "url",
        description: "默认 https://api.deepseek.com/anthropic",
    },
    ConfigField {
        key: "deepseek_app_model",
        title: "默认模型",
        value_type: "string",
        description: "默认 deepseek-flash",
    },
    ConfigField {
        key: "deepseek_app_max_tokens",
        title: "最大 Tokens",
        value_type: "number",
        description: "默认 4096",
    },
    ConfigField {
        key: "deepseek_app_temperature",
        title: "温度",
        value_type: "number",
        description: "默认 0.7",
    },
];

const SILICON_REQUIRED: &[ConfigField] = &[ConfigField {
    key: "silicon_app_key",
    title: "应用 Key",
    value_type: "secret",
    description: "SiliconFlow API Key",
}];

const SILICON_OPTIONAL: &[ConfigField] = &[
    ConfigField {
        key: "silicon_app_base_url",
        title: "API 地址",
        value_type: "url",
        description: "默认 https://api.siliconflow.cn/v1",
    },
    ConfigField {
        key: "silicon_app_model",
        title: "默认模型",
        value_type: "string",
        description: "默认 Qwen/Qwen2.5-7B-Instruct",
    },
    ConfigField {
        key: "silicon_app_max_tokens",
        title: "最大 Tokens",
        value_type: "number",
        description: "默认 4096",
    },
    ConfigField {
        key: "silicon_app_temperature",
        title: "温度",
        value_type: "number",
        description: "默认 0.7",
    },
];

pub const SUPPLIERS: &[SupplierSpec] = &[
    SupplierSpec {
        value: "deepseek",
        label: "DeepSeek",
        endpoint: "/libdeepseek",
        required: DEEPSEEK_REQUIRED,
        optional: DEEPSEEK_OPTIONAL,
    },
    SupplierSpec {
        value: "siliconflow_t2t",
        label: "SiliconFlow",
        endpoint: "/libsiliconflow_t2t",
        required: SILICON_REQUIRED,
        optional: SILICON_OPTIONAL,
    },
];

pub fn lookup(name: &str) -> Option<&'static SupplierSpec> {
    SUPPLIERS.iter().find(|spec| spec.value == name)
}

/// 未配置 `ai_supplier` 时使用的默认供应商。
pub fn default_supplier() -> &'static str {
    "deepseek"
}

/// 把 `ai_supplier` 的取值映射到具体插件。
/// `siliconflow` 是 setting 中的供应商名，对应插件 `siliconflow_t2t`。
pub fn normalize(name: &str) -> Option<&'static SupplierSpec> {
    match name.trim() {
        "siliconflow" => lookup("siliconflow_t2t"),
        other => lookup(other),
    }
}

pub fn supplier_names() -> Vec<&'static str> {
    SUPPLIERS.iter().map(|spec| spec.value).collect()
}

/// 候选供应商的 JSON 描述，供宿主渲染选择。
pub fn suppliers_json() -> Value {
    Value::Array(
        SUPPLIERS
            .iter()
            .map(|spec| {
                json!({
                    "value": spec.value,
                    "label": spec.label,
                    "endpoint": spec.endpoint,
                    "required": fields_json(spec.required),
                    "optional": fields_json(spec.optional),
                })
            })
            .collect(),
    )
}

fn fields_json(fields: &[ConfigField]) -> Value {
    Value::Array(
        fields
            .iter()
            .map(|field| {
                json!({
                    "key": field.key,
                    "title": field.title,
                    "value_type": field.value_type,
                    "description": field.description,
                })
            })
            .collect(),
    )
}

fn field_json(field: &ConfigField) -> Value {
    json!({
        "key": field.key,
        "title": field.title,
        "value_type": field.value_type,
        "description": field.description,
        "required": true,
    })
}

/// 空值或 setup 占位符都视为未配置。
fn is_unconfigured(value: Option<&String>) -> bool {
    match value {
        None => true,
        Some(raw) => {
            let value = raw.trim();
            value.is_empty() || value.contains("你的") || value.contains("your-key")
        }
    }
}

#[derive(Debug)]
pub enum Resolution {
    NeedSupplier,
    InvalidSupplier(String),
    NeedConfig {
        spec: &'static SupplierSpec,
        missing: Vec<&'static ConfigField>,
        config: Value,
    },
    Ready {
        spec: &'static SupplierSpec,
        config: Value,
    },
}

/// 根据已选供应商与 setting 值判定下一步。
pub fn resolve(selected: Option<&str>, values: &HashMap<String, String>) -> Resolution {
    let Some(name) = selected.map(str::trim).filter(|v| !v.is_empty()) else {
        return Resolution::NeedSupplier;
    };
    let Some(spec) = normalize(name) else {
        return Resolution::InvalidSupplier(name.to_string());
    };

    let missing: Vec<&'static ConfigField> = spec
        .required
        .iter()
        .filter(|field| is_unconfigured(values.get(field.key)))
        .collect();

    let config = optional_config_json(spec, values);

    if missing.is_empty() {
        Resolution::Ready { spec, config }
    } else {
        Resolution::NeedConfig {
            spec,
            missing,
            config,
        }
    }
}

fn optional_config_json(spec: &SupplierSpec, values: &HashMap<String, String>) -> Value {
    let mut map = serde_json::Map::new();
    for field in spec.optional {
        if let Some(value) = values.get(field.key) {
            if !value.trim().is_empty() {
                map.insert(field.key.to_string(), json!(value));
            }
        }
    }
    Value::Object(map)
}

pub fn missing_json(missing: &[&'static ConfigField]) -> Value {
    Value::Array(missing.iter().map(|field| field_json(field)).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn values(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn empty_selection_needs_supplier() {
        assert!(matches!(
            resolve(None, &HashMap::new()),
            Resolution::NeedSupplier
        ));
        assert!(matches!(
            resolve(Some("  "), &HashMap::new()),
            Resolution::NeedSupplier
        ));
    }

    #[test]
    fn unknown_selection_is_invalid() {
        assert!(matches!(
            resolve(Some("openai"), &HashMap::new()),
            Resolution::InvalidSupplier(name) if name == "openai"
        ));
    }

    #[test]
    fn deepseek_without_key_needs_config() {
        let result = resolve(Some("deepseek"), &HashMap::new());
        match result {
            Resolution::NeedConfig { missing, .. } => {
                assert_eq!(missing.len(), 1);
                assert_eq!(missing[0].key, "deepseek_app_key");
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn placeholder_key_needs_config() {
        let result = resolve(
            Some("deepseek"),
            &values(&[("deepseek_app_key", "sk-你的deepseek-key")]),
        );
        assert!(matches!(result, Resolution::NeedConfig { .. }));
    }

    #[test]
    fn siliconflow_alias_maps_to_plugin() {
        let result = resolve(
            Some("siliconflow"),
            &values(&[("silicon_app_key", "sk-real")]),
        );
        match result {
            Resolution::Ready { spec, .. } => assert_eq!(spec.value, "siliconflow_t2t"),
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn openai_is_not_supported() {
        assert!(matches!(
            resolve(Some("openai"), &HashMap::new()),
            Resolution::InvalidSupplier(name) if name == "openai"
        ));
    }

    #[test]
    fn configured_supplier_is_ready() {
        let result = resolve(
            Some("siliconflow_t2t"),
            &values(&[("silicon_app_key", "sk-real")]),
        );
        match result {
            Resolution::Ready { spec, config } => {
                assert_eq!(spec.value, "siliconflow_t2t");
                assert_eq!(config, json!({}));
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn ready_includes_optional_config() {
        let result = resolve(
            Some("deepseek"),
            &values(&[
                ("deepseek_app_key", "sk-real"),
                ("deepseek_app_model", "deepseek-flash"),
            ]),
        );
        match result {
            Resolution::Ready { config, .. } => {
                assert_eq!(config["deepseek_app_model"], "deepseek-flash");
            }
            other => panic!("unexpected: {other:?}"),
        }
    }
}
