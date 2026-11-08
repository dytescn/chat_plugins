# think_base

`think` 阶段的测试资源包，只包含一个 skills 文档和一个 prompts JSON 文件，
不包含 Rust 代码或动态库。

## 目录

```text
think_base/
├── dytes.json
├── README.md
├── skills/
│   └── think_base.md
└── prompts/
    └── think_base.json
```

## Skills

`skills/think_base.md` 用来创建对话的基础信息，包括：

- 角色与目标
- 能力边界
- 最终回复规范

## Prompts

`prompts/think_base.json` 是一个 JSON 字符串数组，安装到 kv 的
`chat_think` 分类。词组尽量覆盖常见说法，方便命中；单个词组不超过 50 字。

```json
["你好", "您好", "系统信息", "版本", "配置", "插件", "搜索"]
```

## 流程

```text
用户输入
  → think：命中 prompts 中的短词组，按 | 取最后一段
  → plugins：用该段选择并执行插件
  → reply：按 skills 中的回复规范输出最终内容
```

- `skills`：创建对话基础信息与回复规范；
- `prompts`：短词组，安装到 kv，用于 think 命中；
- `think`：把用户输入转换为 plugins 查询。

本资源包不直接访问数据库、不调用供应商。

## 发布

```bash
dytes publish
```

`dytes.json` 的 `exports` 只映射 `skills/think_base.md` 与
`prompts/think_base.json`。

设计插件安装要求导出目标为扁平文件名，因此安装后目录为：

```text
plugins/think_base/
├── think_base.md
├── think_base.json
└── dytes.json
```


## 推荐测试输入

与 `datasets/basic-dialogue-test` 对齐的推荐输入与期望关键词：

| 输入 | 期望 think 关键词 |
| --- | --- |
| 你好 | 你好 |
| 测试插件回显 | 插件 / 可用插件 |
| 继续测试插件 | 插件 / 可用插件 |
| 系统信息 | 系统信息 / 系统状态 |
| 继续系统信息 | 系统信息 / 系统状态 |
| 总结系统信息 | 系统信息 / 系统状态 |
| 示例插件 | 示例 / 插件 |
| 测试示例插件 | 示例 / 插件 |
| 总结插件结果 | 插件 / 插件结果 |

完整机器可读版本见 [`tests/recommended-inputs.json`](tests/recommended-inputs.json)。
实际选中仍受 `chatllm` 的 `ai_match_min_score` 与 `kv_lib` 排序影响。
