// setup.ts
// 初始化 SiliconFlow 配置：通过 info_lib 写入 setting 与 designdb 分类对象。

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
  key: "silicon_info",
  language: "default",
  title: "硅基配置",
  description: "硅基流动 API 相关配置",
};
const noteEn = {
  key: "silicon_info",
  language: "en",
  title: "Silicon Info",
  description: "SiliconFlow API settings",
};

const settings = [
  {
    key: "silicon_app_key",
    title: "应用 Key",
    value: "sk-你的siliconflow-key",
    value_type: "secret",
    category: "silicon_info",
    description: "硅基流动 API Key",
  },
  {
    key: "silicon_app_base_url",
    title: "API 地址",
    value: "https://api.siliconflow.cn/v1",
    value_type: "url",
    category: "silicon_info",
    description: "硅基流动 API 基础地址",
  },
  {
    key: "silicon_app_model",
    title: "默认模型",
    value: "Qwen/Qwen2.5-7B-Instruct",
    value_type: "string",
    category: "silicon_info",
    description: "默认调用的模型名称",
  },
  {
    key: "silicon_app_max_tokens",
    title: "最大 Tokens",
    value: "4096",
    value_type: "number",
    category: "silicon_info",
    description: "单次请求最大 token 数",
  },
  {
    key: "silicon_app_temperature",
    title: "温度",
    value: "0.7",
    value_type: "number",
    category: "silicon_info",
    description: "采样温度 [0.0 ~ 2.0]",
  },
  {
    key: "silicon_app_top_p",
    title: "TopP",
    value: "0.7",
    value_type: "number",
    category: "silicon_info",
    description: "核采样阈值 [0.0 ~ 1.0]",
  },
  {
    key: "silicon_app_topk",
    title: "TopK",
    value: "40",
    value_type: "number",
    category: "silicon_info",
    description: "采样时保留概率最高的 K 个 token",
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
console.log("SiliconFlow 配置初始化完成");
