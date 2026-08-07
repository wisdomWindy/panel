#!/usr/bin/env python3
"""Focused tests for the AI Frontier Radar data adapter and page builder."""

from __future__ import annotations

import importlib.util
import io
import json
import tempfile
import unittest
from contextlib import redirect_stdout
from datetime import datetime, timezone
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import patch


MODULE_PATH = Path(__file__).with_name("ai_radar.py")
SPEC = importlib.util.spec_from_file_location("ai_radar", MODULE_PATH)
assert SPEC and SPEC.loader
ai_radar = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(ai_radar)


RSS = b"""<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0"><channel><title>Test Feed</title>
  <item><title>New reasoning model released</title><link>https://example.com/post?utm_source=test</link>
  <description><![CDATA[<p>A technical model release with inference improvements.</p>]]></description>
  <pubDate>Wed, 05 Aug 2026 09:00:00 GMT</pubDate><author>Research Team</author></item>
</channel></rss>"""

ATOM = b"""<?xml version="1.0" encoding="utf-8"?>
<feed xmlns="http://www.w3.org/2005/Atom"><title>Test Atom</title>
  <entry><title>Agent workflow techniques</title><link href="https://example.com/agent"/>
  <updated>2026-08-06T01:00:00Z</updated><summary>Tool use and MCP engineering.</summary></entry>
</feed>"""

ARTICLE_HTML = """<!doctype html><html><head><meta property="og:title" content="Original Article Title"></head><body>
<nav>Navigation that must not be included.</nav>
<main><article><h1>Readable AI Article</h1>
<p>This is the first substantial paragraph about a new reasoning architecture and its design.</p>
<p>This is the second paragraph with implementation details, evaluation results, and limitations.</p>
<script>secret_navigation_text()</script>
<p>This final paragraph explains practical deployment guidance for engineering teams.</p>
</article></main><footer>Footer that must not be included.</footer>
</body></html>"""

SITEMAP = b"""<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url><loc>https://example.cn/news/model-update</loc><lastmod>2026-08-05</lastmod></url>
  <url><loc>https://example.cn/docs/api</loc><lastmod>2026-08-04</lastmod></url>
</urlset>"""


