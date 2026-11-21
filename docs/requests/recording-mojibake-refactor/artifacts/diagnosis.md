# 根因诊断

所有结论均来自当前代码，行号对应 commit `415e644`。

## 1. 直播录制不可用

### 1.1 入口是桩实现

`src-tauri/src/ipc/media.rs:679`

```rust
#[tauri::command]
pub async fn recording_start(
    _: State<'_, AppState>,              // 注意：state 未使用
    request: RecordingStartRequestDto,
) -> Result<RecordingStatusDto, DesktopError> {
    crate::domain::validation::RoomId::parse(&request.room_id)
        .map_err(|_| DesktopError::from(MediaIoError::invalid("recording room is invalid")))?;
    Err(MediaIoError::unavailable("live stream resolver is unavailable").into())
}
```

即：校验完房间号就固定返回"流地址解析器不可用"。**无论房间是否在播，录制都不可能开始。**

### 1.2 真正缺失的三个环节

| 环节 | 现状 | 证据 |
| --- | --- | --- |
| 流地址解析 | **完全不存在** | 全仓库检索 `playUrl / play_url / durl / getRoomPlayInfo` 零命中 |
| 录制启动用例 | **不存在** | `application/media/state.rs` 只有 `stop_recording`（:603），没有 `start_recording` |
| ffmpeg 可执行文件解析 | **半成品** | `MediaState.local_ffmpeg_executable`（:139）只在 `set_ffmpeg_source`（:420）被写入，**全仓库无任何读取点**；`FfmpegSource::Bundled` 分支直接给 `None`，从未解析随包资源路径 |

### 1.3 已存在、无需重写的部分

录制并非从零开始，以下机制完整且带测试：

- `RecordingRuntime`：状态机（Idle/Starting/Recording/Stopping/Finalizing/Error）、启动重试 3 次、磁盘水位保护、弹幕高能标记、快捷键（`application/media/recording.rs`）。
- `RecorderProcessPort` + `LocalFfmpegProcessPort`：子进程启停、stdout/stderr 捕获、`kill_on_drop`（`infrastructure/media/ffmpeg.rs`）。
- `RecordingRecord` 持久化、列表、重命名、删除、预览（`infrastructure/media/repository.rs`、`preview.rs`）。
- 配置与校验：`RecordingConfig`、`validate_recording_config`（`domain/media/{models,rules}.rs`）。

**结论**：缺的是"把房间变成一条推进 ffmpeg 的启动参数"这段编排，而非录制基础设施。

## 2. 文本乱码

### 2.1 导出 CSV 无 BOM（用户可见）

| 位置 | 代码 |
| --- | --- |
| `infrastructure/live_data/file_codec.rs:42` | `csv::Writer::from_writer(Vec::new())` |
| `infrastructure/live_data/file_codec.rs:84` | 同上 |
| `infrastructure/live_data/files.rs:142` | 同上（数据统计导出） |
| `infrastructure/live_data/files.rs:178` | 同上（排行导出） |

`csv` crate 输出 UTF-8 且不带 BOM。中文 Windows 的 Excel 打开无 BOM 的 CSV 时按本地 ANSI（GBK）解释，中文与用户名变成乱码。**这是最常见的用户可见乱码。**

### 2.2 进程输出按 UTF-8 强行解码（错误信息乱码）

`infrastructure/media/ffmpeg.rs:137`

```rust
Ok(String::from_utf8_lossy(&captured).into_owned())
```

中文 Windows 下 ffmpeg 的错误输出是 GBK 字节，按 UTF-8 lossy 解码会得到一串 `U+FFFD` 替换字符。该字符串随后被拼进错误信息：

`application/media/recording.rs:158`
```rust
format!("recorder exited with code {:?}: {}", exit.code, exit.stderr)
```

因此录制失败时界面上看到的 ffmpeg 报错是乱码，**用户拿到的是不可诊断的错误**。

### 2.3 同类隐患（导入方向）

`infrastructure/live_data/file_codec.rs:71`：敏感词 `Txt` 导入使用 `String::from_utf8(content)`，GBK 编码的 txt 会直接报 `encoded file is not UTF-8`，而不是尝试解码。属于同一类编码边界缺失，一并纳入修复。

## 3. 架构现状

### 3.1 已具备的分层

```
domain/          纯规则与模型（无 IO）
application/     用例编排 + 端口定义（ports.rs）
infrastructure/  适配器实现（sqlite / reqwest / ffmpeg / tauri）
ipc/             命令边界 + DTO
```

分层本身是成立的，问题不在"没有架构"，而在**边界的实现质量**。

### 3.2 具体问题

| 问题 | 证据 |
| --- | --- |
| 巨型文件 | `ipc/member.rs` 51.5KB、`ipc/media.rs` 51.0KB、`application/member/state.rs` 45.8KB、`application/media/state.rs` 27.2KB |
| IPC 承担了过多职责 | 同一文件内混有命令、DTO 映射、事件发射、URI scheme 处理 |
| DTO 映射样板重复 | 每个子域手写 `x_dto()` / `x_from_dto()` 成对函数，字段逐一手抄 |
| 错误映射靠手写 match | `ipc/error.rs` 用一长串 `match code` 映射中文文案，新增错误码易漏 |
| `MediaState` 聚合过多 | 录制、礼物、特效、TTS、预览的编排都堆在同一个 `state.rs` |

### 3.3 重构方向（详见 `spec/spec.md`）

保持四层不变，把"巨型文件"与"重复样板"收敛：

1. IPC 按子域拆目录，命令只做参数校验与调用。
2. DTO 映射迁到 `ipc/*/mapping.rs`，与命令分离。
3. 错误映射改为表驱动，新增错误码只需加一行。
4. `MediaState` 内部按子域拆成多个服务对象，`state.rs` 只做装配。
