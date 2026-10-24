# Bilibili 直播助手

基于 Tauri 2、Rust、Vue 3 和 TypeScript 的桌面直播辅助工具。项目已从 Electron/Node 运行时迁移到 Tauri：Rust 负责 SQLite、直播事件、媒体进程、文件权限、更新和系统边界，Vue WebView 只负责界面、状态展示与用户交互。

## 当前状态

已落地的本地能力包括：

- 免费版/会员版编译期隔离，会员路由、IPC 命令和代码不进入免费版产物。
- SQLite 数据库、迁移备份、配置导入/导出、语言与界面偏好。
- 弹幕/礼物数据契约、敏感词、黑名单、统计、排行和导出。
- 上舰特效、礼物提醒/音效、系统 TTS、本地 FFmpeg 录制与高光管理边界。
- Tauri Updater 签名校验、GitHub Actions 免费版多平台发布、官方 FFmpeg/Whisper 资产目录。
- 本地 Whisper CPU 语音识别：下载模型后采集默认麦克风，识别结果显示在 AI 助手页；关闭 ASR 时释放采集资源。

下列成功路径需要额外的生产契约或二进制资源，当前会明确返回 `unavailable` / degraded 状态，不会伪造成功：

- Bilibili 真实直播间传输与需要凭据的开放平台接口。
- 会员登录/支付/权益校验、LLM 与云端审核服务。

## 下载

