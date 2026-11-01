# chat_ioc

对话内容处理插件：从对话文本中提取 IOC（IP / 域名 / 文件哈希 / URL）。

- 入口：`post_chat`，请求 `{"prompt": "..."}` 或 `{"messages": [...]}`。
- 响应：`{ "code": 0, "data": { "content": "...", "iocs": { "ips", "domains", "hashes", "urls", "count" } } }`

参考来源：`D:\codes\other_code\Anthropic-Cybersecurity-Skills` 的
`skills/analyzing-indicators-of-compromise`（本地规则实现，不调用模型）。

```bash
cargo build --release
cargo test
```
