# 聊天运行流程（chat_run）

记录一次对话从宿主发起到拿到模型回复、再到消息落库的运行链路。范围是
`chat_plugins` 与 `chatllm` 的协作方式，不展开各供应商 API 的内部协议。

## 1. 组件

| 组件 | 位置 | 职责 |
| --- | --- | --- |
| 宿主 / `chat_ui` | Piksel | 编排对话：创建对话、选择供应商、发起生成、写回结果 |
| `chatllm` | `libary/chatllm.dll` | 对话索引与消息持久化；写 user 消息，不生成回复 |
| `chat_plugins` | `libary/<plugin>.dll`、`plugins/do_test_chat/` | 模型生成：`deepseek`、`siliconflow_t2t`、`do_test_chat` |
| `info_lib` | `libary/info_lib.dll` | `info.db` 的 `setting` / `router` 配置 |
| `cyctron` | 安装根目录 | 加载动态库，统一 `POST /<lib>` + `FFI-Symbol` 调用 |

存储边界：

- `chatllm` 写 `database/dialogue.db` 的 `dialogue` 与 `designdb/<dialogue_id>` 的 `messages`；
- `chat_plugins` 只返回生成内容，不写 `chatllm` 的数据库，也不写 `dialogue.db`。

## 2. 前置条件

```bash
cd plugintest
dytes update          # 安装 cyctron、chatllm、sys 库与 qwen3-embed
```

- `dytes.json` 声明的依赖已安装到 `libary/`（`chatllm`、`database`、`designdb`、
  `info_lib`、`kv_lib`、`markparse`、`plugin_lib`）；
- 供应商插件已构建：`deepseek`、`siliconflow_t2t` 复制到 `libary/`，
  `do_test_chat` 安装到 `plugins/do_test_chat/`；
- 供应商配置由各插件的 `setup.ts` 写入 `info.db` 的 `setting`；
- `cyctron` 已启动，默认 `127.0.0.1:44944`。

## 3. 主流程

```text
chat_ui / 宿主
   │
   │ 1. POST /chatllm  FFI-Symbol: chat
   │    新建/续聊，写入 user 消息
   ▼
chatllm ── database/dialogue.db、designdb/<uuid>
   │
   │ 2. 返回 { uuid, system_prompt, session_id, ... }
   ▼
chat_ui / 宿主
   │
   │ 3. POST /lib<provider>  FFI-Symbol: chat
   │    prompt / messages / system / model
   ▼
chat_plugin（deepseek / siliconflow_t2t / do_test_chat）
   │
   │ 4. 读取 info.db setting（测试插件可跳过）
   │    调用供应商 API，或直接返回确定性内容
   ▼
{ code, data: { content, model, usage, ... }, msg }
   │
   │ 5. 宿主把 assistant 消息写回对话，并更新标题/关键字/计数
   ▼
chat_ui 刷新
```

关键点：

1. `chatllm` 的 `chat` 只负责创建 / 续聊并写入 **user** 消息，不调用模型；
2. 模型选择由宿主决定，`chatllm` 不感知供应商；
3. `chatllm` 当前不写 assistant 消息，回复落库由宿主（或后续接口）负责；
4. `system` 优先使用请求级字段，其次使用供应商配置；
5. 同一次对话的多轮上下文由宿主从 `chatllm` 读取后传给供应商插件。

## 4. 协议

### 4.1 chatllm

```http
POST http://127.0.0.1:44944/chatllm
Content-Type: application/libary
FFI-Symbol: chat
```

```json
{ "prompt": "你好", "uuid": "可选，续聊", "system_prompt": "可选" }
```

成功响应 `code = 200`，`data` 至少包含 `uuid`、`created`、`prompt`、
`system_prompt`、`session_id`；`content` 固定为空。

### 4.2 聊天插件

```http
POST http://127.0.0.1:44944/lib<provider>
Content-Type: application/libary
FFI-Symbol: chat
```

```json
{
  "prompt": "你好",
  "messages": [{ "role": "user", "content": "..." }],
  "system": "可选",
  "model": "可选"
}
```

成功响应 `code = 0`，`data.content` 为回复文本。调用方应同时接受 `0` 与 `200`
两种成功码。

两个入口的输出缓冲区固定 64 KiB，超长返回 `500901`，不截断 JSON。

## 5. 供应商

