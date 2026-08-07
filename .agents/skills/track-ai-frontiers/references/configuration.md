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

- `lookback_days`: accept newly fetched entries this many days into the past. Entries without parseable dates are retained and marked with a missing date.
- `retention_days`: keep cached entries for this many days.
- `max_items`: cap the merged dataset after sorting newest first.
- `max_items_per_source`: cap retained entries from any one source so high-volume feeds cannot crowd out the rest.
- `request_timeout_seconds`: timeout for each source request.
- `concurrency`: maximum parallel requests.
- `article_fetch.enabled`: fetch readable text from discovered article pages after the feed refresh.
- `article_fetch.max_articles_per_run`: cap uncached article pages fetched during one run.
- `article_fetch.concurrency`: maximum parallel article page requests.
- `article_fetch.timeout_seconds`: timeout for each article page.
- `article_fetch.min_content_chars`: reject navigation shells and blocked pages shorter than this value.
- `article_fetch.max_content_chars`: cap cached text per article; the item records whether it was truncated.

Command-line flags override these settings where available.

Use `python3 scripts/ai_radar.py articles --article-limit 30` to fill or rebuild readable article bodies without refreshing every RSS/Atom source.

## Runtime outputs

For a project-local Skill, runtime files default to `ai-frontier-radar/` at the repository root so the installed Skill can remain read-only. Set `AI_FRONTIER_RADAR_HOME` to override the runtime root, or pass explicit command-line paths.

- `ai-frontier-radar/data/items.json`: normalized cached dataset, readable article text, and latest source reports.
- `ai-frontier-radar/site/feed.json`: copy of the normalized dataset for integrations.
- `ai-frontier-radar/site/index.html`: standalone dashboard with embedded data; it can be opened directly or served over HTTP.
- `ai-frontier-radar/data/logs/`: scheduler stdout and stderr.

## Scheduling

`schedule.py` installs a user-level job:

- macOS: `~/Library/LaunchAgents/com.codex.track-ai-frontiers.plist`
- Linux: `~/.config/systemd/user/track-ai-frontiers.service` and `.timer`

The installed task runs `ai_radar.py update` with the current Python interpreter and absolute Skill paths. Reinstall after moving the Skill directory or changing the desired time.
