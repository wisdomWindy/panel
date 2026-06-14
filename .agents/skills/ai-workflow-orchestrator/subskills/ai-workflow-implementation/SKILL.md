---
name: ai-workflow-implementation
description: Use when an approved spec needs an execution plan, coordinated implementation tasks, reviews, and verification results.
---

# AI Workflow Implementation

Implementation starts only after the spec is specific enough to build from.

## Required Flow

1. Reject incomplete specs and send them back upstream.
2. Turn the spec into a concrete implementation plan with `references/spec-to-plan.md`.
3. When a minimal-change technical solution is needed, use `references/tech-solution-rules.md` and output affected files, API contracts, data flow, error handling, and regression risks.
4. Split work by disjoint write sets using `references/subagent-execution-rules.md`.
5. When reviewing code changes, use `references/code-review-rules.md` and evaluate strictly across five dimensions: defects, regression risks, boundary conditions, permission issues, and test gaps.
6. Keep review and verification ahead of any completion claim.
7. Capture doc-sync items for follow-up.
8. Send spec drift back to `../ai-workflow-intake/SKILL.md` or `../ai-workflow-feishu-spec/SKILL.md` when needed.

## Code Style Rules

- **Vue 组件样式**: 所有 `.vue` 页面文件禁止使用 `<style>` 块、`<style scoped>` 块、内联样式（`style="..."` 属性）和行内样式。布局和视觉样式只能使用 Tailwind CSS class（如 `class="flex gap-2 p-4 ..."`）。Element Plus 组件内部样式覆盖通过 `class` 属性传入 Tailwind class 完成，不得额外写 `<style>` 或 `style` 属性。

## Output Contract

- `Inputs`
- `Context gathered`
- `Assumptions`
- `Actions taken`
- `Outputs`
- `Next recommended stage`
- `Needs human decision`
