# 实施计划

依赖：`artifacts/diagnosis.md`（根因）、`spec/spec.md`（目标架构）

## 0. 阶段总览

| 阶段 | 内容 | 风险 | 可独立回滚 |
| --- | --- | --- | --- |
| S1 | 文本编码边界 + 两处乱码修复 | 低 | 是 |
| S2 | 录制打通（端口 + 用例 + IPC） | 中 | 是 |
| S3 | 后端分层重构（拆文件、收敛映射） | 中 | 是 |
| S4 | 方案复核与验收 | — | — |

顺序理由：S1 最小且被 S2 依赖（ffmpeg 报错要能读懂才便于调 S2）；S3 放最后，避免与功能改动混在同一批 diff 里。

## 1. S1：文本编码（预计 1 个提交）

**改动**

1. 新增 `src-tauri/src/infrastructure/text.rs`：`decode_external_text`、`with_utf8_bom`。
2. `Cargo.toml` 增 `encoding_rs`（先确认是否已在依赖树，若已传递引入则显式声明版本）。
3. 替换 4 处 CSV 导出、1 处 ffmpeg 输出解码、1 处 TXT 导入解码。

**测试**

- GBK 字节样本（"中文测试"的 GB18030 编码）→ 解码出正确中文。
- 合法 UTF-8 输入行为不变（不误判）。
- `with_utf8_bom` 输出以 `EF BB BF` 开头，且对已带 BOM 输入幂等。
- CSV 导出结果首三字节为 BOM，`csv` 解析器忽略 BOM 后字段数不变。
- 既有 `file_codec` / `files` 测试保持通过。

**验收**：导出的 CSV 在中文 Windows Excel 直接打开无乱码；ffmpeg 失败信息为可读中文。

## 2. S2：录制打通（预计 2–3 个提交）

### 2.1 端口与纯函数（可先行合入）

- `application/media/ports.rs`：`ResolvedStream`、`StreamFormat`、`PlayUrlResolver`、`FfmpegLocator`。
- `domain/media/pipeline.rs`：`build_recording_launch`。
- 单测：参数序列断言（`-c copy`、`-f flv/mp4`、`-headers` 含 Referer、输出路径末位）；非法输入（空 URL、缺失扩展名）报错。

### 2.2 适配器

- `infrastructure/live_data/play_url.rs`：`BilibiliPlayUrlResolver`（reqwest，超时 8s，UA/Referer）+ 响应映射 + `UnavailablePlayUrlResolver`。
- `infrastructure/media/ffmpeg.rs`：`TauriFfmpegLocator`（Bundled → `resource_dir/bin/ffmpeg.exe`；Local → 已授权路径）。
- 单测：用本地 stub HTTP 服务模拟 `code=0/durl`、`durl=[]`、未开播、超时四种响应。

### 2.3 用例与接线

- `MediaState::start_recording(room_id)`：前置校验 → 解析 → 落库 → 启动 → 失败回滚标记。
- `MediaState` 持有 `Arc<dyn PlayUrlResolver>`、`Arc<dyn FfmpegLocator>`；`lib.rs` 注入具体实现。
- `ipc/media.rs::recording_start` 删除桩，调用用例。

### 2.4 测试

- `start_recording` 在四种前置失败下返回对应错误码，且不产生 DB 行。
- 正常路径：stub 解析器 + `RecordingSpeech` 式假进程端口 → 断言 `insert_recording` 被调用一次、`runtime.state == Recording`。
- 进程启动失败 → DB 行状态为 `Failed`，运行时回到 `Error`。
- 停止路径回归：既有 `stop_recording` 测试保持通过。

### 2.5 手工验收（必须真实环境）

1. 对**正在直播**的真实房间点击开始录制，30 秒后停止，确认生成可播放文件。
2. 对未开播房间开始录制，确认提示"房间未开播"而不是通用错误。
3. 断开网络后开始录制，确认 8s 内失败并给出可读原因。
4. 删除录制目录授权后开始录制，确认提示"需要先选择录制目录"。

## 3. S3：后端分层重构（预计 3–4 个提交）

**顺序（每步独立编译 + 测试全绿）**

1. `ipc/error.rs` 改表驱动（改动最小、收益立即）。
2. `ipc/media.rs` → `ipc/media/{mod,commands,mapping}.rs`（纯搬移，禁止改逻辑）。
3. `ipc/member.rs` → `ipc/member/{mod,ai,voice,membership,moderation,mapping}.rs`。
4. `application/media/state.rs` 内部按子域拆服务（录制/礼物/特效/TTS），`MediaState` 只保留装配与共享状态。

**约束**

- 每个提交只做搬移或只做等价改写，**不与行为变更混提**。
- 契约测试（`media_contract_test`、`member_contract_test`、`release_contract_test`）作为"对外面未变"的硬证据。
- 每步结束检查：文件体积、`cargo check` 零告警、150+ 项测试全绿。

## 4. 风险与对策

| 风险 | 影响 | 对策 |
| --- | --- | --- |
| B 站 playUrl 接口变更或风控 | 录制开始失败 | 解析器独立可替换；错误码区分"未开播 / 拿不到地址"；先手工验收再合并 |
| 录制目录含中文/空格 | ffmpeg 路径解析失败 | 参数以数组传递（不经 shell），文件名用 `_` 与数字，S1 的编码修复覆盖日志 |
| mp4 直录中断产生损坏文件 | 播放失败 | 默认容器 FLV（断流仍可播放）；mp4 作为可选项并标注风险 |
| 拆文件过程中误改行为 | 隐性回归 | 纯搬移提交 + 契约测试 + 逐文件 diff 复核 |
| 现有测试覆盖不足 | 重构漏改 | 先补契约测试再搬移；S2 引入的端口测试作为新基线 |

## 5. 回滚

- 每个阶段独立 commit，回滚即 revert 该提交。
- S2 若在真实环境不达标：保留端口与纯函数（有测试），仅把 `recording_start` 恢复为桩并保留错误码，避免半可用状态。
- S3 纯搬移，回滚不影响功能。

## 6. 验收清单

- [ ] `cargo test --lib --features free` 全绿，且新增用例覆盖 S1/S2 的每条分支
- [ ] 导出的 CSV 在中文 Windows Excel 打开无乱码
- [ ] 录制失败时界面给出的 ffmpeg 原因是可读中文
- [ ] 真实直播间可录出可播放文件，时长与文件体积正确入库
- [ ] 未开播 / 无目录 / 无 ffmpeg / 断网 四种前置失败各有明确错误码
- [ ] 重构后 IPC 命令名与 DTO 字段零变更（契约测试为证）
- [ ] 四个巨型文件体积显著下降（目标：单文件 < 20KB）
- [ ] 文档：本节涉及的配置项与排障步骤补入 `docs/` 或 README

## 7. 工作量估算

| 阶段 | 代码量 | 说明 |
| --- | --- | --- |
| S1 | ~200 行（含测试） | 低风险，可当天完成 |
| S2 | ~600 行（端口/适配器/用例/测试） | 主体工作量，含真实环境验收 |
| S3 | ~0 净增（搬移为主） | 取决于拆多细，建议分 4 次提交 |

## 8. 需要你确认的两个取舍

1. **录制容器默认值**：建议默认 FLV（断流可播、无需 faststart），MP4 作为选项。若你更希望默认 MP4，S2 的默认值与风险说明需要调整。
2. **S3 深度**：是否包含"`application/media/state.rs` 按子域拆服务"这一步。它收益明显但 diff 较大；若只想到"拆 IPC 巨型文件"，可把第 4 步拆为独立后续项。
