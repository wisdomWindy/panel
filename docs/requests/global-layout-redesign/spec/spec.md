# 全局布局重设计规格

## background and goals

当前渲染端页面大量使用 Tailwind utility class，但项目实际未通过本地构建链路输出 Tailwind 样式，而是依赖 `src/renderer/index.html` 中的 CDN 注入脚本。这会导致 Electron/Vite 场景下样式来源不稳定、构建产物不可控，也不符合仓库对可维护样式供应链的要求。

本次目标是在保留既有全局布局重设计结果的基础上，修复 Tailwind 接入方式，并把渲染端 Vue 组件的样式表达统一收敛到 Tailwind class，禁止重新引入 `<style>`、`style`、`:style` 等页面内样式机制。同时，统一 Vue SFC 的代码组织顺序，要求 `template` 位于 `script` 之前，避免页面结构风格继续分散。

在样式链路修复完成后，本次还需要继续清理构建阶段的两类告警：一类是 Rollup 对第三方 `@vueuse/core` 产物注释的无害提示，另一类是由于 `element-plus`、`echarts`、`three` 和特效模块静态依赖边界过粗导致的 chunk 体积告警；其中 Element Plus 需要从手写组件入口调整为构建期自动按需加载。进一步地，当前自动生成声明文件虽然可通过命令行 `tsc`，但在 IDE ts-plugin 场景下存在识别不稳定问题，本次需要将其调整为对 Vue 语言服务更稳定的显式接入方式。对 `DanmakuPool.vue` 中表格插槽行数据的数值输入，还需要避免直接将可选字段 `row.interval` 绑定到 `v-model`，改为更显式的单向值传递和更新事件写回，消除 Vue ts-plugin 对可选值和事件值的诊断。

## in scope

- 将渲染端 Tailwind 样式改为本地构建接入，而非 CDN 注入
- 新增并接入渲染端本地 Tailwind 样式入口文件
- 修正 Tailwind 内容扫描路径，覆盖 `src/renderer/**/*.vue|ts`
- 复核 `src/renderer/**/*.vue`，确保不存在 `<style>`、`style=`、`:style=`
- 保持现有页面使用 Tailwind class 的布局和视觉结果不回退
- 将“仅允许 Tailwind CSS 风格样式”写入请求工件与评审结论
- 清理 `@vueuse/core` 第三方注释告警
- 将 Element Plus 接入方式改为 Vite 插件驱动的自动按需加载
- 调整 Element Plus 自动生成声明文件的输出位置与引用方式，消除 IDE ts-plugin 报错
- 统一渲染端 Vue SFC 的 `template -> script` 块顺序
- 修正 `DanmakuPool.vue` 中 `el-input-number` 对行数据可选字段的 ts-plugin 诊断
- 优化渲染端依赖边界，降低主入口和特效相关 chunk 的体积告警

## out of scope

- 不替换 Element Plus 组件库
- 不重做业务交互、表单逻辑和页面信息架构
- 不处理 Vue 组件之外的 Canvas/WebGL 绘制样式能力
- 不引入新的设计系统或主题切换能力

## user flow

1. 用户启动 Electron 应用，渲染端由 `main.ts` 加载 Vue 应用与本地 Tailwind 样式入口。
2. 未连接状态显示 `Login.vue`，已连接状态显示 `App.vue` 包裹的主布局。
3. 各页面继续通过模板中的 Tailwind class 呈现既有布局与视觉层级。
4. 开发者新增或修改页面样式时，只能在模板 class 中使用 Tailwind utility class，不得添加 `<style>` 或 `style` 属性。

## page and module design

### 样式供应链

- `src/renderer/styles/tailwind.css` 作为渲染端唯一 Tailwind 入口
- `src/renderer/main.ts` 必须显式导入该入口文件
- `src/renderer/index.html` 不再包含 Tailwind CDN 脚本
- `tailwind.config.js` 的 `content` 必须匹配渲染端实际文件路径

### 构建与依赖边界

- `vite.config.ts` 允许对已确认无害的第三方注释告警做精准过滤，不屏蔽其他 Rollup 警告
- Vue 页面中的 Element Plus 使用不再维护手写 `element-plus/es/components/*` 导入，改由 Vite 自动解析模板组件与 `ElMessage` / `ElMessageBox` 等服务调用，并只引入实际使用的组件与样式
- 特效链路应避免在常驻主链路中静态引入 `effect-manager` 与 Three.js，按需加载即可
- `echarts`、`zrender`、`three` 继续保持独立异步 vendor chunk

