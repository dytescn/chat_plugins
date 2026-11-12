# do_test_chat

确定性的测试聊天插件。Rust `cdylib`，作为 `design_plugins` 安装到
`plugins/do_test_chat/`，由 cyctron 通过 `POST /do_test_chat` + `FFI-Symbol: chat`
调用。

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

输入优先级与统一协议一致：

1. `messages` 非空时优先：最后一条非空 `user` 消息（`role` 不区分大小写）；
2. 没有 `user` 消息时，取最后一条非空消息；
3. `messages` 为空或没有有效内容时，回落非空 `prompt`；
4. 两者都没有有效内容时返回 `400102`。

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

## 包内容

```text
do_test_chat/
├── dytes.json
├── prompts.json          # plugin_lib 规范的对象数组
├── taskflow.json         # chat_plugins 必需
├── mcp.json              # plugin_lib 规范的对象数组
├── README.md
├── src/
└── tests/
```

- `prompts.json`：安装到 designdb `prompts`，并按 `plugins_type = chat_plugins`
  同步到 kv 的 `chat_plugins` 分类；每条 `content` 不超过 150 字节。
- `taskflow.json`：任务流 `4563e8e6-b965-472e-9a22-c1f8aace423c:do_test_chat`，
  `uid` 与 `mcp.json` 中能力 `uuid` 一致；`callback` 指向插件回调方法 `chat`
  （即 `ffi_symbol`）。安装后写入共享任务流文档。
- `mcp.json`：`tools` 是对象数组，声明能力 `do_test_chat`（`ffi_symbol = chat`）。
  `transport` 为扩展字段，记录 endpoint 与 FFI 符号。

安装后目录：

```text
plugins/do_test_chat/
├── do_test_chat.dll
├── prompts.json
├── taskflow.json
├── mcp.json
└── dytes.json
```

### 与 dytes 清单校验的差异

dytes 的旧版清单校验仍把 `mcp.json` 的 `tools` 当作文件名数组，而 `plugin_lib`
要求对象数组。因此 `mcp.json` 使用 `plugin_lib` 格式，发布 workflow 传入
`--skip-check` 跳过 dytes 的旧校验。

## 构建与测试

```bash
cargo build --release
cargo test
```

产物：`target/release/do_test_chat.dll`（Windows），由 `dytes.json` 的
`platform-exports` 决定发布名。
