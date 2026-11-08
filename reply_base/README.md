# reply_base

`reply` 阶段的测试资源包，只包含一个 skills 文档和一个 prompts JSON 文件，
不包含 Rust 代码或动态库。

## 目录

```text
reply_base/
├── dytes.json
├── README.md
├── skills/
│   └── reply_base.md
└── prompts/
    └── reply_base.json
```

## 作用

和 `think_base` 一致：

- `skills/`：创建 reply 对话的基础信息与回复规范；
- `prompts/`：短词组，安装到 kv 的 `chat_reply` 分类，用于 reply 阶段命中。

区别只在输入：

```text
think_base：输入用户消息
reply_base：输入 plugins 阶段执行完成后的结果
```

## 流程

```text
用户输入
  → think：命中分类，拆分为 plugins 查询数组
  → plugins：逐段执行，合并结果
  → reply：接收 plugins 结果，按 reply_base 规范输出最终内容
```

## 发布

```bash
dytes publish
```

设计插件安装要求导出目标为扁平文件名，安装后目录为：

```text
plugins/reply_base/
├── reply_base.md
├── reply_base.json
└── dytes.json
```


## 推荐测试输入

reply 阶段的输入是 plugins 结果，推荐用以下结果验证资源命中：

| 输入 | 期望资源关键词 |
| --- | --- |
| 测试插件输出 | 插件 / 插件结果 |
| 系统信息已生成 | 系统信息 / 已生成 |
| 插件结果已保存 | 插件结果 / 已保存 |

完整机器可读版本见 [`tests/recommended-inputs.json`](tests/recommended-inputs.json)。

`chat_reply` 资源命中与插件调用是两层判定：资源命中但无可调用插件时，
chatllm 会标记 `resource_hit` 并回退 reply_base 清洗。
