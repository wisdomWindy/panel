#!/usr/bin/env python3
"""Discover AI articles, cache readable article text, and build a static dashboard."""

from __future__ import annotations

import argparse
from copy import deepcopy
import hashlib
import html
import json
import os
import re
import ssl
import sys
from concurrent.futures import ThreadPoolExecutor, as_completed
from datetime import datetime, timedelta, timezone
from email.utils import parsedate_to_datetime
from html.parser import HTMLParser
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from typing import Any, Callable, Iterable
from urllib.parse import parse_qsl, urlencode, urlparse, urlunparse
from urllib.request import Request, urlopen
import xml.etree.ElementTree as ET


SKILL_DIR = Path(__file__).resolve().parents[1]
DEFAULT_SOURCES = SKILL_DIR / "references" / "sources.json"
TEMPLATE_PATH = SKILL_DIR / "assets" / "dashboard.html"
TRACKING_PARAMS = {"fbclid", "gclid", "mc_cid", "mc_eid", "ref", "source"}
ARTICLE_EXTRACTOR_VERSION = 3
TRANSLATOR_VERSION = 1


def default_runtime_root() -> Path:
    configured = os.environ.get("AI_FRONTIER_RADAR_HOME")
    if configured:
        return Path(configured).expanduser().resolve()
    parts = SKILL_DIR.parts
    if ".agents" in parts:
        return Path(*parts[: parts.index(".agents")]) / "ai-frontier-radar"
    return Path.cwd() / "ai-frontier-radar"


DEFAULT_RUNTIME_ROOT = default_runtime_root()
DEFAULT_DATA_DIR = DEFAULT_RUNTIME_ROOT / "data"
DEFAULT_SITE_DIR = DEFAULT_RUNTIME_ROOT / "site"

CATEGORY_KEYWORDS = {
    "Models": (
        "model release", "new model", "foundation model", "language model", "multimodal",
        "reasoning model", "llm", "vlm", "checkpoint", "weights", "parameter",
    ),
    "Agents": (
        "agent", "agentic", "computer use", "tool use", "function calling", "multi-agent",
        "mcp", "workflow automation",
    ),
    "Research": (
        "paper", "research", "benchmark", "evaluation", "alignment", "interpretability",
        "reinforcement learning", "rlhf", "pretraining", "post-training", "dataset",
    ),
    "Engineering": (
        "inference", "training", "serving", "latency", "throughput", "quantization",
        "distillation", "fine-tuning", "finetuning", "gpu", "compiler", "runtime",
    ),
    "Open Source": (
        "open source", "open-source", "github", "apache 2", "mit license", "released weights",
    ),
    "Ideas": (
        "perspective", "essay", "future of ai", "principles", "paradigm", "intelligence",
        "society", "philosophy", "thoughts",
    ),
    "Product": (
        "api", "product", "platform", "preview", "available", "launch", "assistant",
        "copilot", "enterprise",
    ),
}


def trusted_ssl_context() -> ssl.SSLContext:
    """Build a verified TLS context, including common macOS package-manager CA paths."""
    defaults = ssl.get_default_verify_paths()
    candidates = [
        os.environ.get("SSL_CERT_FILE"),
        defaults.cafile,
        "/etc/ssl/cert.pem",
        "/opt/homebrew/etc/ca-certificates/cert.pem",
        "/usr/local/etc/openssl@3/cert.pem",
    ]
    for candidate in candidates:
        if candidate and Path(candidate).is_file():
            return ssl.create_default_context(cafile=candidate)
    try:
        import certifi  # type: ignore[import-not-found]

        return ssl.create_default_context(cafile=certifi.where())
    except ImportError:
        return ssl.create_default_context()


TLS_CONTEXT = trusted_ssl_context()


class TextExtractor(HTMLParser):
    def __init__(self) -> None:
        super().__init__()
        self.parts: list[str] = []

    def handle_data(self, data: str) -> None:
        self.parts.append(data)


class PageMetadataExtractor(HTMLParser):
    def __init__(self) -> None:
        super().__init__()
        self.in_title = False
        self.title_parts: list[str] = []
        self.social_title = ""

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        lowered = tag.lower()
        if lowered == "title":
            self.in_title = True
        if lowered == "meta":
            values = {str(key).lower(): value or "" for key, value in attrs}
            name = (values.get("property") or values.get("name") or "").lower()
            if name in {"og:title", "twitter:title"} and values.get("content"):
                self.social_title = values["content"].strip()

    def handle_endtag(self, tag: str) -> None:
        if tag.lower() == "title":
            self.in_title = False

    def handle_data(self, data: str) -> None:
        if self.in_title:
            self.title_parts.append(data)

    def title(self) -> str:
        return clean_text(self.social_title or " ".join(self.title_parts), 240)


class ArticleTextExtractor(HTMLParser):
    """Extract readable blocks, preferring article, then main, then body."""

    IGNORED_TAGS = {"script", "style", "nav", "footer", "header", "aside", "form", "svg", "noscript", "button"}
    BLOCK_TAGS = {"p", "h1", "h2", "h3", "h4", "li", "blockquote", "pre", "figcaption", "div", "section"}

    def __init__(self) -> None:
        super().__init__()
        self.stack: list[str] = []
        self.buffers: dict[str, list[str]] = {"article": [], "main": [], "body": []}

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        del attrs
        self.stack.append(tag.lower())

    def handle_startendtag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        del tag, attrs

    def handle_endtag(self, tag: str) -> None:
        lowered = tag.lower()
        if lowered in self.BLOCK_TAGS:
            for target in self._targets():
                self.buffers[target].append("\n")
        for index in range(len(self.stack) - 1, -1, -1):
            if self.stack[index] == lowered:
                del self.stack[index:]
                break

    def handle_data(self, data: str) -> None:
        if any(tag in self.IGNORED_TAGS for tag in self.stack):
            return
        value = re.sub(r"\s+", " ", data)
        if not value.strip():
            return
        for target in self._targets():
            self.buffers[target].append(value.strip() + " ")

    def _targets(self) -> list[str]:
        targets: list[str] = []
        if "body" in self.stack:
            targets.append("body")
        if "main" in self.stack:
            targets.append("main")
        if "article" in self.stack:
            targets.append("article")
        return targets

    def text(self, max_chars: int) -> tuple[str, bool]:
        for target in ("article", "main", "body"):
            normalized = normalize_article_text("".join(self.buffers[target]))
            if normalized:
                return normalized[:max_chars].rstrip(), len(normalized) > max_chars
        return "", False


