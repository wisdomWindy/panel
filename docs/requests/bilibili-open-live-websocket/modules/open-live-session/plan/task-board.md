# Task Board：开放平台会话集成

| ID | 任务 | 状态 | 模式 | 前置 | 规格 | 功能单元 | 范围 | 整洁性/边界 | 状态/契约/观测 | 测试切入点 | 待确认或假设 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| T1 | 配置与签名 | complete | serial/TDD | spec approved | 配置、HTTP、安全 | env validation/signing | infrastructure | 纯函数与配置分离 | 不记录 Secret | 固定向量、缺配置 | 使用虚构测试凭证 |
| T2 | Proto 与事件映射 | complete | serial/TDD | T1 | WSS、事件、边界 | codec/mapper | infrastructure/domain adapter | 无网络副作用 | 未知 CMD 忽略 | binary/JSON fixtures | AUTH_REPLY 兼容解析已批准 |
| T3 | HTTP/WSS 会话 | complete | serial/TDD | T1,T2 | 工作流、错误、cleanup | api/transport/task | infrastructure/application port | 单 socket owner | stage/code 日志 | mock servers/lifecycle | 正式平台联调后置 |
| T4 | 应用与前端迁移 | complete | serial/TDD | T3 | IPC、UI、shutdown | DTO/state/wiring/toolbar | backend+renderer | 不暴露凭证 | identityCode -> roomId | contract/store/runtime tests | 原子发布已批准 |
| T5 | 验证与交接 | complete | serial | T1-T4 | 验收、rollout | test/build/scan | whole module | 无无关改动 | evidence only | full suites/build/scan | 缺正式凭证只限制 live smoke |
| T6 | 内置应用标识 | complete | serial/TDD | T5 | 配置、安全、rollout | config identifiers | infrastructure | 当时 Secret 仍由环境读取 | embedded identifiers | config/full/build/scan | 历史阶段，最终语义由 T8 取代 |
| T7 | 固定应用标识 | complete | serial/TDD | T6 | 配置、安全、rollout | config/fixed identifiers | infrastructure | 当时只读取 Secret 环境变量 | fixed identifiers + runtime Secret | config/full/build/scan | 历史阶段，最终语义由 T8 取代 |
| T8 | 固定完整应用凭证 | complete | serial/TDD | T7 | 配置、安全、rollout | config/embedded credentials | infrastructure/bootstrap | 不查询凭证环境变量 | embedded credentials + accepted extraction risk | signature/full/build/binary scan | Secret 可从安装包提取 |
