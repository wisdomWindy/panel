# 项目优化评估请求

## request identifier

`project-optimization-assessment`

## source link or direct user instruction reference

- 来源：当前会话中的用户直接指令
- 原文：`根据当前项目提出优化建议`

## business summary

对当前 Bilibili 直播助手 monorepo 做基于仓库事实的前端工程审查，输出可执行、可排序、可验证的优化建议；本请求不直接修改生产代码。

## goal statement

识别当前项目在工程配置、架构边界、代码质量、测试、构建与文档方面最值得优先处理的问题，并为每项建议提供证据、收益、风险、建议动作与验证方式。

## initial done signal

- 建议均能追溯到当前仓库中的具体文件、配置、命令结果或结构事实。
- 建议按优先级和投入分级，并区分“已确认问题”与“需进一步验证的机会”。
- 不把通用最佳实践包装成当前项目事实，不擅自提出新的产品功能。
- 交付一份可供团队排期的优化建议清单。

## trigger condition

用户要求“根据当前项目提出优化建议”。

## initial context sources

- 根目录 workspace、脚本、锁文件和 README
- `packages/bilibili-live-tool/` Electron + Vue 主应用
- `packages/bilibili-live-tool-homepage/` 官网包
- `tests/` 与 CI 配置
- 与判断相关的现有请求工件、TypeScript / ESLint / Vite 配置及核心代码链路

## human handoff point

在建议清单形成并通过证据复核后交给用户；具体实施范围和顺序由用户或团队确认后另开代码变更请求。

## affected area

仓库级工程质量评估，不修改产品行为或生产代码。

## participating modules

- monorepo 根配置
- Electron 主应用（main / preload / renderer）
- 官网 package
- 测试与 CI
- 文档及依赖治理

## 请求分类与路由

- 类型：稳定的直接评估请求（非 PRD、非缺陷、非代码变更）
- `requirement-analysis` / `requirement-splitting`：不适用；用户指令已能形成明确的评估目标，本次使用请求级交付单元。
- `page-design`：不适用；不产出或修改页面设计。
- `architecture-design`：不适用；本次只评估并提出建议，不批准具体代码架构变更。
- 速度档位：`S3 broad`，因为评估横跨多个 package、工程配置与质量链路，但上下文读取仍按证据驱动方式收敛。

## user intent contract

- stated request：根据当前项目提出优化建议。
- practical goal：得到能指导真实排期、而非泛泛而谈的项目改进清单。
- success criteria：建议有仓库证据、优先级、收益、动作和验证方法。
- forbidden interpretations：不凭偏好扩展产品功能；不直接进行未获授权的生产代码重构；不把未运行的检查声称为通过。
- acceptable approaches：静态结构审查、配置审查、依赖与脚本审查、运行现有质量命令、抽样关键代码链路并记录限制。
