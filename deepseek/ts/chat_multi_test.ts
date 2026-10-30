// ts/chat_multi_test.ts
import { assert } from "jsr:@std/assert";
import { chatWithMessages, type Message } from "./chat.ts";

Deno.test("multi - system + user", async () => {
  const messages: Message[] = [
    { role: "system", content: "你是一个翻译助手。" },
    { role: "user", content: "把'你好'翻译成英文" },
  ];
  const content = await chatWithMessages(messages);
  console.log("[multi] 回复:", content);
  assert(content.length > 0);
});

Deno.test("multi - 带 assistant 历史", async () => {
  const messages: Message[] = [
    { role: "system", content: "你是一个翻译助手。" },
    { role: "user", content: "把'你好'翻译成英文" },
    { role: "assistant", content: "Hello" },
    { role: "user", content: "再翻译'谢谢'" },
  ];
  const content = await chatWithMessages(messages);
  console.log("[multi-history] 回复:", content);
  assert(content.length > 0);
});

Deno.test("multi - 连续对话", async () => {
  const messages: Message[] = [
    { role: "user", content: "记住一个数字：42" },
    { role: "assistant", content: "好的，我记住了。" },
    { role: "user", content: "刚才说的数字是多少？" },
  ];
  const content = await chatWithMessages(messages);
  console.log("[multi-memory] 回复:", content);
  assert(content.includes("42"), "应该记住 42");
});

Deno.test("multi - 只有 user 无 system", async () => {
  const messages: Message[] = [
    { role: "user", content: "你好" },
  ];
  const content = await chatWithMessages(messages);
  console.log("[multi-no-system] 回复:", content);
  assert(content.length > 0);
});

Deno.test("multi - 多轮翻译链", async () => {
  const messages: Message[] = [
    { role: "user", content: "把'苹果'翻译成英文" },
    { role: "assistant", content: "Apple" },
    { role: "user", content: "把'香蕉'翻译成英文" },
    { role: "assistant", content: "Banana" },
    { role: "user", content: "把'橘子'翻译成英文" },
  ];
  const content = await chatWithMessages(messages);
  console.log("[multi-chain] 回复:", content);
  assert(content.length > 0);
});


