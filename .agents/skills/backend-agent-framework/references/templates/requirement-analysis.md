# Requirement Analysis 模板

## 必需工件路径

- `docs/requests/<request-id>/requirements/requirement-analysis.md`

## `requirements/requirement-analysis.md`

必需章节：

- 请求摘要
- 需求目标与业务价值
- 范围内事项
- 范围外事项
- 关键参与者与调用方
- 显式约束归纳
- 隐含约束与假设
- 核心流程与状态分析
- 关键实体、接口与事件观察
- 风险、歧义与待确认项
- 拆分策略与建议拆分轴
- architecture-design 前置信号
- requirement-splitting 输入检查清单

### `显式约束归纳`

至少覆盖：

- 接口、事件、字段、状态、错误码约束
- 权限、幂等、超时、重试、回调、迁移、回滚、观测性要求
- 上游文档明确写出的不可更改前提

### `隐含约束与假设`

至少覆盖：

- 默认读写路径或调用顺序假设
- 兼容性、灰度、历史数据或跨服务依赖假设
- 若假设错误将影响拆分、设计或实现的风险点

### `拆分策略与建议拆分轴`

至少说明：

- 为什么要按当前维度拆分，而不是按页面、接口数量或代码目录机械拆分
- 推荐拆分维度，例如 service、endpoint contract、job/workflow、consumer/subscriber、data model/schema、integration flow、cross-service rule
- 哪些内容必须保持在同一模块内，避免拆分后语义丢失

### `requirement-splitting 输入检查清单`

至少确认：

- 是否已具备稳定的范围边界
- 是否存在阻止无损拆分的关键歧义
- 哪些开放问题只需记录，哪些会阻止进入 requirement-splitting
