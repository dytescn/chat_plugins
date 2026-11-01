# TODO

## 已完成

- [x] 将 `deepseek`、`siliconflow_t2t` 合并到 `chat_plugins`
- [x] 删除嵌套 `.git`，统一由 `chat_plugins` 管理
- [x] 整理根 README 与两个插件 README
- [x] 纳入 `deepseek/Cargo.lock`

## 待办

- [ ] 用真实 DeepSeek Key 跑一次 `chat` 冒烟测试
- [ ] 用真实 SiliconFlow Key 跑一次 `chat` 冒烟测试
- [ ] 在 `plugintest/` 中用 cyctron 联调两个插件
- [ ] 验证 `setup.ts` 写入的 `info.db` 配置能被插件读取
- [ ] 为两个插件补充发布前检查清单
- [ ] 按 dytes 打包/发布流程发布两个 sys 包

## 已知差异

- DeepSeek 的 `setup.ts` 会初始化 `deepseek_app_system_prompt`，
  但当前 `post_chat` 只使用请求级 `system`。
- SiliconFlow 的 `setup.ts` 将 `silicon_app_topk` 初始化为 `40`，
  源码未配置时默认值为 `50`。
