// src/chat/call.rs
// 真实转发到已安装的供应商插件：dlopen ./libary/<plugin>.dll 后调用 post_chat。
use anyhow::{bail, Context, Result};
use libloading::Library;
use serde_json::Value;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::path::PathBuf;

#[cfg(windows)]
const DYLIB_EXT: &str = "dll";
#[cfg(target_os = "linux")]
const DYLIB_EXT: &str = "so";
#[cfg(target_os = "macos")]
const DYLIB_EXT: &str = "dylib";

const OUT_BUF_SIZE: usize = 65536;

type FfiFn = unsafe extern "C" fn(*const c_char, *mut c_char) -> c_int;

/// ai_supplier 取值 -> libary 下的动态库。
fn candidates(supplier: &str) -> Vec<PathBuf> {
    vec![
        PathBuf::from(format!("./libary/{}.{}", supplier, DYLIB_EXT)),
        PathBuf::from(format!("./libary/lib{}.{}", supplier, DYLIB_EXT)),
    ]
}

fn resolve(supplier: &str) -> Result<PathBuf> {
    candidates(supplier)
        .into_iter()
        .find(|path| path.exists())
        .with_context(|| format!("找不到供应商插件 ./libary/{0}.{1}", supplier, DYLIB_EXT))
}

/// 调用供应商插件的 `post_chat`，返回响应 `data`。
pub fn forward(supplier: &str, request: &Value) -> Result<Value> {
    let path = resolve(supplier)?;
    let lib = unsafe { Library::new(&path) }
        .with_context(|| format!("加载供应商插件 {} 失败", path.display()))?;
    let function: libloading::Symbol<'_, FfiFn> =
        unsafe { lib.get(b"post_chat").context("dlsym post_chat 失败")? };

    let req = CString::new(request.to_string()).context("请求 JSON 含 \\0")?;
    let mut out_buf = vec![0u8; OUT_BUF_SIZE];
    let ret = unsafe { function(req.as_ptr(), out_buf.as_mut_ptr() as *mut c_char) };
    if ret == -2 {
        bail!("供应商 {} 输出超过 64 KiB", supplier);
    }
    if ret != 0 {
        bail!("供应商 {} FFI 返回 {}", supplier, ret);
    }

    let out = unsafe { CStr::from_ptr(out_buf.as_ptr() as *const c_char) }
        .to_str()
        .context("供应商响应非 UTF-8")?;
    let value: Value = serde_json::from_str(out)
        .with_context(|| format!("供应商 {} 响应解析失败: {}", supplier, out))?;

    let code = value.get("code").and_then(Value::as_i64).unwrap_or(-1);
    if code != 0 && code != 200 {
        let msg = value
            .get("msg")
            .and_then(Value::as_str)
            .unwrap_or("unknown");
        bail!("供应商 {} 错误 (code={}): {}", supplier, code, msg);
    }

    Ok(value.get("data").cloned().unwrap_or(Value::Null))
}
