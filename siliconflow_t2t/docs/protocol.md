# 聊天插件统一调用协议

适用：`deepseek`、`siliconflow_t2t`。

两个插件都通过 cyctron 暴露 `post_chat`，请求与响应尽量保持一致；
供应商独有的字段在文末单独列出。

## 端点

```http
POST http://127.0.0.1:44944/<endpoint>
Content-Type: application/libary
FFI-Symbol: chat
```

| 插件 | endpoint | 入口 |
| --- | --- | --- |
| deepseek | `/libdeepseek` | `post_chat` |
| siliconflow_t2t | `/libsiliconflow_t2t` | `post_chat` |

## 请求

### 通用字段

| 字段 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- |
| `prompt` | string | 条件 | 单轮输入；`messages` 为空时作为 user 消息 |
| `messages` | array | 条件 | 多轮消息：`{ role, content }`；优先于 `prompt` |
| `system` | string | 否 | 请求级 system；未传时回落 setting 中的 `*_app_system_prompt` |
| `model` | string | 否 | 覆盖 setting 中的默认模型 |
| `max_tokens` | number | 否 | 覆盖 setting |
| `temperature` | number | 否 | 覆盖 setting |
| `thinking` | object | 否 | `{ "type": "enabled" \| "disabled" }` |
| `session_id` | string | 否 | 业务会话标识；仅回显，不发送给上游 |

### messages / prompt 规则

1. `messages` 非空：使用 `messages`，忽略 `prompt`；
2. `messages` 为空或未传，且 `prompt` 有非空内容：自动构造一条 user 消息；
3. 两者都没有有效内容：返回 `400102`；
4. `system` 优先级：请求 `system` > setting 的 `*_app_system_prompt`；
5. 如果 `messages` 首条已经是 `system`，不再重复插入。

### 供应商扩展字段

| 插件 | 扩展字段 | 说明 |
| --- | --- | --- |
| deepseek | `reasoning_effort` | `high` / `max`，写入 `output_config.effort` |
| siliconflow_t2t | `top_p` | 核采样阈值 |
| siliconflow_t2t | `top_k` | Top-K |

## 响应

```json
{
  "code": 0,
  "msg": "success",
  "data": {
    "content": "模型回复",
    "id": "供应商响应 id",
    "model": "实际模型",
    "stop_reason": "end_turn",
    "usage": {},
    "session_id": "请求回显"
  }
}
```

| 字段 | 类型 | 说明 |
| --- | --- | --- |
| `content` | string | 文本回复，必填 |
| `id` | string \| null | 供应商响应 id |
| `model` | string \| null | 实际模型 |
| `stop_reason` | string \| null | 停止原因 |
| `usage` | object \| null | token 用量 |
| `session_id` | string \| null | 请求 `session_id` 回显 |

供应商未返回的字段输出 `null`，不省略字段。

## 错误码

| 码 | 含义 |
| --- | --- |
| `400101` | 请求 JSON 解析失败 |
| `400102` | `prompt` / `messages` 没有有效内容 |
| `500100` | 运行时创建失败 |
| `500101` | 读取配置或调用供应商失败 |
| `500102` | 上游 API 返回错误 |
| `500103` | 上游响应解析失败 |
| `500104` | 上游返回空内容 |
| `500900` | `json_out` 为空 |
| `500901` | 响应超过 64 KiB |

## 配置

| 插件 | 配置前缀 | 关键配置 |
| --- | --- | --- |
| deepseek | `deepseek_app_*` | `key` / `base_url` / `model` / `system_prompt` / `reasoning_effort` |
| siliconflow_t2t | `silicon_app_*` | `key` / `base_url` / `model` / `system_prompt` / `top_p` / `topk` |

API Key 只写入本地 `info.db`，不进入仓库、脚本或日志。
