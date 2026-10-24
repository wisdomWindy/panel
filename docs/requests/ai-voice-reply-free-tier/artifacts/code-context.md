# Code Context

## Structural Analysis Method

- A repository-provided code graph, SCIP index, or team bootstrap command was not found.
- No graph dependency was installed because the repository defines no approved graph tool and adding an arbitrary dependency would expand the change surface.
- Fallback: scoped `rg` import/caller searches plus direct reading of routing, store, contract, IPC, application-state, repository, migration, capability, and build-channel files.

## Governing TypeScript Context

- `packages/bilibili-live-tool/tsconfig.json`
- Strict TypeScript, bundler module resolution, `@/* -> src/renderer/*`, Vue JSX preserve, and Vite/Vue ambient types.
- `vite.config.ts` maps `@member-channel` to `channel.member.ts` or `channel.free.ts` by build mode.
- Relevant declaration source: `src/vue-shims.d.ts`; desktop contracts are repository-owned TypeScript definitions under `src/renderer/platform/desktop/contracts`.

## Current Ownership Graph

```text
App.vue / router / main.ts
  -> @member-channel
       -> channel.free.ts: no routes/runtime
       -> channel.member.ts: membership + AI + moderation routes/runtime
            -> useMemberRuntime
                 -> membership store
                 -> AI store
                 -> moderation store

AI page/store
  -> platform/desktop member contracts
  -> member adapters
  -> ai_* and asr_* Tauri commands

Tauri AppState (member build only)
  -> MemberStates
       -> MembershipState
       -> AiState
       -> ModerationState -> MembershipState
  -> ipc/member.rs
       -> AI commands query membership capability before generate/ASR start
```

## Key Code Facts

- Free navigation currently has no AI route and its contract test explicitly rejects `views/ai`.
- Member navigation hides/disables AI based on `status.capabilities.ai` and member expiry.
- `useMemberRuntime` owns AI event listeners and bootstrap, so the free build never starts the AI store.
- AI desktop types and commands are mixed into `contracts/member.ts` and `MEMBER_COMMAND_NAMES`.
- Rust application/domain/infrastructure/IPC member modules are compiled only with the `member` feature.
- `AppState.member()` is the only access path to `AiState`.
- AI generation and ASR start fetch member capability and reject when it is not `Ready`.
- SQLite already creates `t_ai_config` and `t_ai_log` for the shared database, so schema availability is not inherently member-only.
- Existing `UnavailableLlmProvider` always returns `llm_unavailable`; current AI generation cannot reach a configured endpoint.
- Existing TTS services are shared, but they expose generic TTS configuration/playback rather than a dedicated AI output queue.

## Required Change Closure

- Shared frontend routes/navigation/runtime for AI.
- Shared desktop contract and event registration for AI commands.
- Shared Rust compilation and `AppState.ai()` access independent of member storage availability.
- Remove membership capability arguments/checks from AI generation and ASR lifecycle.
- Preserve membership gating for moderation and membership pages only.
- Extend AI types/state/UI for model profiles, voice-reply activation, routing, and queue controls.
- Update free/member command lists, capability permission allowlist, and boundary tests.

## Regression-Sensitive Neighbors

- Member toolbar/login/order/module flows must remain member-only.
- Moderation continues to depend on membership entitlement.
- Existing generic danmaku TTS must remain separate from the AI reply queue.
- Free build must still exclude membership and moderation pages/stores.
- Event listeners must not be double-registered when shared AI runtime and member runtime both start.

