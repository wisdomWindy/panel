# PRD / 审查摘要

## 来源摘要

- 产品 PRD 将插件系统、直播录制、多语言支持、配置管理等能力标记为已完成
- 最新项目审查发现多个“已完成能力”仍有实际缺口，属于功能闭环不完整而非纯样式或纯类型问题

## 关键业务目标

1. 让 PRD 中已完成的核心能力与当前运行代码保持一致
2. 优先补齐用户可感知的假成功与不可用链路
3. 在不扩展无关范围的前提下，收口插件系统、配置导入导出、录制闭环、多语言闭环
4. 保持实现可验证、可维护，避免仅做页面层面的表面修补

## 相关模块或页面

- 插件系统
  - `src/renderer/views/plugin-manager/index.vue`
  - `src/renderer/plugins/plugin-manager.ts`
  - `src/renderer/plugins/plugin-loader.ts`
  - `src/preload/index.ts`
  - `src/main/*`
- 配置管理
  - `src/renderer/views/settings/index.vue`
  - `src/renderer/stores/config.ts`
- 录制模块
  - `src/renderer/services/danmaku-service.ts`
  - `src/renderer/views/recording/index.vue`
  - `src/renderer/stores/recording.ts`
  - `src/main/modules/recording/*`
- 多语言
  - `src/renderer/main.ts`
  - `src/renderer/App.vue`
  - `src/renderer/views/dashboard/index.vue`
  - `src/renderer/views/settings/index.vue`
  - `src/renderer/views/settings/components/LanguageSettings.vue`
  - `src/renderer/locales/*.json`

## 上游 PRD / 当前代码中的显著缺口

1. 插件管理页面存在真实 `TODO`，核心动作未接线；同时预加载暴露的 `fs:*` 文件系统接口缺少主进程处理器，安装链路存在阻断风险
2. 设置页导入配置只做 JSON 解析后提示成功，未真正写回配置；导出内容落后于当前功能范围
3. 自动录制触发逻辑仍为 `TODO`，手动录制页面将 `roomId` 写死为 `default`
4. 多语言切换只覆盖局部区域，多个主页面和导航仍有大量硬编码中文；Element Plus locale 仅在启动时注入，不随运行时切换同步

## 值得在规格中明确的问题

1. 插件系统本次是否需要补齐“配置编辑”，还是仅做到“配置查看与状态更新”
2. 配置导入导出采用单一扁平配置模型，还是继续兼容当前 settings 页面分组结构
3. 多语言闭环本次覆盖哪些页面，是否以当前审查发现的硬编码高频区域为最小闭环范围
4. 自动录制的触发时机，是否以当前已存在的 `autoRecordStart` 事件为唯一入口
