# think_base_call

think 阶段的基础调用插件（骨架）。

- 包类型：`plugins_type: chat_think`
- 输入：单个字符串（`input`；兼容 chatllm 下发的 `prompt`）
- 输出：`data.prompts`（关键字）与 `data.outputs`（内容）
- 当前为确定性测试实现：`prompts = ["问候"]`，`outputs = ["测试" + 输入]`

## 请求与响应

```json
{ "input": "你好" }
```

```json
{
  "code": 0,
  "data": {
    "prompts": ["问候"],
    "outputs": ["测试你好"]
  },
  "msg": "success"
}
```

输入为空返回 `400102`。

## 包内容

```text
think_base_call/
├── Cargo.toml
├── dytes.json
├── mcp.json              # plugin_lib 规范；只声明 input 一个字符串入参
├── prompts.json          # 一条提示词：问候
├── taskflow.json
├── skills/
│   └── think_base_call.md
└── src/
```

- `chat_think` 包必须带 `skills/*.md`（`designplugin_lib` 发布规范）；
- `prompts.json` 每个元素含 `id` / `name` / `content`，`content` ≤ 150 字节；
- `mcp.json` 的 `tools[].inputSchema` 只有 `input` 一个字符串字段，
  `outputSchema` 含 `prompts`（关键字）与 `outputs`（内容）；
- `taskflow.json` 使用 `plugin_uuid` / `plugin_name` / `desc` / `ffi_symbol` 四个字段；
  `ffi_symbol = chat` 对应动态库导出 `post_chat`（调用方按 `post_<ffi_symbol>` 解析）。

> dytes 旧版清单校验仍把 `mcp.json` 的 `tools` 当作文件名数组，发布 workflow
> 需要传入 `--skip-check`（与 `do_test_chat` 相同）。

```bash
cargo build --release
cargo test
```
