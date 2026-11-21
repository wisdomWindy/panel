#!/usr/bin/env python3
"""Latency baseline for the Qwen3-TTS speech service contract.

Measures the transport + service overhead against the ``--mock`` engine so the
numbers describe everything *except* model inference. Use the same script
against a real deployment (``--base-url``) to capture model latency:

    python bench.py                      # in-process mock baseline
    python bench.py --base-url http://127.0.0.1:8020
"""

from __future__ import annotations

import argparse
import json
import statistics
import sys
import threading
import time
import urllib.request
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import server  # noqa: E402

SAMPLES = (
    ("短弹幕 (10 字)", "用户A进入了直播间"),
    ("常见弹幕 (24 字)", "用户A说：主播今天这个操作真的太秀了，能不能再来一次"),
    ("长播报 (96 字)", "感谢用户A赠送的粉丝团灯牌10个，" * 1 + "这是一段用于测量长文本合成耗时的播报内容，" * 4),
)


def post_speech(base_url: str, text: str) -> float:
    payload = json.dumps({"text": text, "voice": "Serena", "language": "Chinese"}).encode("utf-8")
    request = urllib.request.Request(
        f"{base_url}/v1/audio/speech",
        data=payload,
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    started = time.perf_counter()
    with urllib.request.urlopen(request, timeout=60) as response:
        body = response.read()
    elapsed = (time.perf_counter() - started) * 1000.0
    if not body.startswith(b"RIFF"):
        raise SystemExit("service returned a non-WAV payload")
    return elapsed


def get_json(base_url: str, path: str) -> float:
    started = time.perf_counter()
    with urllib.request.urlopen(base_url + path, timeout=30) as response:
        response.read()
    return (time.perf_counter() - started) * 1000.0


def percentile(values: list[float], fraction: float) -> float:
    ordered = sorted(values)
    index = min(len(ordered) - 1, max(0, round(fraction * (len(ordered) - 1))))
    return ordered[index]


def report(label: str, values: list[float]) -> None:
    print(
        f"| {label} | {len(values)} | {statistics.mean(values):.1f} | "
        f"{percentile(values, 0.5):.1f} | {percentile(values, 0.95):.1f} | {max(values):.1f} |"
    )


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--base-url", default=None, help="measure an already running service")
    parser.add_argument("--iterations", type=int, default=20)
    args = parser.parse_args(argv)

    httpd = None
    if args.base_url:
        base_url = args.base_url.rstrip("/")
        print(f"# Qwen3-TTS 延迟基线（外部服务 {base_url}）\n")
    else:
        state = server.ServiceState(server.MockEngine())
        httpd = server.build_server("127.0.0.1", 0, state)
        base_url = f"http://127.0.0.1:{httpd.server_address[1]}"
        thread = threading.Thread(target=httpd.serve_forever, daemon=True)
        thread.start()
        print("# Qwen3-TTS 延迟基线（进程内 mock 引擎，仅含传输与服务开销）\n")

    try:
        # Warm up so the first request does not own the distribution.
        get_json(base_url, "/health")
        post_speech(base_url, "预热")

        print("| 场景 | 样本数 | 均值(ms) | p50(ms) | p95(ms) | 最大(ms) |")
        print("| --- | --- | --- | --- | --- | --- |")
        report("/health 探测", [get_json(base_url, "/health") for _ in range(args.iterations)])
        report("/v1/voices 目录", [get_json(base_url, "/v1/voices") for _ in range(args.iterations)])
        for label, text in SAMPLES:
            report(label, [post_speech(base_url, text) for _ in range(args.iterations)])
    finally:
        if httpd is not None:
            httpd.shutdown()
            httpd.server_close()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
