#!/usr/bin/env python3
"""Qwen3-TTS speech service for biliLiveHelper.

The service exposes a small, stable HTTP contract that the desktop client
(Rust/Tauri) talks to. It is intentionally dependency-light: the HTTP layer
uses only the Python standard library, and the heavyweight model runtime
(``qwen-tts`` + ``torch``) is imported lazily so that the process can start,
report health, and serve the contract even on machines without a GPU.

Contract (frozen):
    GET  /health
        -> 200 {"status":"ok"|"loading"|"degraded","ready":bool,
                "engine":"qwen3-tts"|"mock","model":str,"sample_rate":int,
                "languages":[str],"voices":[{voice_id,display_name,language}],
                "error":str|null}
    GET  /v1/voices
        -> 200 {"voices":[{voice_id,display_name,language}],"languages":[str]}
    POST /v1/audio/speech
        body {"text":str,"voice":str|null,"language":str|null,
              "instruct":str|null,"response_format":"wav"}
        -> 200 audio/wav
        -> 4xx/5xx {"error":{"code":str,"message":str,"retryable":bool}}

Reference: https://github.com/QwenLM/Qwen3-TTS#model-architecture
"""

from __future__ import annotations

import argparse
import io
import json
import math
import os
import struct
import sys
import tempfile
import threading
import time
import traceback
import wave
from http import HTTPStatus

# Fail fast when the Hub is blocked instead of retrying for minutes.
os.environ.setdefault("HF_HUB_ETAG_TIMEOUT", "8")
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from typing import Any, Protocol

DEFAULT_HOST = "127.0.0.1"
DEFAULT_PORT = 8020

# Qwen3-TTS-12Hz sample rate is reported by the model at runtime; the mock
# engine uses the same 24 kHz rate so client-side assumptions stay identical.
MOCK_SAMPLE_RATE = 24_000

# Speakers shipped with Qwen3-TTS-12Hz-*-CustomVoice, per the upstream README
# ("Custom Voice Generate" speaker table).
CUSTOM_VOICE_SPEAKERS: tuple[tuple[str, str, str], ...] = (
    ("Vivian", "Vivian", "Chinese"),
    ("Serena", "Serena", "Chinese"),
    ("Uncle_Fu", "Uncle Fu", "Chinese"),
    ("Dylan", "Dylan", "Chinese"),
    ("Eric", "Eric", "Chinese"),
    ("Ryan", "Ryan", "English"),
    ("Aiden", "Aiden", "English"),
    ("Ono_Anna", "Ono Anna", "Japanese"),
    ("Sohee", "Sohee", "Korean"),
)

# Languages supported by every released Qwen3-TTS model, per the upstream
# "Released Models Description and Download" table, plus Auto detection.
SUPPORTED_LANGUAGES: tuple[str, ...] = (
    "Auto",
    "Chinese",
    "English",
    "Japanese",
    "Korean",
    "German",
    "French",
    "Russian",
    "Portuguese",
    "Spanish",
    "Italian",
)

MAX_TEXT_CHARS = 2_000

# The released Qwen3-TTS-12Hz checkpoints, per the upstream model table. The
# desktop app offers a one-click download of the default (1.7B CustomVoice).
DEFAULT_MODEL_ID = "Qwen/Qwen3-TTS-12Hz-1.7B-CustomVoice"
MODEL_CATALOG: tuple[dict[str, Any], ...] = (
    {
        "model_id": "Qwen/Qwen3-TTS-12Hz-1.7B-CustomVoice",
        "display_name": "Qwen3-TTS 1.7B CustomVoice",
        "task": "custom_voice",
        "is_default": True,
    },
    {
        "model_id": "Qwen/Qwen3-TTS-12Hz-1.7B-VoiceDesign",
        "display_name": "Qwen3-TTS 1.7B VoiceDesign",
        "task": "voice_design",
        "is_default": False,
    },
    {
        "model_id": "Qwen/Qwen3-TTS-12Hz-1.7B-Base",
        "display_name": "Qwen3-TTS 1.7B Base",
        "task": "voice_clone",
        "is_default": False,
    },
    {
        "model_id": "Qwen/Qwen3-TTS-12Hz-0.6B-CustomVoice",
        "display_name": "Qwen3-TTS 0.6B CustomVoice",
        "task": "custom_voice",
        "is_default": False,
    },
    {
        "model_id": "Qwen/Qwen3-TTS-12Hz-0.6B-Base",
        "display_name": "Qwen3-TTS 0.6B Base",
        "task": "voice_clone",
        "is_default": False,
    },
)



