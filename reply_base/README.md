# reply_base

`reply` 阶段的测试资源包，只包含一个 skills 文档、一个 prompts 文件和清单，
不包含 Rust 代码或动态库。

## 目录

```text
reply_base/
├── dytes.json
├── README.md
├── skills/
│   └── reply_base.md
├── prompts.json
└── stubs/
    ├── reply_base.dll
    ├── libreply_base.so
    └── libreply_base.dylib
```

`skills/` 与 `prompts.json` 均直接位于包根，符合 `plugin_lib` 的
[插件包规范](https://github.com/dytescn/designplugin_lib/blob/main/docs/plugin-package.md)。

## 作用

和 `think_base` 一致：

- `skills/`：创建 reply 阶段系统对话的提示词与回复规范；
- `prompts.json`：包根对象数组，安装到 kv 的 `chat_reply` 分类，用于 reply 阶段命中。

区别只在输入：

```text
think_base：输入用户消息
reply_base：输入 plugins 阶段执行完成后的结果
```

## Prompts

`prompts.json` 是对象数组，安装到 designdb 的 `prompts` 集合，并按
`plugins_type = chat_reply` 同步到 kv 的 `chat_reply` 分类。

```json
{
  "prompts": [
    {
      "id": "reply_base_answer",
      "name": "回复结果",
      "content": "回复|回答|结果",
      "category": "reply",
      "sort": 10,
      "is_enabled": 1
    }
  ]
}
```

约定：

- 每个元素必须含稳定 `id`（或 `name`）和非空 `content`；
- `content` 去首尾空白后按 UTF-8 编码不超过 150 字节（约 50 个汉字）；
- kv 记录的 `key` 为 `{uuid}:prompt:{id}`，`value` 与 `text` 都是 `content`；
- 整体覆盖语义：发布时必须提供完整数组。

## 流程

```text
用户输入
  → think：命中分类，拆分为 plugins 查询数组
  → plugins：逐段执行，合并结果
  → reply：检索 chat_reply
       ├─ 命中可调用插件：按插件生成最终回复
       └─ 只命中 reply_base / 未命中：按 reply_base 清洗结果
```

`reply_base` 清洗规则（chatllm `src/utils/reply_base.rs`）：

1. 按空行拆分为多段；
2. 每段去掉 `do_chat_test` 及其后的数字时间；
3. 保留前面的文本；
4. 多段用空行连接；
5. 清洗后为空时回退用户输入。

## 发布

```bash
dytes publish
```

`dytes.json` 的导出映射：

| 本地文件 | 安装后文件 |
| --- | --- |
| `./skills/reply_base.md` | `reply_base.md` |
| `./prompts.json` | `prompts.json` |

设计插件安装要求导出目标为扁平文件名，安装后目录为：

```text
plugins/reply_base/
├── reply_base.md
├── prompts.json
└── dytes.json
```

> `chatllm` 的 `reply_base::prompts()` 目前仍尝试读取安装目录下的 `reply_base.json`；
> 该读取只用于日志，失败不影响清洗流程。后续需同步改为 `prompts.json`。

> `plugin_lib` 的 `install_plugin` 当前严格要求当前平台库文件。本包是纯资源包，
> 在 `stubs/` 提供三平台占位库（0 字节），只用于通过安装校验，不会被调用；
> 发布时按平台导出为 `reply_base.dll` / `libreply_base.so` / `libreply_base.dylib`。
> 资源包发布 workflow 因此需要三平台分别执行 `dytes publish`。

## 推荐测试输入

reply 阶段的输入是 plugins 结果，推荐用以下结果验证资源命中：

| 输入 | 期望命中的 prompt |
| --- | --- |
| 测试插件输出 | 插件结果（`插件结果`、`执行结果`、`查询结果`） |
| 系统信息已生成 | 系统信息（`系统信息`、`环境信息`、`版本信息`）/ 完成状态 |
| 插件结果已保存 | 插件结果 / 完成状态 |

`chat_reply` 资源命中与插件调用是两层判定：资源命中但无可调用插件时，
chatllm 会标记 `resource_hit` 并回退 reply_base 清洗。
