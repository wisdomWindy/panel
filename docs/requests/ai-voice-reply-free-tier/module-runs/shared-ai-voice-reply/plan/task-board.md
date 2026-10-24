# Task Board

| ID | Task | Status | Evidence |
| --- | --- | --- | --- |
| T1 | Add failing tests for shared AI routes, commands, runtime ownership, and output rules | completed | Red tests failed on missing `channel.shared`, `useAiRuntime`, and Rust `ai_command_names` |
| T2 | Move frontend AI route, navigation, contract, adapter, and runtime ownership out of membership | completed | Type check and focused Vitest tests pass |
| T3 | Move Rust AI state, IPC, invoke registration, permissions, and build-channel ownership out of membership | completed | Free/member Rust suites pass |
| T4 | Implement the revised AI voice-reply product surface | completed | Persisted model profiles, Windows Credential Manager secrets, OpenAI-compatible transport, master enablement, route confirmation, automatic triggers, and the independent priority playback queue are implemented |
| T5 | Verify the implemented slice in both channels and responsive layouts | completed | Both web builds pass; full frontend/Rust suites pass; 1440px and 390px checks show no horizontal overflow or current-page console errors |
| T6 | Remove manual generation UI and implement selectable ASR model acquisition | completed | Page controls removed; selectable Rust-owned tiny/base/small catalog and verified download flow implemented |
| T7 | Verify the amended flow and rebuild the Windows NSIS installer | completed | Free/member tests and builds pass; responsive dialog has no horizontal overflow; NSIS artifact rebuilt |
| T8 | Remove the proxy URL field from AI model profiles and rebuild the Windows installer | completed | Field/draft removed; save normalizes to null; tests, browser check, builds, and NSIS packaging pass |
| T9 | Replace model-profile temperature with reasoning effort end to end | completed | Closed enum, IPC/storage/provider migration, tests, live UI inspection, and NSIS rebuild pass |

## Current Focus

- Current: completed; no implementation task remains for this amendment
- External validation: configure a real provider and verify the selected route is visible in OBS/Bilibili Live Assistant
- Platform note: Windows has native Credential Manager and SAPI device routing; other targets fail closed when secure storage or device routing is unavailable