_HUB_NETWORK_MARKERS = (
    "couldn't connect",
    "connection",
    "connectionerror",
    "max retries",
    "timed out",
    "timeout",
    "offline",
    "huggingface.co",
    "name or service not known",
    "temporary failure in name resolution",
    "ssl",
)


def _is_hub_network_error(exc: BaseException) -> bool:
    """True when a load failure looks like "the Hub is unreachable"."""
    text = f"{type(exc).__name__}: {exc}".lower()
    return any(marker in text for marker in _HUB_NETWORK_MARKERS)


def _host_reachable(host: str, port: int = 443, timeout: float = 2.0) -> bool:
    """Fast reachability probe.

    HuggingFace retries each file five times before failing, which costs minutes
    on a blocked network; probing first keeps the fallback decision quick.
    """
    import socket  # noqa: PLC0415 - only needed here

    try:
        with socket.create_connection((host, port), timeout=timeout):
            return True
    except OSError:
        return False


def _prefer_modelscope() -> bool:
    """True when the mirror should be used without trying HuggingFace first.

    ModelScope is the checkpoint publisher's own mirror and stays reachable from
    mainland networks, while a TCP probe of huggingface.co is not trustworthy
    there (the connection often succeeds and the file download still fails).
    HuggingFace is therefore opt-in via ``QWEN3_TTS_HUB=hf``.
    """
    hub = os.environ.get("QWEN3_TTS_HUB", "modelscope").strip().lower()
    return hub != "hf"


def _modelscope_cache_root() -> Path:
    override = os.environ.get("QWEN3_TTS_MODELSCOPE_CACHE")
    if override:
        return Path(override)
    return Path(os.environ.get("LOCALAPPDATA", Path.home())) / "biliLiveHelper" / "qwen3-tts-models"


class SynthesisError(Exception):
    """A synthesis failure with a stable machine-readable code."""

    def __init__(self, code: str, message: str, status: int = 500, retryable: bool = True) -> None:
        super().__init__(message)
        self.code = code
        self.message = message
        self.status = status
        self.retryable = retryable


class Engine(Protocol):
    name: str
    model: str
    sample_rate: int

    def voices(self) -> list[dict[str, str]]: ...

    def synthesize(self, text: str, voice: str | None, language: str | None, instruct: str | None) -> bytes: ...


def encode_wav_pcm16(frames: bytes, sample_rate: int, channels: int = 1) -> bytes:
    """Wrap raw little-endian PCM16 frames in a RIFF/WAVE container."""
    buffer = io.BytesIO()
    with wave.open(buffer, "wb") as handle:
        handle.setnchannels(channels)
        handle.setsampwidth(2)
        handle.setframerate(sample_rate)
        handle.writeframes(frames)
    return buffer.getvalue()


def tone_frames(text: str, sample_rate: int) -> bytes:
    """Deterministic filler audio for ``--mock``: a short, quiet rising tone.

    Duration scales with the input length so latency measurements exercise a
    realistic playback path without needing model weights.
    """
    duration = min(1.6, 0.35 + len(text) / 40.0)
    total = int(duration * sample_rate)
    amplitude = 0.08
    base = 320.0
    frames = bytearray()
    for index in range(total):
        t = index / sample_rate
        envelope = min(1.0, t / 0.03) * min(1.0, max(0.0, (duration - t) / 0.05))
        value = amplitude * envelope * math.sin(2.0 * math.pi * base * (1.0 + 0.35 * t) * t)
        frames += struct.pack("<h", int(max(-1.0, min(1.0, value)) * 32767))
    return bytes(frames)


class MockEngine:
    """Contract-compatible engine that needs no model runtime (CI / smoke tests)."""

    name = "mock"
    model = "mock-qwen3-tts"
    sample_rate = MOCK_SAMPLE_RATE

    def voices(self) -> list[dict[str, str]]:
        return [
            {"voice_id": voice_id, "display_name": display, "language": language}
            for voice_id, display, language in CUSTOM_VOICE_SPEAKERS
        ]

    def synthesize(self, text: str, voice: str | None, language: str | None, instruct: str | None) -> bytes:
        return encode_wav_pcm16(tone_frames(text, self.sample_rate), self.sample_rate)


