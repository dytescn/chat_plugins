# chat_plugins

Piksel 的 AI 对话插件集合。除 `think_base/`、`reply_base/` 资源包外，
每个子目录是一个 Rust `cdylib`，由 cyctron 或 chatllm 加载，统一暴露 `post_chat`，
只负责处理对话中的一段能力。

`think_base/`、`reply_base` 只包含 `skills/` 与 `prompts/`。

> 供应商适配（DeepSeek / SiliconFlow）已迁移到独立系统库
> [`dytescn/cloud_lib`](https://github.com/dytescn/cloud_lib)，本仓库不再包含
> `deepseek/`、`siliconflow_t2t/`。本仓库不保存对话；对话索引与 user 消息由
> `chatllm` 负责。

## 插件

| 目录 | 包名 | cyctron 端点 | 产物 | 配置前缀 |
| --- | --- | --- | --- | --- |
| `do_test_chat/` | `do_test_chat` | `/do_test_chat` | `do_test_chat.dll` / `.dylib` / `.so` | 无（测试插件） |
| `do_test_call/` | `do_test_call` | `/do_test_call` | `do_test_call.dll` / `.dylib` / `.so` | 无（测试 call_chat） |
| `chat_code_check/` | `chat_code_check` | `/chat_code_check` | `chat_code_check.dll` / `.dylib` / `.so` | 无 |
| `chat_ioc/` | `chat_ioc` | `/chat_ioc` | `chat_ioc.dll` / `.dylib` / `.so` | 无 |
| `chat_text_stats/` | `chat_text_stats` | `/chat_text_stats` | `chat_text_stats.dll` / `.dylib` / `.so` | 无 |
| `think_base/` | `think_base` | 无（资源包） | 仅 `skills/`、`prompts/` | 无 |
| `reply_base/` | `reply_base` | 无（资源包） | 仅 `skills/`、`prompts/` | 无 |

`do_test_chat` 是 `type: design_plugins`，安装到 `plugins/do_test_chat/`；
`think_base`、`reply_base` 是只包含 skills 与 prompts 的资源包。

## 与 chatllm / cloud_lib 的关系

```text
宿主 / chat_ui
      │
      ▼
  chatllm ── 写 dialogue / messages
      │
      ├── 需要模型回复时 → cloud_lib（DeepSeek / SiliconFlow）
      └── 业务插件 → /do_test_chat 等 chat_plugins 包
```

`chatllm` 不再内置 provider 调用；供应商选择与 HTTP 调用由 `cloud_lib` 包统一负责。
插件只返回内容，不写 `chatllm` 的数据库。

## 统一调用协议

```http
POST http://127.0.0.1:44944/<endpoint>
Content-Type: application/libary
FFI-Symbol: chat
```

```json
{
  "prompt": "你好",
  "system": "你是一个助手",
  "model": "可选，覆盖 setting"
}
```

响应包络：

```json
{
  "code": 0,
  "data": {
    "content": "...",
    "id": null,
    "model": null,
    "stop_reason": null,
    "usage": null,
    "session_id": null
  },
  "msg": "success"
}
```

- 成功码为 `0`；部分调用方也把 `200` 当作成功；
- 输出缓冲区固定 64 KiB；
- 统一规则：`messages` 非空时优先；为空时回落 `prompt`；两者都无有效内容返回 `400102`；
- `system` 优先级：请求 `system` > setting 中的 `*_app_system_prompt`。

供应商请求 / 响应细节见
[cloud_lib/docs/ffi.md](https://github.com/dytescn/cloud_lib/blob/main/docs/ffi.md)。

## 初始化配置

插件自身如需配置，通过 `setup.json` 调用 `info_lib` 写入 `info.db` 的 `setting` 表。
供应商配置（`deepseek_app_*` / `silicon_app_*`）由 `cloud_lib` 包的 `setup.json` 初始化。

```bash
dytes install sys cloud_lib
```

## 发布

每个包含 `dytes.json` 的包都有独立 workflow，按包名 tag 触发：

| 包 | workflow | 类型 |
| --- | --- | --- |
| `do_test_chat` | `do_test_chat-release.yml` | 代码（三平台） |
| `do_test_call` | `do_test_call-release.yml` | 代码（三平台） |
| `chat_code_check` | `chat_code_check-release.yml` | 代码（三平台） |
| `chat_ioc` | `chat_ioc-release.yml` | 代码（三平台） |
| `chat_text_stats` | `chat_text_stats-release.yml` | 代码（三平台） |
| `think_base` | `think_base-release.yml` | 资源（仅发布） |
| `reply_base` | `reply_base-release.yml` | 资源（仅发布） |

发布步骤：

1. 更新对应目录 `dytes.json` 的 `version`；
2. 提交并推送；
3. 推送 tag：`<包名>-v<版本>`，例如 `do_test_chat-v0.1.4`；
4. workflow 校验 tag 与清单版本一致后，调用
   [publish-package.yml](.github/workflows/publish-package.yml)：
   - 代码包在 Windows / Linux / macOS 三平台分别 `cargo build --release`
     并执行 `dytes publish`；
   - 资源包只在 Ubuntu 上执行 `dytes publish`。

`plugintest/` 是本地 cyctron 测试环境（`type: app`），不是可分发包，不创建发布 workflow。

## 构建与测试

```bash
cd do_test_chat
cargo build --release
cargo test
```

本地单元测试不访问外部 API；真实对话需要 cyctron、`database.dll` / `info.db` 配置和可用的供应商 Key。

## 目录

```text
chat_plugins/
├── do_test_chat/         # 确定性测试聊天插件
├── do_test_call/         # call_chat 测试插件（回调 info_lib.info_list）
├── chat_code_check/      # 对话代码检查插件
├── chat_ioc/             # 对话 IOC 提取插件
├── chat_text_stats/      # 对话文本统计插件
├── think_base/           # think 阶段资源包（skills + prompts）
├── reply_base/           # reply 阶段资源包（skills + prompts）
├── plugintest/           # 本地 cyctron 插件测试环境（运行产物不入库）
├── todo/                 # 待办与验证记录
├── skills/               # 开发、测试、联调、发布规范
├── README.md
├── .gitignore
└── LICENSE
```

各插件文档：

- [think_base/README.md](think_base/README.md)
- [reply_base/README.md](reply_base/README.md)
- [do_test_chat/README.md](do_test_chat/README.md)

工作规范与目录：

- [skills/README.md](skills/README.md)
- [runflow/chat_run.md](runflow/chat_run.md)
- [todo/TODO.md](todo/TODO.md)
- [plugintest/README.md](plugintest/README.md)
