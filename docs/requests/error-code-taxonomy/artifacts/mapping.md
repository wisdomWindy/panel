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

## 待你确认

1. 前缀后接 `snake_case`（`ROOM_invalid_id`）还是全大写（`ROOM_INVALID_ID`）？
2. `media_permission_denied` 归 `AUDIO` 是否合适？它也可能来自系统权限而非音频。
3. 是否保留旧码作为别名一段时间（前端同时识别），避免日志与文档断档？

确认后我按表逐条替换并跑全量测试。
