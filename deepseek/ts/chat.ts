// ts/chat.ts
import { callFfi } from "./ffi.ts";

// ============ 类型定义 ============

/** 消息角色 */
export type Role = "system" | "user" | "assistant";

/** 单条消息 */
export interface Message {
  role: Role;
  content: string;
}

/** 思考模式 */
export interface Thinking {
  type: "enabled" | "disabled";
}

/**
 * chat 请求（与 siliconflow_t2t 统一）：
 * - prompt / messages 至少提供一个有效内容；
 * - messages 优先；messages 为空时回落 prompt；
 * - system 优先请求值，其次 setting.deepseek_app_system_prompt。
 */
export interface ChatRequest {
  /** 单轮：简单 prompt，自动拼 [system, user] */
  prompt?: string;
  /** 多轮：完整 messages；为空时回落 prompt */
  messages?: Message[];
  /** 请求级 system；未传时回落到 setting */
  system?: string;
  /** 请求级模型与采样参数；未传时回落到 setting */
  model?: string;
  max_tokens?: number;
  temperature?: number;
  thinking?: Thinking;
  /** DeepSeek 扩展：high / max */
  reasoning_effort?: string;
  /** 业务会话标识；仅回显，不发送给上游 */
  session_id?: string;
}

/** chat 响应（与 siliconflow_t2t 统一） */
export interface ChatResponse {
  content: string;
  /** 供应商返回的唯一标识；chatllm 用它写入 dialogue.session_id */
  id?: string | null;
  model?: string | null;
  stop_reason?: string | null;
  usage?: {
    input_tokens?: number;
    output_tokens?: number;
    [key: string]: unknown;
  } | null;
  session_id?: string | null;
}

// ============ 调用封装 ============

/** 单轮对话（只取 content，兼容旧调用） */
export async function chat(prompt: string): Promise<string> {
  const result = await callFfi<ChatResponse>("chat", { prompt });
  return result.content;
}

/** 多轮对话（只取 content，兼容旧调用） */
export async function chatWithMessages(
  messages: Message[],
): Promise<string> {
  const result = await callFfi<ChatResponse>("chat", { messages });
  return result.content;
}

/** 通用：传 ChatRequest，只取 content（兼容旧调用） */
export async function chatRaw(req: ChatRequest): Promise<string> {
  const result = await callFfi<ChatResponse>("chat", req);
  return result.content;
}

/** 通用：返回完整响应（含 id / usage / session_id） */
export async function chatResult(req: ChatRequest): Promise<ChatResponse> {
  return await callFfi<ChatResponse>("chat", req);
}
