# Live Data And Safety Task Board

| ID | 任务 | 状态 | Readiness | 模式 / 前置 | Spec | 范围与目标 | Source / delta | 验证切入点 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| LDS-T01 | 合同镜像与失败测试 | done | ready | 串行 TDD / plan approved | LDS-01..15 | Rust/TS DTO、commands、events、errors、fixtures；仅合同和红测 | approved spec / none | serde、TS contracts、public-name count |
| LDS-T02 | Rust 领域规则与聚合器 | done | ready | 串行 TDD / T01 | LDS-01..04、06、08..11 | validation、identity、safety pipeline、stats、rank | PRD+confirmed / none | pure unit、边界、negative pipeline |
| LDS-T03 | Repository、codec、path grant | done | ready | 串行 TDD / T02 | LDS-05、07、12 | shared DB、CRUD、transaction import、round-trip、save grant | approved formats only | temp DB rollback、codec、purpose ACL |
| LDS-T04 | LiveDataState 与连接管线 | done | ready | 串行 TDD / T03 | LDS-01、02、04、06、08、10 | session/reconnect/cancel/pipeline/cadence/unavailable adapter | approved states/events | fake transport、paused time、cleanup |
| LDS-T05 | IPC、权限与 gateway | done | ready | 串行 TDD / T04 | LDS-01..14 | thin commands、9 events、DesktopError、free/member registry | approved contract / none | Rust+TS contract、ACL、dual feature |
| LDS-T06 | Root runtime、stores、projection | done | ready | 串行 TDD / T05 | LDS-03、08..10、14、15 | one listener owner、generation、adapters、formatters、media projection | architecture/code facts / none | cleanup、stale、immutable、timer tests |
| LDS-T07 | 连接、弹幕与安全页面 | done | ready | 串行 / T06 | LDS-01..07、13、15 | approved controls/forms/tables/states; remove login gate | page design / allowlist only | UI state、validation、keyboard/a11y |
| LDS-T08 | 看板、排行与导出 | done | ready | 串行 / T07 | LDS-08..13 | five metrics/three trends/three ranks/export dialog | page design / allowlist only | adapter/store/chart/viewport tests |
| LDS-T09 | 删除闭包与全量验收 | done | ready | 串行 / T08 | LDS-01..15 | remove old renderer owners, full evidence, changelog | acceptance+code facts / none | full suite、dual builds、runtime、symbol scans |

## 通用执行合同

- 执行顺序固定为 T01 -> T09；每项遵循 red -> green -> refactor，完成后立即更新状态与 `execution/changelog.md`。
- Context 为 `S3 broad/full`，但每项只读目标文件、direct callers/consumers、focused tests、approved spec/architecture。
- Rust 是业务真相；TS adapter/store/page 不计算过滤、统计、排名或授权。生产 fake/simulated live data 禁止。
- 所有 product-content 必须在 spec closed-world allowlist；`missing-source` 只能变成 unavailable，不得进入实现。
- 删除/修改前后均检查完整功能链；清理 import/helper/type/request/state/watch/timer/test/mock/comment 闭包。
- Robustness：外部值校验、Promise/async 错误、loading finally、late response、cleanup、DB/path failure 都必须有测试或运行证据。
- Maintainability：不引入 catch-all utils、generic CRUD、class manager、hidden side effect、无 owner timer/listener 或单 caller helper 文件。
- 待确认项：none。已批准纯技术假设仅限 clock 测试机制和满足公开语义的有界 event delivery 方式。
