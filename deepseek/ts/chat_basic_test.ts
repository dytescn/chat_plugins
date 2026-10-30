// ts/chat_basic_test.ts
import { assert } from "jsr:@std/assert";
import { chat } from "./chat.ts";

Deno.test("basic - 简单问候", async () => {
  const content = await chat("你好");
  console.log("[basic] 回复:", content);
  assert(typeof content === "string");
  assert(content.length > 0);
});

Deno.test("basic - 明确指令", async () => {
  const content = await chat("用一句话解释什么是 FFI");
  console.log("[basic] 回复:", content);
  assert(content.length > 0);
});

Deno.test("basic - 数学计算", async () => {
  const content = await chat("1+1等于几？只回答数字");
  console.log("[basic] 回复:", content);
  assert(content.includes("2"), "应包含 2");
});

Deno.test("basic - 长文本输入", async () => {
  const long = "请总结以下内容：".concat("这是一段测试文本。".repeat(50));
  const content = await chat(long);
  console.log("[basic] 回复长度:", content.length);
  assert(content.length > 0);
});

Deno.test("basic - 中文回复", async () => {
  const content = await chat("用中文说一句话");
  console.log("[basic] 回复:", content);
  assert(/[\u4e00-\u9fa5]/.test(content), "应包含中文");
});


