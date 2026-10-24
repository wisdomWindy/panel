# 归一化需求快照

## Verbatim Source Evidence Index

见 `artifacts/bugfix-source.md` U1-U4。

## 来源摘要与目标

修复主题切换与混色缺陷，默认品牌主色采用 bilibili 粉 `#FB7299`。

## Greenfield Scaffold Sensitivity

不适用：既有应用局部修复。

## 明确行为约束

- light 强制浅色，dark 强制深色，system 使用并持续跟随系统。
- 所有 Tailwind `dark:` 与 Element Plus dark variables 必须读取同一根节点状态。
- 品牌主色的基础、hover、active、浅色背景色必须同源。
- 非旧默认自定义颜色不得被覆盖。

## Closed-World Product Scope Allowlist

只允许改变主题生效状态、视觉一致性、默认品牌色及旧默认迁移；无新增控件、文案或工作流。

## 表单、展示与交互

沿用现有主题 segmented control、主色 color picker、预览、保存与恢复默认行为；不新增 UI。

## 状态规则

主题选择预览立即投影；system 状态在系统偏好变化时重新投影；runtime stop 移除监听。

## 相关模块与问题

`projection.ts`、shell runtime、Tailwind CSS 配置、InterfaceSettingsPanel、Rust shell defaults、相关测试。未决问题 none。

