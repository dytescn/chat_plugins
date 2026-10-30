// setup.ts
// 初始化 DeepSeek 配置：通过 info_lib 写入 setting 与 designdb 分类对象。

const INFO_URL = "http://127.0.0.1:44944/info_lib";
type InfoError = Error & { code?: number };

async function infoCall<T = unknown>(
  symbol: string,
  body: Record<string, unknown>,
): Promise<T> {
  const response = await fetch(INFO_URL, {
    method: "POST",
    headers: {
      "Content-Type": "application/libary",
      "FFI-Symbol": symbol,
    },
    body: JSON.stringify(body),
  });
  const payload = await response.json();
  if (!response.ok || payload.code !== 200) {
    const error = new Error(
      payload?.msg ?? `info_lib ${symbol} failed: ${response.status}`,
    ) as InfoError;
    error.code = payload?.code ?? response.status;
    throw error;
  }
  return payload.data as T;
}

const noteDefault = {
  key: "deepseek_info",
  language: "default",
  title: "DeepSeek 配置",
  description: "DeepSeek API 相关配置",
};
const noteEn = {
  key: "deepseek_info",
  language: "en",
  title: "DeepSeek Info",
  description: "DeepSeek API settings",
};

const settings = [
  {
    key: "deepseek_app_key",
    title: "应用 Key",
    value: "sk-你的deepseek-key",
    value_type: "secret",
    category: "deepseek_info",
    description: "DeepSeek API Key",
  },
  {
    key: "deepseek_app_base_url",
    title: "API 地址",
    value: "https://api.deepseek.com/anthropic",
    value_type: "url",
    category: "deepseek_info",
    description: "DeepSeek Anthropic 兼容端点",
  },
  {
    key: "deepseek_app_model",
    title: "默认模型",
    value: "deepseek-flash",
    value_type: "string",
    category: "deepseek_info",
    description: "deepseek-flash / deepseek-v4-pro",
  },
  {
    key: "deepseek_app_max_tokens",
    title: "最大 Tokens",
    value: "4096",
    value_type: "number",
    category: "deepseek_info",
    description: "单次请求最大 token 数",
  },
  {
    key: "deepseek_app_temperature",
    title: "温度",
    value: "0.7",
    value_type: "number",
    category: "deepseek_info",
    description: "采样温度 [0.0 ~ 2.0]",
  },
  {
    key: "deepseek_app_system_prompt",
    title: "System 提示词",
    value: "你是一个乐于助人的中文 AI 助手。",
    value_type: "string",
    category: "deepseek_info",
    description: "顶层 system 字段",
  },
  {
    key: "deepseek_app_thinking",
    title: "思考模式",
    value: "disabled",
    value_type: "enum",
    category: "deepseek_info",
    description: "enabled / disabled",
  },
  {
    key: "deepseek_app_reasoning_effort",
    title: "推理强度",
    value: "high",
    value_type: "enum",
    category: "deepseek_info",
    description: "high / max",
  },
];

for (const item of settings) {
  try {
    await infoCall("info_get", { key: item.key });
    await infoCall("info_update", { key: item.key, note: noteDefault });
  } catch (error) {
    if ((error as InfoError).code !== 404100) throw error;
    await infoCall("info_post", { ...item, note: noteDefault });
  }
}
await infoCall("info_update", { key: settings[0].key, note: noteEn });
console.log("DeepSeek 配置初始化完成");
