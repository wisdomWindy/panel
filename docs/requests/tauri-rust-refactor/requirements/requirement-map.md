# Tauri / Rust 重构需求拆分总览

## Splitting Summary

根据依赖顺序、业务能力所有权和独立验收边界，将 PRD 的 20 个功能及横切要求拆成 6 个顺序模块。拆分承接 `analysis/requirement-analysis.md` 的闭世界范围、前端/Rust/外部服务边界与外部接口待补口径，不新增产品行为。

## Source Modules

- 架构/版本/会员：PRD 0、2-5
- 用户与功能清单：PRD 6-7
- 应用壳与页面：PRD 8、9.1-9.20
- IPC/Sidecar/数据/灾备/隐私：PRD 10-15

## Split Modules And Functional Units

| 顺序 | ID | 内容 | PRD 功能 | Page design | Architecture design |
| --- | --- | --- | --- | --- | --- |
| 1 | `platform-foundation` | Tauri scaffold、typed IPC/Event、Rust 分层、SQLite、配置底座、双 build feature、安全边界 | 16 的存储底座 + 横切 2-5/10/12 | 否 | 是 |
| 2 | `live-data-safety` | 房间连接、只读事件、身份、敏感词、黑名单、统计、榜单、CSV | 1、2、3、5、9、10、11 | 是 | 是 |
| 3 | `interaction-media` | 上舰特效、礼物提醒/音效、TTS、FFmpeg 录制与高光 | 6、7、8、12、13 | 是 | 是 |
| 4 | `application-shell` | 四区主界面、导航/状态、设置、主题布局、多语言 | 19、20 + 16 的 UI | 是 | 是 |
| 5 | `membership-intelligence` | 会员中心、模块状态机、AI/ASR、刷屏与自动禁言 | 4、14、15 | 是 | 是 |
| 6 | `extensions-release-resilience` | 插件 WASM、更新、完整配置导入导出、迁移、日志保留、灾备/隐私/合规/发布 | 16、17、18 + 11/13-15 横切 | 是 | 是 |

## Requirement-analysis Carry-forward

- 只读/无发送能力是所有模块的硬不变量。
- 免费包必须从编译与 Command registry 排除会员能力。
- 外部 OAuth/支付/manifest/update/Cookie API 和真实 Sidecar artifact 只允许作为 typed port/config/unavailable state，不得伪造生产成功。
- SQLite 采用 PRD 显式 SQL 的 21 表；迁移必须备份、事务、幂等、失败回滚。
- 现有 Electron caller 是迁移/删除闭包，不是目标架构的一部分。

## Source Traceability

每个模块工件包含原始证据锚点、模块闭世界 allowlist、角色职责、外部接口缺口及 spec carry-forward。原 PRD 已是 Markdown；没有需要转换的图片表格/富文本，因此不产生额外 normalized companion 文件，模块工件中的 raw excerpts 保持原文，结构化表只用于导航。

## Module Dependencies

```text
platform-foundation
  -> live-data-safety
  -> interaction-media
  -> application-shell
  -> membership-intelligence
  -> extensions-release-resilience
```

- `live-data-safety` 依赖统一 Rust state、repository、gateway、error/event contract。
- `interaction-media` 依赖 live event 和路径/Sidecar guard。
- `application-shell` 依赖公共 stores/status/event 已稳定。
- `membership-intelligence` 依赖 build feature、module registry、free capability 已稳定。
- `extensions-release-resilience` 在完整模块存在后验证插件、更新、迁移和灾备闭环。

## Execution Sequence

严格按表格顺序。每个模块完成 `page-design? -> spec -> architecture-design -> plan -> execute -> verify -> review` 后才推进下一个模块。

## Page-design Routing Hint

- `platform-foundation`：无用户页面，直接进入 spec。
- 其余模块：均包含 PRD 明确布局、表单、列表、状态、错误或交互，先进入 page-design。

## Spec Input Checklist

- 保留模块原始字段、按钮、默认值、状态、异常、阈值和验收。
- 指明 Tauri Command/Event DTO、Rust/Vue ownership、取消与资源清理。
- 指明 free/member compile feature 与 runtime capability。
- 指明外部接口 `available/unavailable` 边界。
- 指明 Electron caller 迁移/删除和测试适配闭包。
- 每模块必须有 pattern-fit 与 source-grounding 标记。

## Open Questions

- 仅外部接口/资源待补，不阻断本地模块 spec：会员服务/OpenAPI、公钥、模块/update manifests、Cookie API、FFmpeg/whisper binaries。
- 生产集成材料到达时，只回滚对应模块，不改变已完成公共底座。
