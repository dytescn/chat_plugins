#!/usr/bin/env python3
"""直接加载插件动态库并调用 FFI 接口，不依赖 cyctron。

示例：

    cd think_base_call
    cargo build --release
    python ../scripts/ffi_call.py --lib target/release/think_base_call.dll \
        --input '{"input":"你好"}'

    python scripts/ffi_call.py --lib plugintest/libary/cloud_lib.dll \
        --cwd plugintest --symbol post_chat \
        --input '{"supplier":"deepseek","prompt":"你好","max_tokens":64}'

约定：

- 入参为 UTF-8 JSON 的 NUL 结尾字符串；
- 出参缓冲区固定 64 KiB；
- 返回码 `0` 且响应 `code` 为 `0` / `200` 视为成功。
"""

import argparse
import ctypes
import json
import os
import sys

BUFFER_SIZE = 64 * 1024


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="直接调用插件动态库的 FFI 接口")
    parser.add_argument("--lib", required=True, help="动态库路径（.dll / .so / .dylib）")
    parser.add_argument("--symbol", default="post_chat", help="导出符号，默认 post_chat")
    parser.add_argument("--input", default="{}", help="请求 JSON 字符串")
    parser.add_argument("--cwd", default=None, help="调用前切换的工作目录")
    parser.add_argument("--json", action="store_true", help="只输出响应 JSON")
    return parser.parse_args()


def main() -> int:
    args = parse_args()

    lib_path = os.path.abspath(args.lib)
    if not os.path.isfile(lib_path):
        print(f"找不到动态库: {lib_path}", file=sys.stderr)
        return 2

    if args.cwd:
        os.chdir(args.cwd)

    lib = ctypes.CDLL(lib_path)
    try:
        function = getattr(lib, args.symbol)
    except AttributeError:
        print(f"动态库未导出符号: {args.symbol}", file=sys.stderr)
        return 2

    function.argtypes = [ctypes.c_char_p, ctypes.c_char_p]
    function.restype = ctypes.c_int

    payload = args.input.encode("utf-8")
    output = ctypes.create_string_buffer(BUFFER_SIZE)
    ret = function(payload, output)
    text = output.value.decode("utf-8", errors="replace")

    if not args.json:
        print(json.dumps({"return": ret, "response": text}, ensure_ascii=False, indent=2))

    try:
        parsed = json.loads(text)
    except json.JSONDecodeError:
        return 1

    code = parsed.get("code")
    return 0 if ret == 0 and code in (0, 200) else 1


if __name__ == "__main__":
    sys.exit(main())
