# Clarifications：开放平台会话集成

## 问题

用户后续提供了 AccessKeyId，并要求将 appid 与 AccessKeyId 写为项目默认值；主播身份码仍由运行时输入，是否阻止实现？

## 回答

不阻止协议适配和离线验证；缺少有效主播身份码只阻止正式平台联调。

## 最终决策

appid、AccessKeyId 与 Secret 都使用 Rust 后端固定值，不从环境变量读取或覆盖；主播身份码由连接工具栏输入。Secret 会随源码和安装包分发且可被提取，此风险由用户明确接受。

## 受影响的 spec 区域

触发条件、IPC 契约、安全、验收和人工交接。

---

## 问题

是否继续以房间号作为连接入参？

## 回答

否。官方 `/v2/app/start` 要求身份码，真实房间号由响应返回。

## 最终决策

`room_connect` payload 改为 `identityCode`；`RoomSnapshot.roomId` 仍为真实房间号。

## 受影响的 spec 区域

请求流程、接口契约、renderer 和应用端口。

---

## 问题

是否把 Secret 存入现有 SQLite 配置？

## 回答

否。现有普通配置明确拒绝 secret key，且没有可用的系统凭证库实现。

## 最终决策

仅使用进程环境；后续若引入 OS credential store，再作为独立安全需求处理。

## 受影响的 spec 区域

数据所有权、安全、rollout。
