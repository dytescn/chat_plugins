// ts/chat.ts
import { callFfi } from "./ffi.ts";

// ============ 类型定义 ============

export type Role = "system" | "user" | "assistant";

export interface Message {
  role: Role;
  content: string;
}

export interface Thinking {
  type: "enabled" | "disabled";
}

export interface ChatRequest {
  prompt?: string;
  messages?: Message[];
  system?: string;
  model?: string;
  max_tokens?: number;
  temperature?: number;
  thinking?: Thinking;
  reasoning_effort?: string;
}

export interface ChatResponse {
  content: string;
}

// ============ 调用封装 ============

/** 单轮对话 */
export async function chat(prompt: string): Promise<string> {
  if (!prompt || prompt.trim() === "") {
    throw new Error("prompt 不能为空");
  }
  const result = await callFfi<ChatResponse>("chat", { prompt });
  return result.content;
}

/** 多轮对话 */
export async function chatWithMessages(messages: Message[]): Promise<string> {
  if (!messages || messages.length === 0) {
    throw new Error("messages 不能为空");
  }
  const result = await callFfi<ChatResponse>("chat", { messages });
  return result.content;
}

/** 通用 */
export async function chatRaw(req: ChatRequest): Promise<string> {
  if (!req.prompt && (!req.messages || req.messages.length === 0)) {
    throw new Error("prompt 或 messages 至少提供一个");
  }
  const result = await callFfi<ChatResponse>("chat", req);
  return result.content;
}

