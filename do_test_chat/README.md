# do_test_chat

确定性的测试聊天插件。Rust `cdylib`，作为 `design_plugins` 包安装到
`plugins/do_test_chat/`，由 cyctron 以 `POST /do_test_chat` + `FFI-Symbol: chat`
加载，用于端到端测试 `chat_ui / 宿主 → chatllm → 模型插件` 链路。
配置完整时会真实转发到已安装的供应商插件（deepseek / siliconflow_t2t）。

## 包信息

| 项 | 内容 |
| --- | --- |
| 包名 | `do_test_chat` |
| uuid | `4563e8e6-b965-472e-9a22-c1f8aace423c` |
| 版本 | `0.1.2` |
| 类型 | `design_plugins` |
| 安装目录 | `plugins/do_test_chat/` |
| 端点 | `/do_test_chat` |
| 入口 | `post_chat` |
| 依赖 | 无 |

## 请求

```http
POST http://127.0.0.1:44944/do_test_chat
Content-Type: application/libary
FFI-Symbol: chat
```

```json
{
  "prompt": "你好",
  "messages": [{ "role": "user", "content": "多轮输入" }],
  "system": "可选，原样回显",
  "model": "可选，原样回显",
  "session_id": "可选，原样回显",
  "reply": "可选，指定固定回复",
  "fail_code": 500101,
  "fail_msg": "可选，配合 fail_code 模拟失败",
  "ai_supplier": "可选，临时指定供应商；缺省读取 info.db，未配置时默认 deepseek",
  "no_ai": false
}
```

- `prompt` 与 `messages` 至少提供一个；两者都会转发给已安装的供应商；
- `ai_supplier` 未配置时默认 `deepseek`；对应 token 未配置时返回错误；
- `no_ai: true` 跳过 AI 配置，直接返回确定性内容（仅测试用）；
- `reply` 非空时在 `no_ai` 模式下作为固定回复，便于断言；
- `fail_code` 非 0 时不生成回复，直接返回该错误；
- `ai_supplier` 只覆盖本次请求，不写回 `info.db`。

## ai_supplier 流程

插件读取 `info.db` 的 `setting.ai_supplier`（请求字段优先）；
未配置时默认 `deepseek`。随后检查对应 token：

| 情况 | 行为 |
| --- | --- |
| token 未配置 | 返回错误 `500102`：`AI 配置错误：未配置 deepseek token：deepseek_app_key` |
| 供应商未知 | 返回错误 `500102`，提示可选 `deepseek` / `siliconflow_t2t` |
| 读取 `info.db` 失败 | 返回错误 `500101` |
| 配置完整 | **真实转发**到对应插件，返回其 `content` 与 `upstream` |

`ai_supplier` 取值到插件的映射：

| `ai_supplier` | 插件 | label | endpoint | 动态库 | 必填 |
| --- | --- | --- | --- | --- | --- |
| `deepseek`（默认） | `deepseek` | DeepSeek | `/libdeepseek` | `./libary/libdeepseek.dll` | `deepseek_app_key` |
| `siliconflow_t2t`（或 `siliconflow`） | `siliconflow_t2t` | SiliconFlow | `/libsiliconflow_t2t` | `./libary/libsiliconflow_t2t.dll` | `silicon_app_key` |
| `openai` | 未支持 | — | — | — | — |

配置完整时插件通过 `dlopen` 调用供应商的 `post_chat`，请求体透传
`prompt` / `messages` / `system` / `model` / `session_id`；供应商自己读取
`info.db` 中的配置。插件不写 setting。

## 响应

未配置 token 时（默认 deepseek）：

```json
{
  "code": 500102,
  "data": {},
  "msg": "AI 配置错误：未配置 deepseek token：deepseek_app_key"
}
```

配置完整时返回 `status: "ready"`、`supplier`、`endpoint`、非敏感的 `config`，
以及供应商回复：

```json
{
  "code": 0,
  "data": {
    "status": "ready",
    "supplier": "siliconflow_t2t",
    "endpoint": "/libsiliconflow_t2t",
    "content": "模型回复",
    "upstream": { "id": "msg_...", "model": "...", "usage": {} }
  },
  "msg": "success"
}
```

供应商返回空内容时 `code = 500104`；调用失败时 `code = 500101`。
错误码沿用仓库约定：解析失败 `400101`，输入为空 `400102`，输出超限 `500901`。

## 构建与测试

```bash
cargo build --release   # target/release/do_test_chat.dll
cargo test
```

产物在 Windows 为 `do_test_chat.dll`，Linux / macOS 为
`do_test_chat.so` / `do_test_chat.dylib`（由 `dytes.json` 的导出名决定）。

## 在 plugintest 中使用

```bash
cd ../plugintest
dytes update
# 本地调试时也可直接部署到设计插件目录
mkdir -p plugins/do_test_chat
cp ../do_test_chat/target/release/do_test_chat.dll plugins/do_test_chat/do_test_chat.dll
./cyctron.exe start
```

```bash
curl -s -X POST http://127.0.0.1:44944/do_test_chat \
  -H "Content-Type: application/libary" \
  -H "FFI-Symbol: chat" \
  -d '{"prompt":"hello","reply":"pong"}'
```
