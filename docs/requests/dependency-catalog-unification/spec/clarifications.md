# 依赖 catalog 统一澄清记录

## 澄清 1：公共依赖的含义

- **问题**：这里的“公共依赖”是指根包统一安装，还是统一版本来源？
- **结论**：统一版本来源，不改变依赖归属。
- **影响**：采用 `catalog`，不采用根 `package.json` 公共依赖方案。

## 澄清 2：实施方案

- **问题**：统一版本使用哪种 pnpm 机制？
- **结论**：采用 `catalog` 方案。
- **影响**：修改 `pnpm-workspace.yaml` 和子项目 `package.json`，不使用 `overrides` 替代。

## 澄清 3：覆盖范围

- **问题**：是所有依赖都 catalog 化，还是只处理真实重复依赖？
- **结论**：只处理当前真实重复依赖。
- **影响**：本轮仅覆盖 `tailwindcss`、`@tailwindcss/postcss`、`postcss`、`autoprefixer`。
