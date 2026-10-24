# Code Context: AI Voice Reply Warning Spacing

- Owner: `packages/bilibili-live-tool/src/renderer/views/ai/components/AiVoiceReplyControl.vue`.
- Target: the two mutually exclusive `el-alert` branches immediately after the top control row.
- Current style: both alerts use `class="mt-3"`.
- Caller/test surface: AI page renders the component; `views/ai/ai-page.test.ts` provides source-contract coverage.
- No data, state, event, API, or backend path is involved.
