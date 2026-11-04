# do_test_chat

最简单的测试聊天插件。Rust `cdylib`，作为 `design_plugins` 安装到
`plugins/do_test_chat/`，由 cyctron 通过 `POST /do_test_chat` + `FFI-Symbol: chat` 调用。

## 行为

传入什么就返回：

```text
content = 输入 + do_chat_test + 时间（Unix 毫秒）
```

不访问网络、不读写数据库、不调用供应商。

## 请求

```http
POST http://127.0.0.1:44944/do_test_chat
Content-Type: application/libary
FFI-Symbol: chat
```

```json
{
  "prompt": "你好"
}
```

也支持多轮：

```json
{
  "messages": [
    { "role": "user", "content": "第一句" },
    { "role": "user", "content": "第二句" }
  ]
}
```

输入优先级：

1. 非空 `prompt`；
2. 最后一条非空 `user` 消息；
3. 最后一条非空消息。

两者都没有有效内容时返回 `400102`。

## 响应

```json
{
  "code": 0,
  "data": {
    "content": "你好do_chat_test1793780000000",
    "input": "你好",
    "tag": "do_chat_test",
    "time": 1793780000000
  },
  "msg": "success"
}
```

## Prompts

`prompts/do_test_chat.json` 是测试用短词组数组，安装时导出为 `prompts.json`。

## MCP

`dytes.json` 声明 `"mcp": "mcp.json"`。安装后文件位于：

```text
plugins/do_test_chat/
├── do_test_chat.dll
├── prompts.json
├── mcp.json
└── chat.json
```

- `mcp.json`：`{"tools":["chat.json"]}`
- `chat.json`：MCP 工具 `do_test_chat`
  - endpoint：`/do_test_chat`
  - ffiSymbol：`chat`
  - 输入：`prompt` 或 `messages`
  - 输出：`content` / `input` / `tag` / `time`

## 构建与测试

```bash
cargo build --release
cargo test
```

产物：`target/release/do_test_chat.dll`（Windows），由 `dytes.json` 的
`platform-exports` 决定发布名。
