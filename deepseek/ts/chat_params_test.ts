// ts/chat_params_test.ts
import { assert } from "jsr:@std/assert";
import { chatRaw } from "./chat.ts";

Deno.test("params - 覆盖 model（deepseek-flash）", async () => {
  const content = await chatRaw({
    prompt: "1+1=?",
    model: "deepseek-flash",
  });
  console.log("[params-model] 回复:", content);
  assert(content.length > 0);
});

Deno.test("params - 覆盖 max_tokens（小值）", async () => {
  const content = await chatRaw({
    prompt: "你好",
    max_tokens: 16,
  });
  console.log("[params-maxtokens] 回复:", content);
  assert(content.length > 0);
});

Deno.test("params - 覆盖 temperature 低值", async () => {
  const content = await chatRaw({
    prompt: "说一句话",
    temperature: 0.1,
  });
  console.log("[params-temp-low] 回复:", content);
  assert(content.length > 0);
});

Deno.test("params - 覆盖 temperature 高值", async () => {
  const content = await chatRaw({
    prompt: "说一句话",
    temperature: 1.8,
  });
  console.log("[params-temp-high] 回复:", content);
  assert(content.length > 0);
});

Deno.test("params - thinking enabled", async () => {
  const content = await chatRaw({
    prompt: "12 的平方是多少？",
    thinking: { type: "enabled" },
  });
  console.log("[params-thinking] 回复:", content);
  assert(content.length > 0);
  assert(content.includes("144"), "应该算出 144");
});

Deno.test("params - thinking + reasoning_effort", async () => {
  const content = await chatRaw({
    prompt: "3 的平方是多少？",
    thinking: { type: "enabled" },
    reasoning_effort: "high",
  });
  console.log("[params-thinking-effort] 回复:", content);
  assert(content.length > 0);
});

Deno.test("params - 组合覆盖", async () => {
  const content = await chatRaw({
    prompt: "你好",
    model: "deepseek-flash",
    max_tokens: 100,
    temperature: 0.5,
    thinking: { type: "disabled" },
  });
  console.log("[params-combo] 回复:", content);
  assert(content.length > 0);
});

