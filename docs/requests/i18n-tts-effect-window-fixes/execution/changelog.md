# 执行记录

- 验证回流：发现 TTS 纯 helper 的文件位置与已批准架构文档不一致，回流 `architecture-design -> plan -> execute`。基于 SFC/Element Plus CSS 导入会破坏无 DOM 纯单元测试的新证据，批准同 feature 目录单调用方例外；产品语义不变。

## 2026-09-22

- T1 `completed`：补充 locale key 对称性、HuiHui 默认选择、特效窗口尺寸默认值与边界、运行时窗口规格及屏幕定位回归测试。
- T2 `completed`：系统语音引擎在无已保存声音时优先选择 Microsoft HuiHui；重置默认现在持久化并对成功/失败给出明确提示。纯选择逻辑拆到同特性目录的 `voice-selection.ts`，便于无 UI 依赖测试。
- T3 `completed`：上舰特效新增宽高配置，完成 Vue -> TypeScript contract -> IPC DTO -> Rust domain -> Tauri window 的全链路传递；五种显示位置均基于当前主显示器逻辑坐标计算。
- T4 `completed`：免费版路由页面及直接子组件全部接入 `vue-i18n`，路由标题随 locale 反应式更新，中英文 key 对称测试通过。
- T4 运行时验收：1280x800 Web 桌面视口下设置页布局正常；Web 预览按设计禁用需要 Tauri 的设置操作，语言切换由 locale 运行时测试和桌面构建验证覆盖。
- 阶段证据：Vitest 43 文件 / 132 测试通过；Rust 118 测试通过；TypeScript 类型检查通过；ESLint 0 错误（仅保留 2 个已有 logger `no-console` 警告）；free Web 构建通过。
- T5 `completed`：最终复验通过：TypeScript type-check；ESLint 0 错误（2 条已有 logger 警告）；Vitest 43 文件 / 132 测试；`cargo fmt --check`；Cargo 118 测试；free Web + Rust release + NSIS 构建。
- 安装包：`packages/bilibili-live-tool/src-tauri/target/release/bundle/nsis/biliLiveHelper_0.1.3_x64-setup.exe`。
