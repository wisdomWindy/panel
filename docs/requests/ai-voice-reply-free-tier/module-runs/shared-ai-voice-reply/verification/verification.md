# Verification Report: 2026-10-23 Amendment

Status: PASS

## Acceptance Coverage

| Criterion | Method | Result | Evidence |
| --- | --- | --- | --- |
| Manual context/input/generate/regenerate/clear-result UI is absent | Source contract test and browser DOM inspection | PASS | `ai-page.test.ts`; desktop DOM contains ASR followed by the read-only result only |
| Latest automatic result remains read-only and copyable | Source contract test and browser DOM inspection | PASS | `copyResult` action and read-only result textarea remain |
| Missing ASR model opens an actionable dialog | Browser interaction and component test | PASS | ASR switch opens `选择识别模型`; the former terminal unavailable copy is absent |
| Catalog offers selectable tiny/base/small models | Rust catalog test and typed frontend dialog | PASS | `WhisperAsrProvider::list_models`; radio list keyed by `modelId` |
| Frontend cannot supply URL or destination path | Contract inspection and test | PASS | `asr_model_download` accepts only `modelId` and `isLicenseAccepted` |
| Unknown models and unaccepted license fail closed | Rust/IPC validation | PASS | `asr_model_unknown` and `asr_license_required` paths |
| Download is verified before ready state | Rust implementation and catalog test | PASS | `.part` write, SHA-1 verification, rename, marker; partial files are not installed |
| Model absence and engine absence are distinct | State/error mapping inspection | PASS | `asr_model_missing` versus `asr_engine_unavailable` |
| Shared free/member contract remains aligned | Automated suites | PASS | 24 AI commands in TS/Rust channel contracts; both feature suites pass |
| Responsive dialog has no horizontal overflow | Browser viewport check | PASS | Effective 454px viewport: body width 454px; dialog 426.75px wide with 13.625px side margins |
| Windows installer contains the amendment | Release build and checksum | PASS | NSIS artifact, 7,727,707 bytes; SHA-256 `96d9b725b8a56ee5bbd8c8fd5784ade3f7a090692b4d475daf562464accf1ce3` |

## Verification Commands

- `pnpm type-check`: PASS
- `pnpm lint:check`: PASS with two pre-existing `no-console` warnings in `modules/utils/logger.ts`
- `pnpm exec vitest run --mode free`: PASS, 47 files / 153 tests
- `pnpm exec vitest run --mode member`: PASS, 47 files / 153 tests
- `cargo test --features free`: PASS, 150 tests
- `cargo test --features member`: PASS, 151 tests
- `pnpm build:web:free`: PASS
- `pnpm build:web:member`: PASS
- `cargo fmt --all -- --check`: PASS
- `pnpm build:free`: PASS, NSIS bundle created

## Compliance

- Spec constraint compliance: PASS. No outgoing Bilibili path, entitlement dependency, frontend-owned download URL/path, or bundled model weights were introduced.
- Spec-plan granularity alignment: PASS. T6/T7 implement only the approved UI removal, model acquisition, verification, and packaging work.
- API contract conformance: PASS. Rust camelCase DTOs, TypeScript types, adapters, Store actions, command registration, and capability permissions agree.
- TDD exception check: PASS. User-visible and catalog behaviors were observed failing before implementation and passing afterward.

## Residual Boundary

The repository still has no bundled speech-recognition engine. Downloaded weights become `ready`; attempting recognition without an engine reports `asr_engine_unavailable` explicitly. This is intentionally distinct from the corrected model-selection/download flow.

## 2026-10-23 Model Profile Proxy Removal Verification

Status: PASS

| Criterion | Method | Result | Evidence |
| --- | --- | --- | --- |
| Add/edit UI has no proxy URL field | Contract test plus live browser DOM/screenshot inspection | PASS | Drawer ends with timeout and enabled controls, followed by cancel/save |
| Component has no hidden proxy draft or hydration path | Source search and contract test | PASS | No production `draft.proxyUrl`, proxy label, or profile proxy hydration remains |
| Save contract remains compatible without a user-controlled proxy | Source contract test and TypeScript check | PASS | `AiModelProfileSaveRequest.profile.proxyUrl` is emitted as `null` |
| Free/member frontends remain aligned | Full Vitest suites and web builds | PASS | 47 files / 153 tests passed in each mode; both builds passed |
| Windows installer contains the change | Tauri free build and checksum | PASS | 7,736,489 bytes; SHA-256 `2546d4ec4a68c23cc57e4126e4e2daf182cea982e3a8334e1c0b1c849f7a45cb` |

Verification commands: focused `ai-page.test.ts`, `pnpm exec vitest run --mode free`, `pnpm exec vitest run --mode member`, `pnpm type-check`, `pnpm lint:check`, `pnpm build:web:free`, `pnpm build:web:member`, and `pnpm build:free` all passed. Lint retained only the two pre-existing `no-console` warnings in `modules/utils/logger.ts`.

- Source grounding assessment: pass.
- Spec-plan granularity alignment: pass; T8 contains only the requested proxy-field removal, compatibility normalization, verification, and packaging work.
- API contract assessment: pass; the public nullable field is retained and normalized at the component boundary.
- TDD exception check: pass; the user-visible behavior failed before implementation and passed afterward.

## 2026-10-23 Model Profile Reasoning Effort Verification

Status: PASS

| Criterion | Method | Result | Evidence |
| --- | --- | --- | --- |
| Temperature is replaced by reasoning effort in the model drawer | Focused contract test and live browser DOM inspection | PASS | Drawer exposes `推理强度`; `温度` is absent |
| Values use a closed agent-style set | Adapter validation, Rust enum, and live expanded select | PASS | `none`, `minimal`, `low`, `medium`, `high`, `xhigh`, `max`; `media` rejected |
| Existing profiles migrate predictably | SQLite migration and repository round-trip test | PASS | `reasoning_effort` is non-null, checked, and defaults to `medium` |
| Provider payload uses the selected value | Rust source-contract test and implementation inspection | PASS | `reasoning_effort` is emitted; model-profile `temperature` is absent |
| Free/member contracts remain aligned | Full Vitest and Rust suites | PASS | Frontend: 47 files / 153 tests in each mode; Rust: free 150, member 151 |
| Removed proxy field stays absent | Live browser DOM inspection | PASS | Drawer contains neither `代理 URL` nor a proxy input |
| Windows installer contains the amendment | Tauri free build and checksum | PASS | 7,739,214 bytes; SHA-256 `674b38f5b87d62e19896e740ef1bd69ca3f18322e7d2cefc7ee77ca4d434e246` |

Verification commands: focused Vitest/Rust tests, `pnpm exec vitest run --mode free`, `pnpm exec vitest run --mode member`, `cargo test --no-default-features --features free`, `cargo test --no-default-features --features member`, `pnpm type-check`, `pnpm lint:check`, `cargo fmt --all -- --check`, both web builds, and `pnpm build:free` all passed. Lint retained only the two pre-existing `no-console` warnings in `modules/utils/logger.ts`; Vite retained its existing large-chunk warning.

- Source grounding assessment: pass; current values were checked against official OpenAI API and Agents SDK documentation.
- Spec-plan granularity alignment: pass; T9 contains only the requested control/contract migration, verification, and packaging work.
- Migration assessment: pass; the additive migration preserves existing databases and makes `medium` deterministic.
- TDD exception check: pass; targeted tests were observed red before production changes and green afterward.
