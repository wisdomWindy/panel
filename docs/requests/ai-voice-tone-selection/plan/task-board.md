# Task Board: AI Voice Tone Selection

| ID | Task | Status | Evidence |
| --- | --- | --- | --- |
| T1 | Add failing frontend/Rust contract tests | completed | Frontend failed on missing voice contract; Rust failed on missing `voice_id` |
| T2 | Implement Rust port, migration, persistence, IPC, and platform voice selection | completed | 131 Rust unit tests; `free` tests and `member` check pass |
| T3 | Implement frontend contract, adapter, store, and dialog selector | completed | 6 focused Vitest tests, type-check, scoped ESLint, both web builds pass |
| T4 | Recheck references, regressions, diff hygiene, and verification evidence | completed | `verification/verification.md`; all applicable verdicts pass |

## Scope Guard

- Allowed product change: voice-tone selector and `温柔桃子` default only.
- No ASR model behavior, cloud provider, general TTS page, dependency, or unrelated refactor changes.

