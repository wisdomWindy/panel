# 请求模板

## 必需工件路径

- `docs/requests/<request-id>/request.md`
- `docs/requests/<request-id>/state.json`
- `docs/requests/<request-id>/artifacts/prd-snapshot.md`
- PRD 驱动请求在 requirement splitting 之后还需要 `docs/requests/<request-id>/requirements/requirement-map.md`

## `request.md`

必需章节：

- 请求标识
- 来源链接
- 业务摘要
- 目标陈述
- 初始完成信号
- 触发条件
- 初始上下文来源
- 人工交接点
- 影响范围
- 参与服务或模块

## `state.json`

必需字段：

- `request_id`
- `source`
- `stage`
- `status`
- `artifacts`
- `approvals`
- `loop`

必需结构：

```json
{
  "request_id": "...",
  "source": {},
  "stage": "requirement-splitting",
  "status": "in_progress",
  "artifacts": {},
  "approvals": {},
  "loop": {
    "iteration": 1,
    "state": "running",
    "last_completed_stage": null,
    "pending_gate": null,
    "reentry_reason": "new_request"
  }
}
```

对于 PRD 驱动请求，初始化 `stage` 为 `requirement-splitting`。  
对于经 `bugfix-intake` 归一化的 bugfix 请求，初始化 `stage` 为：

- `architecture-design`：当修复变更了对外契约形态、服务边界、持久化模型、异步拓扑或跨服务集成边界
- 否则为 `spec`

## `artifacts/prd-snapshot.md`

必需章节：

- 来源摘要
- 关键业务目标
- 显式行为约束
- 从来源提取的接口、事件、数据模型与流程规则
- 相关服务或模块
- 上游 PRD 中值得关注的开放问题

### `显式行为约束`

如果 PRD 已经明确写出要求，这一节必须保留这些上游显式细节。

至少提取并归一：

- 来源中已存在的接口相关约束
- 来源中已存在的事件或回调约束
- 来源中已存在的数据模型或 schema 约束
- 来源中已存在的流程、重试、超时、幂等、权限、迁移、回滚与观测性规则

## `requirements/requirement-map.md`

PRD 驱动请求的必需章节：

- 拆分摘要
- 来源模块
- 拆分后的模块与功能单元
- 来源追踪关系
- 模块依赖
- architecture-design 路由提示
- spec 输入检查清单
- 开放问题

## `requirements/modules/<module-id>.md`

每个拆分模块建议章节：

- 模块名称
- 来源快照
- 来源追踪引用
- 业务目标
- 调用方可见行为
- 来源中的接口、事件、数据与流程规则
- spec 必须保留的上游约束
- 是否需要 architecture-design
- 开放问题
