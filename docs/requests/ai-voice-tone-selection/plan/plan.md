# 实施计划：AI 声色选择

## 交付单元

`ai-voice-tone-selection`

## 执行目标

在现有 AI 语音回复链路中加入可持久化、可测试、实际影响播报的声色选择，未配置时默认 `温柔桃子`，并保持 ASR 与相邻 TTS 功能不变。

## 上游合同

- 行为：`spec/spec.md`
- 页面：`design/page-design.md`
- 架构：`design/architecture-design.md`
- 调用链：`artifacts/code-context.md`

## 任务 T1：先写失败测试与精确类型合同

- Grounding: `SRC-1`；spec 的 Type and Contract Changes、Testing Contract。
- Product-content delta: `声色` 字段和 `温柔桃子` 默认。
- 前端：在 `contracts/ai.ts` 规划 `AiVoiceId`、`AiVoice`、snapshot/catalog、payload 字段；先更新 `adapters.test.ts` 与 `ai-page.test.ts`，断言 `voices`、`selectedVoiceId`、`声色`、`温柔桃子` 及 payload 贯通，运行得到红灯。
- Rust：在 repository/application/IPC 测试中先断言默认 `gentle-peach`、持久化回读、测试/队列传参和 camelCase 字段，运行得到红灯。
- 保留：现有设备、音量、确认和队列行为。
- 验证：聚焦 Vitest 与 member Rust tests；失败原因必须是缺少新合同而非环境或旧故障。

## 任务 T2：扩展 Rust 端口、持久化与平台适配

- Grounding: spec 的 Persistence、Playback Resolution、Architecture；ADR-1/2。
- Product-content delta: none，落实已批准合同。
- 新增 `0006_ai_voice_tone.sql`：非空 `voice_id`，默认 `gentle-peach`，写入 schema state。
- `ports.rs`：增加稳定默认常量、`AiVoiceRecord`、配置/快照字段、`AiSpeechOutput::voices` 和四参数 `speak`。
- `repository.rs`：SELECT/map/UPDATE/validation 同步新列，拒绝空 ID。
- `state.rs`：snapshot 带选中 ID；队列和测试播放传 voice；set route 同时验证 device/voice/volume，device 或 voice 变化时取消 enable/confirmation。
- `ai.rs`：Windows 通过 SAPI 枚举 token；预设优先 HuiHui 后回退当前/default；具体 ID 精确选择；非 Windows 通过 `tts` 枚举并选择；未知 ID 返回 `ai_tts_voice_invalid`。
- `ipc/member.rs`：增加 voice DTO/catalog，扩展请求和 snapshot 映射。
- 更新全部 `AiSpeechOutput` 测试双和 record literal，禁止保留三参数旧调用。
- 验证：`cargo fmt --check`、member/application/infrastructure/IPC scoped tests、`cargo check`。

## 任务 T3：接入前端适配、store 与弹窗

- Grounding: `SRC-1`；spec User-Visible Behavior；page design。
- Product-content delta: 新增 `声色` 选择和 `温柔桃子` 默认选项。
- `contracts/ai.ts`：使用 branded voice ID，readonly catalog，payload 添加 `voiceId`。
- `adapters.ts`：校验并冻结 `voices`，读取非空 `selectedVoiceId`；模板不得自行兜底 DTO。
- `stores/ai.ts`：set/test action 显式接收并发送 voice ID，保持现有 operation dedupe。
- `views/ai/index.vue`：事件处理函数传递 voice ID。
- `AiVoiceReplyControl.vue`：草稿增加 `voiceId`，从 snapshot 回填；新增 required `声色` select；测试和确认携带 voice；缺失时禁用。
- 样式：仅复用 `w-full` Tailwind utility 与 Element Plus，不新增 CSS，不抽新组件。
- 验证：适配器测试、AI 页面合同测试、TypeScript type-check、ESLint scoped check。

## 任务 T4：链路清理、回归与证据

- Grounding: acceptance criteria 1-5；human review readiness。
- Product-content delta: none。
- 用 `rg` 复查 `AiSpeechOutput`、`speak(`、`ai_tts_set_output_device`、`ai_tts_test`、`selectedVoiceId` 的所有生产调用和测试适配面。
- 检查迁移顺序、SQL 列顺序、未使用 import/export、陈旧 mock/comment、临时名称、调试代码及无关格式化。
- 运行 scoped frontend tests/type-check/lint、Rust fmt/check/tests；记录未能执行的命令和原因。
- 对照 spec 验证没有改变 ASR、通用 TTS 页面或引入额外品牌声色。

## TDD 顺序

1. 前端/Rust 新行为测试红灯。
2. 最小合同与后端链路变绿。
3. UI 接入变绿。
4. 重构只限消除本次改动产生的重复或编译噪音，再跑完整 scoped checks。

## 状态与错误边界

- UI 临时草稿与服务端 snapshot 维持单一 owner。
- 空/未知 voice ID 在 adapter/application/repository/platform 边界显式失败。
- 具体系统声音失效不静默替换；默认预设允许受控回退。
- 不新增请求并发路径，不新增 retry/cancel 语义。

## 代码评审清单合同

- 健壮性：所有外部 DTO、SQLite 值和系统 voice token 有非空/存在性验证；Promise/Result 沿现有错误边界处理。
- 可维护性：扩展既有 owner，不新增 manager/factory/helper 文件；平台分支保持局部。
- 性能/内存：目录体量小，不加缓存；COM 初始化/释放和已有 cancel epoch 保持闭合。
- 技术栈：Vue/Pinia/Element Plus/Tailwind、Tauri/Rust/sqlx/SAPI 不变，不加依赖。
- 评审准备：每个 changed hunk 映射 T1-T4，拒绝顺手重构与全文件格式噪音。

## 回退条件

- 若 Windows crate 不暴露所需 SAPI voice token API，回退 architecture-design 调整平台实现，不在 execute 猜测新依赖。
- 若测试发现 snapshot 无法原子提供目录与选择，回退 plan/architecture；不新增未批准命令。
- 若实现需要新增产品文案或额外品牌声色，回退 intake 确认。

## 完成证据

- 测试命令与结果。
- 类型检查、Rust check、格式检查结果。
- 关键引用链复查结果。
- 迁移默认与实际 playback 参数的测试证据。

