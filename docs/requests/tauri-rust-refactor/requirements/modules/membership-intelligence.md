# Module: Membership Intelligence

## Boundary

PRD 功能 4、14、15：会员中心/令牌/模块、AI/ASR、本地回复建议、刷屏检测与可选自动禁言。

## Raw Source Snapshot

- 5：Ed25519、7 天离线、module download/hash/version、到期暂停保留、续费恢复。
- 9.4：自动禁言默认关闭、首次风险确认、接口失败降本地告警。
- 9.14：用户主动生成、流式 delta、过滤/截断、仅本地预览、无发送 Command。
- 9.15：login/payment/order/module/active/expired UI 与流程；free 包无会员中心。

## Markdown Normalization

不适用：来源已是 Markdown 流程/表格/wireframe。

## Source Grounding And Allowlist

- 允许页面：会员中心各状态、AI 助手/配置/日志/模型下载、刷屏防护/规则/日志/风险确认。
- member center prepay shell 内置会员 build；postpay assets 可由签名 module 扩展。
- external-interface-pending：OAuth/payment/token/manifest/Cookie API；只定义 ports 与 unavailable UI。
- forbidden：free build member code、未验签激活、自动 AI、任何发送弹幕、无确认自动禁言。

## Responsibilities

- Vue：会员/模块/AI/告警状态 UI、用户输入、只读预览。
- Rust member feature：token/module state、LLM proxy、ASR lifecycle、rules/moderation port。
- External：membership backend、LLM、Cookie API、whisper artifacts。

## Acceptance

- 会员安装 <=5min、续费恢复 <=1min、离线 7 天；AI local assembly <=500ms；缺外部服务时免费能力不受影响。

## Downstream Obligations

- page_design_required: `true`
- architecture_design_required: `true`
- entry_stage: `page-design`
- execution_order: `5`
