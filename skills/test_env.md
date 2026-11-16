# 测试环境

## 前置

| 组件 | 要求 |
| --- | --- |
| 操作系统 | Windows / Linux / macOS |
| Rust | stable + MSVC 工具链（Windows） |
| Deno | 2.x（FFI 使用 `Deno.dlopen`） |
| dytes | 包含 `mcp.tools` 对象数组校验修复的版本 |
| Rust 测试 | `cargo test`，不依赖外部 API |
| 接口测试 | `dytes test` 搭建环境，`ts/ffi_test.ts` 直连动态库 |

## 环境变量

| 变量 | 默认值 | 说明 |
| --- | --- | --- |
| `DYTES_PLATFORM` | 当前系统 | dytes 平台覆盖 |
| `DYTES_S3_*` | — | 发布时由 dytes / CI 注入，不写入仓库 |

不再使用 `CYCTRON_BASE` 或 `127.0.0.1:44944` 端点。

## 环境搭建

```bash
cd think_base_call
cargo build --release
dytes test .
```

`dytes test` 在包目录创建 `.dytes/`，依赖下载到 `.dytes/env/`，并在
`.dytes/run-<pid>/` 做安装演练。`test-dependency` 用于补充仅测试需要的依赖。

## 配置

需要 `info.db` 时，用 `--cwd` 指向包含 `database/info.db` 的目录。真实 Key 只保留在
本地环境，不提交；清空测试库时重新执行对应 `setup.json`。

## 版本与位数

- Deno、动态库必须同为 x64；
- 修改代码后重新 `cargo build --release`；
- `.dytes/` 由 dytes test 管理，不手工修改。

## 自检

- [ ] `cargo test` 通过
- [ ] `cargo build --release` 产物存在
- [ ] `dytes test .` 返回 `OK: 包结构检查通过`
- [ ] `deno task ffi --lib <产物> --input ...` 返回成功
- [ ] 不把 `.dytes/`、`target/` 与运行数据提交到 git
