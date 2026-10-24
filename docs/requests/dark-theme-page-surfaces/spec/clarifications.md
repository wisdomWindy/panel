# Clarifications

## Confirmed Decisions

- 受影响范围严格限定为用户点名的四个页面及其主题契约测试。
- 保留浅色主题、现有布局、业务数据、交互和文案。
- 使用项目现有 `.dark` Tailwind variant 与 `zinc` 深色层级。

## Framework Assumptions

- 静态页面 utility 足以修复当前截图中的主要白块；该假设可逆且由 RED 测试和运行时截图验证。

## Missing Source / Open Questions

无。若 ECharts canvas 仍产生明显浅色块，作为验证发现回到 spec，而非在 execute 临时扩展范围。

