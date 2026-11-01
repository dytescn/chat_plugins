// src/chat/reply.rs
//! 系统错误码 / AI 配置状态 → 面向用户的语义提示。

/// 把系统错误码转换成语义提示。
///
/// 兼容上游简写：`40013` 视为 `400103`，`4005` 视为 `400500`。
pub fn describe_error(code: i32, detail: &str) -> String {
    let detail = detail.trim();
    match code {
        40013 | 400103 => append_detail("对话标识无效，请重新打开会话。", detail),
        400300 => append_detail("创建对话失败，请稍后重试。", detail),
        400301 => append_detail("消息保存失败，请稍后重试。", detail),
        400303 => append_detail("读取对话失败，请重新打开会话。", detail),
        400305 => append_detail("更新会话信息失败，请稍后重试。", detail),
        4005 | 400500 => append_detail("该功能尚未实现，请稍后再试。", detail),
        500101 => append_detail("调用 AI 供应商失败，请检查网络或 token 配置。", detail),
        500102 | 500104 => {
            append_detail("AI 供应商未配置完成：token未绑定，请填写token信息。", detail)
        }
        _ => {
            if detail.is_empty() {
                "系统提示：操作未完成，请稍后重试。".to_string()
            } else {
                format!("系统提示：{detail}")
            }
        }
    }
}

fn append_detail(message: &str, detail: &str) -> String {
    if detail.is_empty() {
        message.to_string()
    } else {
        format!("{message}（{detail}）")
    }
}

/// AI 供应商选择提示。
pub fn supplier_hint(selected: Option<&str>, token: Option<&str>) -> String {
    let selected = selected.map(str::trim).filter(|value| !value.is_empty());
    let token = token.map(str::trim).filter(|value| !value.is_empty());

    match (selected, token) {
        (None, _) => "请选择 AI 供应商：deepseek / siliconflow_t2t".to_string(),
        (Some(supplier), None) => {
            format!("已选择 {supplier}，token未绑定，请填写token信息。")
        }
        (Some(supplier), Some(_)) => format!("已选择 {supplier}，可以开始对话。"),
    }
}
