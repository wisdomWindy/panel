# Configuration Reference

## Source schema

Edit `sources.json` to manage feeds. Each source supports:

- `id`: stable lowercase identifier used for deduplication and filters.
- `name`: display name.
- `url`: RSS or Atom URL. Local `file://` URLs are useful for tests.
- `kind`: `official`, `research`, or `community`.
- `region`: `CN` for domestic Chinese sources or `Global` for international sources.
- `format`: optional parser type; omit for RSS/Atom or use `sitemap` for an XML sitemap.
- `include_patterns`: optional URL substrings used to keep only relevant sitemap entries.
- `default_category`: fallback category when keywords do not provide a stronger match.
- `fetch_articles`: optional boolean; disable it for high-volume indexes or sources whose feed summary already is the desired content.
- `update_title_from_article`: replace a sitemap-derived placeholder title with the original page title after article extraction.
- `enabled`: optional boolean; set to `false` to skip a source.
- `headers`: optional non-secret HTTP headers.

Do not put API tokens in `headers`. Add credential-aware fetching to the adapter and read tokens from environment variables when a private source is required.

## Settings

- `lookback_days`: secondary merge window for newly discovered entries. The stricter `freshness.max_item_age_hours` limit always wins.
- `retention_days`: secondary cache-retention limit. Cached items outside the freshness window are removed even when this value is larger.
- `max_items`: cap the merged dataset after sorting newest first.
- `max_items_per_source`: cap retained entries from any one source so high-volume feeds cannot crowd out the rest.
- `request_timeout_seconds`: timeout for each source request.
- `concurrency`: maximum parallel requests.
- `freshness.max_item_age_hours`: hard publication-age limit for feed, sitemap, and cached entries. The default is 168 hours (7 days).
- `freshness.future_tolerance_hours`: reject timestamps farther into the future than this clock-skew allowance.
- `freshness.allow_undated`: retain entries without a valid source publication time. Keep this `false` when freshness must be provable.
- `article_fetch.enabled`: fetch readable text from discovered article pages after the feed refresh.
- `article_fetch.max_articles_per_run`: cap uncached article pages fetched during one run.
- `article_fetch.concurrency`: maximum parallel article page requests.
- `article_fetch.timeout_seconds`: timeout for each article page.
- `article_fetch.refresh_after_hours`: re-fetch a previously cached successful article after this interval. A failed refresh keeps the last successful body but is reported as pending and failed.
- `article_fetch.min_content_chars`: reject navigation shells and blocked pages shorter than this value.
- `article_fetch.max_content_chars`: cap cached text per article; the item records whether it was truncated.
- `translation.enabled`: translate predominantly English titles, summaries, page titles, and cached article bodies into Simplified Chinese during `update` and `articles`.
- `translation.provider`: translation adapter; currently use `openai-compatible`.
- `translation.api_key_env`: environment variable containing the translation API key. Keep the key itself out of configuration.
- `translation.base_url_env` / `translation.model_env`: optional environment-variable overrides for the compatible endpoint and model.
- `translation.base_url` / `translation.model`: defaults used when the override environment variables are unset.
- `translation.concurrency`: maximum items translated concurrently.
- `translation.timeout_seconds`: timeout for each translation request.
- `translation.max_chars_per_request`: split long article bodies into requests no longer than this many characters.

Command-line flags override these settings where available.

Feed and article requests send `Cache-Control: no-cache, no-store, max-age=0` and `Pragma: no-cache`. `update` applies the freshness gate before cache merging, article fetching, translation, and dashboard generation. The dataset records the cutoff, newest and oldest retained publication times, dropped stale/undated/future item counts, and sources that returned no current entries under `freshnessReport`.

Use `rtk python3 scripts/ai_radar.py articles --article-limit 30` to fill or rebuild readable article bodies without refreshing every RSS/Atom source.

Set the environment variable named by `translation.api_key_env` before refreshing. Use `rtk python3 scripts/ai_radar.py translate` to translate or retry English content already in the cache. Successful translations replace the normalized display fields while `item.translation.fields` retains the original English text and translation metadata. Translation failures leave the original value intact, remain retryable, and make the command report `partial`.

## Runtime outputs

For a project-local Skill, runtime files default to `ai-frontier-radar/` at the repository root so the installed Skill can remain read-only. Set `AI_FRONTIER_RADAR_HOME` to override the runtime root, or pass explicit command-line paths.

- `ai-frontier-radar/data/items.json`: normalized cached dataset, Chinese translations with retained English originals, readable article text, and latest source reports.
- `ai-frontier-radar/site/feed.json`: copy of the normalized dataset for integrations.
- `ai-frontier-radar/site/index.html`: standalone dashboard with embedded data; it can be opened directly or served over HTTP.
- `ai-frontier-radar/data/logs/`: scheduler stdout and stderr.

## Scheduling

`schedule.py` installs a user-level job:

- macOS: `~/Library/LaunchAgents/com.codex.track-ai-frontiers.plist`
- Linux: `~/.config/systemd/user/track-ai-frontiers.service` and `.timer`

The installed task runs `ai_radar.py update` with the current Python interpreter and absolute Skill paths. Reinstall after moving the Skill directory or changing the desired time.
