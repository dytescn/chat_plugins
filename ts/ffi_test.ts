#!/usr/bin/env -S deno run --allow-ffi --allow-read
/**
 * 直接加载插件动态库并调用 FFI 接口（Deno FFI）。
 *
 * 用法：
 *   deno task ffi --lib think_base_call/target/release/think_base_call.dll \
 *     --input '{"input":"你好"}'
 *
 * 约定：
 * - 入参为 UTF-8 JSON（自动补 NUL）；
 * - 出参缓冲区默认 64 KiB；
 * - FFI 返回 0 且响应 code 为 0 / 200 时退出码为 0。
 */

interface Options {
  lib: string;
  symbol: string;
  input: string;
  cwd?: string;
  buffer: number;
  json: boolean;
}

function parseArgs(args: string[]): Options {
  const options: Options = {
    lib: "",
    symbol: "post_chat",
    input: "{}",
    buffer: 64 * 1024,
    json: false,
  };

  for (let i = 0; i < args.length; i += 1) {
    const arg = args[i];
    switch (arg) {
      case "--lib":
        options.lib = args[++i] ?? "";
        break;
      case "--symbol":
        options.symbol = args[++i] ?? "post_chat";
        break;
      case "--input":
        options.input = args[++i] ?? "{}";
        break;
      case "--cwd":
        options.cwd = args[++i];
        break;
      case "--buffer":
        options.buffer = Number(args[++i] ?? options.buffer);
        break;
      case "--json":
        options.json = true;
        break;
      case "-h":
      case "--help":
        printUsage();
        Deno.exit(0);
      default:
        throw new Error(`未知参数: ${arg}`);
    }
  }

  if (!options.lib) {
    throw new Error("缺少 --lib");
  }
  if (!Number.isInteger(options.buffer) || options.buffer <= 0) {
    throw new Error("--buffer 必须是正整数");
  }
  return options;
}

function printUsage(): void {
  console.log(
    `用法: deno run --allow-ffi --allow-read ts/ffi_test.ts --lib <动态库> [选项]

选项:
  --lib <path>       动态库路径（.dll / .so / .dylib），必填
  --symbol <name>    导出符号，默认 post_chat
  --input <json>     请求 JSON，默认 {}
  --cwd <path>       调用前切换的工作目录
  --buffer <bytes>   输出缓冲区大小，默认 65536
  --json             只输出响应 JSON`,
  );
}

function main(): number {
  const options = parseArgs(Deno.args);

  if (options.cwd) {
    Deno.chdir(options.cwd);
  }

  const libPath = Deno.realPathSync(options.lib);
  const lib = Deno.dlopen(libPath, {
    [options.symbol]: { parameters: ["buffer", "buffer"], result: "i32" },
  });

  try {
    const output = new Uint8Array(options.buffer);
    const input = new TextEncoder().encode(`${options.input}\0`);
    const ret = lib.symbols[options.symbol](input, output);

    const end = output.indexOf(0);
    const text = new TextDecoder().decode(
      output.subarray(0, end === -1 ? output.length : end),
    );

    if (!options.json) {
      console.log(JSON.stringify({ return: ret, response: text }, null, 2));
    } else {
      console.log(text);
    }

    try {
      const parsed = JSON.parse(text);
      return ret === 0 && (parsed.code === 0 || parsed.code === 200) ? 0 : 1;
    } catch {
      return 1;
    }
  } finally {
    lib.close();
  }
}

try {
  Deno.exit(main());
} catch (error) {
  console.error(error instanceof Error ? error.message : String(error));
  Deno.exit(2);
}
