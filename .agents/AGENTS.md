# Agent Workflow Constitution

This repository uses a strict request workflow for frontend delivery in Codex/Codex App.

## Default Entrypoint

- Use `frontend-agent-framework` as the single default entry skill.
- New requirement: let the main skill decompose the request into the full lifecycle and start from `intake`.
- Existing request: resume from `docs/requests/<request-id>/state.json`, let the main skill recover the current stage, and continue orchestrating from there.

## Non-Negotiable Gates

- Never start coding before a request workspace exists under `docs/requests/<request-id>/`.
- Never implement before approved `spec/spec.md` and `plan/plan.md` exist.
- Never declare work complete before `verification/verification.md` and `review/review.md` both exist and pass.
- When requirements change, update request artifacts first and roll the request back to `spec` or `plan`.

## Skill Architecture

- The main skill owns lifecycle decomposition, progression control, gate checks, and stage routing.
- Stage subskills own stage-local execution rules only.
- Policy and template material lives under `frontend-agent-framework/references/` and is loaded on demand.
- Legacy top-level `workflow-*`, `policy-*`, and `templates-*` skills are removed and should not be reintroduced.

## Request Lifecycle

The canonical request lifecycle is:

`intake -> spec -> plan -> execute -> verify -> review -> complete`

For page-oriented requests, insert the design stage before spec:

`intake -> page-design -> spec -> plan -> execute -> verify -> review -> complete`
