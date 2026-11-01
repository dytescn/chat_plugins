# chat_text_stats

对话内容处理插件：统计对话文本的字数、行数、句子数与高频词。

- 入口：`post_chat`，请求 `{"prompt": "..."}` 或 `{"messages": [...]}`。
- 响应：`{ "code": 0, "data": { "content": "...", "stats": { "chars", "lines", "sentences", "top_words" } } }`

参考来源：`D:\codes\other_code\claude-plugins-official` 中 learning-/explanatory-output-style 对输出结构的整理
（本地规则实现，不调用模型）。

```bash
cargo build --release
cargo test
```
