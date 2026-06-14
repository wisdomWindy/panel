---
name: ai-workflow-release
description: Use when preparing a change for release and a subagent must produce structured checks for risks, verification status, and rollback awareness without automating the final release action.
---

# AI Workflow Release

This skill supports release discipline. It does not replace the human release owner.

## Required Flow

1. Gather PRs, specs, and verification results.
2. Summarize changes and unresolved risks.
3. Flag missing validation, dependency gaps, and rollback concerns with `references/release-safety-rules.md`.
4. Produce a release checklist and explicit blockers.
5. Send the work back upstream when the release packet is incomplete.

## Output Contract

- `Inputs`
- `Context gathered`
- `Assumptions`
- `Actions taken`
- `Outputs`
- `Next recommended stage`
- `Needs human decision`
