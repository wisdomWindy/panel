---
name: track-ai-frontiers
description: Discover recent domestic Chinese and international AI ideas, articles, research, engineering techniques, model releases, tools, and industry updates from configurable primary sources; fetch and cache readable article text; normalize, region-label, and deduplicate results; generate a searchable static reading dashboard; and install a daily user-level refresh schedule. Use when Codex needs a China-and-global AI frontier/news/research/model intelligence page, article reader, or daily AI radar.
---

# Track AI Frontiers

Build a durable domestic-and-international AI intelligence feed from primary sources, cache readable article bodies, and render it as a local static dashboard.

## Workflow

1. Inspect `references/sources.json` before fetching. Add or disable sources there when the user names preferred organizations or topics.
2. Run a first refresh:

   ```bash
   python3 scripts/ai_radar.py update
   ```

3. Fetch readable article text for eligible uncached entries. Treat RSS/Atom as discovery metadata, not as the final reading content.
4. Report the number of successful sources, failed sources, new items, readable articles, and total retained items. Do not describe a partial refresh as complete without mentioning failed sources.
5. Serve the dashboard when the user wants to view it:

   ```bash
   python3 scripts/ai_radar.py serve --port 8765
   ```

6. Install daily refresh only when the user requests scheduling or approves the system change:

   ```bash
   python3 scripts/schedule.py install --time 08:00
   ```

7. Verify the schedule with `python3 scripts/schedule.py status`.

Use paths relative to this Skill directory, or pass explicit `--sources`, `--data-dir`, and `--site-dir` arguments when generating an independent dashboard.

## Data Rules

- Normalize source-specific RSS/Atom values in `ai_radar.py` before writing the page data.
- Extract article text in the data adapter and store it under `item.article`. Never scrape or repair article semantics in the dashboard template.
- Prefer `<article>`, then `<main>`, and use `<body>` only as a fallback. Ignore navigation, scripts, styles, headers, footers, forms, and sidebars.
- Keep a previously cached successful article body when a later refresh or article request fails.
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
python3 scripts/ai_radar.py update

# Refresh only discovery metadata without requesting article pages
python3 scripts/ai_radar.py update --skip-articles

# Fetch or repair article bodies from the cached discovery list
python3 scripts/ai_radar.py articles --article-limit 30

# Build the page from cached data without network access
python3 scripts/ai_radar.py build

# Preview the page locally
python3 scripts/ai_radar.py serve --port 8765

# Install, inspect, or remove the daily user-level task
python3 scripts/schedule.py install --time 08:00
python3 scripts/schedule.py status
python3 scripts/schedule.py uninstall
```

Read `references/configuration.md` when customizing sources, retention, output locations, or scheduling behavior.

## Output Contract

Return:

- dashboard URL or absolute `site/index.html` path;
- refresh timestamp and source success/failure counts;
- readable article count and current article-fetch success/failure counts;
- item totals by category;
- scheduler status when scheduling was requested;
- exact failed source names and errors, if any.
