# Source Snapshot: AI Voice Reply Warning Spacing

## Verbatim Evidence

- `SRC-1`: “增加红框中内容的上下两个部分的间隔”
- Screenshot evidence: the red frame encloses the AI voice-reply control row above and the orange warning alert below.

## Normalized Requirement

- Source-backed: increase the vertical gap between those two highlighted parts.
- Code-fact-backed: both possible alerts currently use `mt-3`; they must stay visually consistent.
- Confirmed interpretation: use the next project spacing step, `mt-4`, for a restrained increase.

## Product Content Allowlist

- Layout-only change: alert top margin from `mt-3` to `mt-4` in both branches.
- Product-content delta: none.

## Out Of Scope

- Alert text, visibility rules, controls, outer section padding, queue layout, ASR layout, and responsive behavior.
