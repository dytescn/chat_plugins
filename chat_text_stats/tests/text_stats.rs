// tests/text_stats.rs
use chat_text_stats::chat::ffi::stats;

#[test]
fn counts_basic_stats() {
    let text = "你好。今天天气不错！\n我们聊聊天气。";
    let result = stats(text);
    assert_eq!(result["lines"], 2);
    assert!(result["chars"].as_u64().unwrap() > 0);
    assert!(result["sentences"].as_u64().unwrap() >= 2);
}

#[test]
fn top_words_are_ranked() {
    let text = "天气 天气 天气 聊天";
    let result = stats(text);
    assert_eq!(result["top_words"][0]["word"], "天气");
    assert_eq!(result["top_words"][0]["count"], 3);
}