class Qwen3TtsEngine:
    """Thin wrapper over the upstream ``qwen-tts`` package.

    Supported tasks:
      * ``custom_voice``  -> Qwen3-TTS-12Hz-*-CustomVoice (fixed speaker set)
      * ``voice_design``  -> Qwen3-TTS-12Hz-1.7B-VoiceDesign (natural-language timbre)
    """

    def __init__(self, model: str, task: str, device: str, dtype: str, attn: str) -> None:
        self.model = model
        self.task = task
        self._device = device
        self._dtype = dtype
        self._attn = attn
        self._model: Any = None
        self._lock = threading.Lock()
        self._sample_rate = MOCK_SAMPLE_RATE
        self.name = "qwen3-tts"

    @property
    def sample_rate(self) -> int:
        return self._sample_rate

    def load(self) -> None:
        import torch  # noqa: PLC0415 - optional heavy dependency
        from qwen_tts import Qwen3TTSModel  # noqa: PLC0415

        # Respect the machine instead of the flags: CPU-only boxes must not ask for
        # cuda, and flash-attn is an optional compile-from-source extra.
        device = self._device
        if device.startswith("cuda") and not torch.cuda.is_available():
            print(f"cuda is unavailable; loading {self.model} on cpu", file=sys.stderr, flush=True)
            device = "cpu"
        dtype = {"bfloat16": torch.bfloat16, "float16": torch.float16, "float32": torch.float32}[self._dtype]
        if device == "cpu" and self._dtype in ("bfloat16", "float16"):
            dtype = torch.float32
        attn = self._attn
        if attn == "flash_attention_2":
            try:
                import flash_attn  # noqa: F401,PLC0415 - optional speed-up
            except ImportError:
                print("flash_attn is not installed; using sdpa attention", file=sys.stderr, flush=True)
                attn = "sdpa"
        kwargs: dict[str, Any] = {"device_map": device, "dtype": dtype}
        if attn:
            kwargs["attn_implementation"] = attn
        self._model = self._from_pretrained(Qwen3TTSModel, self.model, kwargs)
        # Probe the tokenizer sample rate without paying for a full synthesis.
        probe = getattr(self._model, "sample_rate", None)
        if isinstance(probe, int) and probe > 0:
            self._sample_rate = probe
        elif hasattr(self._model, "processor") and getattr(self._model.processor, "sampling_rate", None):
            self._sample_rate = int(self._model.processor.sampling_rate)

    def _from_pretrained(self, model_class: Any, model_id: str, kwargs: dict[str, Any]) -> Any:
        """Loads the checkpoint, falling back to ModelScope when the Hub is blocked.

        HuggingFace is unreachable from mainland networks, which used to leave the
        service degraded with a raw OSError. ``QWEN3_TTS_HUB`` selects the source:
        ``auto`` (default) tries HuggingFace then ModelScope, ``modelscope`` goes
        straight to the mirror, ``hf`` keeps the old behaviour.
        """
        hub = os.environ.get("QWEN3_TTS_HUB", "auto").strip().lower()
        if _prefer_modelscope():
            print("huggingface not reachable; loading through modelscope", file=sys.stderr, flush=True)
            return model_class.from_pretrained(self._modelscope_snapshot(model_id), **kwargs)
        try:
            return model_class.from_pretrained(model_id, **kwargs)
        except Exception as exc:  # noqa: BLE001 - network failures surface as many types
            if hub == "hf" or not _is_hub_network_error(exc):
                raise
            print(
                f"huggingface is unreachable ({type(exc).__name__}); retrying through modelscope",
                file=sys.stderr,
                flush=True,
            )
            return model_class.from_pretrained(self._modelscope_snapshot(model_id), **kwargs)

    def _modelscope_snapshot(self, model_id: str) -> str:
        try:
            from modelscope import snapshot_download  # noqa: PLC0415
        except ImportError as exc:  # pragma: no cover - depends on the environment
            raise SynthesisError(
                "tts_download_unavailable",
                "无法连接 HuggingFace，且未安装 modelscope 镜像下载器；请执行 pip install modelscope 后重试",
                503,
                True,
            ) from exc
        cache = os.path.join(_modelscope_cache_root(), model_id.replace("/", "--"))
        os.makedirs(cache, exist_ok=True)
        print(f"downloading {model_id} from modelscope into {cache}", file=sys.stderr, flush=True)
        return str(snapshot_download(model_id, cache_dir=cache))

    def voices(self) -> list[dict[str, str]]:
        if self.task == "custom_voice" and self._model is not None:
            try:
                speakers = list(self._model.get_supported_speakers())
            except Exception:  # pragma: no cover - model-version dependent
                speakers = [voice_id for voice_id, _, _ in CUSTOM_VOICE_SPEAKERS]
            return [{"voice_id": name, "display_name": name, "language": "Auto"} for name in speakers]
        return [
            {"voice_id": voice_id, "display_name": display, "language": language}
            for voice_id, display, language in CUSTOM_VOICE_SPEAKERS
        ]

    def synthesize(self, text: str, voice: str | None, language: str | None, instruct: str | None) -> bytes:
        if self._model is None:
            raise SynthesisError("tts_model_loading", "TTS model is still loading", 503, True)
        import numpy as np  # noqa: PLC0415

        language = language or "Auto"
        with self._lock:
            if self.task == "voice_design":
                if not instruct:
                    raise SynthesisError(
                        "tts_instruct_required", "voice_design task requires an instruct prompt", 400, False
                    )
                wavs, sample_rate = self._model.generate_voice_design(
                    text=text, language=language, instruct=instruct
                )
            else:
                wavs, sample_rate = self._model.generate_custom_voice(
                    text=text,
                    language=language,
                    speaker=voice or "Serena",
                    instruct=instruct or "",
                )
        self._sample_rate = int(sample_rate)
        return encode_wav_pcm16(to_pcm16(wavs[0], np), int(sample_rate))


