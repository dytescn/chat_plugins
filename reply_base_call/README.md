# reply_base_call

回复阶段（`reply`）的基础能力插件。

- 包类型：`plugins_type: chat_reply`
- 入口：`post_chat`，输入单个字符串（`input`；兼容 chatllm 下发的 `prompt`）
- 输出：`data.content = "回复" + 输入`，`data.input` 原样回传
- 命中方式：`chat_reply` 默认关键词 `base` 命中本插件的 `prompts.json`

## 请求与响应

```json
{ "input": "插件你好" }
```

```json
{
  "code": 0,
  "data": {
    "content": "回复插件你好",
    "input": "插件你好"
  },
  "msg": "success"
}
```

输入为空返回 `400102`。

## 包内容

```text
reply_base_call/
├── Cargo.toml
├── dytes.json
├── mcp.json              # 单字符串入参，输出 content / input
├── prompts.json          # chat_reply 关键词：base、回复/回答/结果
├── taskflow.json         # 回调 post_chat
├── skills/
│   └── reply_base_call.md
└── src/
```

- `chat_reply` 包必须带 `skills/*.md`；
- `prompts.json` 每个元素含 `id` / `name` / `content`，`content` ≤ 150 字节；
- `taskflow.json` 使用 `plugin_uuid` / `plugin_name` / `desc` / `ffi_symbol` 四个字段；
- `mcp.json` 的 `inputSchema` 只有 `input` 一个字符串字段。

## 测试

```bash
cargo build --release
dytes test .
cd ../ts
deno task ffi --lib ../reply_base_call/target/release/reply_base_call.dll \
  --input '{"input":"插件你好"}'
```

预期 `data.content = "回复插件你好"`。
