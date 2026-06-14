---
name: ai-workflow-orchestrator
description: Use when a request may belong to different AI delivery stages and a main agent must route execution to the correct workflow subagent with backflow control.
---

# AI Workflow Orchestrator

Route the request into exactly one of: `intake_analysis`, `feishu_spec_sync`, `implementation`, `bugfix`, `release`.
Use `references/stage-routing.md` before deciding.

## Main-Agent Rules

- Gather enough context to classify the task before acting.
- Invoke exactly one internal subskill guide per stage decision.
- If the request already contains enough inputs, dispatch directly instead of stopping at a recommendation.
- If the request lacks required inputs, return the missing inputs and keep the task at the current stage.
- Send work back upstream when the current stage exposes missing assumptions, document drift, or verification gaps.
- Escalate business-boundary decisions to `Needs human decision`.
- Treat AI as a copilot, not an autopilot.

## Internal Subskill Map

- `intake_analysis` -> `subskills/ai-workflow-intake/SKILL.md`
- `feishu_spec_sync` -> `subskills/ai-workflow-feishu-spec/SKILL.md`
- `implementation` -> `subskills/ai-workflow-implementation/SKILL.md`
- `bugfix` -> `subskills/ai-workflow-bugfix/SKILL.md`
- `release` -> `subskills/ai-workflow-release/SKILL.md`

When a stage is selected, load the mapped internal subskill file and follow it as the execution guide.

## Output Contract

- `Inputs`
- `Context gathered`
- `Assumptions`
- `Actions taken`
- `Outputs`
- `Next recommended stage`
- `Needs human decision`