def to_pcm16(waveform: Any, np: Any) -> bytes:
    """Normalise whatever the model returned (float or int array) to PCM16."""
    array = np.asarray(waveform)
    if array.ndim > 1:
        array = array.reshape(-1)
    if np.issubdtype(array.dtype, np.floating):
        array = np.clip(array, -1.0, 1.0)
        array = (array * 32767.0).astype(np.int16)
    elif array.dtype != np.int16:
        array = np.clip(array, -32768, 32767).astype(np.int16)
    return array.tobytes()


class HubModelDownloader:
    """Downloads checkpoints through the Hub cache the model loader already uses.

    Prefers ``huggingface_hub`` and falls back to ``modelscope`` (mirror for
    mainland networks). Progress is measured by watching the cache directory
    grow, so it stays backend-agnostic and never needs a tqdm hook.
    """

    def __init__(self, cache_dir: str | None = None) -> None:
        self.cache_dir = cache_dir

    # -- backend ----------------------------------------------------------
    def _hub(self) -> Any:
        if not _prefer_modelscope():
            try:
                from huggingface_hub import HfApi, snapshot_download  # noqa: PLC0415

                return "hf", HfApi, snapshot_download
            except ImportError:
                pass
        try:
            from modelscope import snapshot_download  # noqa: PLC0415
            from modelscope.hub.api import HubApi  # noqa: PLC0415

            return "ms", HubApi, snapshot_download
        except ImportError as exc:
            raise SynthesisError(
                "tts_download_unavailable",
                "neither huggingface_hub nor modelscope is installed; run pip install -r requirements.txt",
                503,
                False,
            ) from exc

    def _cache_root(self) -> Path:
        if self.cache_dir:
            return Path(self.cache_dir)
        return Path(os.environ.get("HF_HOME", Path.home() / ".cache" / "huggingface")) / "hub"

    def _repo_dir(self, model_id: str) -> Path:
        return self._cache_root() / ("models--" + model_id.replace("/", "--"))

    # -- interface --------------------------------------------------------
    def total_bytes(self, model_id: str) -> int | None:
        try:
            kind, hub_api, _ = self._hub()
            api = hub_api()
            info = api.model_info(model_id, files_metadata=True)
            sizes = [sibling.size for sibling in (info.siblings or []) if getattr(sibling, "size", None)]
        except Exception:
            return None
        return sum(sizes) or None

    def is_installed(self, model_id: str) -> bool:
        repo = self._repo_dir(model_id)
        snapshots = repo / "snapshots"
        if not snapshots.is_dir():
            return False
        return any(any(child.iterdir()) for child in snapshots.iterdir() if child.is_dir())

    def download(self, model_id: str, on_progress: Any) -> str:
        kind, _, snapshot_download = self._hub()
        total = self.total_bytes(model_id)
        done = threading.Event()
        observed = {"bytes": 0}

        def poll() -> None:
            while not done.wait(0.5):
                size = _dir_bytes(self._cache_root())
                observed["bytes"] = size
                percent = int(min(99, size * 100 / total)) if total else None
                on_progress(percent, size, total)

        poller = threading.Thread(target=poll, name="qwen3-tts-download-progress", daemon=True)
        poller.start()
        try:
            if kind == "hf":
                path = snapshot_download(repo_id=model_id, cache_dir=self.cache_dir)
            else:
                path = snapshot_download(model_id, cache_dir=self.cache_dir)
        finally:
            done.set()
            poller.join(timeout=1.0)
        on_progress(100, _dir_bytes(self._cache_root()), total)
        return str(path)


