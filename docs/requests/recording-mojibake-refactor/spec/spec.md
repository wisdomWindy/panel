# 目标架构与冻结契约

版本：v1（方案，待评审）｜ 状态：**未实施**

## 1. 录制链路目标设计

### 1.1 数据流

```
recording_start(room_id)
  └─ MediaState::start_recording            [新增] 用例编排
      ├─ 前置校验（目录授权 / ffmpeg / 磁盘水位 / 是否已录制）
      ├─ PlayUrlResolver::resolve(room_id)  [新增端口] 房间 → 流地址
      ├─ FfmpegLocator::resolve(source)     [新增端口] 来源 → 可执行文件
      ├─ build_recording_launch(..)         [新增纯函数] → RecorderLaunch
      ├─ repository.insert_recording(row)    [已有] 先落库，status=Recording
      └─ RecordingRuntime::start(room_id, launch)   [已有] 状态机 + 重试
           └─ RecorderProcessPort::spawn → ffmpeg → 文件
stop:  MediaState::stop_recording [已有] → RecordingRuntime::stop → 收尾入库 → 事件
```

### 1.2 新增端口（`application/media/ports.rs`）

```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StreamFormat { Flv, Hls }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedStream {
    pub url: String,
    pub format: StreamFormat,
    /// B 站 qn 值，仅用于诊断回显
    pub quality: u32,
}

#[async_trait::async_trait]
pub trait PlayUrlResolver: Send + Sync {
    async fn resolve(&self, room_id: &str) -> Result<ResolvedStream, MediaIoError>;
}

pub trait FfmpegLocator: Send + Sync {
    fn resolve(
        &self,
        source: FfmpegSource,
        authorized: Option<&std::path::Path>,
    ) -> Result<std::path::PathBuf, MediaIoError>;
}
```

实现：

| 端口 | 实现 | 位置 |
| --- | --- | --- |
| `PlayUrlResolver` | `BilibiliPlayUrlResolver` | `infrastructure/live_data/play_url.rs` |
| `PlayUrlResolver` | `UnavailablePlayUrlResolver` | 同文件，`spawn` 不可用时的降级 |
| `FfmpegLocator` | `TauriFfmpegLocator` | `infrastructure/media/ffmpeg.rs` |

### 1.3 流地址解析契约（实现要点）

- 接口：`GET https://api.live.bilibili.com/room/v1/Room/playUrl?cid={room_id}&qn={qn}&platform=web`
- 必带请求头：`User-Agent`（桌面 UA）、`Referer: https://live.bilibili.com/`。缺少二者时接口可能返回 `code != 0`。
- 响应判定：
  - `code == 0` 且 `data.durl` 非空 → 取 `durl[0].url`，`format = Flv`
  - `code == 0` 但 `durl` 空 → `recording_stream_unavailable`（在播但无可用地址）
  - `code != 0` → 按 `msg` 归类：能判定为"未开播/房间不存在"时返回 `recording_room_offline`，其余按 `recording_stream_unavailable` 处理
  - 网络/超时 → `recording_stream_unavailable`（可重试）
  > 具体错误码与 `msg` 文案以实施时的接口实测为准，实现中以表格集中映射，不在逻辑里散落字符串。
- 清晰度映射（**建议值，实施时以实测为准**）：`RecordingQuality::{Original, High, Standard}` → `qn::{10000, 400, 250}`；映射放在解析器内，不进入领域模型。
- **不引入 Cookie/登录态**：仅公开房间的公开流；付费/大会员清晰度不在本期范围。

### 1.4 ffmpeg 启动参数（纯函数，可单测）

```
<exe> -hide_banner -loglevel warning
      -user_agent <UA>
      -headers "Referer: https://live.bilibili.com/\r\n"
      -rw_timeout 15000000
      -y -i <stream_url>
      -c copy
      -f <flv|mp4>
      <output_path>
```

- `-c copy`：不转码，CPU 占用与画质损失最小。
- `-headers` / `-user_agent`：B 站 CDN 校验来源，缺省会 403（这是录制"开始后立刻失败"的常见原因）。
- `-rw_timeout`：网络读超时，避免断流后进程悬挂。
- 输出容器：`RecordingFormat::Flv` → `-f flv`，`Mp4` → `-f mp4`（mp4 用 `-movflags +faststart` 便于预览，本期可选）。

