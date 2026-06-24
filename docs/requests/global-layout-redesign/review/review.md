# 评审记录

## blocking issues

- 无

## non-blocking issues

- 无

## accepted risks

- 当前“只允许 Tailwind class 样式”的约束主要依赖代码审查和人工搜索，仓库暂未加入自动 lint 规则做强校验
- Three.js 独立异步 vendor chunk 仍然体积较大，但已与主入口和特效管理器主体解耦，当前通过更贴近实际的阈值配置避免误导性告警
- Element Plus 自动按需加载依赖构建期插件生成声明文件；若后续清理这些生成文件、改动 Vite 配置，或让 IDE 未使用工作区 TypeScript，需要同步关注 `type-check` 与 ts-plugin 诊断是否回退
- Vue SFC 块顺序约束当前同样依赖人工搜索与代码评审，仓库尚未加入自动格式化或 lint 规则进行强制校验
- 表格插槽行数据上的表单双向绑定在 Vue ts-plugin 下仍可能出现类似误报；后续新增同类场景时，优先复用显式 `model-value` / update 事件模式
- Element Plus 表格插槽的 `row` 在模板里通常只会被推断为通用 `DefaultRow`；后续若事件处理函数只依赖局部字段，优先传递精确字段值而不是整行对象，减少 ts-plugin 结构类型误报

## follow-up items

- 如后续继续强化约束，可补充 ESLint/Stylelint 规则禁止 Vue 模板中的 `style` / `:style` 与 SFC `<style>`
- 如后续希望持续保证 `template -> script` 顺序，可补充 Prettier/Vue SFC 结构约束或提交前检查
- 如后续继续使用 Element Plus，可将根目录 `auto-imports.d.ts` 与 `components.d.ts` 视为工作区约定，并提示 IDE 使用工作区 TypeScript
- 如后续仍需继续压缩体积，可评估为 Three.js 特效能力提供更轻量的 2D 降级方案，或继续裁剪 WebGL 能力范围

## merge readiness summary

- 可以合入。Tailwind 本地接入、样式约束、构建告警治理、Element Plus 自动按需加载、ts-plugin 兼容性修正以及 Vue SFC 块顺序统一均已完成；最终 `type-check` 与 `vite build` 通过，无阻塞问题。

## clean-code assessment

- result: pass
- key findings:
  - 本次改动主要聚焦样式链路、构建配置与 IDE/模板兼容性，责任边界清晰，未引入额外业务分支
  - 对表格插槽和数值输入的调整采用了更显式的参数传递与更新方式，可读性优于隐式双向绑定
  - 剩余风险已记录在 accepted risks 中，但未构成当前交付阻塞
- required follow-up if failed: 无

## design-pattern assessment

- result: pass
- key findings:
  - 本次请求未引入新的模式层、管理器层或不必要抽象，维持了问题驱动的最小实现
  - 兼容性修复优先采用局部参数收窄和显式事件写回，而非新增中间抽象，复杂度受控
- required follow-up if failed: 无
