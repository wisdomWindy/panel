#!/usr/bin/env python3
"""Focused tests for the AI Frontier Radar data adapter and page builder."""

from __future__ import annotations

import importlib.util
import json
import tempfile
import unittest
from datetime import datetime, timezone
from pathlib import Path


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
