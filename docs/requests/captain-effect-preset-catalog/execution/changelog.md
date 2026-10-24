# 执行记录

- Rust 快照内置 12 个上舰特效模板，包含 `golden-premium / 金色尊享`，每项携带默认主色和辅色。
- TypeScript 桌面边界增加模板颜色字段校验，拒绝非法十六进制颜色。
- 模板下拉切换时同步模板默认配色，之后仍可由用户手动修改；保存时沿用现有配置链路。
- Canvas 按模板 ID 映射 12 种本地图案，未知历史 ID 回退经典样式。
- 删除“模板素材尚未提供”提示及对应中英文文案。
- 素材来源：全部为本地 Canvas 程序绘制，无外部图片、字体、音频或第三方素材。

## 测试驱动证据

- 红灯：前端定向测试 4 个失败；Rust 缺少 `captain_effect_presets`，且首次命令提示必须选择构建 feature。
- 绿灯：前端定向测试 10/10；Rust 定向测试在 `--features free` 下 6 个通过。
- 回归：前端完整测试 139/139；Rust 完整测试 119/119；TypeScript 类型检查、scoped ESLint、rustfmt 和 `git diff --check` 通过；Web production build 成功。