当前公开版本为 [`biliLiverHelper free v0.2.0`](https://github.com/wisdomWindy/panel/releases/tag/free-v0.2.0)：

- Windows x64：[`biliLiverHelper_0.2.0_Windows_x64-setup.exe`](https://github.com/wisdomWindy/panel/releases/download/free-v0.2.0/biliLiverHelper_0.2.0_Windows_x64-setup.exe)
- macOS Intel：[`biliLiverHelper_0.2.0_macOS_Intel.dmg`](https://github.com/wisdomWindy/panel/releases/download/free-v0.2.0/biliLiverHelper_0.2.0_macOS_Intel.dmg)（macOS 10.15 或更高版本）
- macOS Apple Silicon：[`biliLiverHelper_0.2.0_macOS_AppleSilicon.dmg`](https://github.com/wisdomWindy/panel/releases/download/free-v0.2.0/biliLiverHelper_0.2.0_macOS_AppleSilicon.dmg)（macOS 10.15 或更高版本）

安装包统一使用 `biliLiverHelper_<version>_<platform>.<extension>` 命名。Windows 安装包未进行 Authenticode 签名，macOS 磁盘映像未进行 Apple Developer 签名或公证；仅在确认文件来自本仓库后跳过操作系统警告。

## 技术栈

| 层级     | 技术                                               |
| -------- | -------------------------------------------------- |
| 桌面容器 | Tauri 2.11                                         |
| 本地核心 | Rust 2021、Tokio、SQLx/SQLite                      |
| 前端     | Vue 3、TypeScript、Pinia、Vue Router、Element Plus |
| 构建     | Vite 5、pnpm workspace、Cargo features             |
| 测试     | Vitest、Node test runner、Cargo test/Clippy        |
| 发布     | GitHub Actions、Tauri Action、Tauri Updater        |

## 工程结构

```text
.
├─ .github/workflows/release-tauri.yml       # 签名、免费版多平台发布
├─ packages/
│  ├─ bilibili-live-tool/
│  │  ├─ src/renderer/                # Vue 页面、Pinia、桌面适配器
│  │  ├─ src-tauri/src/
│  │  │  ├─ domain/               # 纯领域规则
│  │  │  ├─ application/          # 用例、状态与端口
│  │  │  ├─ infrastructure/       # SQLite、系统 I/O、Updater
│  │  │  └─ ipc/                  # Tauri Command/Event DTO
│  │  ├─ src-tauri/migrations/         # SQLite 迁移
│  │  └─ scripts/                     # 发布配置与资产校验
│  └─ bilibili-live-tool-homepage/        # VitePress 官网
├─ docs/
│  ├─ requirements/                       # PRD
│  └─ requests/                           # 需求、设计、执行、验证与评审
└─ description-doc/                          # Bilibili 开放平台参考资料
```

主要依赖方向为：

```text
Vue View -> Pinia/Runtime -> Desktop Adapter -> Tauri IPC
         -> Application State -> Domain/Port -> Infrastructure
```

`main` 窗口只获得经白名单授权的桌面命令；`effect-*` 特效窗口是纯展示窗口，不具备 IPC 权限。

## 环境要求

- Node.js `>=24 <25`
- pnpm 9
- Rust stable，项目最低 Rust 版本 `1.88`（Whisper 绑定要求）
- 对应平台的 Tauri 2 系统构建依赖
- Windows 运行需要 WebView2；Windows 构建使用 MSVC 工具链
- Whisper 原生编译需要 CMake 和 LLVM/libclang；将 `LIBCLANG_PATH` 指向包含 `libclang.dll` 的目录（标准 LLVM 安装通常为 `C:\Program Files\LLVM\bin`）。这些是构建依赖，终端用户无需安装。

Ubuntu 构建与 CI 使用的系统依赖：

```bash
sudo apt-get install -y \
  libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev \
  patchelf xdg-utils libasound2-dev libspeechd-dev cmake libclang-dev
```

## 安装与开发

```powershell
pnpm install --frozen-lockfile

# 免费版 Tauri 桌面应用
pnpm dev

# 会员版 Tauri 桌面应用
pnpm --filter @monorepo/bilibili-live-tool dev:member

# 仅运行 Web 调试界面
pnpm --filter @monorepo/bilibili-live-tool dev:web

# 官网
pnpm homepage:dev
```

默认开发命令启动 `free` 通道。桌面数据库由 Tauri 写入应用数据目录的 `app.db`；不应在 Vue 中拼接或传递本地原始路径。

## 构建通道

| 通道     | Vite mode | Cargo feature | 范围                                         |
| -------- | --------- | ------------- | -------------------------------------------- |
| `free`   | `free`    | `free`        | 通用直播、安全、媒体、配置、更新界面         |
| `member` | `member`  | `member`      | 包含通用能力，并编译会员、AI、ASR 和审核边界 |

```powershell
# 仅构建前端
pnpm --filter @monorepo/bilibili-live-tool build:web:free
pnpm --filter @monorepo/bilibili-live-tool build:web:member

# 构建 Tauri 安装包
pnpm --filter @monorepo/bilibili-live-tool build:free
pnpm --filter @monorepo/bilibili-live-tool build:member

# 构建官网
pnpm homepage:build
```

Tauri 安装包默认生成于：

```text
packages/bilibili-live-tool/src-tauri/target/release/bundle/
```

`free` 和 `member` Cargo feature 必须且只能选择一个，不要直接使用无 feature 的 Cargo 构建。

## 测试与质量检查

```powershell
# TypeScript 与 ESLint
pnpm type-check
pnpm lint:check

# 发布脚本契约
pnpm --filter @monorepo/bilibili-live-tool test:release

# Vue/TypeScript 双通道测试
pnpm --filter @monorepo/bilibili-live-tool exec vitest run --mode free
pnpm --filter @monorepo/bilibili-live-tool exec vitest run --mode member

# Rust 双通道测试
cargo test --manifest-path packages/bilibili-live-tool/src-tauri/Cargo.toml --no-default-features --features free
cargo test --manifest-path packages/bilibili-live-tool/src-tauri/Cargo.toml --no-default-features --features member

# Rust 格式与严格静态检查
cargo fmt --manifest-path packages/bilibili-live-tool/src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path packages/bilibili-live-tool/src-tauri/Cargo.toml --no-default-features --features free --all-targets -- -D warnings
cargo clippy --manifest-path packages/bilibili-live-tool/src-tauri/Cargo.toml --no-default-features --features member --all-targets -- -D warnings
```

## 签名与发布

`.github/workflows/release-tauri.yml` 支持手动触发，也会在推送 `v*` tag 时运行。Tag 中的版本必须与 `src-tauri/tauri.conf.json` 一致，例如：

```powershell
git tag v0.2.0
git push origin v0.2.0
```

发布流程会：

1. 运行发布契约和工作流格式检查。
2. 构建 `free` 通道的 Windows x64、macOS Intel 和 macOS Apple Silicon 产物。
3. 使用 Tauri Updater 私钥生成强制签名的更新产物。
4. 生成 `free/latest.json` 并发布到 `updates` 分支。
5. 生成 `free-v<version>` GitHub Release，写入版本变更和各平台直接下载链接。
6. 按 `biliLiverHelper_<version>_<platform>.<extension>` 命名安装包和更新产物。
7. 解析并校验官方运行时资产，下载后二次计算 SHA-256。

会员版仍可在本地通过 `member` feature 构建和测试，但发布工作流不会创建会员版 Release。

仓库 Actions 需要以下配置：

| 类型     | 名称                                 | 用途                       |
| -------- | ------------------------------------ | -------------------------- |
| Secret   | `TAURI_SIGNING_PRIVATE_KEY`          | Tauri Updater 私钥         |
| Secret   | `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` | 私钥密码                   |
| Variable | `TAURI_SIGNING_PUBLIC_KEY`           | 注入客户端的 minisign 公钥 |

私钥和密码不得写入 Git。发布脚本生成的 `tauri.release.*.conf.json` 和 `.release-staging/` 均已忽略。平台安装包代码签名/公证与 Tauri Updater 签名是两套独立凭据。

## 第三方运行时资产

- FFmpeg：只允许 FFmpeg 官网链接的 `BtbN/FFmpeg-Builds` LGPL shared 包。
- Whisper：只允许 `ggml-org/whisper.cpp` GitHub Releases 的官方二进制包。
- 当前目录覆盖 Windows/Linux x64 和 ARM64；官方不存在完全匹配的平台资产时，保持不可用，不使用非官方镜像回退。
- 资产目录包含 `artifactId`、版本、平台/架构、下载 URL、SHA-256、归档类型、选中条目和许可证来源。

## 文档

- 产品需求：`docs/requirements/Bilibili 直播助手 - 产品需求文档 (PRD).md`
- 重构请求与验收证据：`docs/requests/tauri-rust-refactor/`
- 最终验证：`docs/requests/tauri-rust-refactor/module-runs/extensions-release-resilience/verification/verification.md`
- 最终评审：`docs/requests/tauri-rust-refactor/module-runs/extensions-release-resilience/review/review.md`

## 许可证

项目代码当前声明为 MIT。FFmpeg、Whisper 及其他第三方组件保持各自的许可证和源码/通知义务，不随本项目 MIT 声明改变。
