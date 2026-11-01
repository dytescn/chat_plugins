# chat_code_check

对话内容处理插件：对对话中的代码片段做本地检查。

- 检查项：`TODO` / `FIXME` / `XXX`、调试输出（`console.log`、`dbg!`、`println!`、`print(`）、疑似硬编码密钥、超过 120 字符的行。
- 入口：`post_chat`，请求 `{"prompt": "..."}` 或 `{"messages": [...]}`。
- 响应：`{ "code": 0, "data": { "content": "...", "issues": [{ "line", "kind", "message" }] } }`

参考来源：`D:\codes\other_code\claude-plugins-official` 的 `plugins/code-review` 与
`plugins/code-simplifier`（本地规则实现，不调用模型）。

```bash
cargo build --release
cargo test
```