class RadarTests(unittest.TestCase):
    def source(self) -> dict[str, str]:
        return {"id": "test", "name": "Test Lab", "kind": "official", "default_category": "Technology"}

    def test_parse_rss_normalizes_fields(self) -> None:
        items = ai_radar.parse_feed(RSS, self.source(), "2026-08-06T02:00:00Z")
        self.assertEqual(len(items), 1)
        self.assertEqual(items[0]["url"], "https://example.com/post")
        self.assertEqual(items[0]["category"], "Models")
        self.assertEqual(items[0]["summary"], "A technical model release with inference improvements.")
        self.assertEqual(items[0]["publishedAt"], "2026-08-05T09:00:00Z")
        self.assertEqual(items[0]["source"]["region"], "Global")

    def test_parse_atom_and_classify_agent(self) -> None:
        items = ai_radar.parse_feed(ATOM, self.source(), "2026-08-06T02:00:00Z")
        self.assertEqual(items[0]["category"], "Agents")
        self.assertEqual(items[0]["url"], "https://example.com/agent")

    def test_detects_predominantly_english_content(self) -> None:
        self.assertTrue(ai_radar.contains_english_text("A new reasoning model is now available."))
        self.assertFalse(ai_radar.contains_english_text("全新的 GPT 模型现已发布"))
        self.assertFalse(ai_radar.contains_english_text("OpenAI 发布 GPT-5 推理模型"))

    def test_translate_items_normalizes_content_and_retains_originals(self) -> None:
        items = ai_radar.parse_feed(RSS, self.source(), "2026-08-06T02:00:00Z")
        items[0]["article"] = {
            "status": "ok", "pageTitle": "Original Article Title",
            "content": "This article explains a new model architecture in practical detail.",
        }
        translations = {
            items[0]["title"]: "全新推理模型发布",
            items[0]["summary"]: "一次包含推理改进的技术模型发布。",
            items[0]["article"]["pageTitle"]: "原始文章标题",
            items[0]["article"]["content"]: "本文详细介绍了一种新的模型架构及其实践方法。",
        }
        report = ai_radar.translate_items(
            items, {"provider": "openai-compatible", "concurrency": 1}, "2026-08-06T03:00:00Z",
            translator=lambda value: translations[value],
        )
        self.assertEqual(report["translatedItems"], 1)
        self.assertEqual(report["pendingItems"], 0)
        self.assertEqual(items[0]["title"], "全新推理模型发布")
        self.assertEqual(items[0]["article"]["content"], "本文详细介绍了一种新的模型架构及其实践方法。")
        self.assertEqual(items[0]["article"]["characterCount"], len("本文详细介绍了一种新的模型架构及其实践方法。"))
        self.assertEqual(items[0]["translation"]["fields"]["title"]["original"], "New reasoning model released")

    def test_translate_items_reports_missing_key_once(self) -> None:
        items = ai_radar.parse_feed(RSS, self.source(), "2026-08-06T02:00:00Z")
        report = ai_radar.translate_items(
            items, {"provider": "openai-compatible", "api_key_env": "MISSING_TEST_TRANSLATION_KEY"},
            "2026-08-06T03:00:00Z",
        )
        self.assertEqual(report["failedItems"], 1)
        self.assertEqual(len(report["errors"]), 1)
        self.assertEqual(report["errors"][0]["itemId"], "configuration")

    def test_translate_items_reuses_cached_translation(self) -> None:
        items = ai_radar.parse_feed(RSS, self.source(), "2026-08-06T02:00:00Z")
        calls: list[str] = []
        ai_radar.translate_items(
            items, {"provider": "openai-compatible", "concurrency": 1}, "2026-08-06T03:00:00Z",
            translator=lambda value: calls.append(value) or "这是缓存的中文翻译内容。",
        )
        first_call_count = len(calls)
        report = ai_radar.translate_items(
            items, {"provider": "openai-compatible", "concurrency": 1}, "2026-08-06T04:00:00Z",
            translator=lambda value: calls.append(value) or "不应再次调用。",
        )
        self.assertEqual(len(calls), first_call_count)
        self.assertEqual(report["attemptedItems"], 0)

    def test_parse_sitemap_filters_urls_and_marks_domestic_region(self) -> None:
        source = {
            "id": "cn-news", "name": "国内官方动态", "kind": "official", "region": "CN",
            "format": "sitemap", "include_patterns": ["/news/"], "default_category": "Models",
            "update_title_from_article": True,
        }
        items = ai_radar.parse_feed(SITEMAP, source, "2026-08-06T02:00:00Z")
        self.assertEqual(len(items), 1)
        self.assertEqual(items[0]["source"]["region"], "CN")
        self.assertTrue(items[0]["source"]["updateTitleFromArticle"])
        self.assertEqual(items[0]["publishedAt"], "2026-08-05T00:00:00Z")

    def test_extract_article_prefers_article_and_ignores_navigation(self) -> None:
        content, truncated = ai_radar.extract_article_text(ARTICLE_HTML, 1000)
        self.assertIn("Readable AI Article", content)
        self.assertIn("implementation details", content)
        self.assertNotIn("Navigation", content)
        self.assertNotIn("secret_navigation_text", content)
        self.assertFalse(truncated)

    def test_fetch_article_from_html_page(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "article.html"
            path.write_text(ARTICLE_HTML, encoding="utf-8")
            item_id, result = ai_radar.fetch_article(
                {"id": "article-1", "url": path.as_uri()}, 2, 1000, 120, "2026-08-06T02:00:00Z"
            )
        self.assertEqual(item_id, "article-1")
        self.assertEqual(result["status"], "ok")
        self.assertGreater(result["characterCount"], 120)
        self.assertEqual(result["pageTitle"], "Original Article Title")

    def test_freshness_filter_drops_stale_undated_and_future_items(self) -> None:
        items = [
            {"id": "current", "publishedAt": "2026-08-06T10:00:00Z"},
            {"id": "stale", "publishedAt": "2026-07-29T10:00:00Z"},
            {"id": "undated", "publishedAt": None, "fetchedAt": "2026-08-06T12:00:00Z"},
            {"id": "future", "publishedAt": "2026-08-07T02:00:00Z"},
        ]
        accepted, report = ai_radar.filter_items_by_freshness(
            items, datetime(2026, 8, 6, 12, tzinfo=timezone.utc), 168, 6, False
        )
        self.assertEqual([item["id"] for item in accepted], ["current"])
        self.assertEqual(report["staleItemsDropped"], 1)
        self.assertEqual(report["undatedItemsDropped"], 1)
        self.assertEqual(report["futureItemsDropped"], 1)

    def test_freshness_filter_can_explicitly_allow_undated_items(self) -> None:
        items = [{"id": "undated", "publishedAt": None, "fetchedAt": "2020-01-01T00:00:00Z"}]
        accepted, report = ai_radar.filter_items_by_freshness(
            items, datetime(2026, 8, 6, 12, tzinfo=timezone.utc), 24, 6, True
        )
        self.assertEqual(accepted, items)
        self.assertEqual(report["undatedItemsDropped"], 0)

    def test_article_refresh_failure_preserves_last_successful_body(self) -> None:
        item = {
            "id": "article-1", "url": "https://example.com/article",
            "source": {"fetchArticles": True},
            "article": {
                "status": "ok", "content": "Last successful body", "pageTitle": "Cached title",
                "extractorVersion": ai_radar.ARTICLE_EXTRACTOR_VERSION,
                "fetchedAt": "2026-08-01T00:00:00Z",
            },
        }
        failed = {
            "status": "error", "content": None, "pageTitle": None, "characterCount": 0,
            "truncated": False, "fetchedAt": "2026-08-06T12:00:00Z", "durationMs": 1,
            "error": "TimeoutError: refresh failed",
        }
        with patch.object(ai_radar, "fetch_article", return_value=("article-1", failed)):
            report = ai_radar.enrich_articles(
                [item], 1, 1, 2, 1000, 100, "2026-08-06T12:00:00Z", 24
            )
        self.assertEqual(item["article"]["status"], "ok")
        self.assertEqual(item["article"]["content"], "Last successful body")
        self.assertEqual(item["article"]["refreshError"], "TimeoutError: refresh failed")
        self.assertEqual(report["failed"], 1)
        self.assertEqual(report["pending"], 1)

    def test_update_writes_only_current_items_and_freshness_report(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            feed_path = root / "feed.xml"
            feed_path.write_bytes(RSS)
            sources_path = root / "sources.json"
            sources_path.write_text(json.dumps({
                "settings": {
                    "freshness": {
                        "max_item_age_hours": 48, "future_tolerance_hours": 6, "allow_undated": False,
                    },
                    "article_fetch": {"enabled": False},
                    "translation": {"enabled": False},
                },
                "sources": [{
                    "id": "test", "name": "Test Lab", "url": feed_path.as_uri(),
                    "kind": "official", "region": "Global", "default_category": "Models",
                }],
            }), encoding="utf-8")
            args = SimpleNamespace(
                sources=sources_path, data_dir=root / "data", site_dir=root / "site",
                timeout=None, concurrency=None, lookback_days=None, retention_days=None,
                max_items=None, max_items_per_source=None, max_item_age_hours=None,
                allow_undated=False, article_limit=None, article_timeout=None,
                skip_articles=True, skip_translation=True,
            )
            with patch.object(ai_radar, "utc_now", return_value=datetime(2026, 8, 6, 12, tzinfo=timezone.utc)):
                with redirect_stdout(io.StringIO()):
                    result = ai_radar.update(args)
            dataset = json.loads((root / "data" / "items.json").read_text(encoding="utf-8"))
        self.assertEqual(result, 0)
        self.assertEqual(dataset["schemaVersion"], 5)
        self.assertEqual([item["id"] for item in dataset["items"]], [ai_radar.parse_feed(RSS, self.source(), "2026-08-06T12:00:00Z")[0]["id"]])
        self.assertEqual(dataset["freshnessReport"]["maxItemAgeHours"], 48)
        self.assertEqual(dataset["freshnessReport"]["newestItemAt"], "2026-08-05T09:00:00Z")

    def test_merge_deduplicates_url_and_title(self) -> None:
        fresh = ai_radar.parse_feed(RSS, self.source(), "2026-08-06T02:00:00Z")
        cached = [dict(fresh[0], id="cached-id", url="https://example.com/post?utm_medium=email")]
        merged, new_count = ai_radar.merge_items(
            fresh, cached, datetime(2026, 8, 6, tzinfo=timezone.utc), 21, 120, 50
        )
        self.assertEqual(len(merged), 1)
        self.assertEqual(new_count, 1)

    def test_merge_preserves_cached_article_body(self) -> None:
        fresh = ai_radar.parse_feed(RSS, self.source(), "2026-08-06T02:00:00Z")
        fresh[0]["source"]["updateTitleFromArticle"] = True
        cached = [dict(fresh[0], article={
            "status": "ok", "content": "Cached readable body", "pageTitle": "Original cached title",
        })]
        merged, _ = ai_radar.merge_items(
            fresh, cached, datetime(2026, 8, 6, tzinfo=timezone.utc), 21, 120, 50
        )
        self.assertEqual(merged[0]["article"]["content"], "Cached readable body")
        self.assertEqual(merged[0]["title"], "Original cached title")

    def test_merge_reuses_cached_translation_when_original_is_unchanged(self) -> None:
        cached = ai_radar.parse_feed(RSS, self.source(), "2026-08-06T02:00:00Z")
        ai_radar.translate_items(
            cached, {"provider": "openai-compatible", "concurrency": 1}, "2026-08-06T03:00:00Z",
            translator=lambda value: "全新推理模型发布" if "reasoning model" in value else "技术模型发布与推理改进。",
        )
        fresh = ai_radar.parse_feed(RSS, self.source(), "2026-08-06T04:00:00Z")
        merged, _ = ai_radar.merge_items(
            fresh, cached, datetime(2026, 8, 6, 4, tzinfo=timezone.utc), 21, 120, 50
        )
        calls: list[str] = []
        report = ai_radar.translate_items(
            merged, {"provider": "openai-compatible", "concurrency": 1}, "2026-08-06T05:00:00Z",
            translator=lambda value: calls.append(value) or "不应重新翻译。",
        )
        self.assertEqual(merged[0]["title"], "全新推理模型发布")
        self.assertEqual(calls, [])
        self.assertEqual(report["attemptedItems"], 0)

    def test_merge_caps_each_source(self) -> None:
        fresh = []
        for index in range(4):
            fresh.append({
                "id": f"a-{index}", "title": f"Item {index}", "url": f"https://a.example/{index}",
                "publishedAt": "2026-08-06T01:00:00Z", "fetchedAt": "2026-08-06T02:00:00Z",
                "category": "Research", "source": {"id": "a", "name": "A", "kind": "research"},
            })
        fresh.append({
            "id": "b-1", "title": "Different source", "url": "https://b.example/1",
            "publishedAt": "2026-08-06T00:30:00Z", "fetchedAt": "2026-08-06T02:00:00Z",
            "category": "Models", "source": {"id": "b", "name": "B", "kind": "official"},
        })
        merged, _ = ai_radar.merge_items(
            fresh, [], datetime(2026, 8, 6, tzinfo=timezone.utc), 21, 120, 50, 2
        )
        self.assertEqual([item["source"]["id"] for item in merged], ["a", "a", "b"])

    def test_build_dashboard_embeds_data(self) -> None:
        dataset = {"generatedAt": "2026-08-06T02:00:00Z", "stats": {}, "sourceReports": [], "items": []}
        with tempfile.TemporaryDirectory() as directory:
            output = ai_radar.build_dashboard(dataset, Path(directory))
            page = output.read_text(encoding="utf-8")
            self.assertIn("AI Frontier Radar", page)
            self.assertNotIn("__RADAR_DATA__", page)
            self.assertEqual(json.loads((Path(directory) / "feed.json").read_text(encoding="utf-8"))["items"], [])

    def test_region_counts_normalizes_unknown_to_global(self) -> None:
        counts = ai_radar.region_counts([
            {"source": {"region": "CN"}}, {"source": {"region": "Global"}}, {"source": {}},
        ])
        self.assertEqual(counts, {"CN": 1, "Global": 2})


if __name__ == "__main__":
    unittest.main()
