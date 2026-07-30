# Self-Healing Loop Policy

## Scope

Use this policy for every frontend-agent-framework delivery unit after implementation begins.

The loop is not a single pass through `execute -> verify -> review`. The loop is a self-healing control cycle that keeps re-checking the changed product behavior and code quality, then automatically returns to the stage that owns the defect until no framework-recoverable blocker remains.

## Required Self-Healing Cycle

After every `execute` pass, run this cycle:

1. `execute`: implement the framework-approved plan and record changed files, changed symbols, assumptions, and known risks.
2. `verify`: validate runtime-observable behavior against acceptance criteria, source-grounded requirements, tests, type/lint/build signals, and changed feature-chain evidence.
3. `review`: inspect the diff and feature chain for requirement mismatch, logic errors, syntax/type errors, runtime errors, code organization errors, maintainability errors, and technical-debt blockers.
4. If any blocker is framework-recoverable, route automatically to the owning stage and continue the same workflow run.
5. Only mark `complete` when review records `merge-ready`, all applicable verdicts pass, and the self-healing audit records no remaining blocker.

Do not stop after a successful command if review has not inspected the resulting code and the feature chain.
Do not stop after review if review found fixable blockers.
Do not ask the user to rediscover bugs that the framework can find from documents, code, tests, type checks, lint, runtime reasoning, or changed-file review.

## Loop Bounds and Dead-Loop Prevention

The self-healing loop is bounded. It must continue only while each iteration produces new evidence, a new repair, or a justified escalation to the correct owning stage.

Limits:

- Maximum self-healing iterations for one delivery unit: `7`.
- Maximum consecutive attempts for the same blocker in the same owning stage: `2`.
- Maximum total attempts for the same blocker across all owning stages: `3`.
- If the same blocker fails twice in `execute`, escalate to `plan`, `spec`, or `architecture-design` based on the missing decision.
- If the same blocker fails three times total, stop the loop and mark the delivery unit `blocked`.
- If one iteration produces no code change, no artifact repair, no new evidence, and no narrower diagnosis, stop immediately and mark `blocked` for no progress.

Same blocker identity is determined by:

- defect class
- affected file / symbol / behavior
- failed acceptance item or review finding
- owning stage selected for repair

When blocking for loop bounds, record:

- blocker identity
- attempts made
- stages tried
- changes attempted
- evidence collected
- why no further safe progress can be made without human input or external information
- exact question, missing source, or external condition needed to unblock

## Defect Taxonomy and Owning Re-entry

Classify every issue found in verify or review:

| Defect class | Examples | Re-entry |
| --- | --- | --- |
| `requirement-mismatch` | Missing PRD behavior, extra unapproved behavior, wrong user intent, wrong frontend/server responsibility | `spec` or front-loaded confirmation gate when a human decision is truly missing |
| `plan-gap` | Plan lacks exact file, symbol, field, state transition, cleanup, or verification detail needed to fix safely | `plan` |
| `architecture-gap` | Wrong module boundary, wrong abstraction, missed approved reuse, dependency direction conflict, unstable pattern choice | `architecture-design` when structural, otherwise `spec` / `plan` |
| `logic-error` | Wrong condition, branch, mapping, validation, formatter, state derivation, payload, permission, or adapter behavior | `execute` |
| `syntax-type-error` | TypeScript error, import/export error, lint syntax error, invalid template syntax, alias/declaration mismatch | `execute`, or `plan` if the type contract is missing |
| `runtime-error` | Null access, rejected promise, stale async response, missing cleanup, invalid component prop, broken route/state lifecycle | `execute`, or `spec` / `plan` if behavior semantics are underspecified |
| `code-organization-error` | Orphan helper, unused import/export, single-caller file extraction without approved reason, wrong SFC/component boundary, stale tests/mocks/comments | `execute` |
| `maintainability-error` | Overlong mixed-responsibility function, magic values, unclear naming, hidden dependencies, unreadable data transformation, reviewer-hostile diff | `execute`, or `plan` if refactor work is missing |
| `technical-debt-blocker` | New debt created by the change, duplicated semantic logic that should be resolved now, temporary compatibility path without owner/cleanup trigger, TODO/debug/dead code | `execute`, or `architecture-design` if structural |

When the issue is recoverable from repository sources and approved artifacts, fix it without asking the user.
Ask the user only when the missing decision cannot be recovered from source documents, code facts, project constraints, or safe source-derived consequences.

## Completion Bar

Before a delivery unit may complete, verify and review must prove all of these are true:

- Requirement consistency: implementation covers every approved acceptance item and adds no unapproved product behavior.
- Business logic consistency: conditions, field semantics, permissions, state transitions, validation, adapters, payloads, and display states match the approved spec / plan.
- Syntax and type correctness: relevant type checks, lint checks, template checks, imports, exports, aliases, and declaration assumptions are checked or have justified substitute evidence.
- Runtime safety: nullability, async failures, loading/error/empty states, cleanup, race conditions, stale responses, and route/component lifecycle hazards are checked.
- Code organization correctness: changed files, symbols, imports, exports, tests, mocks, comments, helper locality, component boundaries, and dependency closures are clean.
- Maintainability correctness: naming, function size/responsibility, constants, readable transformations, side-effect boundaries, and reviewer readability pass.
- Technical-debt control: the change does not introduce avoidable debt, fake abstraction, stale dual paths, unowned compatibility paths, dead code, debug code, or duplicated semantic logic that the approved architecture requires resolving.

## Loop Evidence

Each verify and review artifact must record:

- self-healing iteration number from `state.json.loop.iteration`
- attempt count for each remaining blocker
- whether this iteration produced new evidence, new repair, or escalation
- defect inventory grouped by defect class
- automatic re-entry target for every blocker
- what was fixed in the current pass
- what was rechecked after the fix
- remaining blockers, or `none`
- whether completion is allowed: `yes` only when no blocker remains and all applicable verdicts pass

## Review Blockers

Treat these as blockers:

- Review only summarizes completed work without independently inspecting changed code and feature-chain behavior.
- Verification records command success but does not map behavior back to acceptance criteria and source-grounded spec / plan.
- A fixable blocker is reported to the user instead of being routed back into the loop.
- The same blocker is retried beyond the configured limits instead of escalating or blocking.
- A loop iteration continues even though it produced no new evidence, no repair, and no narrower diagnosis.
- The workflow marks `complete` while any defect class above has unresolved framework-recoverable findings.
- The workflow relies on the user to find logic, syntax, runtime, organization, maintainability, or technical-debt issues that could be found by scoped code review, tests, type/lint/build checks, or source-grounded reasoning.
