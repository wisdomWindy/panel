# 澄清记录

## 问题

是否通过向 CSP 添加 `'unsafe-eval'` 来兼容 `vue-i18n` 默认编译器？

## 答案

否。严格 CSP 是已有桌面端安全边界，放宽它会超出恢复界面的最小修复范围。

## 最终决策

在既有 `i18n-runtime.ts` 中提供符合 `MessageCompiler` 的无 eval 纯函数，并由主 `createI18n` 显式注入。

## 影响的规格区域

安全边界、i18n 启动配置、资源语法、单元测试和生产运行时验证。

## 锚定状态和所需来源

`code-fact-backed`：`tauri.conf.json` CSP、CDP 捕获的 `EvalError`、locale JSON 语法扫描和 `vue-i18n` 公开类型声明。无未决的 `missing-source`。

