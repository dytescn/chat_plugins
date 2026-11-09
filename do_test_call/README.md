# do_test_call

测试 `chatllm.call_chat` 的插件：被调用后回调 `manager_baseinfo`（实际执行
`info_lib` 的 `info_list`），返回 `info.db.setting` 的信息条数。

| 项 | 内容 |
| --- | --- |
| 包名 | `do_test_call` |
| 版本 | `0.1.0` |
| 类型 | `design_plugins`（`plugins_type: plugins`） |
| 适用 | `targets: ["common"]` |
| 导出符号 | `post_chat`（`FFI-Symbol: chat`） |
| 底层依赖 | `./libary/info_lib.{dll,so,dylib}` |
| HTTP 端点 | `POST http://127.0.0.1:44944/do_test_call` |
| MCP | `mcp/mcp.json` → 安装为 `mcp.json` |
| Prompts | `prompts/do_test_call.json` → 安装为 `prompts.json` |

## 为什么直接调用 info_lib

`manager_baseinfo` 是纯 MCP 资源包，本身没有动态库，它的工具最终都指向
`info_lib` 的 FFI。插件运行在 cyctron 的同步主线程里，如果用 HTTP 再回调同一个
cyctron，主线程会等待自己返回，形成死锁。因此本插件直接 `dlopen`
`./libary/info_lib.*` 并调用 `post_info_list`，等价于 manager_baseinfo 的
`info_info_list` 工具。

## 请求

```http
POST http://127.0.0.1:44944/do_test_call
Content-Type: application/libary
FFI-Symbol: chat
```

支持 `call_chat` 的四字段透传：

```json
{
  "plugins_uid": "6f2b8d41-9c3a-4e7d-8d5f-1a2c3e4b5d60",
  "category": "chat_plugin",
  "prompts": "查询有多少信息",
  "context": { "category": "base", "keyword": "语言" }
}
```

- `plugins_uid` / `category` / `prompts` 只回显；
- `context.category` / `context.keyword` 作为 `info_list` 的过滤条件，省略时统计全部；
- 也兼容普通 chat 的 `prompt` / `messages`，仅用于回显。

## 响应

```json
{
  "code": 0,
  "data": {
    "content": "manager_baseinfo: 共 12 条信息",
    "count": 12,
    "total": 12,
    "tag": "do_test_call",
    "endpoint": "info_lib",
    "symbol": "info_list",
    "input": "查询有多少信息",
    "plugins_uid": "6f2b8d41-9c3a-4e7d-8d5f-1a2c3e4b5d60",
    "call_category": "chat_plugin",
    "category": null,
    "keyword": null,
    "prompts": "查询有多少信息"
  },
  "msg": "success"
}
```

`count` / `total` 来自 `info_list` 的 `data.total`；请求固定 `page=1`、`page_size=1`，
只取总数，避免返回大列表。

## 错误码

| 码 | 含义 |
| --- | --- |
| `400100` | `json_in` 为空 |
| `400101` | 请求不是 UTF-8 |
| `400102` | 请求 JSON 非法 |
| `500100` | 找不到 / 加载 `info_lib` 失败，或 `info_list` 返回失败 |

## 与 call_chat 的配合

`plugin_lib scan` 会把本包的 MCP 工具导入 `chat_plugin` 集合，并把
`ffi_symbol` 固定为 `chat`。因此可以这样直接调用：

```bash
curl -s -X POST http://127.0.0.1:44944/chatllm \
  -H "Content-Type: application/libary" \
  -H "FFI-Symbol: call_chat" \
  -d '{"category":"chat_plugin","prompts":"查询有多少信息","context":{}}'
```

## 构建与测试

```bash
cargo build --release
cargo test
```

产物：`target/release/do_test_call.dll`（Windows）、`libdo_test_call.so`（Linux）、
`libdo_test_call.dylib`（macOS）。