### 页面与组件约束

- `src/renderer/**/*.vue` 仅允许通过 `class` 和 `:class` 使用 Tailwind utility class
- 禁止出现 `<style>` 标签、`style=""`、`:style=""`、`v-bind:style`
- `src/renderer/**/*.vue` 统一采用 `template` 在前、`script` 在后的块顺序
- 对表格/插槽中的可选数值字段，优先使用显式 `:model-value` 与 `@update:model-value`，避免直接 `v-model` 触发 ts-plugin 误报
- 允许保留 Element Plus 的结构属性、尺寸属性、`width="500px"` 这类组件 props，但不将其视为页面内联 style 方案

### 现有页面兼容

- `App.vue`、`Login.vue`、`Settings.vue`、`StatCard.vue` 等现有组件继续保留当前 Tailwind class 布局
- 若发现 class 依赖的 Tailwind 样式因本地接入问题缺失，应以配置修正为主，不引入额外 CSS 补丁

## API and data contracts

- 本次不变更任何 API、store 数据结构或组件对外接口
- 仅新增一个渲染端静态样式入口文件 `src/renderer/styles/tailwind.css`
- 允许新增自动生成声明文件 `auto-imports.d.ts` 与 `components.d.ts`
- 允许调整 `src/vue-shims.d.ts` 以显式接入工作区声明

## edge cases

- 如果某页面没有 `<style>` 或 `style` 残留，则无需为了“满足约束”做无意义模板改写
- 如果某些 class 使用 Tailwind 任意值语法（如 `bg-[#F5F6FA]`、`h-[calc(100vh-100px)]`），本地构建后必须继续生效
- 如果构建验证显示 Tailwind class 未被扫描，应优先修正 `content` 路径而不是恢复 CDN

## acceptance criteria

1. AC-1：`src/renderer/main.ts` 已导入本地 Tailwind 样式入口，渲染端样式不再依赖 CDN。
2. AC-2：`src/renderer/index.html` 不再包含 Tailwind CDN 脚本。
3. AC-3：`tailwind.config.js` 的扫描范围覆盖渲染端实际模板与脚本路径。
4. AC-4：全量检查 `src/renderer/**/*.vue`，不存在 `<style>`、`style=`、`:style=`、`v-bind:style`。
5. AC-5：应用构建通过，且现有页面布局/视觉层级不因样式入口变更而明显回退。
6. AC-6：功能行为保持不变，样式相关调整不引入业务回归。
7. AC-7：`vite build` 不再输出 `@vueuse/core` 注释相关 Rollup 告警。
8. AC-8：渲染端主入口与特效链路的依赖边界收敛后，构建结果不再出现除 Three.js 独立异步 chunk 之外的无意义体积膨胀。
9. AC-9：`src/renderer/**` 与相关 service 文件中不再保留手写 `element-plus/es/components/*` 导入，Element Plus 组件与服务改由构建期自动按需解析。
10. AC-10：自动生成的 Element Plus 声明文件被 TypeScript 与 Vue 语言服务显式接入，IDE 中不再出现与 `ElMessage`、`ElMessageBox`、`<el-*>` 相关的 ts-plugin 报错或告警。
11. AC-11：`src/renderer/**/*.vue` 中所有 SFC 统一为 `template` 在前、`script` 在后，不改动既有模板与逻辑内容。
12. AC-12：`src/renderer/views/DanmakuPool.vue` 中弹幕池表格的间隔输入不再触发 ts-plugin 报错，且保存逻辑继续以毫秒写回数据层。

## risks

- Tailwind v4 本地入口声明方式若与当前工具链不兼容，可能导致构建失败
- 依赖 CDN 时曾隐式可用的 class，切换到本地构建后如果扫描路径错误会直接丢样式
- 仅靠人工约束无法永久防止后续新增 `<style>`，需要在 review 中持续检查
- Three.js 的 WebGL 渲染器本身依赖面较大，即使按异步 vendor chunk 独立分包，产物仍可能接近默认 500 kB 告警阈值
