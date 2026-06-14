---
name: ai-workflow-feishu-spec
description: Use when internal technical analysis must be turned into a Feishu tech spec or an existing Feishu spec needs structured updates after review.
---

# AI Workflow Feishu Spec

Convert internal analysis into a collaborative Feishu tech document without losing review history or project alignment.

## Required Flow

1. Decide whether to create a new spec or update an existing one.
2. Map content into the team tech-doc structure with `references/feishu-doc-structure.md`.
3. Sync project metadata, linked references, and review-driven deltas with `references/project-sync-checklist.md`.
4. When decomposing a requirement into independently developable modules, use `references/requirement-decomposition-rules.md` and output each module with inputs, outputs, dependencies, acceptance criteria, and parallelizability.
5. Prefer incremental updates over full rewrites.
6. Use `assets/feishu-tech-spec.xml` when creating a new spec.
7. Hand off to `../ai-workflow-implementation/SKILL.md` once the shared spec is aligned.

## Output Contract

- `Inputs`
- `Context gathered`
- `Assumptions`
- `Actions taken`
- `Outputs`
- `Next recommended stage`
- `Needs human decision`
