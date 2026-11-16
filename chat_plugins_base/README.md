# chat_plugins_base

插件阶段（`plugins`）的基础能力插件。

- 包类型：`plugins_type: chat_plugins`
- 入口：`post_chat`，输入单个字符串（`input`；兼容 chatllm 下发的 `prompt`）
- 输出：`data.content = "插件" + 输入`，`data.input` 透传输入
- 命中方式：think 输出的关键词检索 `chat_plugins` 分类，命中本插件的 `prompts.json` 后执行 `taskflow.json` 回调

## 请求与响应

```json
{ "input": "你好" }
```

```json
{
  "code": 0,
  "data": {
    "content": "插件你好",
    "input": "你好"
  },
  "msg": "success"
}
```

输入为空返回 `400102`。

## 包内容

```text
chat_plugins_base/
├── Cargo.toml
├── dytes.json
├── mcp.json              # 单字符串入参，输出 content / input
├── prompts.json          # chat_plugins 检索关键词：base / 插件 / 问候
├── taskflow.json         # chat_plugins 必需，ffi_symbol = post_chat
└── src/
```

- `taskflow.json` 使用 `plugin_uuid` / `plugin_name` / `desc` / `ffi_symbol` 四个字段；
- `prompts.json` 每个元素含 `id` / `name` / `content`，`content` ≤ 150 字节；
- `mcp.json` 的 `inputSchema` 只有 `input` 一个字符串字段。

## 测试

```bash
cargo build --release
dytes test .
cd ../ts
deno task ffi --lib ../chat_plugins_base/target/release/chat_plugins_base.dll \
  --input '{"input":"你好"}'
```

预期 `data.content = "插件你好"`。
