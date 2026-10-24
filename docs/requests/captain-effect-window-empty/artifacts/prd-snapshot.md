# 归一化缺陷快照

## 原文证据索引

- U1：“特效窗口里面怎么什么都没有，特效呢”

## 来源摘要

上舰特效窗口可打开但不显示特效。

## 关键业务目标

窗口在触发后稳定显示当前上舰特效。

## Greenfield Scaffold Sensitivity

不适用：这是现有桌面功能的缺陷修复。

## 明确行为约束

- 窗口打开时必须存在可绘制的当前活动 payload。
- 首次预览不能依赖监听器建立前发送的一次性事件。
- 真实上舰事件应保持原有用户、动作、舰长等级和配置语义。

## 闭集产品范围

- 修复窗口空白。
- 保持已有模板视觉和配置行为不变。

## 表单、表格、展示和交互

展示：现有 `CaptainEffectSurface` 继续负责绘制；本缺陷不改变表单或交互结构。

## 工作流和状态规则

触发进入队列 -> 活动特效建立 -> 窗口监听建立 -> 获取活动 payload -> 绘制 -> 到期关闭。

## 相关模块或页面

上舰特效预览命令、`EffectRuntime` 活动状态、desktop contract/adapter、`EffectWindowRoot.vue`。

## 未决问题

无。

## 2026-09-23 Semantic Reopen Snapshot

- Source U2 identifies a semantic contradiction between “上舰特效” and “欢迎加入直播间”.
- Code facts resolve the feature meaning as captain purchase/renewal, not audience entry.
- Approved product delta: remove the contradictory entry sentence; add no replacement copy.
- Preserve page/menu naming, purchase/renewal action, captain level, username, visual templates, and configuration.
