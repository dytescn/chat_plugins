# chat_plugins

Piksel 的 AI 对话供应商插件集合。每个子目录是一个 Rust `cdylib`，由 cyctron 加载，
统一暴露 `post_chat`，只负责把对话请求转发给对应供应商并返回生成内容。

本仓库不保存对话；对话索引与 user 消息由 `chatllm` 负责。

## 插件

| 目录 | 包名 | cyctron 端点 | 产物 | 配置前缀 |
| --- | --- | --- | --- | --- |
| `deepseek/` | `deepseek` | `/libdeepseek` | `libdeepseek.dll` / `.dylib` / `.so` | `deepseek_app_*` |
| `siliconflow_t2t/` | `siliconflow_t2t` | `/libsiliconflow_t2t` | `libsiliconflow_t2t.dll` | `silicon_app_*` |
| `do_test_chat/` | `do_test_chat` | `/do_test_chat` | `do_test_chat.dll` | 无（测试插件） |

`deepseek`、`siliconflow_t2t` 是 `type: sys`，安装到 `libary/`；`do_test_chat`
是 `type: design_plugins`，安装到 `plugins/do_test_chat/`。

## 与 chatllm 的关系

```text
宿主 / chat_ui
      │
      ▼
  chatllm ── 写 dialogue / messages
      │
      └── 需要模型回复时，由调用方选择：
              ├─ /libdeepseek
              ├─ /libsiliconflow_t2t
              └─ /do_test_chat（测试）
```

`chatllm` 不再内置 siliconflow、skills 或向量能力；本仓库的插件独立提供模型生成。
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
  "data": { "content": "..." },
  "msg": "success"
}
```

- 成功码为 `0`；部分调用方也把 `200` 当作成功。
- 输出缓冲区固定 64 KiB。
- `prompt` 与 `messages` 至少提供一个；详细字段见各插件 README。

## 初始化配置

两个包都通过 `setup.ts` 调用 `info_lib`，把供应商配置写入 `info.db` 的 `setting` 表。
配置值中包含 API Key，仓库内只保留占位符，不提交真实密钥。

```bash
# 由 dytes 执行 init 脚本
dytes install sys deepseek
dytes install sys siliconflow_t2t
```

## 构建与测试

```bash
cd deepseek
cargo build --release
cargo test

cd ../siliconflow_t2t
cargo build --release
cargo test
```

本地单元测试不访问外部 API；真实对话需要 cyctron、`database.dll` / `info.db` 配置和可用的供应商 Key。

## 目录

```text
chat_plugins/
├── deepseek/             # DeepSeek Anthropic 兼容聊天
├── siliconflow_t2t/      # SiliconFlow 文本对话
├── plugintest/           # 本地 cyctron 插件测试环境（运行产物不入库）
├── todo/                 # 待办与验证记录
├── skills/               # 开发、测试、联调、发布规范
├── README.md
├── .gitignore
└── LICENSE
```

各插件文档：

- [deepseek/README.md](deepseek/README.md)
- [siliconflow_t2t/README.md](siliconflow_t2t/README.md)
- [do_test_chat/README.md](do_test_chat/README.md)

工作规范与目录：

- [skills/README.md](skills/README.md)
- [runflow/chat_run.md](runflow/chat_run.md)
- [todo/TODO.md](todo/TODO.md)
- [plugintest/README.md](plugintest/README.md)
