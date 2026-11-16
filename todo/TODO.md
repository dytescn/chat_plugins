# TODO

## 已完成

- [x] 将 `deepseek`、`siliconflow_t2t` 从 `chat_plugins` 迁出
- [x] 供应商调用合并到独立系统库 `dytescn/cloud_lib`
- [x] 删除嵌套 `.git`，统一由 `chat_plugins` 管理
- [x] 整理根 README 与插件 README
- [x] 发布 `think_base_call` 0.1.0：三平台 CI 发布成功；本地 FFI 冒烟通过
  （`input`/`prompt` 返回 `prompts=["问候"]`、`outputs=["测试"+输入]`，空输入返回 `400102`）
- [x] 发布 `think_base_call` 0.1.1：`taskflow.json` 改为规范四字段（`ffi_symbol = chat`），
  三平台 CI 发布成功；`plugins_lib` 文档与 issue 已同步
- [x] 发布 `think_base_call` 0.1.3：`mcp.json` / `taskflow.json` 的 `ffi_symbol` 统一为
  `post_chat`，三平台 CI 发布成功（0.1.2 的 linux 任务失败，升版重发后通过）
- [x] 测试环境改为 `dytes test` + Deno（`ts/ffi_test.ts`）直连动态库，不再使用 cyctron
- [x] 发布 `think_base_call` 0.1.4：`prompts.json` 增加 `base` 关键词，三平台 CI 发布成功
- [ ] 确认 `think_base_call` 技能安装方案（dytes 拍平 `skills/`，`plugin_lib` 未登记技能）

## 待办

- [ ] 用真实 DeepSeek Key 在 `cloud_lib` 仓库跑一次 `chat` 冒烟测试
- [ ] 用真实 SiliconFlow Key 在 `cloud_lib` 仓库跑一次 `chat` 冒烟测试
- [ ] 用 `dytes test` + `deno task ffi` 联调 `cloud_lib`（不再使用 cyctron）
- [ ] 验证 `setup.json` 写入的 `info.db` 配置能被 `cloud_lib` 读取
- [ ] 在 `cloud_lib` 仓库按 dytes 流程发布 1.0.0

## 已知差异

- `cloud_lib` 的 `setup.json` 将 `silicon_app_topk` 初始化为 `40`，
  源码未配置时默认值为 `50`。
