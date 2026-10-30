// ts/chat_error_test.ts
import { assert } from "jsr:@std/assert";
import { chat, chatRaw, chatWithMessages } from "./chat.ts";

Deno.test("error - 空 prompt", async () => {
  let threw = false;
  try {
    await chat("");
  } catch (e) {
    threw = true;
    console.log("[error] ", (e as Error).message);
  }
  assert(threw);
});

Deno.test("error - 空白 prompt", async () => {
  let threw = false;
  try {
    await chat("   ");
  } catch (e) {
    threw = true;
    console.log("[error] ", (e as Error).message);
  }
  assert(threw);
});

Deno.test("error - 空对象", async () => {
  let threw = false;
  try {
    await chatRaw({});
  } catch (e) {
    threw = true;
    console.log("[error] ", (e as Error).message);
  }
  assert(threw);
});

Deno.test("error - 空 messages", async () => {
  let threw = false;
  try {
    await chatWithMessages([]);
  } catch (e) {
    threw = true;
    console.log("[error] ", (e as Error).message);
  }
  assert(threw);
});

Deno.test("error - 不存在的模型（应报 API 错）", async () => {
  let threw = false;
  try {
    await chatRaw({
      prompt: "你好",
      model: "not-a-real-model",
    });
  } catch (e) {
    threw = true;
    console.log("[error] ", (e as Error).message.slice(0, 100));
  }
  assert(threw, "无效模型应该报错");
});


