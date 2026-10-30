// ts/ffi.ts
const BASE_URL = "http://127.0.0.1:44944/libdeepseek";

/**
 * 调用 FFI 符号（POST + JSON body）。
 *
 * symbol 传原始函数名（如 "chat"），桥接层会自动拼 "post_" 前缀。
 */
export async function callFfi<T>(
  symbol: string,
  payload: unknown,
  options?: { timeoutMs?: number },
): Promise<T> {
  const timeoutMs = options?.timeoutMs ?? 120_000;

  const response = await fetch(BASE_URL, {
    method: "POST",
    headers: {
      "Content-Type": "application/libary",
      "FFI-Symbol": symbol,
    },
    body: JSON.stringify(payload),
    signal: AbortSignal.timeout(timeoutMs),
  });

  if (!response.ok) {
    throw new Error(`HTTP 错误: ${response.status} - ${response.statusText}`);
  }

  const data = await response.json();
  if (data.code === 0 || data.code === 200) return data.data as T;
  throw new Error(`err info: ${data.msg} (code=${data.code})`);
}
