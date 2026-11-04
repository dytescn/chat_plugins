# siliconflow_t2t

SiliconFlow 文本对话插件。Rust `cdylib`，通过 cyctron 的 `/libsiliconflow_t2t` 端点导出
`post_chat`。

## 包信息

| 项 | 内容 |
| --- | --- |
| 包名 | `siliconflow_t2t` |
| uuid | `c8b1acac-2b36-463b-84f7-c2170d31ab63` |
| 类型 | `sys` |
| 端点 | `/libsiliconflow_t2t` |
| 入口 | `post_chat` |
| 依赖 | `info_lib` |
| 配置前缀 | `silicon_app_*` |

## 配置

`setup.ts` 通过 `info_lib` 写入以下 `setting` key：

| key | 说明 | 初始化值 |
| --- | --- | --- |
| `silicon_app_key` | API Key | 占位符，必须替换 |
| `silicon_app_base_url` | API 基础地址 | `https://api.siliconflow.cn/v1` |
| `silicon_app_model` | 默认模型 | `Qwen/Qwen2.5-7B-Instruct` |
| `silicon_app_max_tokens` | 单次最大 token | `4096` |
| `silicon_app_temperature` | 采样温度 | `0.7` |
| `silicon_app_top_p` | 核采样阈值 | `0.7` |
| `silicon_app_topk` | Top-K | `40` |
| `silicon_app_system_prompt` | system 提示词 | 由 setting 读取 |

未配置时源码默认 `top_k = 50`；`setup.ts` 初始值为 `40`。

## 请求

```http
POST http://127.0.0.1:44944/libsiliconflow_t2t
Content-Type: application/libary
FFI-Symbol: chat
```

| 字段 | 类型 | 说明 |
| --- | --- | --- |
| `prompt` | string | 单轮输入；`messages` 为空时作为 user 消息 |
| `messages` | array | 多轮消息：`{role, content}`；非空时优先于 `prompt` |
| `system` | string | 请求级 system；未传时回落到 `silicon_app_system_prompt` |
| `model` | string | 覆盖 setting |
| `max_tokens` | number | 覆盖 setting |
| `temperature` | number | 覆盖 setting |
| `top_p` | number | 覆盖 setting |
| `top_k` | number | 覆盖 setting |
| `thinking` | object | `{ "type": "enabled" \| "disabled" }` |
| `session_id` | string | 业务会话标识；仅回传，不发送给上游 |

统一规则：

1. `messages` 非空时使用 `messages`，忽略 `prompt`；
2. `messages` 为空时回落 `prompt`，自动构造一条 user 消息；
3. 两者都没有有效内容时返回 `400102`；
4. `system` 优先级：请求 `system` > `silicon_app_system_prompt`。

## 响应

```json
{
  "code": 0,
  "msg": "success",
  "data": {
    "content": "模型回复",
    "id": "供应商响应 id",
    "model": "实际模型",
    "stop_reason": "stop",
    "usage": {},
    "session_id": "请求回显"
  }
}
```

## 错误码

| 码 | 含义 |
| --- | --- |
| `400101` | 请求 JSON 解析失败 |
| `400102` | `prompt` / `messages` 没有有效内容 |
| `500100` | 运行时创建失败 |
| `500101` | 读取配置或调用 SiliconFlow 失败 |
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

发布产物见 `dytes.json` 的 `platform-exports`；真实调用需要配置
`silicon_app_key` 并启动 cyctron。

统一调用协议见 [docs/protocol.md](docs/protocol.md)。
