#!/usr/bin/env python3
"""Contract tests for the Qwen3-TTS speech service (stdlib only).

Run with:  python -m unittest discover -s services/qwen3-tts/tests -v
"""

from __future__ import annotations

import io
import json
import sys
import threading
import unittest
import urllib.error
import urllib.request
import wave
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import server  # noqa: E402


class ServiceContractTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.state = server.ServiceState(server.MockEngine())
        cls.httpd = server.build_server("127.0.0.1", 0, cls.state)
        cls.base = f"http://127.0.0.1:{cls.httpd.server_address[1]}"
        cls.thread = threading.Thread(target=cls.httpd.serve_forever, daemon=True)
        cls.thread.start()

    @classmethod
    def tearDownClass(cls) -> None:
        cls.httpd.shutdown()
        cls.httpd.server_close()
        cls.thread.join(timeout=5)

    # -- helpers ---------------------------------------------------------
    def get(self, path: str) -> tuple[int, dict]:
        with urllib.request.urlopen(self.base + path, timeout=5) as response:
            return response.status, json.loads(response.read().decode("utf-8"))

    def post(self, payload: dict) -> tuple[int, bytes, str]:
        data = json.dumps(payload).encode("utf-8")
        request = urllib.request.Request(
            self.base + "/v1/audio/speech",
            data=data,
            headers={"Content-Type": "application/json"},
            method="POST",
        )
        try:
            with urllib.request.urlopen(request, timeout=10) as response:
                return response.status, response.read(), response.headers.get("Content-Type", "")
        except urllib.error.HTTPError as error:
            return error.code, error.read(), error.headers.get("Content-Type", "")

    # -- tests -----------------------------------------------------------
    def test_health_reports_ready_engine_and_voices(self) -> None:
        status, body = self.get("/health")
        self.assertEqual(status, 200)
        self.assertEqual(body["status"], "ok")
        self.assertTrue(body["ready"])
        self.assertEqual(body["engine"], "mock")
        self.assertEqual(body["sample_rate"], 24000)
        self.assertGreaterEqual(len(body["voices"]), 9)
        self.assertIn("Chinese", body["languages"])

    def test_voices_lists_custom_voice_speakers(self) -> None:
        status, body = self.get("/v1/voices")
        self.assertEqual(status, 200)
        ids = {voice["voice_id"] for voice in body["voices"]}
        self.assertIn("Serena", ids)
        self.assertIn("Vivian", ids)

    def test_speech_returns_playable_wav(self) -> None:
        status, body, content_type = self.post({"text": "用户A说：今天直播真不错", "voice": "Serena"})
        self.assertEqual(status, 200)
        self.assertEqual(content_type, "audio/wav")
        self.assertEqual(body[:4], b"RIFF")
        with wave.open(io.BytesIO(body), "rb") as handle:
            self.assertEqual(handle.getframerate(), 24000)
            self.assertEqual(handle.getnchannels(), 1)
            self.assertEqual(handle.getsampwidth(), 2)
            self.assertGreater(handle.getnframes(), 1000)

    def test_empty_text_is_rejected(self) -> None:
        status, body, _ = self.post({"text": "   "})
        self.assertEqual(status, 400)
        error = json.loads(body.decode("utf-8"))["error"]
        self.assertEqual(error["code"], "tts_invalid_request")
        self.assertFalse(error["retryable"])

    def test_oversized_text_is_rejected(self) -> None:
        status, body, _ = self.post({"text": "字" * (server.MAX_TEXT_CHARS + 1)})
        self.assertEqual(status, 413)
        self.assertEqual(json.loads(body.decode("utf-8"))["error"]["code"], "tts_invalid_request")

    def test_unsupported_language_is_rejected(self) -> None:
        status, body, _ = self.post({"text": "hi", "language": "Klingon"})
        self.assertEqual(status, 400)
        self.assertEqual(json.loads(body.decode("utf-8"))["error"]["code"], "tts_unsupported_language")

    def test_unsupported_format_is_rejected(self) -> None:
        status, body, _ = self.post({"text": "hi", "response_format": "mp3"})
        self.assertEqual(status, 400)
        self.assertEqual(json.loads(body.decode("utf-8"))["error"]["code"], "tts_unsupported_format")

    def test_unknown_path_is_404(self) -> None:
        with self.assertRaises(urllib.error.HTTPError) as context:
            urllib.request.urlopen(self.base + "/nope", timeout=5)
        self.assertEqual(context.exception.code, 404)

    def test_requests_are_counted(self) -> None:
        before = self.get("/health")[1]["requests"]
        self.post({"text": "count me"})
        after = self.get("/health")[1]["requests"]
        self.assertEqual(after, before + 1)

    def test_loading_state_returns_retryable_503(self) -> None:
        # A non-mock engine starts un-ready and must fail closed but retryably.
        state = server.ServiceState(server.MockEngine())
        state.ready = False
        state.status = "loading"
        httpd = server.build_server("127.0.0.1", 0, state)
        thread = threading.Thread(target=httpd.serve_forever, daemon=True)
        thread.start()
        try:
            request = urllib.request.Request(
                f"http://127.0.0.1:{httpd.server_address[1]}/v1/audio/speech",
                data=json.dumps({"text": "hi"}).encode("utf-8"),
                headers={"Content-Type": "application/json"},
                method="POST",
            )
            with self.assertRaises(urllib.error.HTTPError) as context:
                urllib.request.urlopen(request, timeout=5)
            self.assertEqual(context.exception.code, 503)
            payload = json.loads(context.exception.read().decode("utf-8"))
            self.assertEqual(payload["error"]["code"], "tts_model_loading")
            self.assertTrue(payload["error"]["retryable"])
        finally:
            httpd.shutdown()
            httpd.server_close()
            thread.join(timeout=5)


if __name__ == "__main__":
    unittest.main()
