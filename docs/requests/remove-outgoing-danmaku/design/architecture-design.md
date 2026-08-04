# Architecture Design: remove-outgoing-danmaku

## 架构目标与触发

在不改变接收链路的前提下，删除跨 renderer/main/plugin/docs 的主动发送能力和所有专属状态。触发原因是 sender、manager、service、store、页面、插件与持久化形成跨层依赖闭包。

## 上游输入与假设

- 行为合同：`spec/spec.md` S-1 至 S-6、AC-1 至 AC-7。
- 技术假设：历史 electron-store 键不迁移删除，只停止消费。
- scaffold：不适用。

## Architecture Decision Records

### ADR-1：删除发送垂直切片

- problem：发送能力散布在网络、业务编排、状态、UI、插件和持久化层。
- choice：从最底层 `DanmakuSender` 向所有调用方闭包直接删除；保留的 `DanmakuManager` 只接收和分发。
- alternatives：空实现、feature flag、deprecated wrapper。均会保留已删除能力的公开面或死代码，拒绝。
- dependency before：views/plugins -> service -> manager -> sender -> non-public endpoint。
- dependency after：views -> service -> manager -> official receive client；plugins 仅订阅事件。
- state/data owner after：接收消息由 danmaku store；连接由 room store；无发送状态 owner。
- side-effect owner after：Bilibili client 负责连接/接收，service 负责写 store/database；无发送 I/O。
- cleanup：删除 sender、pool timers、send events、send state、入口、配置、IPC、文档。
- evidence：残留关键词搜索、编译、lint、测试、构建。

### ADR-2：不保留兼容层

- problem：插件和旧路由可能依赖发送 API。
- choice：公开类型和沙箱暴露一起删除，不提供空函数或重定向。
- why minimal：用户要求移除功能；兼容层会使能力仍可被发现或调用。
- rollback：通过 Git diff 恢复，不在运行时维护双路径。

### ADR-3：保留官方资料和历史记录

- problem：文档既包含当前产品宣称，也包含官方快照/历史审计。
- choice：修改当前产品/需求/架构说明；不改 `description-doc/`、已完成 request 和历史 changelog。
- owner：当前产品事实由 README/requirements/architecture/plugin docs 承担；历史材料保持不可追溯改写。

## 业务能力与 ownership map

| 能力 | 当前 owner | 变更后 owner |
| --- | --- | --- |
| 连接和事件接收 | `BilibiliClient` / `DanmakuManager` | 保持 |
| 收到弹幕/礼物状态 | `DanmakuService` / `useDanmakuStore` | 保持 |
| 主动发送 HTTP | `DanmakuSender` | 删除 |
| 自动/事件发送编排 | `DanmakuManager` | 删除 |
| 弹幕池与发送统计 | manager/store/database | 删除 |
| 插件弹幕订阅 | `PluginAPIImpl` | 保持 |
| 插件弹幕发送 | `PluginAPIImpl` / sandbox | 删除 |

## 模块与文件结构

### 删除文件

- `src/renderer/modules/danmaku/sender.ts`
- `src/renderer/views/danmaku-pool/index.vue`
- `src/renderer/views/settings/components/AutoSendSettings.vue`
- `src/renderer/views/settings/components/DanmakuSettings.vue`（内容全部属于发送能力）

### 就地收窄文件

- manager/service/store/types：删除发送职责，保留接收职责。
- router/App/settings/dashboard/danmaku-list/connection/locales：删除入口和展示。
- plugin types/api/sandbox：删除发送 API，保留监听。
- main database/IPC 与 renderer database service：删除弹幕池和发送配置。
- tests：删除只验证已移除类型/配置的断言，保留接收类型验证。
- docs：删除当前能力宣称，不改历史材料。

## 依赖规则矩阵

| From | To | 规则 |
| --- | --- | --- |
| views | DanmakuService/store | 允许，仅接收/查询/清理列表 |
| DanmakuService | DanmakuManager | 允许，仅连接、事件、黑名单/录制协作 |
| DanmakuManager | BilibiliClient/parser | 允许 |
| plugins | event API | 允许订阅，不允许发送 |
| renderer | `/msg/send` 或 Cookie/CSRF | 禁止 |
| receive modules | deleted pool/sender state | 禁止 |

