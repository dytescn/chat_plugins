# Chat Completions API

根据输入的上下文，来让模型补全对话内容。

## 接口

```http
POST /chat/completions
```

operationId: `createChatCompletion`

### 服务地址

- `https://api.deepseek.com`

## 身份验证

### 验证方式 1

**bearerAuth**

`Authorization: Bearer <API_KEY>`

```yaml
type: http
scheme: bearer
```

## 请求体

**必填**

### application/json

- 引用: [CreateChatCompletionRequest](#schema-create-chat-completion-request) (`#/components/schemas/CreateChatCompletionRequest`)

## 响应

### 200 (No streaming)

OK, 返回一个 `chat completion` 对象。

#### application/json

- 引用: [CreateChatCompletionResponse](#schema-create-chat-completion-response) (`#/components/schemas/CreateChatCompletionResponse`)

**示例**

```json
{
  "id": "88beaa90-17fd-45fc-939c-2ef75e2d913c",
  "object": "chat.completion",
  "created": 1793084842,
  "model": "deepseek-flash",
  "choices": [
    {
      "index": 0,
      "message": {
        "role": "assistant",
        "content": "Hello! 👋 How can I help you today?"
      },
      "logprobs": null,
      "finish_reason": "stop"
    }
  ],
  "usage": {
    "prompt_tokens": 7,
    "completion_tokens": 11,
    "total_tokens": 18,
    "prompt_tokens_details": {
      "cached_tokens": 0
    },
    "prompt_cache_hit_tokens": 0,
    "prompt_cache_miss_tokens": 7
  },
  "system_fingerprint": "e72d3c7f663d3e1383b00b3884cb4501"
}
```

### 200 (Streaming)

OK, 返回包含一系列 `chat completion chunk` 对象的流式输出。

#### text/event-stream

- `type`: `array`

- **数组元素**

  - 引用: [CreateChatCompletionStreamResponse](#schema-create-chat-completion-stream-response) (`#/components/schemas/CreateChatCompletionStreamResponse`)

**示例**

```
data: {"id":"071f229d-daa0-4557-a580-106a22e8df73","object":"chat.completion.chunk","created":1793084907,"model":"deepseek-flash","system_fingerprint":"e72d3c7f663d3e1383b00b3884cb4501","choices":[{"index":0,"delta":{"role":"assistant","content":""},"logprobs":null,"finish_reason":null}],"usage":null}

data: {"id":"071f229d-daa0-4557-a580-106a22e8df73","object":"chat.completion.chunk","created":1793084907,"model":"deepseek-flash","system_fingerprint":"e72d3c7f663d3e1383b00b3884cb4501","choices":[{"index":0,"delta":{"content":"Hello"},"logprobs":null,"finish_reason":null}],"usage":null}

data: {"id":"071f229d-daa0-4557-a580-106a22e8df73","object":"chat.completion.chunk","created":1793084907,"model":"deepseek-flash","system_fingerprint":"e72d3c7f663d3e1383b00b3884cb4501","choices":[{"index":0,"delta":{"content":"!"},"logprobs":null,"finish_reason":null}],"usage":null}

data: {"id":"071f229d-daa0-4557-a580-106a22e8df73","object":"chat.completion.chunk","created":1793084907,"model":"deepseek-flash","system_fingerprint":"e72d3c7f663d3e1383b00b3884cb4501","choices":[{"index":0,"delta":{"content":" 👋"},"logprobs":null,"finish_reason":null}],"usage":null}

data: {"id":"071f229d-daa0-4557-a580-106a22e8df73","object":"chat.completion.chunk","created":1793084907,"model":"deepseek-flash","system_fingerprint":"e72d3c7f663d3e1383b00b3884cb4501","choices":[{"index":0,"delta":{"content":" How"},"logprobs":null,"finish_reason":null}],"usage":null}

data: {"id":"071f229d-daa0-4557-a580-106a22e8df73","object":"chat.completion.chunk","created":1793084907,"model":"deepseek-flash","system_fingerprint":"e72d3c7f663d3e1383b00b3884cb4501","choices":[{"index":0,"delta":{"content":" can"},"logprobs":null,"finish_reason":null}],"usage":null}

data: {"id":"071f229d-daa0-4557-a580-106a22e8df73","object":"chat.completion.chunk","created":1793084907,"model":"deepseek-flash","system_fingerprint":"e72d3c7f663d3e1383b00b3884cb4501","choices":[{"index":0,"delta":{"content":" I"},"logprobs":null,"finish_reason":null}],"usage":null}

data: {"id":"071f229d-daa0-4557-a580-106a22e8df73","object":"chat.completion.chunk","created":1793084907,"model":"deepseek-flash","system_fingerprint":"e72d3c7f663d3e1383b00b3884cb4501","choices":[{"index":0,"delta":{"content":" help"},"logprobs":null,"finish_reason":null}],"usage":null}

data: {"id":"071f229d-daa0-4557-a580-106a22e8df73","object":"chat.completion.chunk","created":1793084907,"model":"deepseek-flash","system_fingerprint":"e72d3c7f663d3e1383b00b3884cb4501","choices":[{"index":0,"delta":{"content":" you"},"logprobs":null,"finish_reason":null}],"usage":null}

data: {"id":"071f229d-daa0-4557-a580-106a22e8df73","object":"chat.completion.chunk","created":1793084907,"model":"deepseek-flash","system_fingerprint":"e72d3c7f663d3e1383b00b3884cb4501","choices":[{"index":0,"delta":{"content":" today"},"logprobs":null,"finish_reason":null}],"usage":null}

data: {"id":"071f229d-daa0-4557-a580-106a22e8df73","object":"chat.completion.chunk","created":1793084907,"model":"deepseek-flash","system_fingerprint":"e72d3c7f663d3e1383b00b3884cb4501","choices":[{"index":0,"delta":{"content":"?"},"logprobs":null,"finish_reason":null}],"usage":null}

data: {"id":"071f229d-daa0-4557-a580-106a22e8df73","object":"chat.completion.chunk","created":1793084907,"model":"deepseek-flash","system_fingerprint":"e72d3c7f663d3e1383b00b3884cb4501","choices":[{"index":0,"delta":{"content":""},"logprobs":null,"finish_reason":"stop"}],"usage":{"prompt_tokens":7,"completion_tokens":11,"total_tokens":18,"prompt_tokens_details":{"cached_tokens":0},"prompt_cache_hit_tokens":0,"prompt_cache_miss_tokens":7}}

data: [DONE]
```

## 代码示例

### curl

```bash
# Set DEEPSEEK_API_KEY to your API key before running this example.
curl --silent --show-error --max-time 60 "https://api.deepseek.com/chat/completions" \
  --header "Authorization: Bearer ${DEEPSEEK_API_KEY}" \
  --header "Content-Type: application/json" \
  --data '{
  "model": "deepseek-flash",
  "messages": [
    {
      "role": "user",
      "content": "Say hello."
    }
  ],
  "max_tokens": 32,
  "temperature": 0,
  "thinking": {
    "type": "disabled"
  },
  "stream": false
}'
```

### Python — OpenAI SDK

```python
# Set DEEPSEEK_API_KEY to your API key before running this example.
import os
from openai import OpenAI

client = OpenAI(
    api_key=os.environ["DEEPSEEK_API_KEY"],
    base_url="https://api.deepseek.com",
    timeout=60.0,
)

response = client.chat.completions.create(
    model="deepseek-flash",
    messages=[{"role": "user", "content": "Say hello."}],
    max_tokens=32,
    temperature=0,
    stream=False,
    extra_body={"thinking": {"type": "disabled"}},
)
print(response.model_dump_json(indent=2))
```

## 引用的数据结构

### schema-create-chat-completion-request

**CreateChatCompletionRequest** — `#/components/schemas/CreateChatCompletionRequest`

- `type`: `object`

- `required`: `["model","messages"]`

- **字段**

  - **`messages`** (必填)

    对话的消息列表。

    - `type`: `array`

    - `minItems`: `1`

    - **数组元素**

      - 引用: [ChatCompletionRequestMessage](#schema-chat-completion-request-message) (`#/components/schemas/ChatCompletionRequestMessage`)

    **示例**

    ```json
    [
      {
        "content": "You are a helpful assistant",
        "role": "system"
      },
      {
        "content": "Hi",
        "role": "user"
      }
    ]
    ```

  - **`model`** (必填)

    使用的模型的 ID。请使用 `deepseek-flash` 或 `deepseek-v4-pro`。

    - `type`: `string`

    - `enum`: `["deepseek-flash","deepseek-v4-pro"]`

    **示例**

    ```json
    "deepseek-flash"
    ```

  - **`thinking`** (可选)

    控制思考模式与非思考模式的转换。省略 `thinking` 或传 `null` 时，模式由 `reasoning_effort` 决定；如果 `reasoning_effort` 也未提供，则默认开启思考模式。提供此对象时，`type` 必填，其模式设置优先于 `reasoning_effort`。

    - `type`: `object`

    - `nullable`: `true`

    - `required`: `["type"]`

    - **字段**

      - **`type`** (必填)

        如果设为 `enabled`，则使用思考模式。如果设为 `disabled`，则使用非思考模式

        - `type`: `string`

        - `enum`: `["enabled","disabled"]`

  - **`reasoning_effort`** (可选)

    控制思考模式开关与思考强度。`none` 关闭思考模式；`low` / `high` / `max` 开启思考模式。默认强度为 `high`。出于兼容考虑，`minimal` 映射为 `low`，`medium` / `xhigh` 映射为 `high`。

    - `type`: `string`

    - `enum`: `["none","low","high","max"]`

  - **`max_tokens`** (可选)

    限制一次请求中模型生成 completion 的最大 token 数。取值范围为 1 到 384K（393216）。未设置时，非思考模式默认 8K，思考模式默认 64K（`reasoning_effort` 为 `max` 时为 128K）。输入 token 和输出 token 的总长度受模型的上下文长度的限制。详见[模型 & 价格](https://api-docs.deepseek.com/zh-cn/quick_start/pricing.md)。

    - `exclusiveMinimum`: `true`

    - `type`: `integer`

    - `nullable`: `true`

    **示例**

    ```json
    4096
    ```

  - **`response_format`** (可选)

    一个 object，指定模型必须输出的格式。省略或传 `null` 时使用普通文本；提供此对象时，`type` 必填。

    设置为 { "type": "json\_object" } 以启用 JSON 模式，该模式保证模型生成的消息是有效的 JSON。

    **注意:** 使用 JSON 模式时，你还必须通过系统或用户消息指示模型生成 JSON。如果模型的输出无法被识别为 JSON，API 会将其中的非 JSON 内容替换为空格（`U+0020`），因此返回的消息内容可能仅包含空格（看起来为空，此时 finish\_reason 仍可能是 stop）。此外，如果 finish\_reason="length"，这表示生成超过了 max\_tokens 或对话超过了最大上下文长度，消息内容可能会被部分截断。

    - `type`: `object`

    - `nullable`: `true`

    - `required`: `["type"]`

    - **字段**

      - **`type`** (必填)

        必须是 `text` 或 `json_object`。

        - `type`: `string`

        - `enum`: `["text","json_object"]`

        **示例**

        ```json
        "text"
        ```

  - **`stop`** (可选)

    一个 string 或最多包含 16 个 string 的 list，在遇到这些词时，API 将停止生成更多的 token。

    - `nullable`: `true`

    - `default`: `null`

    - **oneOf**

      - oneOf 1

        - title: `Single stop sequence`

        - `type`: `string`

        - `nullable`: `true`

        **示例**

        ```json
        null
        ```

      - oneOf 2

        - title: `Stop sequence list`

        - `type`: `array`

        - `minItems`: `1`

        - `maxItems`: `16`

        - **数组元素**

          - `type`: `string`

  - **`stream`** (可选)

    如果设置为 `true`，将会以 SSE（server-sent events）的形式以流式发送消息增量。消息流以 `data: [DONE]` 结尾。

    - `type`: `boolean`

    - `nullable`: `true`

    - `default`: `false`

  - **`stream_options`** (可选)

    - 引用: [ChatCompletionStreamOptions](#schema-chat-completion-stream-options) (`#/components/schemas/ChatCompletionStreamOptions`)

  - **`temperature`** (可选)

    采样温度，介于 0 和 2 之间。更高的值，如 0.8，会使输出更随机，而更低的值，如 0.2，会使其更加集中和确定。 我们通常建议可以更改这个值或者更改 `top_p`，但不建议同时对两者进行修改。思考模式下不生效。

    - `type`: `number`

    - `minimum`: `0`

    - `maximum`: `2`

    - `default`: `1`

    - `nullable`: `true`

    **示例**

    ```json
    1
    ```

  - **`top_p`** (可选)

    作为调节采样温度的替代方案，模型会考虑前 `top_p` 概率的 token 的结果。所以 0.1 就意味着只有包括在最高 10% 概率中的 token 会被考虑。取值必须大于 0 且不超过 1。我们通常建议修改这个值或者更改 `temperature`，但不建议同时对两者进行修改。实际生效值取决于模型和模式：

    - `deepseek-flash`：思考模式下默认值为 1.0，低于 0.95 的取值会按 0.95 处理；非思考模式下恒为 1.0，传入的值会被忽略。

    - `deepseek-v4-pro`：思考模式下，启用工具调用时恒为 0.95，否则恒为 1.0，传入的值会被忽略。即使提供了 `tools`，将 `tool_choice` 设为 `none` 也会关闭工具调用。非思考模式下使用传入的值，默认值为 1.0。

    - `type`: `number`

    - `minimum`: `0`

    - `exclusiveMinimum`: `true`

    - `maximum`: `1`

    - `nullable`: `true`

    **示例**

    ```json
    1
    ```

  - **`tools`** (可选)

    模型可能会调用的 tool 的列表。目前，仅支持 function 作为工具。使用此参数来提供以 JSON 作为输入参数的 function 列表。tool 名称必须唯一。

    - `type`: `array`

    - `nullable`: `true`

    - **数组元素**

      - 引用: [ChatCompletionTool](#schema-chat-completion-tool) (`#/components/schemas/ChatCompletionTool`)

    **示例**

    ```json
    null
    ```

  - **`tool_choice`** (可选)

    - 引用: [ChatCompletionToolChoiceOption](#schema-chat-completion-tool-choice-option) (`#/components/schemas/ChatCompletionToolChoiceOption`)

  - **`logprobs`** (可选)

    是否返回所输出 token 的对数概率。如果为 `true`，则在 `message` 的 `content` 中返回每个输出 token 的对数概率。

    - `type`: `boolean`

    - `default`: `false`

    - `nullable`: `true`

  - **`top_logprobs`** (可选)

    一个介于 0 到 20 之间的整数 N，指定每个输出位置返回输出概率 top N 的 token，且返回这些 token 的对数概率。指定此参数时，logprobs 必须为 `true`。

    - `type`: `integer`

    - `minimum`: `0`

    - `maximum`: `20`

    - `nullable`: `true`

    - `default`: `null`

    **示例**

    ```json
    null
    ```

  - **`user_id`** (可选)

    您自定义的 user\_id。建议仅使用英文字母、数字、下划线（`_`）和连字符（`-`）。UTF-8 编码后不得超过 512 字节。请不要在 user\_id 中包含用户隐私信息。

    - user\_id 可用于区分您业务侧的用户身份，以帮助我们进行内容安全处理。

    - user\_id 可用于 KVCache 缓存隔离，以进行隐私管理。

    - user\_id 可用于我们对您业务侧用户进行调度隔离。

    - 关于 user\_id 参数更详细的描述，请参考[限速与隔离](https://api-docs.deepseek.com/zh-cn/quick_start/rate_limit.md)

    * `nullable`: `true`

  - **`frequency_penalty`** (可选)

    该参数已不再支持。传入该参数将不会产生任何效果。

    - `deprecated`: `true`

  - **`presence_penalty`** (可选)

    该参数已不再支持。传入该参数将不会产生任何效果。

    - `deprecated`: `true`

### schema-create-chat-completion-response

**CreateChatCompletionResponse** — `#/components/schemas/CreateChatCompletionResponse`

Represents a chat completion response returned by model, based on the provided input.

- `type`: `object`

- `required`: `["choices","created","id","model","system_fingerprint","object"]`

- **字段**

  - **`id`** (必填)

    该对话的唯一标识符。

    - `type`: `string`

  - **`choices`** (必填)

    模型生成的 completion 的选择列表。

    - `type`: `array`

    - **数组元素**

      - `type`: `object`

      - `required`: `["finish_reason","index","message","logprobs"]`

      - **字段**

        - **`finish_reason`** (必填)

          模型停止生成 token 的原因。

          `stop`：模型自然停止生成，或遇到 `stop` 序列中列出的字符串。

          `length` ：输出长度达到了模型上下文长度限制，或达到了 `max_tokens` 的限制。

          `content_filter`：输出内容因触发过滤策略而被过滤。

          `tool_calls`：模型进行了工具调用。

          `insufficient_system_resource`：系统推理资源不足，生成被打断。

          `aborted`：生成过程被中断。

          - `type`: `string`

          - `enum`: `["stop","length","content_filter","tool_calls","insufficient_system_resource","aborted"]`

        - **`index`** (必填)

          该 completion 在模型生成的 completion 的选择列表中的索引。

          - `type`: `integer`

        - **`message`** (必填)

          - 引用: [ChatCompletionResponseMessage](#schema-chat-completion-response-message) (`#/components/schemas/ChatCompletionResponseMessage`)

        - **`logprobs`** (必填)

          该 choice 的对数概率信息。

          - `type`: `object`

          - `nullable`: `true`

          - `required`: `["content"]`

          - **字段**

            - **`content`** (必填)

              一个包含输出 token 对数概率信息的列表。

              - `type`: `array`

              - `nullable`: `true`

              - **数组元素**

                - 引用: [ChatCompletionTokenLogprob](#schema-chat-completion-token-logprob) (`#/components/schemas/ChatCompletionTokenLogprob`)

            - **`reasoning_content`** (可选)

              一个包含输出 token 对数概率信息的列表。

              - `type`: `array`

              - `nullable`: `true`

              - **数组元素**

                - 引用: [ChatCompletionTokenLogprob](#schema-chat-completion-token-logprob) (`#/components/schemas/ChatCompletionTokenLogprob`)

  - **`created`** (必填)

    创建聊天完成时的 Unix 时间戳（以秒为单位）。

    - `type`: `integer`

  - **`model`** (必填)

    生成该 completion 的模型名。

    - `type`: `string`

  - **`system_fingerprint`** (必填)

    This fingerprint represents the backend configuration that the model runs with.

    - `type`: `string`

  - **`object`** (必填)

    对象的类型, 其值为 `chat.completion`。

    - `type`: `string`

    - `enum`: `["chat.completion"]`

  - **`usage`** (可选)

    - 引用: [CompletionUsage](#schema-completion-usage) (`#/components/schemas/CompletionUsage`)

### schema-create-chat-completion-stream-response

**CreateChatCompletionStreamResponse** — `#/components/schemas/CreateChatCompletionStreamResponse`

基于提供的输入，由模型返回的对话补全响应的流式输出。

- `type`: `object`

- `required`: `["choices","created","id","model","system_fingerprint","object"]`

- **字段**

  - **`id`** (必填)

    该对话的唯一标识符。

    - `type`: `string`

  - **`choices`** (必填)

    模型生成的 completion 的选择列表。

    - `type`: `array`

    - **数组元素**

      - `type`: `object`

      - `required`: `["delta","finish_reason","index"]`

      - **字段**

        - **`delta`** (必填)

          - 引用: [ChatCompletionStreamResponseDelta](#schema-chat-completion-stream-response-delta) (`#/components/schemas/ChatCompletionStreamResponseDelta`)

        - **`logprobs`** (可选)

          该 choice 的对数概率信息。

          - `type`: `object`

          - `nullable`: `true`

          - `required`: `["content"]`

          - **字段**

            - **`content`** (必填)

              一个包含输出 token 对数概率信息的列表。

              - `type`: `array`

              - `nullable`: `true`

              - **数组元素**

                - 引用: [ChatCompletionTokenLogprob](#schema-chat-completion-token-logprob) (`#/components/schemas/ChatCompletionTokenLogprob`)

            - **`reasoning_content`** (可选)

              一个包含输出 token 对数概率信息的列表。

              - `type`: `array`

              - `nullable`: `true`

              - **数组元素**

                - 引用: [ChatCompletionTokenLogprob](#schema-chat-completion-token-logprob) (`#/components/schemas/ChatCompletionTokenLogprob`)

        - **`finish_reason`** (必填)

          模型停止生成 token 的原因。

          `stop`：模型自然停止生成，或遇到 `stop` 序列中列出的字符串。

          `length` ：输出长度达到了模型上下文长度限制，或达到了 `max_tokens` 的限制。

          `content_filter`：输出内容因触发过滤策略而被过滤。

          `tool_calls`：模型进行了工具调用。

          `insufficient_system_resource`: 由于后端推理资源受限，请求被打断。

          `aborted`：生成过程被中断。

          - `type`: `string`

          - `enum`: `["stop","length","content_filter","tool_calls","insufficient_system_resource","aborted"]`

          - `nullable`: `true`

        - **`index`** (必填)

          该 completion 在模型生成的 completion 的选择列表中的索引。

          - `type`: `integer`

  - **`created`** (必填)

    创建聊天完成时的 Unix 时间戳（以秒为单位）。流式响应的每个 chunk 的时间戳相同。

    - `type`: `integer`

  - **`model`** (必填)

    生成该 completion 的模型名。

    - `type`: `string`

  - **`system_fingerprint`** (必填)

    This fingerprint represents the backend configuration that the model runs with.

    - `type`: `string`

  - **`object`** (必填)

    对象的类型, 其值为 `chat.completion.chunk`。

    - `type`: `string`

    - `enum`: `["chat.completion.chunk"]`

### schema-chat-completion-request-message

**ChatCompletionRequestMessage** — `#/components/schemas/ChatCompletionRequestMessage`

- **oneOf**

  - oneOf 1

    - 引用: [ChatCompletionRequestSystemMessage](#schema-chat-completion-request-system-message) (`#/components/schemas/ChatCompletionRequestSystemMessage`)

  - oneOf 2

    - 引用: [ChatCompletionRequestUserMessage](#schema-chat-completion-request-user-message) (`#/components/schemas/ChatCompletionRequestUserMessage`)

  - oneOf 3

    - 引用: [ChatCompletionRequestAssistantMessage](#schema-chat-completion-request-assistant-message) (`#/components/schemas/ChatCompletionRequestAssistantMessage`)

  - oneOf 4

    - 引用: [ChatCompletionRequestToolMessage](#schema-chat-completion-request-tool-message) (`#/components/schemas/ChatCompletionRequestToolMessage`)

### schema-chat-completion-stream-options

**ChatCompletionStreamOptions** — `#/components/schemas/ChatCompletionStreamOptions`

流式输出相关选项。必须与 `stream: true` 一起使用；如果 `stream` 未设置为 `true`，API 会返回 `400` 错误。

- `type`: `object`

- `nullable`: `true`

- `default`: `null`

- **字段**

  - **`include_usage`** (可选)

    如果设置为 `true`，流式返回的所有块都会包含 `usage` 字段，其中除最后一个块外，该字段的值均为 `null`。如果不设置或设置为 `false`，则除最后一个块外，其余块都不含 `usage` 字段。

    无论是否设置，`data: [DONE]` 之前的最后一个块都会在其 `usage` 字段中给出整个请求的 token 使用统计信息。请注意，这里不会单独下发一个只含 usage 的块：统计信息附加在最后一个内容块上，该块的 `choices` 数组始终只包含一个元素，其中不含新增内容且 `finish_reason` 非 null。

    - `type`: `boolean`

**示例**

```json
null
```

### schema-chat-completion-tool

**ChatCompletionTool** — `#/components/schemas/ChatCompletionTool`

- `type`: `object`

- `required`: `["type","function"]`

- **字段**

  - **`type`** (必填)

    tool 的类型。目前仅支持 function。

    - `type`: `string`

    - `enum`: `["function"]`

  - **`function`** (必填)

    - 引用: [FunctionObject](#schema-function-object) (`#/components/schemas/FunctionObject`)

### schema-chat-completion-tool-choice-option

**ChatCompletionToolChoiceOption** — `#/components/schemas/ChatCompletionToolChoiceOption`

控制模型调用 tool 的行为。

`none` 意味着模型不会调用任何 tool，而是生成一条消息。

`auto` 意味着模型可以选择生成一条消息或调用一个或多个 tool。

`required` 意味着模型必须调用一个或多个 tool。

通过 `{"type": "function", "function": {"name": "my_function"}}` 指定特定 tool，会强制模型调用该 tool。

当没有 tool 时，默认值为 `none`。如果有 tool 存在，默认值为 `auto`。

思考模式下不支持 `required` 和指定具体 tool 的用法，API 会返回 `400` 错误。请先关闭思考模式。

- `type`: `["string","object"]`

- `nullable`: `true`

- **oneOf**

  - oneOf 1

    - 引用: [ChatCompletionToolChoice](#schema-chat-completion-tool-choice) (`#/components/schemas/ChatCompletionToolChoice`)

  - oneOf 2

    - 引用: [ChatCompletionNamedToolChoice](#schema-chat-completion-named-tool-choice) (`#/components/schemas/ChatCompletionNamedToolChoice`)

### schema-chat-completion-response-message

**ChatCompletionResponseMessage** — `#/components/schemas/ChatCompletionResponseMessage`

模型生成的 completion 消息。

- `type`: `object`

- `required`: `["role","content"]`

- **字段**

  - **`content`** (必填)

    该 completion 的内容。

    - `type`: `string`

    - `nullable`: `true`

  - **`reasoning_content`** (可选)

    仅适用于思考模式。内容为 assistant 消息中在最终答案之前的推理内容。

    - `type`: `string`

    - `nullable`: `true`

  - **`tool_calls`** (可选)

    - 引用: [ChatCompletionMessageToolCalls](#schema-chat-completion-message-tool-calls) (`#/components/schemas/ChatCompletionMessageToolCalls`)

  - **`role`** (必填)

    生成这条消息的角色。

    - `type`: `string`

    - `enum`: `["assistant"]`

### schema-chat-completion-token-logprob

**ChatCompletionTokenLogprob** — `#/components/schemas/ChatCompletionTokenLogprob`

- `type`: `object`

- `required`: `["token","logprob","bytes","top_logprobs"]`

- **字段**

  - **`token`** (必填)

    输出的 token。

    - `type`: `string`

  - **`logprob`** (必填)

    该 token 的对数概率。`-9999.0` 代表该 token 的输出概率极小，不在 top 20 最可能输出的 token 中。

    - `type`: `number`

  - **`bytes`** (必填)

    一个包含该 token UTF-8 字节表示的整数列表。一般在一个 UTF-8 字符被拆分成多个 token 来表示时有用。如果 token 没有对应的字节表示，则该值为 `null`。

    - `type`: `array`

    - `nullable`: `true`

    - **数组元素**

      - `type`: `integer`

  - **`top_logprobs`** (必填)

    一个包含在该输出位置上，输出概率 top N 的 token 的列表，以及它们的对数概率。在罕见情况下，返回的 token 数量可能少于请求参数中指定的 `top_logprobs` 值。

    - `type`: `array`

    - **数组元素**

      - `type`: `object`

      - `required`: `["token","logprob","bytes"]`

      - **字段**

        - **`token`** (必填)

          输出的 token。

          - `type`: `string`

        - **`logprob`** (必填)

          该 token 的对数概率。`-9999.0` 代表该 token 的输出概率极小，不在 top 20 最可能输出的 token 中。

          - `type`: `number`

        - **`bytes`** (必填)

          一个包含该 token UTF-8 字节表示的整数列表。一般在一个 UTF-8 字符被拆分成多个 token 来表示时有用。如果 token 没有对应的字节表示，则该值为 `null`。

          - `type`: `array`

          - `nullable`: `true`

          - **数组元素**

            - `type`: `integer`

### schema-completion-usage

**CompletionUsage** — `#/components/schemas/CompletionUsage`

该对话补全请求的用量信息。

- `type`: `object`

- `required`: `["prompt_tokens","prompt_tokens_details","prompt_cache_hit_tokens","prompt_cache_miss_tokens","completion_tokens","total_tokens"]`

- **字段**

  - **`completion_tokens`** (必填)

    模型 completion 产生的 token 数。

    - `type`: `integer`

  - **`prompt_tokens`** (必填)

    用户 prompt 所包含的 token 数。该值等于 `prompt_cache_hit_tokens + prompt_cache_miss_tokens`

    - `type`: `integer`

  - **`prompt_tokens_details`** (必填)

    prompt tokens 的详细信息。

    - `type`: `object`

    - **字段**

      - **`cached_tokens`** (可选)

        用户 prompt 中，命中上下文缓存的 token 数。与 `prompt_cache_hit_tokens` 相同。

        - `type`: `integer`

  - **`prompt_cache_hit_tokens`** (必填)

    用户 prompt 中，命中上下文缓存的 token 数。

    - `type`: `integer`

  - **`prompt_cache_miss_tokens`** (必填)

    用户 prompt 中，未命中上下文缓存的 token 数。

    - `type`: `integer`

  - **`total_tokens`** (必填)

    该请求中，所有 token 的数量（prompt + completion）。

    - `type`: `integer`

  - **`completion_tokens_details`** (可选)

    completion tokens 的详细信息。

    - `type`: `object`

    - **字段**

      - **`reasoning_tokens`** (可选)

        推理模型所产生的思维链 token 数量

        - `type`: `integer`

### schema-chat-completion-stream-response-delta

**ChatCompletionStreamResponseDelta** — `#/components/schemas/ChatCompletionStreamResponseDelta`

流式返回的一个 completion 增量。

- `type`: `object`

- **字段**

  - **`content`** (可选)

    completion 增量的内容。

    - `type`: `string`

    - `nullable`: `true`

  - **`reasoning_content`** (可选)

    仅适用于思考模式。内容为 assistant 消息中在最终答案之前的推理内容。

    - `type`: `string`

    - `nullable`: `true`

  - **`role`** (可选)

    产生这条消息的角色。

    - `type`: `string`

    - `enum`: `["assistant"]`

  - **`tool_calls`** (可选)

    模型生成的 tool 调用，例如 function 调用。每个 tool 调用的第一个 chunk 携带 `id`、`type` 和 `function` 字段，后续 chunk 只携带 function 参数。

    - `type`: `array`

    - **数组元素**

      - 引用: [ChatCompletionMessageToolCallChunk](#schema-chat-completion-message-tool-call-chunk) (`#/components/schemas/ChatCompletionMessageToolCallChunk`)

### schema-chat-completion-request-system-message

**ChatCompletionRequestSystemMessage** — `#/components/schemas/ChatCompletionRequestSystemMessage`

- title: `System message`

- `type`: `object`

- `required`: `["content","role"]`

- **字段**

  - **`content`** (必填)

    system 消息的内容。

    - `type`: `string`

  - **`role`** (必填)

    该消息的发起角色，其值为 `system`。

    - `type`: `string`

    - `enum`: `["system"]`

  - **`name`** (可选)

    可以选填的参与者的名称，为模型提供信息以区分相同角色的参与者。

    - `type`: `string`

### schema-chat-completion-request-user-message

**ChatCompletionRequestUserMessage** — `#/components/schemas/ChatCompletionRequestUserMessage`

- title: `User message`

- `type`: `object`

- `required`: `["content","role"]`

- **字段**

  - **`content`** (必填)

    user 消息的内容。可以是字符串，也可以是内容块数组（可携带图片）。详见[图像理解指南](https://api-docs.deepseek.com/zh-cn/guides/vision.md)。

    - **oneOf**

      - oneOf 1

        - title: `Text content`

        - `type`: `string`

      - oneOf 2

        - title: `Array of content parts`

        - `type`: `array`

        - `minItems`: `1`

        - **数组元素**

          - 引用: [ChatCompletionRequestContentPart](#schema-chat-completion-request-content-part) (`#/components/schemas/ChatCompletionRequestContentPart`)

  - **`role`** (必填)

    该消息的发起角色，其值为 `user`。

    - `type`: `string`

    - `enum`: `["user"]`

  - **`name`** (可选)

    可以选填的参与者的名称，为模型提供信息以区分相同角色的参与者。

    - `type`: `string`

### schema-chat-completion-request-assistant-message

**ChatCompletionRequestAssistantMessage** — `#/components/schemas/ChatCompletionRequestAssistantMessage`

- title: `Assistant message`

- `type`: `object`

- `required`: `["role","content"]`

- **字段**

  - **`content`** (必填)

    assistant 消息的内容。

    - `nullable`: `true`

    - `type`: `string`

  - **`role`** (必填)

    该消息的发起角色，其值为 `assistant`。

    - `type`: `string`

    - `enum`: `["assistant"]`

  - **`name`** (可选)

    可以选填的参与者的名称，为模型提供信息以区分相同角色的参与者。

    - `type`: `string`

  - **`prefix`** (可选)

    (Beta) 设置此参数为 `true`，来强制模型在其回答中以此 `assistant` 消息中提供的前缀内容开始。

    您必须设置 `base_url="https://api.deepseek.com/beta"` 来使用此功能。

    - `type`: `bool`

  - **`reasoning_content`** (可选)

    (Beta) 用于思考模式下在[对话前缀续写](https://api-docs.deepseek.com/zh-cn/guides/chat_prefix_completion.md)功能下，作为最后一条 assistant 思维链内容的输入。使用此功能时，`prefix` 参数必须设置为 `true`。

    - `nullable`: `true`

    - `type`: `string`

### schema-chat-completion-request-tool-message

**ChatCompletionRequestToolMessage** — `#/components/schemas/ChatCompletionRequestToolMessage`

- title: `Tool message`

- `type`: `object`

- `required`: `["role","content","tool_call_id"]`

- **字段**

  - **`role`** (必填)

    该消息的发起角色，其值为 `tool`。

    - `type`: `string`

    - `enum`: `["tool"]`

  - **`content`** (必填)

    - title: `Text content`

    tool 消息的内容。可以是字符串，也可以是内容块数组（可以携带图片）。详见 [Vision 指南](https://api-docs.deepseek.com/zh-cn/guides/vision.md)。

    - **oneOf**

      - oneOf 1

        - title: `Text content`

        - `type`: `string`

      - oneOf 2

        - title: `Array of content parts`

        - `type`: `array`

        - `minItems`: `1`

        - **数组元素**

          - 引用: [ChatCompletionRequestContentPart](#schema-chat-completion-request-content-part) (`#/components/schemas/ChatCompletionRequestContentPart`)

  - **`tool_call_id`** (必填)

    此消息所响应的 tool call 的 ID。

    - `type`: `string`

### schema-function-object

**FunctionObject** — `#/components/schemas/FunctionObject`

- `type`: `object`

- `required`: `["name"]`

- **字段**

  - **`description`** (可选)

    function 的功能描述，供模型理解何时以及如何调用该 function。

    - `type`: `string`

  - **`name`** (必填)

    要调用的 function 名称。必须由 a-z、A-Z、0-9 字符组成，或包含下划线和连字符，最大长度为 128 个字符。

    - `type`: `string`

  - **`parameters`** (可选)

    - 引用: [FunctionParameters](#schema-function-parameters) (`#/components/schemas/FunctionParameters`)

  - **`strict`** (可选)

    如果设置为 `true`，API 将在函数调用中使用 strict 模式，以确保输出始终符合函数的 JSON schema 定义。该功能为 Beta 功能，详细使用方式请参阅[Tool Calls 指南](https://api-docs.deepseek.com/zh-cn/guides/tool_calls.md)

    - `type`: `boolean`

    - `default`: `false`

### schema-chat-completion-tool-choice

**ChatCompletionToolChoice** — `#/components/schemas/ChatCompletionToolChoice`

- `type`: `string`

- `enum`: `["none","auto","required"]`

### schema-chat-completion-named-tool-choice

**ChatCompletionNamedToolChoice** — `#/components/schemas/ChatCompletionNamedToolChoice`

指定模型应使用的 tool。用于强制模型调用特定 function。

- `type`: `object`

- `required`: `["type","function"]`

- **字段**

  - **`type`** (必填)

    tool 的类型。目前，仅支持 `function`。

    - `type`: `string`

    - `enum`: `["function"]`

  - **`function`** (必填)

    - `type`: `object`

    - `required`: `["name"]`

    - **字段**

      - **`name`** (必填)

        要调用的函数名称。

        - `type`: `string`

### schema-chat-completion-message-tool-calls

**ChatCompletionMessageToolCalls** — `#/components/schemas/ChatCompletionMessageToolCalls`

模型生成的 tool 调用，例如 function 调用。

- `type`: `array`

- **数组元素**

  - 引用: [ChatCompletionMessageToolCall](#schema-chat-completion-message-tool-call) (`#/components/schemas/ChatCompletionMessageToolCall`)

### schema-chat-completion-message-tool-call-chunk

**ChatCompletionMessageToolCallChunk** — `#/components/schemas/ChatCompletionMessageToolCallChunk`

- `type`: `object`

- `required`: `["index"]`

- **字段**

  - **`index`** (必填)

    - `type`: `integer`

  - **`id`** (可选)

    tool 的 ID。

    - `type`: `string`

  - **`type`** (可选)

    tool 的类型。目前仅支持 `function`。

    - `type`: `string`

    - `enum`: `["function"]`

  - **`function`** (可选)

    - `type`: `object`

    - **字段**

      - **`name`** (可选)

        要调用的 function 名。

        - `type`: `string`

      - **`arguments`** (可选)

        要调用的 function 的参数，由模型生成，格式为 JSON。请注意，模型并不总是生成有效的 JSON，并且可能会臆造出你的函数模式中未定义的参数。在调用函数之前，请在代码中验证这些参数。

        - `type`: `string`

### schema-chat-completion-request-content-part

**ChatCompletionRequestContentPart** — `#/components/schemas/ChatCompletionRequestContentPart`

- **oneOf**

  - oneOf 1

    - 引用: [ChatCompletionRequestContentPartText](#schema-chat-completion-request-content-part-text) (`#/components/schemas/ChatCompletionRequestContentPartText`)

  - oneOf 2

    - 引用: [ChatCompletionRequestContentPartImage](#schema-chat-completion-request-content-part-image) (`#/components/schemas/ChatCompletionRequestContentPartImage`)

  - oneOf 3

    - 引用: [ChatCompletionRequestContentPartFile](#schema-chat-completion-request-content-part-file) (`#/components/schemas/ChatCompletionRequestContentPartFile`)

### schema-function-parameters

**FunctionParameters** — `#/components/schemas/FunctionParameters`

function 的输入参数，以 JSON Schema 对象描述。请参阅[Tool Calls 指南](https://api-docs.deepseek.com/zh-cn/guides/tool_calls.md)获取示例，并参阅[JSON Schema 参考](https://json-schema.org/understanding-json-schema/)了解有关格式的文档。省略 `parameters` 会定义一个参数列表为空的 function。

- `type`: `object`

- **additionalProperties**

  - 数据结构: `true`

### schema-chat-completion-message-tool-call

**ChatCompletionMessageToolCall** — `#/components/schemas/ChatCompletionMessageToolCall`

- `type`: `object`

- `required`: `["id","type","function"]`

- **字段**

  - **`id`** (必填)

    tool 调用的 ID。

    - `type`: `string`

  - **`type`** (必填)

    tool 的类型。目前仅支持 `function`。

    - `type`: `string`

    - `enum`: `["function"]`

  - **`function`** (必填)

    模型调用的 function。

    - `type`: `object`

    - `required`: `["name","arguments"]`

    - **字段**

      - **`name`** (必填)

        模型调用的 function 名。

        - `type`: `string`

      - **`arguments`** (必填)

        要调用的 function 的参数，由模型生成，格式为 JSON。请注意，模型并不总是生成有效的 JSON，并且可能会臆造出你函数模式中未定义的参数。在调用函数之前，请在代码中验证这些参数。

        - `type`: `string`

### schema-chat-completion-request-content-part-text

**ChatCompletionRequestContentPartText** — `#/components/schemas/ChatCompletionRequestContentPartText`

- title: `Text content part`

- `type`: `object`

- `required`: `["type","text"]`

- **字段**

  - **`type`** (必填)

    内容块的类型，此场景下为 `text`。

    - `type`: `string`

    - `enum`: `["text"]`

  - **`text`** (必填)

    文本内容。

    - `type`: `string`

### schema-chat-completion-request-content-part-image

**ChatCompletionRequestContentPartImage** — `#/components/schemas/ChatCompletionRequestContentPartImage`

- title: `Image content part`

- `type`: `object`

- `required`: `["type","image_url"]`

- **字段**

  - **`type`** (必填)

    内容块的类型，此场景下为 `image_url`。

    - `type`: `string`

    - `enum`: `["image_url"]`

  - **`image_url`** (必填)

    - `type`: `object`

    - `required`: `["url"]`

    - **字段**

      - **`url`** (必填)

        图片的 `http(s)` URL（最多 8192 个字符）或 base64 编码的 data URL（`data:image/jpeg;base64,...`）。支持的格式：JPEG、PNG、GIF、WebP。

        - `type`: `string`

      - **`detail`** (可选)

        控制图片的处理方式。`low` 将图片缩小到 512x512（更快、更省 token）；`high`、`original` 与 `auto` 保留原图。

        - `type`: `string`

        - `enum`: `["low","high","original","auto"]`

### schema-chat-completion-request-content-part-file

**ChatCompletionRequestContentPartFile** — `#/components/schemas/ChatCompletionRequestContentPartFile`

- title: `File content part`

`file_id` 与 `file_data` 必须二选一：两者都不传返回 `400` 错误（"file must have a file\_id or file\_data"）；两者都传返回 `400` 错误（"file cannot have both file\_id and file\_data"）。

- `type`: `object`

- `required`: `["type"]`

- **字段**

  - **`type`** (必填)

    内容块的类型，此场景下为 `file`。

    - `type`: `string`

    - `enum`: `["file"]`

  - **`file_id`** (可选)

    通过 [Files API](https://api-docs.deepseek.com/zh-cn/guides/files_api.md) 上传的文件 ID，形如 `file-api-...`。与 `file_data` 互斥。

    - `type`: `string`

  - **`file_data`** (可选)

    图片的 base64 编码 data URL（`data:image/jpeg;base64,...`）。与 `file_id` 互斥。

    - `type`: `string`

  - **`filename`** (可选)

    可选的文件名，仅在配合 `file_data` 时有效。

    - `type`: `string`