`build_recording_launch` 放在 `domain/media/pipeline.rs`，输入输出均为纯数据，返回 `RecorderLaunch`，不触碰 IO，便于用参数断言测试覆盖。

### 1.5 前置条件与错误码

| 情况 | 错误码 | 可重试 |
| --- | --- | --- |
| 未配置录制目录授权 | `recording_directory_required` | 否 |
| ffmpeg 不可用/未授权 | `recording_ffmpeg_unavailable` | 否 |
| 房间未开播 | `recording_room_offline` | 是 |
| 在播但拿不到地址 | `recording_stream_unavailable` | 是 |
| 已在录制中 | 复用既有 `media_conflict` | 否 |

### 1.6 落盘与入库顺序

先 `insert_recording`（status=Recording）再启动进程：进程若启动失败，将该行标记为 `Failed`，保证"文件存在必有记录、记录存在必有归属"。文件名 `<room_id>_<yyyyMMdd_HHmmss>.<ext>`，目录取 `RecordingConfig.directory` 的授权路径。

## 2. 文本编码边界

### 2.1 新增模块 `infrastructure/text.rs`

```rust
/// 外部字节 → 文本：严格 UTF-8 优先，失败回退 GB18030（Windows 中文默认编码）。
pub fn decode_external_text(bytes: &[u8]) -> String;

/// 导出 CSV 前置 UTF-8 BOM，避免 Excel 按 ANSI 解释。
pub fn with_utf8_bom(bytes: Vec<u8>) -> Vec<u8>;
```

- 依赖 `encoding_rs`（**已在 Cargo.lock 中，版本 0.8.41，经 reqwest 传递引入**；只需在 `Cargo.toml` 显式声明以稳定版本，不新增实际下载体积）。
- `with_utf8_bom` 幂等：已有 BOM 不重复添加。

### 2.2 改造点

| 位置 | 变更 |
| --- | --- |
| `infrastructure/media/ffmpeg.rs:136` | `from_utf8_lossy` → `decode_external_text` |
| `infrastructure/live_data/file_codec.rs:42,84` | 输出包 `with_utf8_bom` |
| `infrastructure/live_data/files.rs:142,178` | 同上 |
| `infrastructure/live_data/file_codec.rs:71` | TXT 导入改 `decode_external_text`（GBK txt 可导入） |

不改：内部 JSON/TXT 持久化格式、数据库文本（均为 UTF-8，无此问题）。

## 3. 架构重构目标

### 3.1 目录目标

```
ipc/
  media/
    mod.rs        // 仅 re-export
    commands.rs   // 命令实现（参数校验 + 调用用例）
    mapping.rs    // DTO ⇄ domain 映射
    preview.rs    // media-preview URI scheme（已独立，保留）
  member/
    mod.rs
    ai.rs / voice.rs / membership.rs / moderation.rs
    mapping.rs
  error.rs        // 改为表驱动
```

### 3.2 规则

1. **命令层**只做三件事：反序列化、校验入参、调用 `application`；不得出现业务分支。
2. **映射层**是纯函数集合，禁止 IO；DTO 与领域模型一一对应。
3. **错误映射**表驱动：`const ERRORS: &[(&str, &str)]` + 一次查找，新增错误码只加一行。
4. `application/*/state.rs` 中的视图组装（事件 payload、DTO）迁到 `ipc/*/mapping.rs`。
5. `MediaState` 保留为装配者，录制/礼物/特效/TTS 各自的内聚逻辑下沉到同层子模块。

### 3.3 不改变的东西

- 四层分层本身、端口命名习惯、`t_*` 表结构、IPC 命令名与 DTO 字段名（对外契约零变更）。
- 前端接口面：仍通过 `platform/desktop/contracts` 访问，命令集合不变。

## 4. 兼容性

| 面 | 影响 |
| --- | --- |
| IPC 命令名/DTO | 不变（新增仅随录制功能） |
| 数据库 | 不变，无迁移 |
| 配置文件 | 不变；`RecordingConfig.ffmpeg_source` 语义首次真正生效 |
| 更新清单/发布 | 不受影响 |
