# DeepSeek API 文档

本目录保存 DeepSeek 官方 API 文档的本地快照，便于离线查阅与插件实现对照。

## 文档

| 文档 | 内容 | 官方来源 |
| --- | --- | --- |
| [create-chat-completion.md](create-chat-completion.md) | Chat Completions 接口、请求 / 响应结构、流式输出、代码示例、全部 schema | <https://api-docs.deepseek.com/zh-cn/api/create-chat-completion> |
| [vision.md](vision.md) | 图像理解：图片传入方式、detail、Files API、Token 用量、限制、Anthropic / Responses API | <https://api-docs.deepseek.com/zh-cn/guides/vision> |
| [thinking_mode.md](thinking_mode.md) | 思考模式开关、思考强度、输入输出参数、多轮拼接 | <https://api-docs.deepseek.com/zh-cn/guides/thinking_mode> |
| [multi_round_chat.md](multi_round_chat.md) | 多轮对话的消息拼接方式 | <https://api-docs.deepseek.com/zh-cn/guides/multi_round_chat> |
| [chat_prefix_completion.md](chat_prefix_completion.md) | 对话前缀续写（Beta） | <https://api-docs.deepseek.com/zh-cn/guides/chat_prefix_completion> |
| [fim_completion.md](fim_completion.md) | FIM 补全（Beta）与 Continue 配置 | <https://api-docs.deepseek.com/zh-cn/guides/fim_completion> |
| [json_mode.md](json_mode.md) | JSON Output 模式与注意事项 | <https://api-docs.deepseek.com/zh-cn/guides/json_mode> |
| [tool_calls.md](tool_calls.md) | Tool Calls：非思考模式、思考模式与示例 | <https://api-docs.deepseek.com/zh-cn/guides/tool_calls> |
| [files_api.md](files_api.md) | Files API：上传、列出、查询文件 | <https://api-docs.deepseek.com/zh-cn/guides/files_api> |

## 说明

- 快照日期：2026-10-31；
- 内容来自官方文档的 Markdown 源，未做接口语义改写；
- 官方文档更新后，以 <https://api-docs.deepseek.com/zh-cn/> 为准；
- 插件实现与文档不一致时，优先检查官方最新文档和本仓库 `deepseek/README.md`。

## 与插件的关系

`deepseek` 插件使用 Anthropic 兼容端点：

```text
base_url: https://api.deepseek.com/anthropic
```

官方文档中的 OpenAI 兼容示例使用：

```text
base_url: https://api.deepseek.com
```

两者请求格式不同，使用插件配置时以 `deepseek_app_base_url` 为准。
