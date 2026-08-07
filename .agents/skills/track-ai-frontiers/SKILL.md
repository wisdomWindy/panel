---
name: track-ai-frontiers
description: Discover current domestic Chinese and international AI ideas, articles, research, engineering techniques, model releases, tools, and industry updates from configurable primary sources; bypass network caches, enforce a publication-time freshness window, refresh cached article bodies, translate fetched English content into Simplified Chinese, normalize and deduplicate results, generate a searchable static dashboard, and install a daily refresh schedule. Use when Codex needs an up-to-date Chinese-language China-and-global AI frontier/news/research/model intelligence page, article reader, or daily AI radar.
---

# Track AI Frontiers

Build a current domestic-and-international AI intelligence feed from primary sources, enforce publication-time freshness before caching, translate English content into Simplified Chinese, and render it as a local static dashboard.

## Workflow

1. Inspect `references/sources.json` before fetching. Add or disable sources there when the user names preferred organizations or topics.
2. Ensure the environment variable named by `settings.translation.api_key_env` is available. Never write its value into the Skill or generated data.
3. Run a first refresh:

   ```bash
   rtk python3 scripts/ai_radar.py update
   ```

4. Fetch feeds and article pages with cache-bypass headers. Apply `settings.freshness` before cache merging; do not use an old fetch timestamp as proof that old source content is current.
5. Fetch readable article text for eligible entries and refresh cached bodies after `article_fetch.refresh_after_hours`. Treat RSS/Atom as discovery metadata, not as the final reading content.
6. Translate every predominantly English title, summary, extracted page title, and readable article body into Simplified Chinese before writing the dashboard. Retry cached translation failures with `rtk python3 scripts/ai_radar.py translate`.
7. Verify `freshnessReport`: report the cutoff, newest retained publication time, stale/undated/future drop counts, and sources without current entries. Also report source, article, and translation failures. Never call `build` a refresh because it performs no network requests.
8. Serve the dashboard when the user wants to view it:

   ```bash
   rtk python3 scripts/ai_radar.py serve --port 8765
   ```

9. Install daily refresh only when the user requests scheduling or approves the system change:

   ```bash
   rtk python3 scripts/schedule.py install --time 08:00
   ```

10. Verify the schedule with `rtk python3 scripts/schedule.py status`.

Use paths relative to this Skill directory, or pass explicit `--sources`, `--data-dir`, and `--site-dir` arguments when generating an independent dashboard.

## Data Rules

- Normalize source-specific RSS/Atom values in `ai_radar.py` before writing the page data.
- Send cache-bypass headers on every feed and article request. Filter on the source `publishedAt`, never `fetchedAt`, using `freshness.max_item_age_hours` as a hard limit.
- Drop undated entries by default because their freshness cannot be proven. Drop entries older than the cutoff and timestamps beyond the configured future tolerance before merging with cache, fetching articles, translating, or rendering.
- Apply the same freshness gate to cached data in `update`, `articles`, and `translate`. Cached content may enrich a current entry with the same identity, but must never keep an expired entry visible.
- Extract article text in the data adapter and store it under `item.article`. Never scrape or repair article semantics in the dashboard template.
- Translate English content in the data adapter after article extraction. Write Chinese into `item.title`, `item.summary`, `item.article.pageTitle`, and `item.article.content`; retain exact English originals and translator metadata under `item.translation.fields`. Never translate or repair content in the dashboard template.
- Preserve product names, model names, code, URLs, numbers, Markdown, paragraph boundaries, and factual meaning. Do not summarize, expand, or infer during translation.
- Keep a previously cached successful translation when its English source text is unchanged. Keep failed fields retryable and report their item IDs, field names, and errors.
- Prefer `<article>`, then `<main>`, and use `<body>` only as a fallback. Ignore navigation, scripts, styles, headers, footers, forms, and sidebars.
- Keep a previously cached successful article body when a later refresh or article request fails.
- Re-fetch successful cached article bodies after `article_fetch.refresh_after_hours`. Record refresh failures and pending refreshes; do not silently present an old body as newly refreshed.
- Treat missing or invalid titles, URLs, dates, authors, categories, and summaries as adapter concerns. Never patch those semantics in the dashboard template.
- Prefer official lab, vendor, repository, and research feeds. Clearly label community sources.
- Assign every source `CN` or `Global` in the adapter. Keep region semantics out of view-only fallback logic.
- Maintain usable sources from both regions; report a regional gap when one region returns no retained items.
- Preserve prior data if individual sources fail. Fail the update only when every source fails and there is no usable cached dataset.
- Deduplicate by canonical URL first and normalized title second.
- Keep summaries factual. Do not infer claims that are absent from the source entry.
- Never commit secrets. Sources requiring credentials must read environment variable names from configuration, not literal tokens.

## Commands

```bash
# Refresh data and rebuild the standalone page
rtk python3 scripts/ai_radar.py update

# Use a stricter 72-hour freshness window for this refresh
rtk python3 scripts/ai_radar.py update --max-item-age-hours 72

# Refresh only discovery metadata without requesting article pages
rtk python3 scripts/ai_radar.py update --skip-articles

# Fetch or repair article bodies from the cached discovery list
rtk python3 scripts/ai_radar.py articles --article-limit 30

# Translate or retry all cached English content
rtk python3 scripts/ai_radar.py translate

# Build the page from cached data without network access
rtk python3 scripts/ai_radar.py build

# Preview the page locally
rtk python3 scripts/ai_radar.py serve --port 8765

# Install, inspect, or remove the daily user-level task
rtk python3 scripts/schedule.py install --time 08:00
rtk python3 scripts/schedule.py status
rtk python3 scripts/schedule.py uninstall
```

Read `references/configuration.md` when customizing sources, retention, output locations, or scheduling behavior.

## Output Contract

Return:

- dashboard URL or absolute `site/index.html` path;
- refresh timestamp and source success/failure counts;
- freshness cutoff, newest/oldest retained publication times, and stale/undated/future drop counts;
- exact source names that returned no entries inside the freshness window;
- readable article count and current article-fetch success/failure/pending counts;
- translated item count, pending translation count, and current translation success/failure counts;
- item totals by category;
- scheduler status when scheduling was requested;
- exact failed source names and errors, if any;
- exact failed translation item IDs, fields, and errors, if any.
