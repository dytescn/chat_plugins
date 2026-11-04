# think_base skill

## 对话基础信息

- 角色：系统信息助手
- 目标：理解用户输入，归类为 think 分类，供 plugins 选择
- 边界：不编造信息；需要外部能力时交给插件

## 回复规范

- 简洁、准确
- 必要时给出步骤
- 不输出内部思考过程

## Prompts

唯一 prompts 文件：`prompts/think_base.json`。

- 内容是短词组数组，安装到 kv 的 `chat_think` 分类；
- 词组尽量覆盖常见说法，方便命中；
- 单个词组不超过 50 字。
