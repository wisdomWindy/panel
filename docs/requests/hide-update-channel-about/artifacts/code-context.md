# 代码上下文

## Context Requirement And Graph Check

- 需要理解被移除展示项的引用闭包，避免误删更新通道业务合同。
- 结构影响为单页面直接引用，代码图非必需；使用目标 `rg` 检查完成。
- Graph status：not needed；bootstrap attempted：no；fallback：targeted text/reference search，置信度高。

## Entrypoint And Functional Chain

- 用户入口：系统设置 -> 关于与许可。
- 当前链路：`settings/index.vue` -> `release.about?.channel` -> 页面 `<dl>` 展示。
- 文案链路：`settings.aboutInfo.channel` -> `zh-CN.json` / `en-US.json`。
- 更新检查链路：`handleCheckUpdate -> release.checkUpdate()`，与展示行独立，必须保留。
- release channel 仍被 store/desktop 数据合同拥有，不属于本次删除闭包。

## Impact Scope And Follow-Up

- 改动：设置页模板、设置页合同测试、两份 locale。
- 无 import/export、状态、请求、副作用、路由、IPC 或 Rust 改动。
- 后置检查：全局确认 `settings.aboutInfo.channel` 和 `release.about?.channel` 不再出现在设置页；更新检查合同仍通过。