class MockModelDownloader:
    """Simulates a download so the UI flow can be exercised without a network."""

    def __init__(self) -> None:
        self.root = Path(tempfile.gettempdir()) / "qwen3-tts-mock-models"

    def _marker(self, model_id: str) -> Path:
        return self.root / model_id.replace("/", "--") / "model.marker"

    def total_bytes(self, model_id: str) -> int | None:
        return 512 * 1024

    def is_installed(self, model_id: str) -> bool:
        return self._marker(model_id).is_file()

    def download(self, model_id: str, on_progress: Any) -> str:
        total = self.total_bytes(model_id) or 0
        marker = self._marker(model_id)
        marker.parent.mkdir(parents=True, exist_ok=True)
        for step in range(1, 6):
            time.sleep(0.15)
            on_progress(step * 20, int(total * step / 5), total)
        marker.write_text("mock model\n", encoding="utf-8")
        return str(marker.parent)


def _dir_bytes(root: Path) -> int:
    total = 0
    if not root.is_dir():
        return 0
    for path in root.rglob("*"):
        try:
            if path.is_file():
                total += path.stat().st_size
        except OSError:
            continue
    return total


class ServiceState:
    """Shared, thread-safe service state."""

    def __init__(self, engine: Engine, downloader: Any | None = None) -> None:
        self.engine = engine
        self.ready = engine.name == "mock"
        self.status = "ok" if self.ready else "loading"
        self.error: str | None = None
        self.started_at = time.time()
        self.requests = 0
        self.failures = 0
        self._lock = threading.Lock()
        self.downloader = downloader if downloader is not None else HubModelDownloader()
        self._model_lock = threading.Lock()
        self._download: dict[str, Any] = {
            "model_id": None,
            "status": "idle",
            "progress_percent": None,
            "bytes_downloaded": 0,
            "bytes_total": None,
            "path": None,
            "error": None,
            "updated_at": time.time(),
        }

    def record(self, ok: bool) -> None:
        with self._lock:
            self.requests += 1
            if not ok:
                self.failures += 1

    # -- model catalogue --------------------------------------------------
    def models(self) -> list[dict[str, Any]]:
        return [
            {
                **entry,
                "is_installed": _safe_installed(self.downloader, entry["model_id"]),
            }
            for entry in MODEL_CATALOG
        ]

    def download_snapshot(self) -> dict[str, Any]:
        with self._model_lock:
            return dict(self._download)

    def start_download(self, model_id: str | None) -> dict[str, Any]:
        model_id = (model_id or DEFAULT_MODEL_ID).strip()
        if model_id not in {entry["model_id"] for entry in MODEL_CATALOG}:
            raise SynthesisError(
                "tts_model_unknown", f"unsupported model: {model_id}", 400, False
            )
        with self._model_lock:
            if self._download["status"] == "downloading":
                if self._download["model_id"] == model_id:
                    return dict(self._download)
                raise SynthesisError(
                    "tts_download_busy", "another model download is in progress", 409, True
                )
            self._download = {
                "model_id": model_id,
                "status": "downloading",
                "progress_percent": 0,
                "bytes_downloaded": 0,
                "bytes_total": _safe_total(self.downloader, model_id),
                "path": None,
                "error": None,
                "updated_at": time.time(),
            }
            snapshot = dict(self._download)

        def on_progress(percent: int | None, downloaded: int, total: int | None) -> None:
            with self._model_lock:
                if self._download["model_id"] != model_id:
                    return
                self._download.update(
                    progress_percent=percent,
                    bytes_downloaded=downloaded,
                    bytes_total=total,
                    updated_at=time.time(),
                )

        def run() -> None:
            try:
                path = self.downloader.download(model_id, on_progress)
                with self._model_lock:
                    self._download.update(
                        status="ready",
                        progress_percent=100,
                        path=path,
                        error=None,
                        updated_at=time.time(),
                    )
            except Exception as exc:  # pragma: no cover - backend dependent
                traceback.print_exc()
                with self._model_lock:
                    self._download.update(
                        status="error",
                        error=f"{type(exc).__name__}: {exc}",
                        updated_at=time.time(),
                    )

        threading.Thread(target=run, name="qwen3-tts-download", daemon=True).start()
        return snapshot


