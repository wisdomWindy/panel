# 安装配置

## 系统要求

### 操作系统

- **Windows**: Windows 10 或更高版本
- **macOS**: macOS 10.15 或更高版本
- **Linux**: Ubuntu 20.04 或更高版本

### 运行环境

- **Node.js**: 18.0 或更高版本
- **pnpm**: 8.0 或更高版本

## 安装步骤

### 1. 安装 Node.js

访问 [Node.js 官网](https://nodejs.org/) 下载并安装 Node.js。

验证安装：

```bash
node --version
npm --version
```

### 2. 安装 pnpm

```bash
npm install -g pnpm
```

验证安装：

```bash
pnpm --version
```

### 3. 克隆项目

```bash
git clone https://github.com/wisdomWindy/panel.git
cd panel
```

### 4. 安装依赖

```bash
pnpm install
```

### 5. 启动应用

```bash
cd packages/bilibili-live-tool
pnpm dev
```

## 构建应用

### 开发模式

```bash
pnpm dev
```

### 构建生产版本

```bash
pnpm build
```

### 打包应用

```bash
pnpm build:app
```

## 配置说明

### 应用配置

应用配置文件位于 `src/renderer/stores/config.ts`，包含：

- 连接配置
- 弹幕配置
- 录制配置
- AI 配置

### 环境变量

创建 `.env` 文件配置环境变量：

```env
# Bilibili API 配置
VITE_BILIBILI_API_KEY=your_api_key
VITE_BILIBILI_API_SECRET=your_api_secret

# AI 模型配置
VITE_AI_MODEL=your_ai_model
VITE_AI_API_KEY=your_ai_api_key
```

## 故障排除

### 依赖安装失败

```bash
# 清除缓存
pnpm store prune

# 重新安装
pnpm install
```

### 应用启动失败

1. 检查 Node.js 版本
2. 检查依赖是否安装完整
3. 检查端口是否被占用

### 构建失败

```bash
# 清除构建缓存
rm -rf dist
rm -rf node_modules/.vite

# 重新构建
pnpm build
```