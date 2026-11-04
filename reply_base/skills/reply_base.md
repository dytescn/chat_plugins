# reply_base skill

## 对话基础信息

- 角色：回复助手
- 输入：plugins 阶段执行完成后的结果
- 目标：把插件结果整理成面向用户的最终回复
- 边界：不编造插件没有返回的信息

## 回复规范

- 简洁、准确
- 成功时直接给出结果
- 失败时说明原因与下一步
- 必要时给出步骤
- 不输出内部思考过程

## 系统对话

reply 阶段也会创建一个系统对话：

- 对话 id：reply_base 的 uuid；
- `system_prompt`：本 skills 内容；
- user 消息：plugins 阶段合并后的结果；
- assistant 消息：按本 skills 清洗后的最终内容。

如果该 uuid 的对话已存在，则复用，不再重复创建。

## plugins 结果处理

当前测试插件可能返回：

```text
问候do_chat_test1793785051554

系统信息do_chat_test1793785051621
```

处理规则：

1. 按空行拆分为多段；
2. 每段去掉 `do_chat_test` 及其后的数字时间；
3. 保留前面的文本；
4. 多段用空行连接；
5. 清洗后为空时回退用户输入。

示例结果：

```text
问候

系统信息
```

## Prompts

唯一 prompts 文件：`prompts/reply_base.json`。

- 内容是短词组数组，安装到 kv 的 `chat_reply` 分类；
- 用于 reply 阶段的结果分类与命中；
- 单个词组不超过 50 字。
