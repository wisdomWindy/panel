# Request: Monorepo架构迁移

## Request Identifier
monorepo-migration

## Source Link
用户需求

## Business Summary
将当前项目架构调整为monorepo，把直播助手相关的功能模块组织到同一个目录结构中，提升代码组织性和可维护性。

## Goal Statement
实现Monorepo架构迁移：
1. 重新组织项目目录结构
2. 将直播助手相关功能模块统一管理
3. 配置monorepo工具链（pnpm workspace）
4. 保持现有功能完整性
5. 更新构建和开发脚本

## Initial Done Signal
- 项目目录结构符合monorepo规范
- 直播助手功能模块统一在packages目录下
- pnpm workspace配置正确
- 构建和开发脚本正常工作
- 现有功能保持完整

## Trigger Condition
用户发起架构迁移请求。

## Initial Context Sources
- 现有代码库：src/ 目录
- 技术栈：Vue 3 + TypeScript + Electron + Vite

## Human Handoff Point
完成迁移后需要用户进行功能测试和验收，确认所有功能正常工作。

## Affected Area
- 项目根目录配置
- src/ 目录结构
- 构建配置
- 开发脚本

## Participating Modules
1. 主进程模块
2. 渲染进程模块
3. 预加载脚本模块
4. 共享类型模块