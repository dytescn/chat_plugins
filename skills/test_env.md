# 测试环境

## 前置

| 组件 | 要求 |
| --- | --- |
| 操作系统 | Windows / Linux / macOS |
| Rust | stable + MSVC 工具链（Windows） |
| Python | 3.8+，与动态库位数一致 |
| Rust 测试 | `cargo test`，不依赖外部 API |
| 接口测试 | `scripts/ffi_call.py` 直接加载动态库，不经过 cyctron |

## 环境变量

| 变量 | 默认值 | 说明 |
| --- | --- | --- |
| `DYTES_PLATFORM` | 当前系统 | dytes 平台覆盖 |
| `DYTES_S3_*` | — | 发布时由 dytes / CI 注入，不写入仓库 |

不再使用 `CYCTRON_BASE` 或 `127.0.0.1:44944` 端点。

## 配置

API Key 通过 `cloud_lib` 包的 `setup.json` 写入 `info.db` 的 `setting` 表：

- DeepSeek：`deepseek_app_key`、`deepseek_app_base_url`、`deepseek_app_model` 等；
- SiliconFlow：`silicon_app_key`、`silicon_app_base_url`、`silicon_app_model` 等。

真实 Key 只保留在本地测试环境，不提交。清空测试库时重新执行 `setup.json`。

## 版本与位数

- Python、动态库必须同为 x64；
- 修改代码后重新 `cargo build --release`，直接使用 `target/release/` 下的新产物；
- 依赖 `info.db` 的库通过 `--cwd plugintest` 切换工作目录。

## 自检

- [ ] `cargo test` 通过
- [ ] `cargo build --release` 产物存在
- [ ] `python scripts/ffi_call.py --lib <产物> --input ...` 返回成功
- [ ] `plugintest/database/info.db` 中已配置对应 API Key（需要时）
- [ ] 不把 `plugintest/` 运行数据提交到 git