def _safe_installed(downloader: Any, model_id: str) -> bool:
    try:
        return bool(downloader.is_installed(model_id))
    except Exception:
        return False


def _safe_total(downloader: Any, model_id: str) -> int | None:
    try:
        return downloader.total_bytes(model_id)
    except Exception:
        return None


class Handler(BaseHTTPRequestHandler):
    server_version = "Qwen3TtsService/1.0"
    protocol_version = "HTTP/1.1"

    state: ServiceState  # injected by build_server

    # -- plumbing ---------------------------------------------------------
    def log_message(self, fmt: str, *args: Any) -> None:  # keep stdout clean
        sys.stderr.write("%s - %s\n" % (self.address_string(), fmt % args))

    def _send_json(self, payload: dict[str, Any], status: int = HTTPStatus.OK) -> None:
        body = json.dumps(payload, ensure_ascii=False).encode("utf-8")
        self.send_response(status)
        self.send_header("Content-Type", "application/json; charset=utf-8")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def _send_bytes(self, body: bytes, content_type: str, status: int = HTTPStatus.OK) -> None:
        self.send_response(status)
        self.send_header("Content-Type", content_type)
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def _read_json(self) -> dict[str, Any]:
        length = int(self.headers.get("Content-Length") or 0)
        if length <= 0:
            raise SynthesisError("tts_invalid_request", "request body is required", 400, False)
        if length > 1_000_000:
            raise SynthesisError("tts_invalid_request", "request body is too large", 413, False)
        raw = self.rfile.read(length)
        try:
            payload = json.loads(raw.decode("utf-8"))
        except (UnicodeDecodeError, json.JSONDecodeError) as exc:
            raise SynthesisError("tts_invalid_request", f"invalid JSON: {exc}", 400, False) from exc
        if not isinstance(payload, dict):
            raise SynthesisError("tts_invalid_request", "request body must be an object", 400, False)
        return payload

    # -- routes -----------------------------------------------------------
    def do_GET(self) -> None:  # noqa: N802 - BaseHTTPRequestHandler API
        path = self.path.split("?", 1)[0]
        if path == "/health":
            self._send_json(self._health())
        elif path == "/v1/voices":
            self._send_json(self._voices())
        elif path == "/v1/models":
            self._send_json(
                {"models": self.state.models(), "download": self.state.download_snapshot()}
            )
        else:
            self._send_json({"error": {"code": "tts_not_found", "message": "unknown path", "retryable": False}}, 404)

    def do_POST(self) -> None:  # noqa: N802 - BaseHTTPRequestHandler API
        path = self.path.split("?", 1)[0]
        if path == "/v1/models/download":
            try:
                payload = self._read_json()
                requested = payload.get("model")
                if requested is not None and not isinstance(requested, str):
                    raise SynthesisError("tts_invalid_request", "model must be a string", 400, False)
                snapshot = self.state.start_download(requested)
                self._send_json(snapshot, HTTPStatus.ACCEPTED)
            except SynthesisError as exc:
                self._send_json(
                    {"error": {"code": exc.code, "message": exc.message, "retryable": exc.retryable}},
                    exc.status,
                )
            except Exception as exc:  # pragma: no cover - defensive
                traceback.print_exc()
                self._send_json(
                    {"error": {"code": "tts_download_failed", "message": str(exc), "retryable": True}},
                    500,
                )
            return
        if path != "/v1/audio/speech":
            self._send_json({"error": {"code": "tts_not_found", "message": "unknown path", "retryable": False}}, 404)
            return
        state = self.state
        try:
            payload = self._read_json()
            text = payload.get("text")
            if not isinstance(text, str) or not text.strip():
                raise SynthesisError("tts_invalid_request", "text is required", 400, False)
            if len(text) > MAX_TEXT_CHARS:
                raise SynthesisError("tts_invalid_request", "text is too long", 413, False)
            if not state.ready:
                raise SynthesisError("tts_model_loading", "TTS model is still loading", 503, True)
            response_format = payload.get("response_format") or "wav"
            if response_format != "wav":
                raise SynthesisError("tts_unsupported_format", "only wav is supported", 400, False)
            language = payload.get("language")
            if language is not None and language not in SUPPORTED_LANGUAGES:
                raise SynthesisError("tts_unsupported_language", f"unsupported language: {language}", 400, False)
            voice = payload.get("voice")
            instruct = payload.get("instruct")
            body = state.engine.synthesize(
                text.strip(),
                voice if isinstance(voice, str) and voice else None,
                language if isinstance(language, str) else None,
                instruct if isinstance(instruct, str) and instruct.strip() else None,
            )
            state.record(True)
            self._send_bytes(body, "audio/wav")
        except SynthesisError as exc:
            state.record(False)
            self._send_json(
                {"error": {"code": exc.code, "message": exc.message, "retryable": exc.retryable}}, exc.status
            )
        except Exception as exc:  # pragma: no cover - defensive
            state.record(False)
            traceback.print_exc()
            self._send_json(
                {"error": {"code": "tts_synthesis_failed", "message": str(exc), "retryable": True}}, 500
            )

    # -- payloads ---------------------------------------------------------
    def _health(self) -> dict[str, Any]:
        state = self.state
        return {
            "status": state.status,
            "ready": state.ready,
            "engine": state.engine.name,
            "model": state.engine.model,
            "sample_rate": state.engine.sample_rate,
            "languages": list(SUPPORTED_LANGUAGES),
            "voices": state.engine.voices(),
            "error": state.error,
            "uptime_seconds": round(time.time() - state.started_at, 3),
            "requests": state.requests,
            "failures": state.failures,
        }

    def _voices(self) -> dict[str, Any]:
        return {"voices": self.state.engine.voices(), "languages": list(SUPPORTED_LANGUAGES)}