| 供应商 | 端点 | 配置前缀 | 说明 |
| --- | --- | --- | --- |
| `deepseek` | `/libdeepseek` | `deepseek_app_*` | DeepSeek Anthropic 兼容接口 |
| `siliconflow_t2t` | `/libsiliconflow_t2t` | `silicon_app_*` | SiliconFlow 文本对话 |
| `do_test_chat` | `/do_test_chat` | 无 | 确定性测试插件，安装到 `plugins/do_test_chat/` |

`do_test_chat` 支持 `reply` 指定固定回复、`fail_code` / `fail_msg` 模拟失败，
用于验证 `chatllm + 插件 + cyctron` 链路。

cyctron 会把 FFI 返回值非 0 视为插件调用失败并返回 404，因此插件的业务错误
只写入响应体的 `code`，`post_chat` 本身始终返回 0。

### 5.1 ai_supplier 选择与配置

`do_test_chat` 每次运行都会解析 `ai_supplier`（请求字段优先，其次 `info.db` 的
`setting`），只返回状态与提示，不写配置、不转发请求。

```text
POST /do_test_chat
      │
      ├─ ai_supplier 为空 ───► status = need_supplier
      │      宿主展示 suppliers 供用户选择
      │      宿主经 info_lib 写入 ai_supplier
      │
      ├─ 值不在候选 ─────────► status = invalid_supplier
      │      宿主重新选择
      │
      ├─ 已选但缺必填项 ─────► status = need_config
      │      宿主按 missing 提示用户输入
      │      宿主经 info_lib 写入对应 setting
      │
      └─ 配置完整 ───────────► status = ready
             宿主调用 endpoint 指向的供应商插件
```

| status | 关键字段 | 宿主下一步 |
| --- | --- | --- |
| `need_supplier` | `suppliers[]`、`prompt` | 选择供应商并写入 `ai_supplier` |
| `invalid_supplier` | `supplier`、`suppliers[]` | 重新选择 |
| `need_config` | `supplier`、`endpoint`、`missing[]`、`config` | 提示输入并写入对应 setting |
| `ready` | `supplier`、`endpoint`、`config` | 调用供应商端点 |
| `config_error` | `prompt` | 检查 database / info_lib |

候选与必填参数：

| value | label | endpoint | 必填 |
| --- | --- | --- | --- |
| `deepseek` | DeepSeek | `/libdeepseek` | `deepseek_app_key` |
| `siliconflow_t2t` | SiliconFlow | `/libsiliconflow_t2t` | `silicon_app_key` |

可选参数（有默认值）通过 `optional` 返回，例如 `*_app_base_url`、`*_app_model`；
密钥类字段只出现在 `required` / `missing` 中，不会回传具体值。

## 6. 错误处理

| 现象 | 可能原因 | 处理 |
| --- | --- | --- |
| `404 / 找不到库` | 插件未部署或端点名不符 | 检查 `libary/` 与 `dytes.json` 导出名 |
| `400102` | 请求没有 `prompt` / `messages` | 补齐输入 |
| `500101` | 供应商请求失败 | 检查 Key、`base_url`、网络与 `info.db` |
| `500901` | 回复超过 64 KiB | 缩短回复或由宿主改为流式 / 分片 |
| `dialogue` 查询失败 | `setup.ts` 未执行 | 执行 `libary/chatllm/setup.ts` |
| 配置读不到 | `info.db` 无对应 key | 执行供应商 `setup.ts` |

## 7. 本地复现

```bash
cd plugintest
./cyctron.exe start

# 1) 写入 user 消息
curl -s -X POST http://127.0.0.1:44944/chatllm \
  -H "Content-Type: application/libary" \
  -H "FFI-Symbol: chat" \
  -d '{"prompt":"你好"}'

# 2) 调用测试供应商，验证链路
curl -s -X POST http://127.0.0.1:44944/do_test_chat \
  -H "Content-Type: application/libary" \
  -H "FFI-Symbol: chat" \
  -d '{"prompt":"你好","reply":"pong"}'

# 3) 调用真实供应商
curl -s -X POST http://127.0.0.1:44944/libdeepseek \
  -H "Content-Type: application/libary" \
  -H "FFI-Symbol: chat" \
  -d '{"prompt":"你好","max_tokens":64}'
```

预期：测试插件返回 `"content": "pong"`；真实供应商返回模型生成内容。

## 8. 相关文档

- 仓库结构与依赖：[../skills/instro.md](../skills/instro.md)
- 插件编码约定：[../skills/rust.md](../skills/rust.md)
- 插件测试环境：[../skills/3.plugintest.md](../skills/3.plugintest.md)
- `chatllm` 接口：[../../sys/chatllm/docs/ffi.md](../../../sys/chatllm/docs/ffi.md)
