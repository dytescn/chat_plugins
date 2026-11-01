// src/chat/ffi.rs
// 对话内容处理：统计文本的字数、行数、句子数与高频词。
// 参考：other_code/claude-plugins-official 的 learning-/explanatory-output-style 对输出的结构化整理
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::os::raw::{c_char, c_int};

use crate::utils::cjson::{parse_json, write_error_json, write_success_json};
use crate::utils::errcode::{self, ERR_CHAT_JSON_PARSE, ERR_CHAT_PROMPT_REQUIRED};

#[derive(Debug, Deserialize, Default)]
struct Message {
    #[serde(default)]
    role: String,
    #[serde(default)]
    content: String,
}

#[derive(Debug, Deserialize, Default)]
struct ChatReq {
    prompt: Option<String>,
    messages: Option<Vec<Message>>,
}

fn pick_input(req: &ChatReq) -> Option<String> {
    if let Some(prompt) = req.prompt.as_ref() {
        if !prompt.trim().is_empty() {
            return Some(prompt.clone());
        }
    }
    let messages = req.messages.as_ref()?;
    if let Some(message) = messages
        .iter()
        .rev()
        .find(|message| message.role == "user" && !message.content.trim().is_empty())
    {
        return Some(message.content.clone());
    }
    messages
        .iter()
        .rev()
        .find(|message| !message.content.trim().is_empty())
        .map(|message| message.content.clone())
}

fn respond_error(json_out: *mut c_char, code: i32, msg: &str) -> c_int {
    let _ = write_error_json(json_out, code, msg);
    errcode::OK
}

/// 统计文本：`{chars, lines, sentences, words, top_words}`。
pub fn stats(text: &str) -> Value {
    let chars = text.chars().count();
    let lines = text.lines().count();
    let sentences = text
        .chars()
        .filter(|c| "。！？.!?".contains(*c))
        .count()
        .max(if chars > 0 { 1 } else { 0 });

    let mut freq: HashMap<&str, usize> = HashMap::new();
    for word in text
        .split(|c: char| c.is_whitespace() || "，。！？、；：,.!?;:()（）".contains(c))
        .filter(|word| word.chars().count() >= 2)
    {
        *freq.entry(word).or_insert(0) += 1;
    }

    let mut ranked: Vec<(&str, usize)> = freq.into_iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    let top_words: Vec<Value> = ranked
        .into_iter()
        .take(5)
        .map(|(word, count)| json!({ "word": word, "count": count }))
        .collect();

    json!({
        "chars": chars,
        "lines": lines,
        "sentences": sentences,
        "top_words": top_words,
    })
}

#[no_mangle]
pub extern "C" fn post_chat(json_in: *const c_char, json_out: *mut c_char) -> c_int {
    let req: ChatReq = match parse_json(json_in) {
        Ok(value) => value,
        Err(_) => {
            let code = ERR_CHAT_JSON_PARSE;
            return respond_error(json_out, code, errcode::default_message(code));
        }
    };

    let input = match pick_input(&req) {
        Some(value) => value,
        None => {
            let code = ERR_CHAT_PROMPT_REQUIRED;
            return respond_error(json_out, code, errcode::default_message(code));
        }
    };

    let result = stats(&input);
    let content = format!(
        "对话内容统计：{} 字、{} 行、约 {} 句。",
        result["chars"], result["lines"], result["sentences"]
    );

    write_success_json(
        json_out,
        "success",
        json!({ "content": content, "stats": result, "model": "chat_text_stats" }),
    )
}
