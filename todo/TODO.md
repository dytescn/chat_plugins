# TODO

## 已完成

- [x] 将 `deepseek`、`siliconflow_t2t` 从 `chat_plugins` 迁出
- [x] 供应商调用合并到独立系统库 `dytescn/cloud`
- [x] 删除嵌套 `.git`，统一由 `chat_plugins` 管理
- [x] 整理根 README 与插件 README

## 待办

- [ ] 用真实 DeepSeek Key 在 `cloud` 仓库跑一次 `chat` 冒烟测试
- [ ] 用真实 SiliconFlow Key 在 `cloud` 仓库跑一次 `chat` 冒烟测试
- [ ] 在 `plugintest/` 中用 cyctron 联调 `libcloud`
- [ ] 验证 `setup.json` 写入的 `info.db` 配置能被 `cloud` 读取
- [ ] 在 `cloud` 仓库按 dytes 流程发布 1.0.0

## 已知差异

- `cloud` 的 `setup.json` 将 `silicon_app_topk` 初始化为 `40`，
  源码未配置时默认值为 `50`。
