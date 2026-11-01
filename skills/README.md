# chat_plugins 开发规范（skills）

本目录记录 `chat_plugins` 的开发、测试、联调和发布约定，参考
`coreldraw_plugins/skills` 的组织方式，但只保留适用于聊天供应商插件的内容。

## 文档索引

| 文档 | 用途 |
| --- | --- |
| [instro.md](instro.md) | 仓库结构、两个插件的职责与调用链 |
| [1.build_test.md](1.build_test.md) | 本地构建、单元测试、常见编译问题 |
| [2.publish.md](2.publish.md) | `dytes.json`、版本与发布前检查 |
| [3.plugintest.md](3.plugintest.md) | 使用 `plugintest/` 做插件测试 |
| [rust.md](rust.md) | Rust FFI / JSON 协议的编码约定 |
| [test_env.md](test_env.md) | 测试环境变量与运行前置条件 |

## 阅读顺序

1. 不熟悉仓库：先读 `instro.md`；
2. 写代码：读 `rust.md`；
3. 本地构建与测试：读 `1.build_test.md`；
4. 启动插件测试环境：读 `test_env.md`、`3.plugintest.md`；
5. 准备发布：读 `2.publish.md`。

## 约定

- 每份文档只写一个主题；命令以仓库实际路径为准；
- 真实 API Key 只写入本地 `info.db`，不进入 README、脚本或提交；
- 本地运行产物（`plugintest/libary/`、`database/`、`target/`）不入库；
- 新增插件时同步更新根 README、`dytes.json` 和本文档索引；
- 与 `chatllm` 的边界保持清晰：本仓库负责模型生成，不写对话数据库。
