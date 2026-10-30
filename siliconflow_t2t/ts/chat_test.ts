// ts/chat_test.ts
import { assertEquals, assert } from "jsr:@std/assert";
import { chat, chatWithMessages, chatRaw, type Message } from "./chat.ts";

// ============ 单轮 ============

Deno.test("chat - 单轮简单对话", async () => {
  const content = await chat("你好");
  console.log("[单轮] 回复:", content);
  assert(typeof content === "string");
  assert(content.length > 0, "回复不能为空");
});

Deno.test("chat - 单轮带明确指令", async () => {
  const content = await chat("用一句话解释什么是 FFI");
  console.log("[指令] 回复:", content);
  assert(content.length > 0);
});

// ============ 多轮 ============

Deno.test("chat - 多轮对话（system + user）", async () => {
  const messages: Message[] = [
    { role: "system", content: "你是一个只说英文的助手。" },
    { role: "user", content: "你好" },
  ];
  const content = await chatWithMessages(messages);
  console.log("[多轮] 回复:", content);
  assert(content.length > 0);
});

Deno.test("chat - 多轮对话（含 assistant 历史）", async () => {
  const messages: Message[] = [
    { role: "system", content: "你是一个翻译助手。" },
    { role: "user", content: "把'你好'翻译成英文" },
    { role: "assistant", content: "Hello" },
    { role: "user", content: "再翻译'谢谢'" },
  ];
  const content = await chatWithMessages(messages);
  console.log("[带历史] 回复:", content);
  assert(content.length > 0);
});

// ============ 通用 chatRaw ============

Deno.test("chatRaw - 走 prompt 字段", async () => {
  const content = await chatRaw({ prompt: "1+1等于几？" });
  console.log("[prompt] 回复:", content);
  assert(content.length > 0);
});

Deno.test("chatRaw - 走 messages 字段", async () => {
  const content = await chatRaw({
    messages: [
      { role: "system", content: "你是一个简洁的助手，只回答数字。" },
      { role: "user", content: "2+2等于几？" },
    ],
  });
  console.log("[messages] 回复:", content);
  assert(content.length > 0);
});

// ============ 错误处理 ============

Deno.test("chat - 空 prompt 应报错", async () => {
  let threw = false;
  try {
    await chat("");
  } catch (e) {
    threw = true;
    console.log("[预期错误]", (e as Error).message);
  }
  assert(threw, "空 prompt 应该报错");
});

Deno.test("chatRaw - 空对象应报错", async () => {
  let threw = false;
  try {
    await chatRaw({});
  } catch (e) {
    threw = true;
    console.log("[预期错误]", (e as Error).message);
  }
  assert(threw, "空请求应该报错");
});

