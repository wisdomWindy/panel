# 任务板

| ID | 任务 | 来源 | 状态 | 依赖 | 验证 |
| --- | --- | --- | --- | --- | --- |
| T1 | 建立 locale/TTS/effect RED 测试 | U1-U5 | completed | 无 | focused Vitest/Cargo RED |
| T2 | 修复 HuiHui 默认值与 reset 保存反馈 | U2-U3 | completed | T1 | focused Vitest GREEN |
| T3 | 扩展 effect 尺寸/位置 TS-Rust 全链 | U4-U5 | completed | T1 | adapter/domain/IPC/window tests GREEN |
| T4 | 免费版页面与子组件完整国际化 | U1 | completed | T2,T3 | locale parity、静态扫描、运行时页面验收 |
| T5 | 全量验证、审查与安装包 | U1-U5 | completed | T2-T4 | test/type/lint/cargo/build/runtime |

## 执行约束

- 严格串行；共享 locale 和 media contract 不并行编辑。
- 每完成一项立即更新状态与 execution 记录。
- 任何产品语义缺口回退 spec；结构缺口回退 architecture-design；实现错误回退 execute。
