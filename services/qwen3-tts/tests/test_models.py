#!/usr/bin/env python3
"""Contract tests for the model-download endpoints of the Qwen3-TTS service.

Run with:  python -m unittest discover -s services/qwen3-tts/tests -t services/qwen3-tts -v
"""

from __future__ import annotations

import json
import sys
import threading
import time
import unittest
import urllib.error
import urllib.request
from pathlib import Path
from typing import Any

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import server  # noqa: E402


class FakeDownloader:
    """Deterministic stand-in for the Hub downloader."""

    def __init__(
        self,
        *,
        fail_with: Exception | None = None,
        installed: set[str] | None = None,
        gate: threading.Event | None = None,
    ) -> None:
        self.fail_with = fail_with
        self.installed = set(installed or ())
        self.calls: list[str] = []
        self.gate = gate
        self.started = threading.Event()

    def total_bytes(self, model_id: str) -> int | None:
        return 1_000

    def is_installed(self, model_id: str) -> bool:
        return model_id in self.installed

    def download(self, model_id: str, on_progress: Any) -> str:
        self.calls.append(model_id)
        self.started.set()
        on_progress(50, 500, 1_000)
        if self.gate is not None:
            self.gate.wait(timeout=5)
        if self.fail_with is not None:
            raise self.fail_with
        self.installed.add(model_id)
        on_progress(100, 1_000, 1_000)
        return f"/models/{model_id}"


class ModelDownloadTest(unittest.TestCase):
    def setUp(self) -> None:
        self.downloader = FakeDownloader()
        self.state = server.ServiceState(server.MockEngine(), self.downloader)
        self.httpd = server.build_server("127.0.0.1", 0, self.state)
        self.base = f"http://127.0.0.1:{self.httpd.server_address[1]}"
        self.thread = threading.Thread(target=self.httpd.serve_forever, daemon=True)
        self.thread.start()

    def tearDown(self) -> None:
        self.httpd.shutdown()
        self.httpd.server_close()
        self.thread.join(timeout=5)

    # -- helpers ---------------------------------------------------------
    def get_models(self) -> dict:
        with urllib.request.urlopen(self.base + "/v1/models", timeout=5) as response:
            return json.loads(response.read().decode("utf-8"))

    def post_download(self, payload: dict) -> tuple[int, dict]:
        request = urllib.request.Request(
            self.base + "/v1/models/download",
            data=json.dumps(payload).encode("utf-8"),
            headers={"Content-Type": "application/json"},
            method="POST",
        )
        try:
            with urllib.request.urlopen(request, timeout=5) as response:
                return response.status, json.loads(response.read().decode("utf-8"))
        except urllib.error.HTTPError as error:
            return error.code, json.loads(error.read().decode("utf-8"))

    def wait_for(self, status: str, timeout: float = 5.0) -> dict:
        deadline = time.time() + timeout
        while time.time() < deadline:
            snapshot = self.get_models()["download"]
            if snapshot["status"] == status:
                return snapshot
            time.sleep(0.05)
        self.fail(f"download never reached {status}: {self.get_models()['download']}")

    # -- tests -----------------------------------------------------------
    def test_catalogue_lists_the_released_models_with_the_1_7b_default(self) -> None:
        body = self.get_models()
        ids = [model["model_id"] for model in body["models"]]
        self.assertIn("Qwen/Qwen3-TTS-12Hz-1.7B-CustomVoice", ids)
        self.assertIn("Qwen/Qwen3-TTS-12Hz-0.6B-CustomVoice", ids)
        default = next(model for model in body["models"] if model["is_default"])
        self.assertEqual(default["model_id"], "Qwen/Qwen3-TTS-12Hz-1.7B-CustomVoice")
        self.assertTrue(all(model["is_installed"] is False for model in body["models"]))
        self.assertEqual(body["download"]["status"], "idle")

    def test_download_defaults_to_the_1_7b_custom_voice_model(self) -> None:
        status, snapshot = self.post_download({})
        self.assertEqual(status, 202)
        self.assertEqual(snapshot["model_id"], "Qwen/Qwen3-TTS-12Hz-1.7B-CustomVoice")
        ready = self.wait_for("ready")
        self.assertEqual(ready["progress_percent"], 100)
        self.assertEqual(self.downloader.calls, ["Qwen/Qwen3-TTS-12Hz-1.7B-CustomVoice"])

    def test_download_marks_the_model_as_installed(self) -> None:
        self.post_download({})
        self.wait_for("ready")
        installed = {
            model["model_id"] for model in self.get_models()["models"] if model["is_installed"]
        }
        self.assertEqual(installed, {"Qwen/Qwen3-TTS-12Hz-1.7B-CustomVoice"})

    def test_unknown_model_is_rejected(self) -> None:
        status, body = self.post_download({"model": "Qwen/not-a-model"})
        self.assertEqual(status, 400)
        self.assertEqual(body["error"]["code"], "tts_model_unknown")
        self.assertFalse(body["error"]["retryable"])
        self.assertEqual(self.downloader.calls, [])

    def test_a_second_download_of_the_same_model_is_idempotent(self) -> None:
        self.downloader.gate = threading.Event()
        first = self.post_download({})
        self.assertEqual(first[0], 202)
        self.assertTrue(self.downloader.started.wait(timeout=5))
        second = self.post_download({})
        self.assertEqual(second[0], 202)
        self.assertEqual(second[1]["model_id"], first[1]["model_id"])
        self.downloader.gate.set()
        self.wait_for("ready")
        self.assertEqual(len(self.downloader.calls), 1)

    def test_a_concurrent_download_of_another_model_is_rejected(self) -> None:
        self.downloader.gate = threading.Event()
        self.post_download({})
        self.assertTrue(self.downloader.started.wait(timeout=5))
        status, body = self.post_download({"model": "Qwen/Qwen3-TTS-12Hz-0.6B-CustomVoice"})
        self.assertEqual(status, 409)
        self.assertEqual(body["error"]["code"], "tts_download_busy")
        self.downloader.gate.set()
        self.wait_for("ready")
        self.assertEqual(len(self.downloader.calls), 1)

    def test_a_failed_download_is_reported_without_crashing_the_service(self) -> None:
        self.downloader.fail_with = RuntimeError("network down")
        self.post_download({})
        snapshot = self.wait_for("error")
        self.assertIn("network down", snapshot["error"])
        # The service keeps answering other requests.
        self.assertEqual(self.get_models()["download"]["status"], "error")


if __name__ == "__main__":
    unittest.main()
