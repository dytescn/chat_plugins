# think_base

`think` 阶段的测试资源包，只包含一个 skills 文档、一个 prompts 文件和清单，
不包含 Rust 代码或动态库。

## 目录

```text
think_base/
├── dytes.json
├── README.md
├── skills/
│   └── think_base.md
├── prompts.json
└── stubs/
    ├── think_base.dll
    ├── libthink_base.so
    └── libthink_base.dylib
```

`skills/` 与 `prompts.json` 均直接位于包根，符合 `plugin_lib` 的
[插件包规范](https://github.com/dytescn/designplugin_lib/blob/main/docs/plugin-package.md)：
技能只读取包根 `skills/` 下一层 `.md`，提示词只读取包根 `prompts.json`。

## Skills

`skills/think_base.md` 用来创建 think 阶段的系统对话提示词，包含：

- 角色与目标
- 触发条件
- 输入与执行步骤
- 输出要求与边界

首个 `# ` 标题写入技能 `name`；文件去 `.md` 后作为技能 `_id`（`think_base`）。
文件不使用 YAML frontmatter，也不包含模板占位符。

## Prompts

`prompts.json` 是对象数组，安装到 designdb 的 `prompts` 集合，并按
`plugins_type = chat_think` 同步到 kv 的 `chat_think` 分类。

```json
{
  "prompts": [
    {
      "id": "think_base_greeting",
      "name": "问候",
      "content": "你好|问候",
      "category": "think",
      "sort": 10,
      "is_enabled": 1
    }
  ]
}
```

约定：

- 每个元素必须含稳定 `id`（或 `name`）和非空 `content`；`id` 发布后不随文案修改；
- `content` 去首尾空白后按 UTF-8 编码不超过 150 字节（约 50 个汉字）；
- `content` 中的 `|` 是 plugins 查询段分隔符，chatllm 会把每段分别检索 `chat_plugin`；
- kv 记录的 `key` 为 `{uuid}:prompt:{id}`，`value` 与 `text` 都是 `content`；
- 整体覆盖语义：发布时必须提供完整数组，不能只写增量。

## 流程

```text
用户输入
  → think：检索 chat_think，命中 think_base 的某条 prompt
  → 取 value，按 | 拆分为 plugins 查询数组
  → plugins：逐段检索并执行插件
  → reply：按 reply_base 或命中的 reply 插件生成最终内容
```

- `meta.plugin_uuid` / `meta.plugin_name` 由 `plugin_lib` 写入，chatllm 用它们定位对话和
  `./plugins/think_base/think_base.md`；
- 本资源包不直接访问数据库、不调用供应商。

## 发布

```bash
dytes publish
```

`dytes.json` 的导出映射：

| 本地文件 | 安装后文件 |
| --- | --- |
| `./skills/think_base.md` | `think_base.md` |
| `./prompts.json` | `prompts.json` |

`type` 保持 `design_plugins` 以便 dytes 安装到 `plugins/think_base/`；
`plugins_type = chat_think` 决定 kv 同步集合；`target = common` 是 `plugin_lib` 读取的目标值，
`targets` 同时保留给 dytes 清单校验。

安装后目录：

```text
plugins/think_base/
├── think_base.md
├── prompts.json
└── dytes.json
```

> `plugin_lib` 的 `install_plugin` 当前严格要求当前平台库文件。本包是纯资源包，
> 在 `stubs/` 提供三平台占位库（1 字节），只用于通过安装校验，不会被调用；
> 发布时按平台导出为 `think_base.dll` / `libthink_base.so` / `libthink_base.dylib`。
> 资源包发布 workflow 因此需要三平台分别执行 `dytes publish`。

## 推荐测试输入

与 `datasets/basic-dialogue-test` 对齐的推荐输入与期望关键词：

| 输入 | 期望命中的 prompt |
| --- | --- |
| 你好 | 问候（`你好`、`问候`） |
| 测试插件回显 | 插件能力（`插件`、`可用插件`） |
| 继续测试插件 | 插件能力（`插件`、`可用插件`） |
| 系统信息 | 系统信息（`系统信息`、`运行环境`） |
| 继续系统信息 | 系统信息（`系统信息`、`运行环境`） |
| 总结系统信息 | 系统信息 / 总结分析 |
| 示例插件 | 示例教程 / 插件能力 |
| 测试示例插件 | 示例教程 / 插件能力 |
| 总结插件结果 | 插件结果（`插件结果`、`执行结果`） |
实际选中仍受 `chatllm` 的 `ai_match_min_score` 与 `kv_lib` 排序影响。
