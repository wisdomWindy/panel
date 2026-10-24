# Requirement Map

## Splitting Summary

The revised PRD is delivered as one source-preserving module, `shared-ai-voice-reply`. The capability cannot be safely split by frontend/backend layer because each build must ship a complete route-runtime-command-persistence chain.

## Source Modules

- PRD sections 0, 1.1, 2.3: free/member ownership and compliance boundary.
- PRD sections 4-5: package composition, module download, expiry, and recovery behavior.
- PRD section 8: route/navigation visibility.
- PRD section 9.14: AI voice reply behavior, fields, flows, errors, and acceptance criteria.
- PRD sections 9.16-9.17: paid-benefit exclusion and configuration redaction.
- PRD section 10: shared IPC security and device-ID validation.
- PRD section 11: revision summary.

## Split Modules And Functional Units

| Order | Module | Functional units | Page design | Architecture design |
| --- | --- | --- | --- | --- |
| 1 | `shared-ai-voice-reply` | Shared build ownership; model profiles; persona/rules; generation/normalization; activation; audio route; queue controls; ASR isolation; member-boundary cleanup | Required | Required |

## Source Traceability

The module snapshot at `requirements/modules/shared-ai-voice-reply.md` preserves the closed-world allowlist and maps each functional unit to the PRD anchors above. No Markdown normalization companion is required because the source is already readable Markdown.

## Module Dependencies

- Existing shared application shell, live-feed context, SQLite bootstrap, Tauri IPC adapter, and media/TTS infrastructure.
- Existing member subsystem remains a downstream neighbor only for cleanup of old AI ownership.
- External model endpoint and audio capture are runtime dependencies behind explicit adapter/error boundaries.

## Execution Sequence

`shared-ai-voice-reply`: page-design -> spec -> architecture-design -> plan -> execute -> verify -> review.

## Page-Design Routing Hint

Required because the AI Assistant gains a new settings/runtime hierarchy, model profile table/editor, first-enable confirmation flow, route feedback, queue controls, and responsive/error states.

## Spec Input Checklist

- Preserve every model-profile field and secret-handling rule.
- Preserve the single-switch activation preconditions and confirmation state.
- Preserve chat-style format and length constraints.
- Preserve identical display/TTS text and queue/cooldown behavior.
- Preserve membership independence and free/member packaging parity.
- Preserve ASR optionality and outgoing-danmaku prohibition.
- Define frontend/Rust contract types and failure states without inventing provider behavior.

## Open Questions

None blocking. External credentials/devices are integration inputs with source-defined failure behavior.

