# 测试环境

## 前置

| 组件 | 要求 |
| --- | --- |
| 操作系统 | Windows 10/11 x64（与本地 Piksel / cyctron 一致） |
| Rust | stable + MSVC 工具链 |
| cyctron | 1.0.13 或与本地 Piksel 一致 |
| Rust 测试 | `cargo test`，不依赖外部 API |
| 真实对话 | cyctron + `info.db` + 供应商 API Key |

## 环境变量

| 变量 | 默认值 | 说明 |
| --- | --- | --- |
| `CYCTRON_BASE` | `http://127.0.0.1:44944` | cyctron 地址 |
| `DYTES_PLATFORM` | 当前系统 | dytes 平台覆盖 |
| `DYTES_S3_*` | — | 发布时由 dytes / CI 注入，不写入仓库 |

## 配置

API Key 通过 `cloud_lib` 包的 `setup.json` 写入 `info.db` 的 `setting` 表：

- DeepSeek：`deepseek_app_key`、`deepseek_app_base_url`、`deepseek_app_model` 等；
- SiliconFlow：`silicon_app_key`、`silicon_app_base_url`、`silicon_app_model` 等。

真实 Key 只保留在本地测试环境，不提交。清空测试库时重新执行 `setup.json`。

## 版本与位数

- DLL、cyctron、宿主必须同为 x64；
- 修改代码后必须重新编译并覆盖 `plugintest/libary/` 中的 DLL；
- 覆盖前停止 cyctron，避免文件被占用。

## 自检

- [ ] `cargo test` 通过
- [ ] `cargo build --release` 产物存在
- [ ] `plugintest/libary/` 中 DLL 时间戳为本次构建
- [ ] cyctron 已启动，端口可访问
- [ ] `info.db` 中已配置对应 API Key
- [ ] 不把 `plugintest/` 运行数据提交到 git
