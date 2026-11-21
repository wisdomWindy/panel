# 验证记录：移除 whisper-rs-sys 与本地语音识别

日期：2026-11-20 ｜ 环境：Windows 11 / rustc stable MSVC / Node 26

## 1. 结论

**`whisper-rs-sys` 已完全移出依赖树，构建不再需要 libclang / CMake 参与 Rust 编译。**
`cargo check` 与 `cargo test` 在 `free` 与 `member` 两种特性下均直接通过，无需任何临时改动或环境隔离。

## 2. 静态检查与测试

| 项 | 命令 | 结果 |
| --- | --- | --- |
| 依赖树 | `Select-String Cargo.lock -Pattern whisper` | ✅ 0 命中 |
| 编译（free，含测试目标） | `cargo check --lib --tests --features free` | ✅ 0 error / 0 warning |
| 编译（member，含测试目标） | `cargo check --lib --tests --features member` | ✅ 0 error / 0 warning |
| 测试（free） | `cargo test --lib --features free` | ✅ **146 passed, 0 failed** |
| 测试（member） | `cargo test --lib --features member` | ✅ **147 passed, 0 failed** |
| 语言包一致性 | 双语言键集合比对 | ✅ 677 / 677，无单边键 |
| 残留扫描 | `rg -i "asr\|whisper" src src-tauri` | ✅ 仅剩断言性字符串（测试中要求命令面不含 `asr`） |

对比：移除前 `cargo check` 需先安装 `libclang` 并初始化 MSVC 环境，且仍需临时摘除该依赖才能通过。

**干净 shell 复核**（未设置 `LIBCLANG_PATH`、未执行 `vcvars64`）：

| 命令 | 结果 |
| --- | --- |
| `cargo check --lib --features free` | ✅ exit 0 |
| `cargo test --lib --features free --no-run`（含链接） | ✅ `Finished test profile`，exit 0 |

即构建与测试二进制链接都不再需要额外工具链准备。

## 3. 命令面变化

| 项 | 变化前 | 变化后 |
| --- | --- | --- |
| AI 命令 | 24 | **19** |
| AI 事件 | 4（含 `asr_transcript`、`asr_status_changed`） | **2** |
| main-only 权限条目 | 含 5 条 `asr_*` | 已删除 |

契约测试同步更新并通过：
- Rust：`ipc::member_contract_test::both_builds_expose_the_shared_ai_surface`（断言 19 条、无 `asr`）
- TS：`platform/desktop/member-contracts.test.ts`（断言 19 条、末位 `ai_log_query`、无 `asr`）
- TS：`views/ai/ai-page.test.ts`（断言 AI 页面与契约无 ASR 残留）

## 4. 数据兼容性

- **无数据库迁移**：`t_asr_config` 表与 `t_ai_config.asr_context_count` 列保留在 schema 中，
  但代码已不再读写 `asr_context_count`（`INSERT` 省略该列，写入时取列默认值）。
  `SCHEMA_VERSION` 保持为 7，`schema_version_7` 标记不变，降级回旧版本仍可启动。
- 旧数据库中的 `asr_context_count` 值不再被读取，属于可接受的静默废弃。

## 5. 未覆盖项

| 项 | 状态 | 说明 |
| --- | --- | --- |
| 前端 tsc / vitest | ⛔ 未执行 | `node_modules` 的 junction 指向已搬迁旧路径，`tsc`/`vitest` 无法启动（与本次改动无关的既存环境问题） |
| 桌面应用端到端 | ⛔ 未执行 | 需 `pnpm install` 后 `tauri dev` 才能验证 AI 页面渲染 |

前端改动均属删除性质（移除类型、命令映射、适配函数、store 字段、页面区块与文案），
并通过以下静态手段交叉核对：全量残留扫描 0 命中、双语言键 677/677 对齐、
Rust 侧契约测试与 `AI_COMMAND_NAMES` 一致性断言通过。

建议在修复前端依赖后执行：

```bash
pnpm install --force
pnpm -F @monorepo/bilibili-live-tool type-check
pnpm -F @monorepo/bilibili-live-tool exec vitest run
```
