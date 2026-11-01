# deepseek

DeepSeek 文本对话插件。Rust `cdylib`，通过 cyctron 的 `/libdeepseek` 端点导出
`post_chat`，使用 DeepSeek 的 Anthropic 兼容接口。

## 包信息

| 项 | 内容 |
| --- | --- |
| 包名 | `deepseek` |
| uuid | `c39f6e4d-042a-4b3e-bc8d-2a7c3336736c` |
| 类型 | `sys` |
| 端点 | `/libdeepseek` |
| 入口 | `post_chat` |
| 依赖 | `info_lib` |
| 配置前缀 | `deepseek_app_*` |

## 配置

`setup.ts` 通过 `info_lib` 写入以下 `setting` key：

| key | 说明 | 默认值 |
| --- | --- | --- |
| `deepseek_app_key` | API Key | 占位符，必须替换 |
| `deepseek_app_base_url` | Anthropic 兼容端点 | `https://api.deepseek.com/anthropic` |
| `deepseek_app_model` | 默认模型 | `deepseek-flash` |
| `deepseek_app_max_tokens` | 单次最大 token | `4096` |
| `deepseek_app_temperature` | 采样温度 | `0.7` |
| `deepseek_app_system_prompt` | system 提示词 | `你是一个乐于助人的中文 AI 助手。` |
| `deepseek_app_thinking` | 思考模式 | `disabled` |
| `deepseek_app_reasoning_effort` | 推理强度 | `high` |

> 当前 `post_chat` 的 system 只取请求里的 `system` 字段；`deepseek_app_system_prompt`
> 会被 `setup.ts` 初始化，但尚未接入请求组装。

## 请求

```http
POST http://127.0.0.1:44944/libdeepseek
Content-Type: application/libary
FFI-Symbol: chat
```

| 字段 | 类型 | 说明 |
| --- | --- | --- |
| `prompt` | string | 单轮输入；与 `messages` 二选一 |
| `messages` | array | 多轮消息：`{role, content}` |
| `system` | string | 请求级 system；为空则不插入 |
| `model` | string | 覆盖 setting 中的模型 |
| `max_tokens` | number | 覆盖 setting |
| `temperature` | number | 覆盖 setting |
| `thinking` | object | `{ "type": "enabled" \| "disabled" }` |
| `reasoning_effort` | string | `high` / `max` |

`prompt` 和 `messages` 都提供时，优先使用 `messages`。

## 响应

```json
{
  "code": 0,
  "msg": "success",
  "data": { "content": "模型回复" }
}
```

DeepSeek 插件只返回 `content`；不返回上游 `id`、`usage`。

## 错误码

| 码 | 含义 |
| --- | --- |
| `400101` | 请求 JSON 解析失败 |
| `400102` | `prompt` / `messages` 为空或缺失 |
| `500100` | 运行时创建失败 |
| `500101` | 读取配置或调用 DeepSeek 失败 |
| `500102` | 上游 API 返回错误 |
| `500103` | 上游响应解析失败 |
| `500104` | 上游返回空内容 |
| `500900` | `json_out` 为空 |
| `500901` | 响应超过 64 KiB |

## 构建与测试

```bash
cargo build --release
cargo test
```

发布产物与版本由 `dytes.json` 管理；真实调用需要在 `info.db` 中配置
`deepseek_app_key`，并启动 cyctron。

## API 文档

- [Chat Completions API](docs/create-chat-completion.md)
- [图像理解](docs/vision.md)
- [思考模式](docs/thinking_mode.md)
- [多轮对话](docs/multi_round_chat.md)
- [对话前缀续写](docs/chat_prefix_completion.md)
- [FIM 补全](docs/fim_completion.md)
- [JSON Output](docs/json_mode.md)
- [Tool Calls](docs/tool_calls.md)
- [Files API](docs/files_api.md)
- [文档索引与来源](docs/README.md)