公开 API 仅保留现有连接/接收/订阅入口。所有发送符号成为私有删除目标，不新增 barrel 或适配器。循环依赖风险不增加。

## State topology 与 data lifecycle

- writable connection state：`useRoomStore`。
- writable received events：`useDanmakuStore.danmakuList/giftList`。
- derived counts/ranks：store computed/actions 现有派生。
- 删除 writable state：pool、auto sending、last sent、failed/sent/thank counts、success rate、cookie/csrf send credentials。
- lifecycle：官方 response -> parser -> manager event -> service -> store/database/view，路径保持。
- 无新 async state、同步规则、stale response 或 timer cleanup；删除发送 timer 反而收窄生命周期。

## 数据结构与类型策略

- 删除 `DanmakuPoolItem`、`DanmakuOptions`。
- `Danmaku` 删除只服务发送状态的 `type/success/message`；接收数据字段保持。
- `DanmakuManager` constructor 删除 `minInterval/csrfToken`。
- `AppConfig` 删除发送字段。
- 不新增 union、兼容 DTO 或 mapper。

## 函数设计与副作用

- 删除 `sendDanmaku`、pool CRUD、start/stop timers、trigger/role/thank generation 和 rate-limit methods。
- `handleDanmaku` 保留 emit、blacklist、spam detection。
- `handleGift`/guard 保留 event emit，不产生发送副作用。
- connected/live-start 只触发仍有 owner 的自动录制事件。
- 所有保留副作用仍处在 manager event/service store/database boundary。

## Reuse 与 Anti-DRY 矩阵

| Candidate | 语义/层级 | 使用点 | 稳定性/变因 | 决策 |
| --- | --- | --- | --- | --- |
| 发送触发路径 | 同一发送语义，跨 manager 分支 | 多处 | 目标是整体删除 | 删除，不抽取 |
| 收到弹幕事件映射 | 接收领域/manager-service | 现有稳定链 | 本次不修改语义 | 复用现有链 |
| 文档能力描述 | 不同文档 owner | 多处 | 当前事实与历史语义不同 | 分别就地修改，禁止共享生成器 |

没有需要晋升为 shared API 的候选；不新增 JSDoc traceability。

## Commonality classification

- 技术/基础设施：官方连接与接收客户端，保留。
- 业务/领域：发送编排与弹幕池，删除。
- UI：发送入口和设置，删除。
- 配置/常量：发送频控与开关，删除。

## Locality 和模式决策

- 本次不新增 helper/hook/mapper/component 文件。
- 所有收窄在现有 owner 就地完成。
- pattern：Level 0 direct code / none。删除路径不需要 facade、strategy、command 或 compatibility adapter。

## Extension seams 与复杂度预算

- 不保留发送 feature flag、兼容接口、空实现或替代 provider seam。
- 新增生产文件：0。
- 新增公开 API：0。
- 新增间接层级：0。
- 响应式 fan-out：减少。
- 迁移范围：发送垂直切片及直接文档/测试引用。

## Migration、compatibility 与 cleanup

- 不做运行时数据迁移；旧持久化键成为未消费数据。
- 不兼容旧插件 `sendDanmaku` 和旧 `/danmaku-pool` 路由，这是批准的删除结果。
- cleanup trigger：本次 execute 必须一次性删除全部生产引用，不保留双路径。
- rollback：Git 文件级恢复。

## Readability guardrails

- 不将接收 manager 顺便重写为其他模式。
- 不做无关格式化。
- 删除后更新模块注释，避免继续宣称自动回复。
- 保留代码不得因空分支或空配置组件而失去明确 owner。

## 风险与开放问题

- 风险：产品文档数量多，需区分当前说明与历史记录。
- 风险：store export/import shape 变化；仅内部调用，无正式持久化 owner，类型检查和引用搜索验证。
- 开放架构问题：无。

