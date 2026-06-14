---
name: ai-workflow-intake
description: Use when a requirement needs structured context gathering, scope clarification, and a grounded technical analysis before documentation or implementation.
---

# AI Workflow Intake

Start with your own initial reading. Do not let AI define the business boundary for you.

## Required Flow

1. Confirm goal, scope, dependencies, and obvious risks.
2. Gather context from linked docs, project fields, prior specs, and code when relevant.
3. Use `references/context-checklist.md` to keep the intake complete.
4. Separate known facts, assumptions, open questions, and non-goals with `references/analysis-output-template.md`.
5. When a requirement document needs formal review, use `references/requirement-review-rules.md` and evaluate across seven dimensions: goal, scope, data source, permissions, edge cases, test points, and launch risks.
6. Produce a delivery-ready technical analysis and summarize it with `assets/analysis-summary.md`.
7. Hand off to `../ai-workflow-feishu-spec/SKILL.md` when the content is ready for shared documentation.

## Output Contract

- `Inputs`
- `Context gathered`
- `Assumptions`
- `Actions taken`
- `Outputs`
- `Next recommended stage`
- `Needs human decision`
