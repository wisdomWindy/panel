---
name: ai-workflow-bugfix
description: Use when a defect ID or bug report needs context retrieval, root-cause framing, repair planning, and regression-aware verification.
---

# AI Workflow Bugfix

Use the defect ID as the standard input whenever possible.

## Required Flow

1. Pull defect context first with `references/bugfix-intake-template.md`.
2. Reconstruct scope, impact, and expected behavior.
3. When analyzing a defect, use `references/bug-analysis-rules.md` and follow the strict order: root cause analysis first, then minimal fix, then verification cases.
4. Classify the issue source before fixing it.
5. Verify the original bug and nearby regression risks with `references/verification-checklist.md`.
6. Summarize the outcome with `assets/bugfix-summary.md`.
7. Escalate design-level issues back to `../ai-workflow-intake/SKILL.md` or `../ai-workflow-feishu-spec/SKILL.md` when needed.

## Output Contract

- `Inputs`
- `Context gathered`
- `Assumptions`
- `Actions taken`
- `Outputs`
- `Next recommended stage`
- `Needs human decision`
