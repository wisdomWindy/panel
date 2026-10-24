# Implementation Plan

1. Add failing Rust mapper/domain/IPC tests for `gift_icon_url` and notification request copy; add frontend adapter/overlay contract tests; add an audio sequence testable helper contract.
2. Extend domain records, all constructors/matches, DTO mapping, TypeScript contract, and adapters with nullable gift image URL.
3. Update the popup to render gift image with deterministic fallback and enable HTTPS image CSP.
4. Update notification content, introduce bounded Bilibili HTTPS image caching, and use the Windows image path with text-only retry.
5. Replace the built-in two-tone alert with a named ascending four-note sequence.
6. Run focused RED/GREEN checks, full frontend/Rust tests, type/lint/build, package, visual review, and scoped diff review.

## Review Checklist

- Robustness: null URL, broken image, network failure, invalid host/content/size, notification retry.
- Maintainability: one owner per mapping/side effect; no duplicate URL or copy logic.
- Performance: timeout, response cap, hashed cache reuse, no renderer timers/listeners leaked.
- Stack fit: Vue/Tailwind, existing adapters, application ports, reqwest/Rodio/Tauri patterns.
