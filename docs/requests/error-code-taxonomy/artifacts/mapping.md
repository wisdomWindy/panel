# PRD §14 错误码分类：映射表（待确认）

现状：代码里在用的错误码共 **23 个**，全部是无前缀的自由命名（如 `recording_room_offline`）。
PRD §14 定义了 13 个分类前缀（`ROOM` / `NET` / `RULE` / `GIFT` / `RECORD` / `TTS` / `AI_MODEL` / `AI_GEN` / `AUDIO` / `CONFIG` / `STORAGE` / `UPDATE` / `SYS`），前端按前缀聚合分类展示。

## 命名规则（建议）

```
<PREFIX>_<kebab 语义>       例：RECORD_room_offline
```

- 前缀大写，语义段小写下划线，便于日志检索与前缀切分
- 一个码只归一类；跨类场景取**用户可行动作**所属的类（如"磁盘满导致录制失败"归 `STORAGE`，不归 `RECORD`）

## 映射（23 个）

| 现码 | 建议新码 | 分类依据 |
| --- | --- | --- |
| `invalid_room_id` | `ROOM_invalid_id` | 房间号格式 |
| `audience_not_found` | `ROOM_audience_not_found` | 房间观众查询 |
| `recording_room_invalid` | `ROOM_room_invalid` | 房间上下文 |
| `recording_room_offline` | `ROOM_room_offline` | 未开播 |
| `external_service_unavailable` | `NET_service_unavailable` | 外部端点不可达 |
| `invalid_sensitive_word` | `RULE_invalid_word` | 敏感词 |
| `invalid_import_file` | `RULE_invalid_import` | 规则导入 |
| `invalid_user_uid` | `RULE_invalid_user` | 黑名单用户标识 |
| `media_conflict` | `RECORD_conflict` | 录制状态冲突 |
| `media_invalid` | `RECORD_invalid_param` | 录制参数 |
| `media_operation_failed` | `RECORD_operation_failed` | 录制流程 |
| `media_unavailable` | `RECORD_unavailable` | 录制能力不可用 |
| `media_permission_denied` | `AUDIO_permission_denied` | 音频/权限 |
| `recording_pipeline_invalid` | `RECORD_pipeline_invalid` | FFmpeg 参数 |
| `qwen3_tts_invalid_config` | `TTS_invalid_config` | TTS 配置 |
| `qwen3_tts_loading` | `TTS_model_loading` | 模型加载中 |
| `qwen3_tts_timeout` | `TTS_timeout` | 合成超时 |
| `invalid_config_key` | `CONFIG_invalid_key` | 配置键 |
| `data_maintenance_failed` | `STORAGE_maintenance_failed` | 数据维护 |
| `storage_unavailable` | `STORAGE_unavailable` | 数据库/文件 |
| `no_export_data` | `STORAGE_no_export_data` | 导出为空 |
| `path_authorization_failed` | `STORAGE_path_denied` | 路径授权 |
| `event_publish_failed` | `SYS_event_publish_failed` | 内部事件通道 |

## 迁移影响面（改前必须确认）

1. **前端文案表**：`ipc/error.rs` 的 `match error.code` 与 locale 里的错误文案 key，全部要跟着改
2. **测试断言**：凡是断言具体 code 的测试（约 15 处）都要同步
3. **用户文档 / 更新日志**：若曾对外提到过 code，需要改
4. **日志检索**：历史日志里的旧码与新码并存，需要说明

## 已确认的决定（2026-11-21）

1. 风格：**全大写**，PREFIX_SEMANTIC，例 ROOM_INVALID_ID
2. media_permission_denied 归 **SYS_PERMISSION_DENIED**（不归 AUDIO）
3. **不保留旧码别名**，一次性替换，不做双写兼容

## 重要修正：清点范围被低估

上一版表里的 23 个码是用窄正则提取的。改用覆盖 code: 字段与全部错误构造器后，源码里的候选字面量是 **103 个**（含少量误报，如 explorer/open/	hreshold）。

也就是说这次迁移的真实规模是原先估计的 **4 倍**，涉及：

- src-tauri 全部错误构造点
- src/renderer 前端文案映射（shell.errors 10 个键 + 各处 errorCode 判断）
- 约 15–25 处断言具体 code 的测试（Rust + TS）
- 成员版（membership/module/auth）独有的一批码，PRD 里没有对应前缀，需要单独归类

## 执行计划（一步不可省）

1. 用精确规则重新生成权威清单（排除误报），按 13 个前缀逐条归类
2. 脚本化替换 + 编译门禁（cargo test 必过）
3. 前端映射与 locale 键同步
4. 全量回归（Rust + vitest + tsc + build）

## 需要你决定的一件事

成员版那批码（uth_*、entitlement_*、module_*、membership_*、moderation_*）在 PRD §14 里**没有对应前缀**。三种处理：

- A：归入 SYS_（最省事，但分类信息丢失）
- B：在 PRD §14 增补 MEMBER 前缀（需改 PRD）
- C：本期只迁移免费版路径上的码，成员版的码保持原样（迁移不彻底）

我倾向 **B**：既然 PRD 定义了分类体系，缺前缀就应该补文档，而不是把成员功能硬塞进 SYS。