def build_server(host: str, port: int, state: ServiceState) -> ThreadingHTTPServer:
    handler = type("BoundHandler", (Handler,), {"state": state})
    server = ThreadingHTTPServer((host, port), handler)
    server.daemon_threads = True
    return server


def load_model_async(state: ServiceState, engine: Qwen3TtsEngine) -> None:
    def run() -> None:
        try:
            engine.load()
            state.ready = True
            state.status = "ok"
            state.error = None
        except Exception as exc:  # pragma: no cover - environment dependent
            state.ready = False
            state.status = "degraded"
            state.error = (
                "无法连接模型下载源（HuggingFace/ModelScope），请检查网络后重试"
                if _is_hub_network_error(exc)
                else f"{type(exc).__name__}: {exc}"
            )
            traceback.print_exc()

    threading.Thread(target=run, name="qwen3-tts-loader", daemon=True).start()


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Qwen3-TTS HTTP speech service")
    parser.add_argument("--host", default=DEFAULT_HOST)
    parser.add_argument("--port", type=int, default=DEFAULT_PORT)
    parser.add_argument("--mock", action="store_true", help="serve the contract with synthetic audio (no model)")
    parser.add_argument("--model", default="Qwen/Qwen3-TTS-12Hz-1.7B-CustomVoice")
    parser.add_argument("--task", choices=("custom_voice", "voice_design"), default="custom_voice")
    parser.add_argument("--device", default="cuda:0")
    parser.add_argument("--dtype", choices=("bfloat16", "float16", "float32"), default="bfloat16")
    parser.add_argument("--attn", default="flash_attention_2", help="transformers attn_implementation ('' to disable)")
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(argv if argv is not None else sys.argv[1:])
    if args.mock:
        state = ServiceState(MockEngine(), MockModelDownloader())
        server = build_server(args.host, args.port, state)
        print(f"qwen3-tts mock service on http://{args.host}:{args.port}", flush=True)
    else:
        engine = Qwen3TtsEngine(args.model, args.task, args.device, args.dtype, args.attn)
        state = ServiceState(engine)
        server = build_server(args.host, args.port, state)
        load_model_async(state, engine)
        print(f"qwen3-tts service on http://{args.host}:{args.port} (loading {args.model})", flush=True)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        server.server_close()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
