# 实施与验证记录

日期：2026-11-21 ｜ 分支：`main`

## 1. 结论

三件诉求已落地：**录制可用**、**乱码消除**、**编码策略统一**。重构采取“先补端口与纯函数、再动调用方”的方式完成结构性整改；巨型文件拆分列为后续项（见第 6 节）。

## 2. 编码策略（已统一）

| 方向 | 规则 | 实现 |
| --- | --- | --- |
| 自有文本（源码、JSON、界面文案） | 一律 UTF-8 | 既有约定，未改 |
| 导出文件（CSV） | UTF-8 **带 BOM** | `text::with_utf8_bom`，4 处写出点 |
| 导入文件（CSV） | 剥离 BOM 后再解析 | `text::strip_utf8_bom`，2 处读取点 |
| 外部进程输出（FFmpeg） | UTF-8 严格 → GB18030 回退 | `text::decode_external_text` |

之所以导出带 BOM、而内部一律 UTF-8：Excel 在中文 Windows 下会按 ANSI 解码无 BOM 的 UTF-8，这是用户实际看到的乱码来源；BOM 是唯一能让 Excel 正确识别且不污染内部数据的做法。导入侧剥离 BOM 是为了让导出文件能原样再导入。

`encoding_rs` 之前在锁文件中已存在（reqwest 传递依赖，0.8.41），本次只是显式声明，未增加下载体积。

## 3. 录制链路

新增两个端口 + 一个纯函数模块，把“房间号 → 一条可执行的 FFmpeg 命令”这段原本缺失的编排补齐：

```
recording_start (IPC)
  └─ MediaState::start_recording            application/media/state.rs
      ├─ validate_recording_room_id          domain/media/rules.rs     纯函数
      ├─ grants.resolve(RecordingDirectory)  既有路径授权
      ├─ FfmpegLocator::locate               infrastructure/media/ffmpeg_locator.rs
      ├─ PlayUrlResolver::resolve            infrastructure/media/play_url.rs
      │    ├─ GET room/v1/Room/get_info      → live_status 必须为 1
      │    └─ GET room/v1/Room/playUrl       → data.durl[0].url
      ├─ recording_arguments                 domain/media/pipeline.rs  纯函数
      ├─ repository.insert_recording         先落库（status=Recording）
      └─ RecordingRuntime::start             既有实现：3 次重试 + 磁盘水位
```

关键设计：参数构造与响应解析都是**纯函数**，因此无需网络或进程即可被单测覆盖（6 + 11 项）；HTTP 与进程只在 `infrastructure` 内。

FFmpeg 参数含 `-user_agent` 与 `Referer: https://live.bilibili.com/`，这是 CDN 反盗链的硬性要求；`-c copy` 不转码。

## 4. 真实端到端验证

```
room    : 1708419126（通过 get_user_recommend 取得，确认在播）
ffmpeg  : 8 秒后强制停止
结果    : recorded 5505024 bytes, header=[70, 76, 86], 即 "FLV"
```

即真实 B 站直播间 → 解析 → 启动 ffmpeg → 落盘 5.5 MB 可识别的 FLV 容器。

测试入口（默认 `#[ignore]`，需要网络与 ffmpeg）：

```bash
RECORDING_ROOM_ID=<在播房间号> RECORDING_FFMPEG=<ffmpeg 路径> \
  cargo test --lib --features free records_a_live_room_end_to_end -- --ignored --nocapture
```

## 5. 验证过程中发现并修掉的两个真问题

**（1）下播房间会照样返回拉流地址。** 首次端到端测试用 21452505，解析成功但 ffmpeg 报 `HTTP 403 Forbidden`。核查发现该房间 `live_status=0`——`playUrl` 对已下播房间仍返回旧的 `durl`，CDN 随后拒绝。若不检查，用户看到的是“FFmpeg 退出码异常”这种无意义报错。现已增加 `get_info` 前置校验，未开播直接返回 `recording_room_offline`（不可重试，文案“直播间当前未开播”）。

**（2）出错后无法重试。** 前端 `startDisabledReason` 把 `error` 状态列为禁用原因，但按钮文案却是“重试录制”，而后端 `RecordingRuntime::start` 明确接受从 `Error` 重启——录制一旦失败只能重启应用。已移除该条件，保持“出错可重试”。

## 6. 结构与重构

已完成：

- 新增 `domain/media/pipeline.rs`（参数构造纯函数）、`infrastructure/text.rs`（编码边界）、`infrastructure/media/play_url.rs`、`infrastructure/media/ffmpeg_locator.rs`；
- 端口层新增 `PlayUrlResolver`、`FfmpegLocator`，并提供 `Unavailable*` 默认实现，使测试与降级启动不受影响；
- `recording_start` 从“校验参数后返回 `unavailable`”的桩改为真实编排，IPC 层只剩一行委托；
- 错误映射补齐 12 个 `recording_*` 码，用户看到的是可操作的中文提示而不是“操作失败”。

未完成（明确不做“假重构”）：

- `ipc/media.rs`（1550 行）、`ipc/member.rs`、`application/member/state.rs`、`application/media/state.rs` 的拆分。这类改动会与本次功能改动混在同一批 diff 中，风险与可审阅性都不划算。建议作为独立一次重构提交，以现有 179 项测试 + 契约测试为安全网。

## 7. 测试结果

| 范围 | 结果 |
| --- | --- |
| `cargo test --lib --features free` | **179 passed, 0 failed, 1 ignored**（ignored 为需网络的端到端录制） |
| 新增：`infrastructure::text` | 7 项（UTF-8 / GBK / 混合 / BOM 幂等 / 无替换字符） |
| 新增：`domain::media::pipeline` | 6 项（copy、请求头、MP4 分片、非法 URL、无父目录、容器映射） |
| 新增：`infrastructure::media::play_url` | 11 项（在播/未开播/未知错误/空地址/非 JSON/房间号校验/qn 映射） |
| 新增：`infrastructure::media::ffmpeg_locator` | 5 项（内置探测顺序、缺失文案、本地授权、未授权、路径失效） |
| 端到端 | 1 项，真实直播间，5.5 MB FLV |

## 8. 未覆盖

| 项 | 说明 |
| --- | --- |
| 前端 tsc / vitest | 依赖已删除（上一轮清理），需 `pnpm install` 后才能跑；前端仅改动一处禁用条件 |
| macOS / Linux 录制 | 仅 Windows 实测；参数与路径处理走同一纯函数，平台差异在 ffmpeg 可执行名与路径 |
| 长时间录制稳定性 | 只验证了 8 秒；分段、水位停止、断流重连属既有逻辑，未做长跑 |
| 自动录制联动 | 配置项 `autoRecord` 仍未接入事件触发（既有缺口，本次未扩大范围） |
