# Rust / FFI 编码约定

## FFI 入口

- 使用 `#[no_mangle] pub extern "C" fn post_chat(json_in: *const c_char, json_out: *mut c_char) -> c_int`；
- 入参为 NUL 结尾的 UTF-8 JSON，出参缓冲区固定 64 KiB；
- 不允许多个 `post_chat` 导出重复；
- 入口不得 panic；错误通过 `{ code, data, msg }` 返回。

## 响应包络

```json
{ "code": 0, "data": { "content": "..." }, "msg": "success" }
```

- 成功码使用 `0`；
- 参数错误使用 `4001xx`，服务端 / 上游错误使用 `5001xx`；
- 错误时 `data` 保持对象，避免客户端解析失败；
- 输出超长返回 `500901`，不截断 JSON。

## JSON 与配置

- 请求结构体使用 `serde::Deserialize`，未知字段默认忽略；
- `prompt` 与 `messages` 至少一个，空值提前拒绝；
- 配置从 `info.db` 的 `setting` 读取；不要从环境变量读 API Key；
- 配置缺失时返回明确错误，不使用 `unwrap()`。

## 错误处理

- 库内部使用 `anyhow::Result`；
- FFI 边界统一转换成错误码 + 消息；
- 不把上游原始响应体直接暴露给客户端，保留必要摘要；
- `eprintln!` 只用于本地诊断，不输出密钥。

## 代码组织

```text
src/
├── lib.rs
├── chat/ffi.rs          # 参数解析、调用、响应
└── utils/
    ├── cjson.rs         # 响应与缓冲区
    ├── db.rs            # database.dll 的 post_db_query（读 info.db）
    ├── errcode.rs       # 错误码
    ├── setting.rs       # setting 读取与默认值
    └── <provider>.rs     # 供应商 HTTP 与协议
```

- `ffi.rs` 只做协议适配，不写复杂业务；
- 供应商请求组装放在 `<provider>.rs`；
- 纯函数（配置解析、参数组装）优先写单元测试。
