# 仓库结构

`chat_plugins` 是 Piksel 的 AI 对话供应商插件集合。每个子目录是一个 Rust `cdylib`，
由 cyctron 加载，对外暴露 `post_chat`。

## 目录

```text
chat_plugins/
├── deepseek/              # DeepSeek Anthropic 兼容接口
├── siliconflow_t2t/       # SiliconFlow 文本对话
├── think_base/            # think 阶段资源包（skills + prompts）
├── reply_base/            # reply 阶段资源包（skills + prompts）
├── do_test_chat/          # 确定性测试聊天插件
├── runflow/               # 运行流程文档
├── plugintest/            # 本地插件测试环境（运行产物不入库）
├── todo/                  # 待办与验证记录
├── skills/                # 开发规范（本目录）
├── README.md
├── .gitignore
└── LICENSE
```

## 插件与资源包

| 名称 | 入口 | 端点 | 配置前缀 |
| --- | --- | --- | --- |
| `deepseek` | `post_chat` | `/libdeepseek` | `deepseek_app_*` |
| `siliconflow_t2t` | `post_chat` | `/libsiliconflow_t2t` | `silicon_app_*` |
| `do_test_chat` | `post_chat` | `/do_test_chat` | 无（测试插件） |
| `think_base` | 无 | 无（资源包） | `chat_think` |
| `reply_base` | 无 | 无（资源包） | `chat_reply` |

## 调用链

```text
chat_ui / 宿主
      │ 需要模型回复
      ▼
POST /libdeepseek、/libsiliconflow_t2t 或 /do_test_chat
      │
      ├─ 读取 info.db 的 setting
      ├─ 组装 messages
      ├─ 调用供应商 API
      └─ 返回 { code, data: { content, ... }, msg }
```

`chatllm` 只负责对话索引和消息存储；本仓库插件不写 `dialogue.db` / `designdb`。

## 公共协议

- `Content-Type: application/libary`
- `FFI-Symbol: chat`
- 输入为 JSON，输出缓冲区 64 KiB
- 成功码 `0`（部分客户端兼容 `200`）
- 失败响应：`{ "code": <错误码>, "data": {}, "msg": "<原因>" }`

## 依赖的系统库

聊天插件自己不建表、不直接链接 SQLite；数据库能力来自 cyctron 加载的 sys 库。
本仓库只依赖 `info_lib`，了解 `designplugin_lib` 是为了划清边界。

### info_lib

系统基础信息库（System Basic Information Repository），产物固定为 `libary/info_lib.dll`。

| 范围 | 作用 |
| --- | --- |
| `database/info.db` 的 `router` | 路由注册与读取：`router_get` / `router_list` / `router_post` / `router_update` |
| `database/info.db` 的 `setting` | 全局配置读写：`info_get` / `info_list` / `info_post` / `info_update` |
| `designdb/setting` | `note` 字段写入的 `setting_note` 分类对象 |

在 `chat_plugins` 中的用法：

- `setup.ts` 通过 `POST /info_lib` 的 `info_post` / `info_update` 写入
  `deepseek_app_*`、`silicon_app_*` 配置；
- 插件运行时用 `utils/db.rs` 调 `database.dll` 的 `post_db_query` 读 `info.db.setting`，
  再用 `utils/setting.rs` 解析成配置结构；
- API Key 只存在于 `info.db`，不进入仓库、脚本或日志；
- `info_lib` 不管对话消息存储，本仓库插件也不写 `dialogue.db` / `designdb`。

### designplugin_lib（产品名 plugin_lib）

设计插件的数据接口库，产物为 `libary/plugin_lib.dll`，服务对象是设计插件管理界面，
不是聊天供应商插件。

| 范围 | 作用 |
| --- | --- |
| `database/plugins.db` | 插件、分类、版本、配置定义、安装状态、动态库、FFI 能力清单的 `get` / `post` / `update` / `patch` |
| `designdb/skills` | 经 `libdesigndb.dll` 写入技能完整内容（best-effort，缺库不影响安装/扫描） |
| 配置值 | 用户值仍存 `info.db.setting`（`category = 'plugin'`，`key = 'plugin.config.<plugin_uuid>.<param_key>'`），`get_configs` 合并默认值并返回 `source` |

边界：

- 不负责设计软件的发现、启动与运行状态（由 `designsoft` 负责）；
- 不提供向量表与语义检索（归 `chatllm`），只提供能力清单与向量化状态；
- `chat_plugins` 的 `deepseek` / `siliconflow_t2t` 不依赖 `designplugin_lib`；
  只有需要读取设计插件元数据或配置时才通过 `/plugin_lib` 访问。

## 单个插件的结构

```text
<plugin>/
├── Cargo.toml
├── Cargo.lock
├── dytes.json
├── setup.ts
├── README.md
├── src/
│   ├── lib.rs
│   ├── chat/ffi.rs
│   └── utils/{cjson,db,errcode,setting,<provider>}.rs
├── tests/
└── ts/
```

新增插件应复用相同结构，并保持 `post_chat` 的请求/响应语义一致。