def utc_now() -> datetime:
    return datetime.now(timezone.utc)


def iso_utc(value: datetime) -> str:
    return value.astimezone(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def clean_text(value: str | None, limit: int = 700) -> str:
    if not value:
        return ""
    parser = TextExtractor()
    try:
        parser.feed(html.unescape(value))
        text = " ".join(parser.parts)
    except Exception:
        text = re.sub(r"<[^>]+>", " ", html.unescape(value))
    text = re.sub(r"\s+", " ", text).strip()
    if len(text) <= limit:
        return text
    return text[: limit - 1].rstrip() + "..."


def normalize_article_text(value: str) -> str:
    lines: list[str] = []
    previous = ""
    for raw_line in value.splitlines():
        line = re.sub(r"\s+", " ", html.unescape(raw_line)).strip()
        if not line or line == previous:
            continue
        if line.lower() in {"accept cookies", "cookie settings", "skip to content", "like", "dislike"}:
            continue
        if len(line) <= 2 and line.isupper():
            continue
        lines.append(line)
        previous = line
    return "\n\n".join(lines)


def contains_english_text(value: str | None) -> bool:
    """Return whether a field is predominantly English prose that needs translation."""
    if not value:
        return False
    latin_words = re.findall(r"[A-Za-z]+(?:['’-][A-Za-z]+)?", value)
    latin_count = sum(len(word) for word in latin_words)
    cjk_count = len(re.findall(r"[\u3400-\u9fff]", value))
    if latin_count < 8 or len(latin_words) < 2:
        return False
    return cjk_count == 0 or (len(latin_words) >= 4 and latin_count >= max(24, cjk_count * 4))


def split_translation_chunks(value: str, max_chars: int) -> list[str]:
    """Split long prose without discarding paragraph boundaries."""
    if len(value) <= max_chars:
        return [value]
    chunks: list[str] = []
    current = ""
    for paragraph in value.split("\n\n"):
        segments = [paragraph[index:index + max_chars] for index in range(0, len(paragraph), max_chars)] or [""]
        for segment in segments:
            candidate = f"{current}\n\n{segment}" if current else segment
            if current and len(candidate) > max_chars:
                chunks.append(current)
                current = segment
            else:
                current = candidate
    if current:
        chunks.append(current)
    return chunks


def openai_compatible_translate(value: str, settings: dict[str, Any]) -> str:
    """Translate one field through a configured OpenAI-compatible chat endpoint."""
    api_key_env = str(settings.get("api_key_env", "OPENAI_API_KEY"))
    api_key = os.environ.get(api_key_env, "").strip()
    if not api_key:
        raise ValueError(f"translation API key environment variable is not set: {api_key_env}")
    base_url_env = str(settings.get("base_url_env", "AI_FRONTIER_TRANSLATION_BASE_URL"))
    model_env = str(settings.get("model_env", "AI_FRONTIER_TRANSLATION_MODEL"))
    base_url = os.environ.get(base_url_env) or str(settings.get("base_url", "https://api.openai.com/v1"))
    model = os.environ.get(model_env) or str(settings.get("model", "gpt-4.1-mini"))
    timeout = int(settings.get("timeout_seconds", 60))
    max_chars = max(500, int(settings.get("max_chars_per_request", 6000)))
    endpoint = f"{base_url.rstrip('/')}/chat/completions"
    translated_chunks: list[str] = []
    for chunk in split_translation_chunks(value, max_chars):
        payload = json.dumps({
            "model": model,
            "temperature": 0,
            "messages": [
                {
                    "role": "system",
                    "content": (
                        "将用户提供的英文内容准确翻译为简体中文。保留段落、Markdown、代码、URL、数字、"
                        "模型名和产品名；不要总结、解释或添加原文没有的信息，只返回译文。"
                    ),
                },
                {"role": "user", "content": chunk},
            ],
        }, ensure_ascii=False).encode("utf-8")
        request = Request(endpoint, data=payload, headers={
            "Authorization": f"Bearer {api_key}",
            "Content-Type": "application/json",
            "Accept": "application/json",
        })
        with urlopen(request, timeout=timeout, context=TLS_CONTEXT) as response:
            result = json.loads(response.read().decode("utf-8"))
        content = result.get("choices", [{}])[0].get("message", {}).get("content")
        if not isinstance(content, str) or not content.strip():
            raise ValueError("translation endpoint returned no translated content")
        translated_chunks.append(content.strip())
    return "\n\n".join(translated_chunks)


def translation_field_targets(item: dict[str, Any]) -> list[tuple[str, dict[str, Any], str]]:
    targets = [("title", item, "title"), ("summary", item, "summary")]
    article = item.get("article")
    if isinstance(article, dict) and article.get("status") == "ok":
        targets.extend([
            ("article.pageTitle", article, "pageTitle"),
            ("article.content", article, "content"),
        ])
    return targets


def translate_item_fields(
    item: dict[str, Any], translator: Callable[[str], str], translated_at: str
) -> tuple[int, int, list[dict[str, str]]]:
    metadata = item.setdefault("translation", {})
    fields = metadata.setdefault("fields", {})
    attempted = 0
    translated = 0
    errors: list[dict[str, str]] = []
    for field_name, container, key in translation_field_targets(item):
        current = str(container.get(key) or "")
        cached = fields.get(field_name, {}) if isinstance(fields.get(field_name), dict) else {}
        if (
            cached.get("translatorVersion") == TRANSLATOR_VERSION
            and current in {cached.get("original"), cached.get("translated")}
            and cached.get("translated")
        ):
            container[key] = cached["translated"]
            continue
        if not contains_english_text(current):
            fields.pop(field_name, None)
            continue
        attempted += 1
        try:
            translated_value = translator(current).strip()
            if not translated_value or contains_english_text(translated_value):
                raise ValueError("translator did not return predominantly Chinese content")
            fields[field_name] = {
                "original": current,
                "translated": translated_value,
                "translatorVersion": TRANSLATOR_VERSION,
            }
            container[key] = translated_value
            if field_name == "article.content":
                container["characterCount"] = len(translated_value)
            translated += 1
        except Exception as exc:
            errors.append({
                "itemId": str(item.get("id", "unknown")),
                "field": field_name,
                "error": f"{type(exc).__name__}: {exc}",
            })
    remaining = any(contains_english_text(str(container.get(key) or "")) for _, container, key in translation_field_targets(item))
    metadata.update({
        "status": "error" if errors and not translated else ("partial" if errors or remaining else "ok"),
        "sourceLanguage": "en",
        "targetLanguage": "zh-CN",
        "translatorVersion": TRANSLATOR_VERSION,
        "translatedAt": translated_at,
        "error": "; ".join(f"{error['field']}: {error['error']}" for error in errors) or None,
    })
    if not fields and not errors:
        item.pop("translation", None)
    return attempted, translated, errors


def pending_translation_count(items: list[dict[str, Any]]) -> int:
    return sum(
        1 for item in items
        if any(contains_english_text(str(container.get(key) or "")) for _, container, key in translation_field_targets(item))
    )


def translate_items(
    items: list[dict[str, Any]], settings: dict[str, Any], translated_at: str,
    translator: Callable[[str], str] | None = None,
) -> dict[str, Any]:
    candidates = [
        item for item in items
        if any(contains_english_text(str(container.get(key) or "")) for _, container, key in translation_field_targets(item))
    ]
    if not candidates:
        return {"attemptedItems": 0, "translatedItems": 0, "failedItems": 0, "translatedFields": 0, "pendingItems": 0, "errors": []}
    if str(settings.get("provider", "openai-compatible")) != "openai-compatible":
        error = f"unsupported translation provider: {settings.get('provider')}"
        return {"attemptedItems": len(candidates), "translatedItems": 0, "failedItems": len(candidates), "translatedFields": 0, "pendingItems": len(candidates), "errors": [{"itemId": "configuration", "field": "translation", "error": error}]}
    if translator is None:
        api_key_env = str(settings.get("api_key_env", "OPENAI_API_KEY"))
        if not os.environ.get(api_key_env, "").strip():
            error = f"translation API key environment variable is not set: {api_key_env}"
            return {"attemptedItems": len(candidates), "translatedItems": 0, "failedItems": len(candidates), "translatedFields": 0, "pendingItems": len(candidates), "errors": [{"itemId": "configuration", "field": "translation", "error": error}]}
        translator = lambda value: openai_compatible_translate(value, settings)
    concurrency = max(1, min(int(settings.get("concurrency", 3)), len(candidates)))
    results: list[tuple[int, int, list[dict[str, str]]]] = []
    with ThreadPoolExecutor(max_workers=concurrency) as pool:
        futures = [pool.submit(translate_item_fields, item, translator, translated_at) for item in candidates]
        for future in as_completed(futures):
            results.append(future.result())
    errors = [error for _, _, item_errors in results for error in item_errors]
    translated_items = sum(1 for _, translated, item_errors in results if translated and not item_errors)
    failed_items = sum(1 for _, _, item_errors in results if item_errors)
    return {
        "attemptedItems": len(candidates),
        "translatedItems": translated_items,
        "failedItems": failed_items,
        "translatedFields": sum(translated for _, translated, _ in results),
        "pendingItems": pending_translation_count(items),
        "errors": errors,
    }


def extract_article_text(payload: str, max_chars: int) -> tuple[str, bool]:
    parser = ArticleTextExtractor()
    parser.feed(payload)
    return parser.text(max_chars)


def extract_page_title(payload: str) -> str:
    parser = PageMetadataExtractor()
    parser.feed(payload)
    return parser.title()


def local_name(tag: str) -> str:
    return tag.rsplit("}", 1)[-1].lower()


def first_text(node: ET.Element, names: Iterable[str]) -> str:
    wanted = {name.lower() for name in names}
    for child in node.iter():
        if child is node:
            continue
        if local_name(child.tag) in wanted and child.text and child.text.strip():
            return child.text.strip()
    return ""


def entry_link(node: ET.Element) -> str:
    for child in node.iter():
        if local_name(child.tag) != "link":
            continue
        href = child.attrib.get("href", "").strip()
        rel = child.attrib.get("rel", "alternate")
        if href and rel in {"alternate", ""}:
            return href
        if child.text and child.text.strip():
            return child.text.strip()
    return first_text(node, ("guid", "id"))


def parse_date(value: str | None) -> str | None:
    if not value:
        return None
    raw = value.strip()
    try:
        parsed = parsedate_to_datetime(raw)
        if parsed.tzinfo is None:
            parsed = parsed.replace(tzinfo=timezone.utc)
        return iso_utc(parsed)
    except (TypeError, ValueError, OverflowError):
        pass
    try:
        parsed = datetime.fromisoformat(raw.replace("Z", "+00:00"))
        if parsed.tzinfo is None:
            parsed = parsed.replace(tzinfo=timezone.utc)
        return iso_utc(parsed)
    except ValueError:
        return None


def canonical_url(value: str) -> str:
    if not value:
        return ""
    try:
        parsed = urlparse(value.strip())
        filtered = [
            (key, val)
            for key, val in parse_qsl(parsed.query, keep_blank_values=True)
            if not key.lower().startswith("utm_") and key.lower() not in TRACKING_PARAMS
        ]
        path = parsed.path.rstrip("/") or "/"
        return urlunparse((parsed.scheme.lower(), parsed.netloc.lower(), path, "", urlencode(filtered), ""))
    except ValueError:
        return value.strip()


def title_key(value: str) -> str:
    return re.sub(r"[^a-z0-9\u4e00-\u9fff]+", "", value.lower())


def classify(title: str, summary: str, fallback: str) -> str:
    haystack = f"{title} {summary}".lower()
    scores = {
        category: sum(1 for keyword in keywords if keyword in haystack)
        for category, keywords in CATEGORY_KEYWORDS.items()
    }
    category, score = max(scores.items(), key=lambda pair: pair[1])
    return category if score else (fallback or "Technology")


def normalize_entry(node: ET.Element, source: dict[str, Any], fetched_at: str) -> dict[str, Any] | None:
    title = clean_text(first_text(node, ("title",)), 240)
    url = canonical_url(entry_link(node))
    if not title or not url:
        return None
    summary = clean_text(first_text(node, ("summary", "description", "content", "encoded")))
    published = parse_date(first_text(node, ("published", "pubdate", "updated", "date")))
    author = clean_text(first_text(node, ("author", "creator", "name")), 120)
    category = classify(title, summary, str(source.get("default_category", "Technology")))
    stable = url or f"{source['id']}:{title_key(title)}:{published or ''}"
    return {
        "id": hashlib.sha256(stable.encode("utf-8")).hexdigest()[:20],
        "title": title,
        "url": url,
        "summary": summary,
        "publishedAt": published,
        "fetchedAt": fetched_at,
        "author": author or None,
        "category": category,
        "source": {
            "id": str(source["id"]),
            "name": str(source["name"]),
            "kind": str(source.get("kind", "community")),
            "region": "CN" if source.get("region") == "CN" else "Global",
            "fetchArticles": bool(source.get("fetch_articles", source.get("kind") != "research")),
            "updateTitleFromArticle": bool(source.get("update_title_from_article", False)),
        },
    }


def parse_feed(payload: bytes, source: dict[str, Any], fetched_at: str) -> list[dict[str, Any]]:
    if source.get("format") == "sitemap":
        return parse_sitemap(payload, source, fetched_at)
    root = ET.fromstring(payload)
    nodes = [node for node in root.iter() if local_name(node.tag) in {"item", "entry"}]
    items: list[dict[str, Any]] = []
    for node in nodes:
        normalized = normalize_entry(node, source, fetched_at)
        if normalized:
            items.append(normalized)
    return items


def parse_sitemap(payload: bytes, source: dict[str, Any], fetched_at: str) -> list[dict[str, Any]]:
    root = ET.fromstring(payload)
    include_patterns = [str(pattern) for pattern in source.get("include_patterns", [])]
    items: list[dict[str, Any]] = []
    for node in root.iter():
        if local_name(node.tag) != "url":
            continue
        url = canonical_url(first_text(node, ("loc",)))
        if not url or (include_patterns and not any(pattern in url for pattern in include_patterns)):
            continue
        slug = urlparse(url).path.rstrip("/").rsplit("/", 1)[-1]
        readable_slug = re.sub(r"[-_]+", " ", slug).strip() or "update"
        title = f"{source['name']}: {readable_slug}"
        published = parse_date(first_text(node, ("lastmod",)))
        stable = url
        items.append({
            "id": hashlib.sha256(stable.encode("utf-8")).hexdigest()[:20],
            "title": title,
            "url": url,
            "summary": "",
            "publishedAt": published,
            "fetchedAt": fetched_at,
            "author": None,
            "category": str(source.get("default_category", "Technology")),
            "source": {
                "id": str(source["id"]),
                "name": str(source["name"]),
                "kind": str(source.get("kind", "official")),
                "region": "CN" if source.get("region") == "CN" else "Global",
                "fetchArticles": bool(source.get("fetch_articles", True)),
                "updateTitleFromArticle": bool(source.get("update_title_from_article", True)),
            },
        })
    return items


def fetch_source(source: dict[str, Any], timeout: int, fetched_at: str) -> tuple[list[dict[str, Any]], dict[str, Any]]:
    headers = {
        "User-Agent": "AI-Frontier-Radar/1.0 (+local research dashboard)",
        "Accept": "application/atom+xml, application/rss+xml, application/xml, text/xml;q=0.9, */*;q=0.1",
        **{str(k): str(v) for k, v in source.get("headers", {}).items()},
        "Cache-Control": "no-cache, no-store, max-age=0",
        "Pragma": "no-cache",
    }
    started = utc_now()
    try:
        request = Request(str(source["url"]), headers=headers)
        with urlopen(request, timeout=timeout, context=TLS_CONTEXT) as response:
            payload = response.read()
        items = parse_feed(payload, source, fetched_at)
        if not items:
            raise ValueError("feed contained no valid RSS/Atom entries")
        elapsed = round((utc_now() - started).total_seconds() * 1000)
        return items, {
            "id": source["id"], "name": source["name"], "status": "ok",
            "itemCount": len(items), "durationMs": elapsed, "error": None,
        }
    except Exception as exc:
        elapsed = round((utc_now() - started).total_seconds() * 1000)
        return [], {
            "id": source.get("id", "unknown"), "name": source.get("name", "Unknown"),
            "status": "error", "itemCount": 0, "durationMs": elapsed,
            "error": f"{type(exc).__name__}: {exc}",
        }


def fetch_article(
    item: dict[str, Any], timeout: int, max_chars: int, min_chars: int, fetched_at: str
) -> tuple[str, dict[str, Any]]:
    started = utc_now()
    page_title = ""
    try:
        headers = {
            "User-Agent": "AI-Frontier-Radar/1.0 (+local readable article cache)",
            "Accept": "text/html, application/xhtml+xml;q=0.9, */*;q=0.1",
            "Cache-Control": "no-cache, no-store, max-age=0",
            "Pragma": "no-cache",
        }
        request = Request(str(item["url"]), headers=headers)
        with urlopen(request, timeout=timeout, context=TLS_CONTEXT) as response:
            content_type = response.headers.get_content_type()
            if content_type not in {"text/html", "application/xhtml+xml"}:
                raise ValueError(f"unsupported article content type: {content_type}")
            charset = response.headers.get_content_charset() or "utf-8"
            payload = response.read(3_000_001)
        if len(payload) > 3_000_000:
            payload = payload[:3_000_000]
        document = payload.decode(charset, errors="replace")
        page_title = extract_page_title(document)
        content, truncated = extract_article_text(document, max_chars)
        if len(content) < min_chars:
            raise ValueError(f"readable article text is too short: {len(content)} characters")
        elapsed = round((utc_now() - started).total_seconds() * 1000)
        return str(item["id"]), {
            "status": "ok",
            "content": content,
            "pageTitle": page_title,
            "characterCount": len(content),
            "truncated": truncated,
            "extractorVersion": ARTICLE_EXTRACTOR_VERSION,
            "fetchedAt": fetched_at,
            "durationMs": elapsed,
            "error": None,
        }
    except Exception as exc:
        elapsed = round((utc_now() - started).total_seconds() * 1000)
        return str(item["id"]), {
            "status": "error",
            "content": None,
            "pageTitle": page_title or None,
            "characterCount": 0,
            "truncated": False,
            "fetchedAt": fetched_at,
            "durationMs": elapsed,
            "error": f"{type(exc).__name__}: {exc}",
        }
def load_json(path: Path, default: Any) -> Any:
    if not path.exists():
        return default
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def timestamp_value(raw: Any) -> datetime | None:
    if not raw:
        return None
    try:
        parsed = datetime.fromisoformat(str(raw).replace("Z", "+00:00"))
        return parsed if parsed.tzinfo else parsed.replace(tzinfo=timezone.utc)
    except ValueError:
        return None


def date_value(item: dict[str, Any]) -> datetime:
    return timestamp_value(item.get("publishedAt") or item.get("fetchedAt")) or datetime.min.replace(tzinfo=timezone.utc)


def filter_items_by_freshness(
    items: list[dict[str, Any]],
    now: datetime,
    max_item_age_hours: int,
    future_tolerance_hours: int,
    allow_undated: bool,
) -> tuple[list[dict[str, Any]], dict[str, Any]]:
    """Keep only items whose source publication time proves they are current."""
    if max_item_age_hours <= 0:
        raise ValueError("freshness.max_item_age_hours must be greater than zero")
    cutoff = now - timedelta(hours=max_item_age_hours)
    future_cutoff = now + timedelta(hours=max(0, future_tolerance_hours))
    accepted: list[dict[str, Any]] = []
    stale = 0
    undated = 0
    future = 0
    for item in items:
        published = timestamp_value(item.get("publishedAt"))
        if published is None:
            if allow_undated:
                accepted.append(item)
            else:
                undated += 1
            continue
        if published < cutoff:
            stale += 1
            continue
        if published > future_cutoff:
            future += 1
            continue
        accepted.append(item)
    accepted_dates = [timestamp_value(item.get("publishedAt")) for item in accepted]
    dated = [value for value in accepted_dates if value is not None]
    return accepted, {
        "inputItems": len(items),
        "acceptedItems": len(accepted),
        "staleItemsDropped": stale,
        "undatedItemsDropped": undated,
        "futureItemsDropped": future,
        "newestAcceptedAt": iso_utc(max(dated)) if dated else None,
        "oldestAcceptedAt": iso_utc(min(dated)) if dated else None,
    }


def summarize_freshness(
    items: list[dict[str, Any]],
    now: datetime,
    max_item_age_hours: int,
    future_tolerance_hours: int,
    allow_undated: bool,
    filter_reports: list[dict[str, Any]],
    sources_without_current_items: list[str] | None = None,
) -> dict[str, Any]:
    published_dates = [timestamp_value(item.get("publishedAt")) for item in items]
    dated = [value for value in published_dates if value is not None]
    return {
        "maxItemAgeHours": max_item_age_hours,
        "cutoffAt": iso_utc(now - timedelta(hours=max_item_age_hours)),
        "futureToleranceHours": future_tolerance_hours,
        "allowUndated": allow_undated,
        "staleItemsDropped": sum(int(report.get("staleItemsDropped", 0)) for report in filter_reports),
        "undatedItemsDropped": sum(int(report.get("undatedItemsDropped", 0)) for report in filter_reports),
        "futureItemsDropped": sum(int(report.get("futureItemsDropped", 0)) for report in filter_reports),
        "sourcesWithoutCurrentItems": sources_without_current_items or [],
        "newestItemAt": iso_utc(max(dated)) if dated else None,
        "oldestItemAt": iso_utc(min(dated)) if dated else None,
    }


def merge_items(
    fresh: list[dict[str, Any]],
    cached: list[dict[str, Any]],
    now: datetime,
    lookback_days: int,
    retention_days: int,
    max_items: int,
    max_items_per_source: int = 50,
) -> tuple[list[dict[str, Any]], int]:
    cached_by_id = {str(item.get("id")): item for item in cached if item.get("id")}
    cached_by_url = {canonical_url(str(item.get("url", ""))): item for item in cached if item.get("url")}
    for item in fresh:
        previous = cached_by_id.get(str(item.get("id"))) or cached_by_url.get(canonical_url(str(item.get("url", ""))))
        if previous and previous.get("article") and not item.get("article"):
            item["article"] = deepcopy(previous["article"])
            if (
                item.get("source", {}).get("updateTitleFromArticle")
                and previous["article"].get("pageTitle")
            ):
                item["title"] = clean_text(str(previous["article"]["pageTitle"]), 240)
        if previous and isinstance(previous.get("translation"), dict):
            translation = deepcopy(previous["translation"])
            fields = translation.get("fields", {})
            for field_name, container, key in translation_field_targets(item):
                cached_field = fields.get(field_name, {}) if isinstance(fields.get(field_name), dict) else {}
                current = str(container.get(key) or "")
                if current == str(cached_field.get("original") or "") and cached_field.get("translated"):
                    container[key] = cached_field["translated"]
                elif current != str(cached_field.get("translated") or ""):
                    fields.pop(field_name, None)
            if fields:
                item["translation"] = translation
    fresh_cutoff = now - timedelta(days=lookback_days)
    retention_cutoff = now - timedelta(days=retention_days)
    accepted_fresh = [
        item for item in fresh
        if not item.get("publishedAt") or date_value(item) >= fresh_cutoff
    ]
    retained_cached = [item for item in cached if date_value(item) >= retention_cutoff]
    merged = sorted(accepted_fresh + retained_cached, key=date_value, reverse=True)
    seen_urls: set[str] = set()
    seen_titles: set[str] = set()
    source_counts: dict[str, int] = {}
    unique: list[dict[str, Any]] = []
    fresh_ids = {item["id"] for item in accepted_fresh}
    new_ids: set[str] = set()
    cached_ids = {item.get("id") for item in cached}
    for item in merged:
        url_key = canonical_url(str(item.get("url", "")))
        name_key = title_key(str(item.get("title", "")))
        source_key = str(item.get("source", {}).get("id") or "unknown")
        if (url_key and url_key in seen_urls) or (name_key and name_key in seen_titles):
            continue
        if source_counts.get(source_key, 0) >= max_items_per_source:
            continue
        if url_key:
            seen_urls.add(url_key)
        if name_key:
            seen_titles.add(name_key)
        source_counts[source_key] = source_counts.get(source_key, 0) + 1
        unique.append(item)
        if item.get("id") in fresh_ids and item.get("id") not in cached_ids:
            new_ids.add(str(item["id"]))
        if len(unique) >= max_items:
            break
    return unique, len(new_ids)


def enrich_articles(
    items: list[dict[str, Any]],
    limit: int,
    concurrency: int,
    timeout: int,
    max_chars: int,
    min_chars: int,
    fetched_at: str,
    refresh_after_hours: int = 24,
) -> dict[str, int]:
    refresh_cutoff = (timestamp_value(fetched_at) or utc_now()) - timedelta(hours=max(0, refresh_after_hours))

    def needs_refresh(item: dict[str, Any]) -> bool:
        article = item.get("article", {})
        article_fetched_at = timestamp_value(article.get("fetchedAt"))
        return (
            article.get("status") != "ok"
            or int(article.get("extractorVersion", 0)) < ARTICLE_EXTRACTOR_VERSION
            or (refresh_after_hours > 0 and (article_fetched_at is None or article_fetched_at < refresh_cutoff))
        )

    all_candidates = [
        item for item in items
        if item.get("source", {}).get("fetchArticles", True)
        and item.get("url")
        and needs_refresh(item)
    ]
    candidates = all_candidates[:max(0, limit)]
    if not candidates:
        apply_article_titles(items)
        return {"attempted": 0, "fetched": 0, "failed": 0, "pending": len(all_candidates)}
    results: dict[str, dict[str, Any]] = {}
    with ThreadPoolExecutor(max_workers=max(1, min(concurrency, len(candidates)))) as pool:
        futures = {
            pool.submit(fetch_article, item, timeout, max_chars, min_chars, fetched_at): item
            for item in candidates
        }
        for future in as_completed(futures):
            item_id, article = future.result()
            results[item_id] = article
    for item in items:
        if str(item.get("id")) in results:
            result = results[str(item["id"])]
            previous = item.get("article", {})
            if result["status"] == "ok" or previous.get("status") != "ok":
                item["article"] = result
            else:
                previous["lastRefreshAttemptAt"] = fetched_at
                previous["refreshError"] = result.get("error")
    apply_article_titles(items)
    fetched = sum(1 for article in results.values() if article["status"] == "ok")
    pending = sum(
        1 for item in items
        if item.get("source", {}).get("fetchArticles", True) and item.get("url") and needs_refresh(item)
    )
    return {"attempted": len(candidates), "fetched": fetched, "failed": len(candidates) - fetched, "pending": pending}


def apply_article_titles(items: list[dict[str, Any]]) -> None:
    for item in items:
        article = item.get("article", {})
        if (
            item.get("source", {}).get("updateTitleFromArticle")
            and article.get("pageTitle")
        ):
            item["title"] = clean_text(str(article["pageTitle"]), 240)


def summary_counts(items: list[dict[str, Any]]) -> dict[str, int]:
    counts: dict[str, int] = {}
    for item in items:
        category = str(item.get("category") or "Technology")
        counts[category] = counts.get(category, 0) + 1
    return dict(sorted(counts.items(), key=lambda pair: (-pair[1], pair[0])))


def region_counts(items: list[dict[str, Any]]) -> dict[str, int]:
    counts = {"CN": 0, "Global": 0}
    for item in items:
        region = "CN" if item.get("source", {}).get("region") == "CN" else "Global"
        counts[region] += 1
    return counts


def write_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_suffix(path.suffix + ".tmp")
    with temporary.open("w", encoding="utf-8") as handle:
        json.dump(value, handle, ensure_ascii=False, indent=2)
        handle.write("\n")
    temporary.replace(path)


def build_dashboard(dataset: dict[str, Any], site_dir: Path) -> Path:
    template = TEMPLATE_PATH.read_text(encoding="utf-8")
    serialized = json.dumps(dataset, ensure_ascii=False).replace("</", "<\\/")
    if "__RADAR_DATA__" not in template:
        raise ValueError(f"dashboard template is missing __RADAR_DATA__: {TEMPLATE_PATH}")
    rendered = template.replace("__RADAR_DATA__", serialized)
    site_dir.mkdir(parents=True, exist_ok=True)
    output = site_dir / "index.html"
    temporary = output.with_suffix(".html.tmp")
    temporary.write_text(rendered, encoding="utf-8")
    temporary.replace(output)
    write_json(site_dir / "feed.json", dataset)
    return output


def update(args: argparse.Namespace) -> int:
    source_config = load_json(args.sources, {})
    settings = source_config.get("settings", {})
    sources = [source for source in source_config.get("sources", []) if source.get("enabled", True)]
    if not sources:
        raise ValueError(f"no enabled sources in {args.sources}")

    timeout = args.timeout or int(settings.get("request_timeout_seconds", 18))
    concurrency = args.concurrency or int(settings.get("concurrency", 6))
    lookback_days = args.lookback_days or int(settings.get("lookback_days", 21))
    retention_days = args.retention_days or int(settings.get("retention_days", 120))
    max_items = args.max_items or int(settings.get("max_items", 600))
    max_items_per_source = args.max_items_per_source or int(settings.get("max_items_per_source", 50))
    article_settings = settings.get("article_fetch", {})
    article_enabled = bool(article_settings.get("enabled", True)) and not args.skip_articles
    article_limit = args.article_limit if args.article_limit is not None else int(article_settings.get("max_articles_per_run", 30))
    article_concurrency = int(article_settings.get("concurrency", 4))
    article_timeout = args.article_timeout or int(article_settings.get("timeout_seconds", 12))
    article_max_chars = int(article_settings.get("max_content_chars", 20000))
    article_min_chars = int(article_settings.get("min_content_chars", 300))
    article_refresh_after_hours = int(article_settings.get("refresh_after_hours", 24))
    freshness_settings = settings.get("freshness", {})
    max_item_age_hours = (
        args.max_item_age_hours
        if args.max_item_age_hours is not None
        else int(freshness_settings.get("max_item_age_hours", 168))
    )
    future_tolerance_hours = int(freshness_settings.get("future_tolerance_hours", 6))
    allow_undated = bool(freshness_settings.get("allow_undated", False)) or args.allow_undated
    translation_settings = settings.get("translation", {})
    translation_enabled = bool(translation_settings.get("enabled", True)) and not args.skip_translation
    now = utc_now()
    fetched_at = iso_utc(now)
    fresh: list[dict[str, Any]] = []
    reports: list[dict[str, Any]] = []

    with ThreadPoolExecutor(max_workers=max(1, min(concurrency, len(sources)))) as pool:
        futures = {pool.submit(fetch_source, source, timeout, fetched_at): source for source in sources}
        for future in as_completed(futures):
            discovered, report = future.result()
            if report["status"] == "ok":
                current, source_freshness = filter_items_by_freshness(
                    discovered, now, max_item_age_hours, future_tolerance_hours, allow_undated
                )
                report["discoveredItemCount"] = report["itemCount"]
                report["itemCount"] = len(current)
                report["freshness"] = source_freshness
                fresh.extend(current)
            reports.append(report)

    reports.sort(key=lambda report: str(report["name"]).lower())
    data_path = args.data_dir / "items.json"
    cached_dataset = load_json(data_path, {"items": []})
    cached_items_raw = cached_dataset.get("items", []) if isinstance(cached_dataset, dict) else []
    cached_items, cached_freshness = filter_items_by_freshness(
        cached_items_raw, now, max_item_age_hours, future_tolerance_hours, allow_undated
    )
    success_count = sum(1 for report in reports if report["status"] == "ok")
    if success_count == 0 and not cached_items:
        errors = "; ".join(f"{report['name']}: {report['error']}" for report in reports)
        raise RuntimeError(f"all sources failed and no cached data is available: {errors}")

    items, new_count = merge_items(
        fresh, cached_items, now, lookback_days, retention_days, max_items, max_items_per_source
    )
    article_report = (
        enrich_articles(
            items, article_limit, article_concurrency, article_timeout,
            article_max_chars, article_min_chars, fetched_at, article_refresh_after_hours,
        )
        if article_enabled else {"attempted": 0, "fetched": 0, "failed": 0, "pending": 0}
    )
    translation_report = (
        translate_items(items, translation_settings, fetched_at)
        if translation_enabled else {
            "attemptedItems": 0, "translatedItems": 0, "failedItems": 0,
            "translatedFields": 0, "pendingItems": pending_translation_count(items), "errors": [],
        }
    )
    readable_articles = sum(1 for item in items if item.get("article", {}).get("status") == "ok")
    source_freshness_reports = [report.get("freshness", {}) for report in reports if report["status"] == "ok"]
    freshness_report = summarize_freshness(
        items, now, max_item_age_hours, future_tolerance_hours, allow_undated,
        [*source_freshness_reports, cached_freshness],
        [
            str(report["name"]) for report in reports
            if report["status"] == "ok" and int(report.get("itemCount", 0)) == 0
        ],
    )
    dataset = {
        "schemaVersion": 5,
        "generatedAt": fetched_at,
        "stats": {
            "totalItems": len(items),
            "newItems": new_count,
            "successfulSources": success_count,
            "failedSources": len(reports) - success_count,
            "readableArticles": readable_articles,
            "pendingArticles": article_report["pending"],
            "translatedItems": sum(1 for item in items if item.get("translation", {}).get("status") == "ok"),
            "pendingTranslations": translation_report["pendingItems"],
            "staleItemsDropped": freshness_report["staleItemsDropped"],
            "regions": region_counts(items),
            "categories": summary_counts(items),
        },
        "articleReport": article_report,
        "translationReport": translation_report,
        "freshnessReport": freshness_report,
        "sourceReports": reports,
        "items": items,
    }
    write_json(data_path, dataset)
    output = build_dashboard(dataset, args.site_dir)
    status = "ok" if (
        success_count == len(reports)
        and not article_report["failed"]
        and not article_report["pending"]
        and not translation_report["failedItems"]
        and not translation_report["pendingItems"]
    ) else "partial"
    print(json.dumps({"status": status, "output": str(output), **dataset["stats"], "freshness": freshness_report, "translation": translation_report}, ensure_ascii=False))
    return 0


def build(args: argparse.Namespace) -> int:
    data_path = args.data_dir / "items.json"
    dataset = load_json(data_path, None)
    if not dataset:
        raise FileNotFoundError(f"no cached dataset at {data_path}; run update first")
    output = build_dashboard(dataset, args.site_dir)
    print(json.dumps({"status": "ok", "output": str(output), "totalItems": len(dataset.get("items", []))}, ensure_ascii=False))
    return 0


def articles(args: argparse.Namespace) -> int:
    data_path = args.data_dir / "items.json"
    dataset = load_json(data_path, None)
    if not dataset:
        raise FileNotFoundError(f"no cached dataset at {data_path}; run update first")
    source_config = load_json(args.sources, {})
    settings = source_config.get("settings", {})
    article_settings = settings.get("article_fetch", {})
    freshness_settings = settings.get("freshness", {})
    max_item_age_hours = int(freshness_settings.get("max_item_age_hours", 168))
    future_tolerance_hours = int(freshness_settings.get("future_tolerance_hours", 6))
    allow_undated = bool(freshness_settings.get("allow_undated", False))
    source_map = {str(source["id"]): source for source in source_config.get("sources", [])}
    now = utc_now()
    items, freshness_filter = filter_items_by_freshness(
        dataset.get("items", []), now, max_item_age_hours, future_tolerance_hours, allow_undated
    )
    dataset["items"] = items
    for item in items:
        source = source_map.get(str(item.get("source", {}).get("id")), {})
        item_source = item.setdefault("source", {})
        item_source["region"] = "CN" if source.get("region") == "CN" else "Global"
        item_source["fetchArticles"] = bool(
            source.get("fetch_articles", item.get("source", {}).get("kind") != "research")
        )
        item_source["updateTitleFromArticle"] = bool(source.get("update_title_from_article", False))
    fetched_at = iso_utc(now)
    report = enrich_articles(
        items,
        args.article_limit if args.article_limit is not None else int(article_settings.get("max_articles_per_run", 30)),
        int(article_settings.get("concurrency", 4)),
        args.article_timeout or int(article_settings.get("timeout_seconds", 12)),
        int(article_settings.get("max_content_chars", 20000)),
        int(article_settings.get("min_content_chars", 300)),
        fetched_at,
        int(article_settings.get("refresh_after_hours", 24)),
    )
    translation_settings = settings.get("translation", {})
    translation_report = (
        translate_items(items, translation_settings, fetched_at)
        if bool(translation_settings.get("enabled", True)) and not args.skip_translation else {
            "attemptedItems": 0, "translatedItems": 0, "failedItems": 0,
            "translatedFields": 0, "pendingItems": pending_translation_count(items), "errors": [],
        }
    )
    freshness_report = summarize_freshness(
        items, now, max_item_age_hours, future_tolerance_hours, allow_undated, [freshness_filter]
    )
    dataset["schemaVersion"] = 5
    dataset["articlesUpdatedAt"] = fetched_at
    dataset["articleReport"] = report
    dataset["translationReport"] = translation_report
    dataset["freshnessReport"] = freshness_report
    dataset.setdefault("stats", {})["readableArticles"] = sum(
        1 for item in items if item.get("article", {}).get("status") == "ok"
    )
    dataset["stats"]["regions"] = region_counts(items)
    dataset["stats"]["translatedItems"] = sum(1 for item in items if item.get("translation", {}).get("status") == "ok")
    dataset["stats"]["pendingTranslations"] = translation_report["pendingItems"]
    dataset["stats"]["totalItems"] = len(items)
    dataset["stats"]["categories"] = summary_counts(items)
    dataset["stats"]["staleItemsDropped"] = freshness_report["staleItemsDropped"]
    write_json(data_path, dataset)
    output = build_dashboard(dataset, args.site_dir)
    status = "ok" if (
        not report["failed"] and not report["pending"]
        and not translation_report["failedItems"] and not translation_report["pendingItems"]
    ) else "partial"
    print(json.dumps({"status": status, "output": str(output), "readableArticles": dataset["stats"]["readableArticles"], **report, "freshness": freshness_report, "translation": translation_report}, ensure_ascii=False))
    return 0


def translate(args: argparse.Namespace) -> int:
    data_path = args.data_dir / "items.json"
    dataset = load_json(data_path, None)
    if not dataset:
        raise FileNotFoundError(f"no cached dataset at {data_path}; run update first")
    source_config = load_json(args.sources, {})
    root_settings = source_config.get("settings", {})
    settings = root_settings.get("translation", {})
    freshness_settings = root_settings.get("freshness", {})
    max_item_age_hours = int(freshness_settings.get("max_item_age_hours", 168))
    future_tolerance_hours = int(freshness_settings.get("future_tolerance_hours", 6))
    allow_undated = bool(freshness_settings.get("allow_undated", False))
    now = utc_now()
    items, freshness_filter = filter_items_by_freshness(
        dataset.get("items", []), now, max_item_age_hours, future_tolerance_hours, allow_undated
    )
    dataset["items"] = items
    translated_at = iso_utc(now)
    report = translate_items(items, settings, translated_at)
    freshness_report = summarize_freshness(
        items, now, max_item_age_hours, future_tolerance_hours, allow_undated, [freshness_filter]
    )
    dataset["schemaVersion"] = 5
    dataset["translationsUpdatedAt"] = translated_at
    dataset["translationReport"] = report
    dataset["freshnessReport"] = freshness_report
    dataset.setdefault("stats", {})["translatedItems"] = sum(
        1 for item in dataset.get("items", []) if item.get("translation", {}).get("status") == "ok"
    )
    dataset["stats"]["pendingTranslations"] = report["pendingItems"]
    dataset["stats"]["totalItems"] = len(items)
    dataset["stats"]["regions"] = region_counts(items)
    dataset["stats"]["categories"] = summary_counts(items)
    dataset["stats"]["staleItemsDropped"] = freshness_report["staleItemsDropped"]
    write_json(data_path, dataset)
    output = build_dashboard(dataset, args.site_dir)
    status = "ok" if not report["failedItems"] and not report["pendingItems"] else "partial"
    print(json.dumps({"status": status, "output": str(output), **report, "freshness": freshness_report}, ensure_ascii=False))
    return 0


def serve(args: argparse.Namespace) -> int:
    index_path = args.site_dir / "index.html"
    if not index_path.exists():
        build(args)
    handler = lambda *handler_args, **handler_kwargs: SimpleHTTPRequestHandler(  # noqa: E731
        *handler_args, directory=str(args.site_dir), **handler_kwargs
    )
    server = ThreadingHTTPServer((args.host, args.port), handler)
    print(f"AI Frontier Radar: http://{args.host}:{args.port}")
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        server.server_close()
    return 0


def parser() -> argparse.ArgumentParser:
    common = argparse.ArgumentParser(add_help=False)
    common.add_argument("--data-dir", type=Path, default=DEFAULT_DATA_DIR)
    common.add_argument("--site-dir", type=Path, default=DEFAULT_SITE_DIR)
    root = argparse.ArgumentParser(description=__doc__)
    subparsers = root.add_subparsers(dest="command", required=True)

    update_parser = subparsers.add_parser("update", parents=[common], help="fetch feeds and rebuild the dashboard")
    update_parser.add_argument("--sources", type=Path, default=DEFAULT_SOURCES)
    update_parser.add_argument("--timeout", type=int)
    update_parser.add_argument("--concurrency", type=int)
    update_parser.add_argument("--lookback-days", type=int)
    update_parser.add_argument("--retention-days", type=int)
    update_parser.add_argument("--max-items", type=int)
    update_parser.add_argument("--max-items-per-source", type=int)
    update_parser.add_argument("--max-item-age-hours", type=int, help="hard freshness window for published items")
    update_parser.add_argument("--allow-undated", action="store_true", help="retain entries without a valid publication time")
    update_parser.add_argument("--article-limit", type=int, help="maximum new article bodies to fetch")
    update_parser.add_argument("--article-timeout", type=int, help="timeout for each article page")
    update_parser.add_argument("--skip-articles", action="store_true", help="refresh feeds without fetching article bodies")
    update_parser.add_argument("--skip-translation", action="store_true", help="skip English-to-Chinese translation")
    update_parser.set_defaults(handler=update)

    build_parser = subparsers.add_parser("build", parents=[common], help="rebuild from cached data")
    build_parser.set_defaults(handler=build)

    articles_parser = subparsers.add_parser("articles", parents=[common], help="fetch article bodies for cached entries")
    articles_parser.add_argument("--sources", type=Path, default=DEFAULT_SOURCES)
    articles_parser.add_argument("--article-limit", type=int, help="maximum article bodies to fetch")
    articles_parser.add_argument("--article-timeout", type=int, help="timeout for each article page")
    articles_parser.add_argument("--skip-translation", action="store_true", help="skip English-to-Chinese translation")
    articles_parser.set_defaults(handler=articles)

    translate_parser = subparsers.add_parser("translate", parents=[common], help="translate cached English content into Chinese")
    translate_parser.add_argument("--sources", type=Path, default=DEFAULT_SOURCES)
    translate_parser.set_defaults(handler=translate)

    serve_parser = subparsers.add_parser("serve", parents=[common], help="serve the generated dashboard")
    serve_parser.add_argument("--host", default="127.0.0.1")
    serve_parser.add_argument("--port", type=int, default=8765)
    serve_parser.set_defaults(handler=serve)
    return root


def main() -> int:
    args = parser().parse_args()
    try:
        return int(args.handler(args))
    except Exception as exc:
        print(json.dumps({"status": "error", "error": f"{type(exc).__name__}: {exc}"}, ensure_ascii=False), file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
