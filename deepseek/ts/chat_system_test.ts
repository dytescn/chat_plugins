// ts/chat_system_test.ts
import { assert } from "jsr:@std/assert";
import { chatRaw } from "./chat.ts";

Deno.test("system - 只说英文", async () => {
  const content = await chatRaw({
    prompt: "你好",
    system: "You must reply in English only.",
  });
  console.log("[system-en] 回复:", content);
  assert(content.length > 0);
  assert(/[a-zA-Z]/.test(content), "应包含英文");
});

Deno.test("system - 只说中文", async () => {
  const content = await chatRaw({
    prompt: "Hello",
    system: "你必须用中文回答。",
  });
  console.log("[system-cn] 回复:", content);
  assert(content.length > 0);
  assert(/[\u4e00-\u9fa5]/.test(content), "应包含中文");
});

Deno.test("system - 角色扮演", async () => {
  const content = await chatRaw({
    prompt: "你是谁？",
    system: "你是一只名叫小白的猫，只用猫的语气回答。",
  });
  console.log("[system-role] 回复:", content);
  assert(content.length > 0);
});

Deno.test("system - 简短回答约束", async () => {
  const content = await chatRaw({
    prompt: "介绍一下人工智能",
    system: "你只能用不超过 10 个字回答。",
  });
  console.log("[system-short] 回复:", content);
  assert(content.length > 0);
  assert(content.length < 50, "应该比较短");
});

Deno.test("system - 空字符串跳过", async () => {
  const content = await chatRaw({
    prompt: "你好",
    system: "",
  });
  console.log("[system-empty] 回复:", content);
  assert(content.length > 0);
});


