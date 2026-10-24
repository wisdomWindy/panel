# 执行变更记录：开放平台会话集成

## 模块标识与范围

`open-live-session`。实现主播身份码经官方 Open Live API/WSS 转换为现有 `LiveEvent` 的完整会话链路。

## 已完成任务

- T1：新增环境配置校验、请求体 MD5、规范化签名头与 HMAC-SHA256。
- T2：新增 16 字节大端 Proto 编解码、Version 2 zlib 递归解包和四类开放平台事件映射。
- T3：新增 start/heartbeat/end HTTP 客户端、WSS 地址回退、AUTH gate、双心跳、取消优先和统一 cleanup。
- T4：生产装配切换到真实 transport，IPC 入参迁移为 `identityCode`，toolbar 改为身份码输入，连接成功后采用平台返回房间号，窗口销毁时断开。
- T5：完成双通道 Rust 测试、前端全量测试、类型/Lint、release 构建和 Secret 扫描。

## 关键实现决策

- appid、AccessKeyId 与 Secret 均使用项目固定值，不读取任何开放平台凭证环境变量。
- Secret 只存在于 Rust 配置边界和编译产物，不进入前端、日志、Debug 输出或 SQLite；客户端内置值可被提取的风险已明确接受。
- `RoomTransport` 接收身份码，`RoomConnection` 返回平台房间号，保持下游事件/统计接口不变。
- receiver drop 和窗口销毁会触发 socket close 与 `/v2/app/end` best effort；取消优先于到期心跳。
- `open_id` 作为字符串 UID 进入领域模型，同时拒绝空值、空白/控制字符和超长输入。

## 测试驱动证据

- IPC `identityCode` 与前端 store 契约先失败后通过。
- 异步错误码提示映射先失败后通过。
- 全量 Rust 测试发现旧 UID 数字断言并迁移为 `open_id` 契约。
- 生命周期测试发现断开与心跳竞争，加入有偏取消后通过。

## 偏差与限制

- 未进行正式平台 smoke test：当前没有有效主播身份码。
- 不新增 Secret 设置页面，符合 approved spec 的范围外约束。
- 无数据库 schema 或数据迁移。

## T6 增量变更（历史阶段，已由 T8 取代）

- 根据用户新增输入，把指定 appid 与 AccessKeyId 写为 Rust 后端编译期默认值。
- appid 与 AccessKeyId 在 T7 收敛为不可运行时覆盖的固定值。
- `BILIBILI_OPEN_LIVE_ACCESS_KEY_SECRET` 仍为必填运行时环境变量，不进入源码、日志、测试 fixture 或安装包。
- 先添加 `from_lookup` 缺失导致的编译失败测试，再实现可注入配置查找并确认转绿。
- 缺配置的界面提示已收敛为只要求设置 Secret。

## T7 固定应用标识（历史阶段，已由 T8 取代）

- 按用户新要求移除 `BILIBILI_OPEN_LIVE_APP_ID` 与 `BILIBILI_OPEN_LIVE_ACCESS_KEY_ID` 的读取和覆盖路径。
- `OpenLiveConfig::from_lookup` 现在只查询 `BILIBILI_OPEN_LIVE_ACCESS_KEY_SECRET`，appid 与 AccessKeyId 始终使用 Rust 常量。
- RED：新测试因旧实现查询 `BILIBILI_OPEN_LIVE_APP_ID` 而按预期失败。
- GREEN：删除两个覆盖查询和 `configured_or_default`后，聚焦测试 1 项通过。
- Secret 的签名用途、环境读取和脱敏边界保持不变。

## T8 固定完整应用凭证

- 按用户明确要求将 Secret 写入 Rust 配置常量，不再从进程环境读取任何开放平台凭证。
- 删除 `OpenLiveConfig::from_env` 与 `from_lookup`，新增 `OpenLiveConfig::embedded`，启动装配同步切换。
- RED：内置凭证签名测试因 `embedded()` 不存在而按预期编译失败。
- GREEN：新增固定 Secret 和 `embedded()` 后，独立计算的 HMAC 固定向量通过。
- 缺配置错误文案已改为内置凭证无效，不再指示用户设置环境变量。
- 已明确接受 Secret 可从源码和安装包中提取的风险；Debug、日志、renderer 和 SQLite 隔离保持不变。
