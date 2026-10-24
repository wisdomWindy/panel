# Code Context

## Context Requirement

需要确认两路由的直接子组件范围和可见样式闭包；bugfix intake 首次需要。代码图不是必需，直接路由/组件搜索足够。

## Graph Availability / Bootstrap / Fallback

- 图状态：未使用；该改动不涉及依赖方向或共享所有权。
- bootstrap：否，`not_needed`。
- fallback：是，使用 `rg` 和直接源码阅读；对静态模板颜色具有高置信度。

## Relevant Entrypoints

- `/tts` -> `views/tts/index.vue` -> `TtsQueuePanel.vue`。
- `/recording` -> `views/recording/index.vue` -> `RecordingStatusBand.vue` / `RecordingSettingsDialog.vue`。

## Functional Chain Under Change

`.dark` 根类 -> Tailwind dark variant -> 现有页面/子组件 DOM。只有颜色 token 属于本改动；store、状态、事件、IPC、录制与语音副作为禁止邻接链路。契约测试是适配面而非生产 owner。

## Key Symbols And Modules

五个 Vue 模板及 `media-pages-contract.test.ts`；无导出符号或组件 API 变更。

## Dependency And Side-Effect Boundaries

样式仍由各 DOM owner 就地拥有。无请求、持久化、音频、录制、事件或共享状态边界变更。

## Impact Scope

五个模板、一个现有契约测试，以及两路由的运行时视觉验证。

## Open Follow-Up Checks

确认录制列表/高光子组件无未配对颜色；实现后复查两路由和浏览器控制台。
