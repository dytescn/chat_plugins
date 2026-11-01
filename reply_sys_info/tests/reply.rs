// tests/reply.rs —— chat/reply.rs 的错误码与供应商提示测试
use reply_sys_info::chat::reply::{describe_error, supplier_hint};

#[test]
fn describes_error_codes() {
    // 兼容简写 40013 → 400103
    assert!(describe_error(40013, "").contains("对话标识无效"));
    assert!(describe_error(400103, "").contains("对话标识无效"));
    // 兼容简写 4005 → 400500
    assert!(describe_error(4005, "").contains("尚未实现"));
    assert!(describe_error(400300, "no such table").contains("创建对话失败"));
    // AI 供应商 token 未绑定
    let message = describe_error(500102, "");
    assert!(message.contains("token未绑定"));
    assert!(message.contains("请填写token信息"));
}

#[test]
fn hints_supplier_selection() {
    assert_eq!(
        supplier_hint(None, None),
        "请选择 AI 供应商：deepseek / siliconflow_t2t"
    );
    assert_eq!(
        supplier_hint(Some("deepseek"), None),
        "已选择 deepseek，token未绑定，请填写token信息。"
    );
    assert_eq!(
        supplier_hint(Some("deepseek"), Some("sk-test")),
        "已选择 deepseek，可以开始对话。"
    );
}
