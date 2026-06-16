# Bilibili 直播助手 - 产品需求文档 (PRD)

**版本号**: v1.0
**创建日期**: 2026-06-03
**文档状态**: 初稿
**最后更新**: 2026-06-16

---

## 实现进度总览

| 优先级 | 总计 | 已完成 | 完成率 |
|--------|------|--------|--------|
| Must Have (v1.0) | 6 | 6 | 100% |
| Should Have (v1.5) | 6 | 5 | 83% |
| Could Have (v2.0) | 5 | 1 | 20% |
| Won't Have (v3.0+) | 4 | 0 | 0% |
| **总计** | **21** | **12** | **57%** |

**已完成的核心功能**:
- ✅ Bilibili 直播间连接与弹幕接收
- ✅ 自动弹幕发送（定时/随机/触发词）
- ✅ 礼物感谢弹幕
- ✅ 分级用户互动配置
- ✅ 上舰特效展示（WebGL/Canvas）
- ✅ 实时数据看板
- ✅ 黑名单管理
- ✅ 配置文件保存与加载
- ✅ 敏感词过滤
- ✅ 更新日志与版本管理
- ✅ 基础 UI 界面
- ✅ 插件系统

**待实现功能**:
- ❌ 语音识别接入（Whisper）
- ❌ AI 对话生成（LLM）
- ❌ 高光录制（自动/手动）
- ❌ 自动禁言与刷屏防护
- ❌ 插件系统
- ❌ 多语言支持

---

## 1. 产品概述

### 1.1 产品愿景
打造一款集自动化互动、智能回复、特效展示于一体的 Bilibili 直播辅助工具，帮助主播降低直播运营成本，提升直播间活跃度与观众体验。

### 1.2 目标用户
- Bilibili 全职/兼职主播
- 运营团队成员
- 需要 24 小时无人值守直播的虚拟主播或挂机直播间

---

## 1.3 用户研究

### 1.3.1 用户画像

#### 用户画像 A：全职主播小王
- **基本信息**: 28岁，全职游戏主播，每天直播 8 小时
- **痛点**:
  - 直播时需要同时关注弹幕、游戏操作和礼物感谢，分身乏术
  - 夜间挂机时无人互动，观众流失严重
  - 手动感谢舰长太慢，错过感谢时机
- **期望**:
  - 自动感谢舰长和礼物
  - 夜间有人工智能代为互动
  - 数据看板帮助分析直播效果

#### 用户画像 B：运营者小张
- **基本信息**: 25岁，运营直播账号，负责互动和内容策划
- **痛点**:
  - 需要同时回复弹幕和管理直播
  - 语音识别不准确，AI 回复不够自然
  - 缺少高光时刻记录工具
- **期望**:
  - AI 智能回复观众问题
  - 自动录制高光片段
  - 智能数据分析辅助运营决策

#### 用户画像 C：兼职主播小李
- **基本信息**: 22岁，大学生，兼职直播，每周直播 3-4 次
- **痛点**:
  - 不熟悉 Bilibili 直播协议
  - 缺少自动化工具，手动操作繁琐
  - 预算有限，不愿付费订阅
- **期望**:
  - 简单易用的免费功能
  - 快速上手，无需学习成本
  - 基础的自动弹幕功能

### 1.3.2 用户场景

#### 场景 1：夜间挂机直播
**背景**: 主播休息时间，直播间继续播放录像或挂机内容
**流程**:
1. 主播配置好自动弹幕列表（每 5 分钟发送一条）
2. 配置礼物感谢规则（收到礼物自动感谢）
3. 开启 AI 机器人模式
4. 离开，系统自动运行
5. 观众发送弹幕，AI 识别并回复
6. 观众赠送礼物，系统自动发送感谢弹幕
7. 第二天主播查看日志和数据看板

#### 场景 2：高光时刻录制
**背景**: 主播正在玩高难度游戏，即将挑战成功
**流程**:
1. 系统检测到弹幕数量激增（热度上升）
2. 自动开始录制
3. 主播挑战成功，弹幕刷屏"牛逼！"
4. 系统继续录制 30 秒后自动停止
5. 生成 MP4 文件
6. 主播一键上传到 B站动态

#### 场景 3：舰长感谢特效
**背景**: 观众"粉丝A"首次开通舰长
**流程**:
1. 系统收到舰长赠送事件
2. 识别为"首次上舰"
3. 触发金色光效动画
4. 播放欢迎音效
5. 弹幕显示"感谢 粉丝A 首次开通舰长！🎉"
6. 特效在屏幕上停留 5 秒后淡出

### 1.3.3 用户故事

| ID | 用户故事 | 优先级 | 验收标准 | 状态 |
|----|----------|--------|----------|------|
| US-001 | 作为全职主播，我想要自动感谢舰长，以便不遗漏任何感谢 | Must | 收到舰长后 3 秒内自动发送感谢弹幕 | ✅ |
| US-002 | 作为虚拟主播，我想要语音识别转写，以便 AI 能理解我的话 | Should | 语音识别准确率达到 90% 以上 | ❌ |
| US-003 | 作为兼职主播，我想要简单配置弹幕池，以便快速上手 | Must | 配置界面在 3 步内完成设置 | ✅ |
| US-004 | 作为运营者，我想要查看直播数据，以便优化直播策略 | Should | 数据看板实时更新，延迟 < 5 秒 | ✅ |
| US-005 | 作为主播，我想要自定义上舰特效，以便突出个人风格 | Could | 支持 10+ 预设模板和自定义颜色 | ✅ |
| US-006 | 作为主播，我想要自动禁言刷屏用户，以便维护直播间秩序 | Must | 检测到刷屏后 2 秒内自动禁言 | ❌ |

---

## 1.4 功能优先级 (MoSCoW)

### Must Have (必须有，v1.0)
- ✅ Bilibili 直播间连接与弹幕接收
- ✅ 基础自动弹幕发送（定时/随机）
- ✅ 简单礼物感谢弹幕
- ✅ 基础 UI 界面
- ✅ 配置文件保存与加载
- ✅ 本地敏感词过滤

### Should Have (应该有，v1.5)
- ✅ 分级用户互动配置
- ✅ 上舰特效展示（Canvas/CSS）
- ✅ 实时数据看板
- ✅ 自动禁言与刷屏防护
- ✅ 黑名单管理
- ❌ 高光手动打点

### Could Have (可以有，v2.0)
- ❌ 语音识别接入（Whisper）
- ❌ AI 对话生成（LLM）
- ❌ 高光自动录制
- ✅ 插件系统
- ❌ 多语言支持

### Won't Have (暂时不做，v3.0+)
- ❌ 云端配置同步
- ❌ 多直播间同时管理
- ❌ 移动端 App
- ❌ 社区市场（插件/特效分享）

---

## 2. 核心功能需求

### 2.1 智能弹幕管理

**功能描述**: 实现弹幕的自动化发送与智能响应。

#### 2.1.1 自动发送弹幕
- **定时发送**: 支持设定固定时间间隔（如每 5 分钟）发送预设弹幕列表。
- **随机发送**: 在弹幕池中随机抽取发送，避免机械化重复。
- **触发词回复**: 当弹幕包含特定关键词时，自动回复对应内容。
- **变量支持**: 支持在弹幕中插入动态变量（如 `{当前时间}`, `{在线人数}`, `{主播昵称}`）。

#### 2.1.2 分级用户互动
- **用户身份识别**: 自动识别观众身份（舰长、房管、老爷、粉丝等级）。
- **差异化配置**:
    - 普通观众：发送欢迎语或通用回复。
    - 舰长：发送专属问候或感谢语。
    - 房管：发送管理指令确认语或特别关注提示。
    - 黑名单用户：自动禁言或回复特定警告信息。

#### 2.1.3 礼物互动弹幕
- **礼物触发机制**:
    - 收到任意礼物时发送统一感谢。
    - 收到特定礼物（如 B 币胶囊、小心心）发送定制感谢。
    - 礼物价值达到阈值（如 100 舰长）触发高亮感谢。
- **连击特效**: 礼物连击期间自动发送弹幕助兴（每 N 次连击发送一次）。

---

### 2.2 上舰特效系统

**功能描述**: 在直播间覆盖层展示精美的舰长赠送感谢特效。

#### 2.2.1 特效触发配置
- **初次上舰**: 用户首次开通舰长时，触发特殊入场特效（如金色光效、全屏欢迎）。
- **续费上舰**: 用户续费舰长时，触发专属续费特效（如绿色光效、再次欢迎）。
- **连击上舰**: 短时间内连续赠送多个舰长，触发连击报数特效。

#### 2.2.2 特效定制化
- **文本模板**: 支持自定义感谢语模板（如“感谢 {用户名} 开通 {舰长等级}！”）。
- **视觉样式**: 预设多种 CSS 动画模板（渐变、旋转、缩放），支持用户自定义颜色。
- **音效配置**: 上舰时播放自定义音效文件。
- **停留时长**: 配置特效在屏幕上的展示时长。

---

### 2.3 AI 智能互动机器人

**功能描述**: 利用语音识别与生成式 AI，实现主播与观众的自然语言对话。

#### 2.3.1 语音识别 (ASR)
- **音频采集**: 实时捕获主播麦克风音频。
- **实时转写**: 将主播语音实时转换为文本。
- **噪音过滤**: 自动过滤背景噪音，提高识别准确率。

#### 2.3.2 AI 对话生成
- **上下文理解**: 维护最近 N 条弹幕的上下文窗口。
- **人设配置**: 设定 AI 机器人的性格（如幽默、温柔、二次元等）。
- **回复策略**:
    - 主动回复：当观众提问时，AI 识别问题并自动生成回复弹幕。
    - 被动接梗：当主播提到特定话题时，AI 自动发送相关梗或表情包。
- **模型接入**: 支持接入 OpenAI、Claude 或本地 LLM（如 Ollama）。

#### 2.3.3 发送控制
- **审核机制**: AI 生成的内容需经过敏感词过滤。
- **发送冷却**: 避免 AI 发送过快导致风控，设置最小发送间隔。
- **手动开关**: 主播可一键开启/关闭 AI 自动回复。

---

## 2.4 详细功能规格 (Functional Specifications)

### 2.4.1 自动发送弹幕

**输入**:
- 弹幕池配置（JSON 格式）
- 发送规则（定时间隔/随机触发词）
- 动态变量映射

**输出**:
- 发送成功/失败状态
- 发送时间戳
- 错误信息（如有）

**前置条件**:
- 已连接 Bilibili 直播间
- 已配置弹幕池和发送规则
- 用户已登录 Bilibili 账号

**后置条件**:
- 弹幕已发送到直播间
- 已更新发送日志
- 已更新下次发送时间

**异常处理**:
- 网络断开：显示错误提示，自动重连
- 弹幕被风控：暂停发送 5 分钟，降低发送频率
- 包含敏感词：自动跳过该弹幕，记录日志

**验收标准**:
- 定时发送误差 < 5 秒
- 随机发送覆盖率 > 95%
- 触发词响应延迟 < 2 秒
- 变量替换准确率 100%

### 2.4.2 上舰特效展示

**输入**:
- 上舰事件数据（用户名、舰长等级、是否首次）
- 特效配置（动画类型、颜色、音效、停留时长）

**输出**:
- 特效动画（Canvas 渲染）
- 音效播放
- 感谢弹幕发送

**前置条件**:
- 已连接 Bilibili 直播间
- 已配置特效模板
- 已加载音效文件（如启用）

**后置条件**:
- 特效已展示
- 音效已播放
- 感谢弹幕已发送
- 特效已从屏幕移除

**异常处理**:
- 音效文件缺失：仅展示视觉特效，记录警告日志
- 动画渲染失败：降级为文本特效
- 特效配置错误：使用默认特效，提示用户

**验收标准**:
- 特效触发延迟 < 1 秒
- 动画流畅度 > 30 FPS
- 音效与动画同步误差 < 100ms
- 多个舰长同时上舰时，队列有序处理

### 2.4.3 AI 智能回复

**输入**:
- 弹幕文本（最近的 N 条）
- 主播语音（音频流）
- AI 配置（人设、模型选择、API Key）

**输出**:
- AI 生成的回复文本
- 语音识别转写结果
- 发送决策（发送/不发送）

**前置条件**:
- 已配置 AI 模型接入
- 已设置人设和回复策略
- 已开启 AI 功能开关

**后置条件**:
- AI 回复已生成
- 已通过敏感词过滤
- 已应用发送冷却
- 决策是否发送

**异常处理**:
- API 调用失败：降级到预设回复，记录错误
- 语音识别超时：跳过该轮对话，继续监听
- 生成内容过长：截断至最大长度，保留核心信息
- 包含敏感词：丢弃该回复，记录日志

**验收标准**:
- 语音识别延迟 < 3 秒
- AI 生成延迟 < 5 秒
- 回复相关性 > 80%（人工评估）
- 敏感词过滤准确率 100%
- 发送冷却时间 < 10 秒

---

## 3. 补充功能建议

### 3.1 直播数据看板 ✅
- ✅ **实时数据**: 展示当前在线人数、弹幕数量、礼物收入。
- ✅ **趋势图表**: 绘制本场直播的人气曲线与收益曲线。
- ❌ **排行榜**: 展示本场送礼榜、弹幕榜、舰长贡献榜。

### 3.2 高光录制 ❌
- ❌ **自动录制**: 当直播间热度激增（如弹幕刷屏、高价值礼物）时，自动开始录制片段。
- ❌ **标记点**: 支持主播手动按下快捷键打点，方便后期剪辑。
- ❌ **格式输出**: 自动导出为 MP4 或 GIF 格式，支持直接上传 B站动态。

### 3.3 敏感词过滤与风控 ✅
- ❌ **自动禁言**: 检测到违禁词（广告、引战）自动禁言用户。
- ❌ **刷屏防护**: 检测同一用户短时间内发送重复内容，自动禁言。
- ✅ **黑名单管理**: 跨直播间共享黑名单用户列表。

### 3.4 更新日志与版本管理 ✅

**功能描述**: 在应用内提供版本更新检测、更新日志查看及"新版本特性"展示能力，使用户及时了解产品迭代内容。

#### 3.4.1 自动检测更新
- **启动检测**: 应用启动时自动检查是否有新版本可用（可在设置中关闭）。
- **后台轮询**: 运行期间每隔固定周期（默认 4 小时）静默检查一次更新。
- **检查源**: 从配置的更新服务器（默认 GitHub Releases）获取最新版本信息，比对本地版本号（遵循 Semantic Versioning）。

#### 3.4.2 更新日志查看器
- **入口**: 通过菜单栏「帮助 → 更新日志」或设置页面「关于 → 查看更新日志」打开。
- **内容格式**: 以时间倒序展示历史版本记录，每条记录包含：
  - 版本号（如 `v1.2.0`）
  - 发布日期
  - 变更分类（新功能 / 改进 / 修复 / 破坏性变更），遵循 [Keep a Changelog](https://keepachangelog.com/) 规范
  - 变更条目列表
- **数据来源**: 本地打包的 `CHANGELOG.md` 文件，启动时解析并缓存。

#### 3.4.3 "新版本特性"弹窗
- **触发条件**: 应用检测到本地版本号发生变更（即用户完成更新后首次启动）。
- **展示内容**: 当前版本的更新日志摘要，突出显示新功能和重要改进。
- **交互**: 用户点击「知道了」关闭弹窗，勾选「不再提醒此版本」后该版本不再弹出。
- **降级**: 弹窗展示失败时静默跳过，不影响正常启动流程。

#### 3.4.4 更新通知与下载
- **通知方式**: 检测到新版本后，在状态栏或菜单栏显示更新提示图标（红点），不强制打断用户操作。
- **更新详情**: 点击提示后展示新版本的更新日志内容及「前往下载」按钮。
- **下载跳转**: 跳转至对应版本的下载页面（GitHub Releases 或官网），暂不支持应用内自动安装。

#### 3.4.5 功能规格

**输入**:
- 当前应用版本号（`package.json` 中的 `version` 字段）
- 更新检查配置（是否开启自动检查、检查间隔）

**输出**:
- 最新版本信息（版本号、发布日期、更新日志内容）
- 更新状态（有更新 / 已是最新 / 检查失败）
- "新版本特性"弹窗

**前置条件**:
- 设备已连接互联网（离线时静默跳过检查）
- 更新服务器可访问

**后置条件**:
- 已记录上次检查时间
- 已更新本地版本状态标记

**异常处理**:
- 网络不可用：静默跳过，不弹出错误提示
- 更新服务器不可达：静默跳过，下次启动重试
- 版本号解析失败：降级为忽略本次检查
- CHANGELOG 文件损坏或缺失：更新日志页面显示"暂无更新日志"

**验收标准**:
- 启动时更新检查耗时 < 3 秒（网络正常情况下）
- 更新检查不阻塞主界面加载（异步执行）
- 更新日志页面支持滚动浏览，渲染 100 条版本记录无卡顿
- "新版本特性"弹窗在版本变更后首次启动时必定展示（除非用户勾选不再提醒）
- 离线状态下无任何错误提示或弹窗

---

## 3.5 非功能性需求 (Non-Functional Requirements)

## 3.5 非功能性需求 (Non-Functional Requirements)

### 3.5.1 性能要求

| 指标 | 要求 | 测量方法 |
|------|------|----------|
| 弹幕接收延迟 | < 100ms | 从 Bilibili 发送弹幕到本地接收的时间差 |
| 弹幕发送延迟 | < 500ms | 从触发发送到弹幕显示在直播间的时间差 |
| 特效渲染帧率 | ≥ 30 FPS | Canvas 动画帧率监控 |
| AI 语音识别延迟 | < 3s | 从说话结束到转写文本返回的时间 |
| AI 生成延迟 | < 5s | 从收到弹幕到生成回复的时间 |
| 内存占用 | < 500MB (空闲) / < 1GB (运行) | 系统资源监控 |
| CPU 占用 | < 10% (空闲) / < 30% (运行) | 系统资源监控 |
| 冷启动时间 | < 5s | 从启动到可用状态的时间 |
| 连接恢复时间 | < 10s | 网络断开后自动重连的时间 |

### 3.5.2 可靠性要求

| 指标 | 要求 | 说明 |
|------|------|------|
| 可用性 | ≥ 99.5% | 系统正常运行时间占比 |
| MTBF (平均故障间隔) | ≥ 72 小时 | 两次故障之间的平均时间 |
| MTTR (平均恢复时间) | ≤ 5 分钟 | 故障恢复的平均时间 |
| 数据丢失率 | 0% | 配置和日志数据不丢失 |
| 异常处理率 | ≥ 95% | 捕获并处理异常的比例 |

### 3.5.3 安全性要求

| 需求 | 描述 |
|------|------|
| API Key 加密 | 所有 API Key 使用 AES-256 加密存储 |
| 数据传输加密 | 与 Bilibili 和 AI 服务通信使用 HTTPS/TLS 1.3 |
| 本地数据保护 | 敏感配置文件权限设置为仅用户可读写 |
| 输入验证 | 所有用户输入进行严格验证和清理 |
| SQL 注入防护 | 使用参数化查询（如使用 SQLite） |
| XSS 防护 | 所有用户生成内容进行 HTML 转义 |
| CSRF 防护 | 跨站请求伪造防护（如涉及 Web 界面） |
| 隐私保护 | 不保存用户语音数据，不在本地存储观众信息 |

### 3.5.4 兼容性要求

| 类别 | 要求 |
|------|------|
| 操作系统 | Windows 10+、macOS 10.15+、Ubuntu 20.04+ |
| Node.js 版本 | ≥ 18.0.0 |
| 内存要求 | ≥ 4GB RAM |
| 磁盘空间 | ≥ 500MB 可用空间 |
| 网络要求 | 稳定的互联网连接（≥ 10 Mbps） |
| Bilibili 账号 | 支持实名认证账号 |

### 3.5.5 可扩展性要求

| 需求 | 描述 |
|------|------|
| 插件系统 | ✅ 支持第三方插件扩展功能 |
| 多语言 | 预留国际化接口，支持多语言 |
| 配置导入导出 | 支持 JSON/YAML 格式的配置导入导出 |
| 模块化架构 | 核心功能模块化，便于单独升级 |
| API 扩展 | 支持自定义 AI 模型接入（自定义 Endpoint） |

### 3.5.6 可维护性要求

| 需求 | 描述 |
|------|------|
| 日志系统 | 结构化日志，支持日志级别（DEBUG/INFO/WARN/ERROR） |
| 错误追踪 | 自动收集错误堆栈和上下文信息 |
| 诊断模式 | 提供诊断工具帮助排查问题 |
| 更新机制 | ✅ 支持自动检测和安装更新 |
| 文档完善 | 提供完整的用户文档和开发者文档 |

### 3.5.7 可用性要求

| 需求 | 描述 |
|------|------|
| 学习曲线 | 新用户能在 15 分钟内完成基础配置 |
| 错误提示 | 错误信息清晰易懂，提供解决方案 |
| 快捷键支持 | 常用操作支持快捷键 |
| 无障碍访问 | 支持屏幕阅读器和高对比度模式 |
| 响应式布局 | 适配不同屏幕分辨率（≥ 1280x720） |

---

## 4. 数据模型设计

### 4.1 数据库设计

#### 4.1.1 配置表 (config)

```sql
CREATE TABLE config (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    key TEXT UNIQUE NOT NULL,
    value TEXT NOT NULL,
    type TEXT NOT NULL,  -- 'string', 'number', 'boolean', 'json'
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 示例数据
INSERT INTO config (key, value, type, description) VALUES
('danmaku.enabled', 'true', 'boolean', '自动弹幕功能开关'),
('danmaku.interval', '300', 'number', '自动弹幕发送间隔（秒）'),
('ai.enabled', 'false', 'boolean', 'AI 智能回复开关'),
('ai.model', 'gpt-4', 'string', 'AI 模型选择');
```

#### 4.1.2 弹幕池表 (danmaku_pool)

```sql
CREATE TABLE danmaku_pool (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    content TEXT NOT NULL,
    type TEXT NOT NULL,  -- 'timed', 'random', 'trigger'
    trigger_word TEXT,   -- 触发词（当 type='trigger' 时）
    enabled BOOLEAN DEFAULT TRUE,
    priority INTEGER DEFAULT 0,  -- 优先级，数字越大优先级越高
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

#### 4.1.3 礼物感谢规则表 (gift_rules)

```sql
CREATE TABLE gift_rules (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    gift_id TEXT UNIQUE NOT NULL,
    gift_name TEXT NOT NULL,
    thank_message TEXT NOT NULL,
    combo_enabled BOOLEAN DEFAULT FALSE,
    combo_interval INTEGER,  -- 连击触发间隔（次）
    enabled BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

#### 4.1.4 特效配置表 (effect_config)

```sql
CREATE TABLE effect_config (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    effect_type TEXT NOT NULL,  -- 'captain_first', 'captain_renew', 'gift_combo'
    template_name TEXT NOT NULL,
    text_template TEXT,  -- 如 "感谢 {username} 开通 {level}！"
    animation_type TEXT NOT NULL,  -- 'fade', 'slide', 'scale', 'rotate'
    primary_color TEXT DEFAULT '#FFD700',  -- 十六进制颜色
    secondary_color TEXT DEFAULT '#FFFFFF',
    duration INTEGER DEFAULT 5000,  -- 毫秒
    sound_file TEXT,  -- 音效文件路径
    enabled BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

#### 4.1.5 黑名单表 (blacklist)

```sql
CREATE TABLE blacklist (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id TEXT UNIQUE NOT NULL,  -- Bilibili 用户 ID
    username TEXT NOT NULL,
    reason TEXT,
    banned_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    banned_until TIMESTAMP,  -- NULL 表示永久封禁
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

#### 4.1.6 日志表 (logs)

```sql
CREATE TABLE logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    level TEXT NOT NULL,  -- 'DEBUG', 'INFO', 'WARN', 'ERROR'
    module TEXT NOT NULL,  -- 'danmaku', 'effect', 'ai', 'system'
    message TEXT NOT NULL,
    metadata TEXT,  -- JSON 格式的额外信息
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 索引
CREATE INDEX idx_logs_level ON logs(level);
CREATE INDEX idx_logs_module ON logs(module);
CREATE INDEX idx_logs_created_at ON logs(created_at);
```

#### 4.1.7 直播数据表 (live_stats)

```sql
CREATE TABLE live_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    room_id TEXT NOT NULL,
    viewer_count INTEGER DEFAULT 0,
    danmaku_count INTEGER DEFAULT 0,
    gift_value INTEGER DEFAULT 0,  -- 舰长数
    recorded_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 索引
CREATE INDEX idx_live_stats_room_id ON live_stats(room_id);
CREATE INDEX idx_live_stats_recorded_at ON live_stats(recorded_at);
```

### 4.2 数据结构定义

#### 4.2.1 弹幕对象 (Danmaku)

```typescript
interface Danmaku {
  id: string;
  userId: string;
  username: string;
  content: string;
  timestamp: number;
  userType: 'normal' | 'captain' | 'admin' | 'owner';
  badge?: {
    level: number;
    name: string;
  };
  medal?: {
    level: number;
    name: string;
    color: string;
  };
}
```

#### 4.2.2 礼物对象 (Gift)

```typescript
interface Gift {
  id: string;
  giftId: string;
  giftName: string;
  userId: string;
  username: string;
  count: number;
  price: number;  // 舰长数
  timestamp: number;
  isCombo: boolean;
  comboId?: string;
}
```

#### 4.2.3 舰长对象 (Captain)

```typescript
interface Captain {
  userId: string;
  username: string;
  level: number;  // 1: 督察, 2: 提督, 3: 总督
  giftName: string;
  giftId: string;
  count: number;
  timestamp: number;
  isFirstTime: boolean;  // 是否首次上舰
  isRenewal: boolean;    // 是否续费
}
```

#### 4.2.4 AI 配置对象 (AIConfig)

```typescript
interface AIConfig {
  enabled: boolean;
  provider: 'openai' | 'anthropic' | 'ollama' | 'custom';
  apiKey: string;  // 加密存储
  model: string;
  endpoint?: string;  // 自定义 Endpoint
  personality: {
    name: string;
    tone: 'humorous' | 'gentle' | 'professional' | 'anime';
    responseStyle: 'casual' | 'formal' | 'friendly';
  };
  contextWindow: number;  // 上下文弹幕数量
  minInterval: number;   // 最小发送间隔（秒）
  sensitiveWords: string[];
}
```

#### 4.2.5 特效对象 (Effect)

```typescript
interface Effect {
  id: string;
  type: 'captain_first' | 'captain_renew' | 'gift_combo';
  template: {
    text: string;  // 支持 {username}, {level}, {giftName} 等变量
    animation: 'fade' | 'slide' | 'scale' | 'rotate' | 'particle';
    colors: {
      primary: string;
      secondary: string;
    };
    duration: number;  // 毫秒
    fontSize: number;
  };
  sound?: {
    enabled: boolean;
    file: string;
    volume: number;  // 0-1
  };
}
```

---

## 5. API 设计

### 5.1 Bilibili 直播间 API

#### 5.1.1 连接直播间

```typescript
// 请求
interface ConnectRoomRequest {
  roomId: string;        // 直播间号
  cookie: string;        // Bilibili Cookie
}

// 响应
interface ConnectRoomResponse {
  success: boolean;
  message: string;
  roomInfo?: {
    roomId: string;
    roomName: string;
    ownerId: string;
    ownerName: string;
    onlineCount: number;
  };
}
```

#### 5.1.2 发送弹幕

```typescript
// 请求
interface SendDanmakuRequest {
  roomId: string;
  content: string;
  color?: string;       // 十六进制颜色
  fontSize?: number;    // 字体大小 (18-25)
}

// 响应
interface SendDanmakuResponse {
  success: boolean;
  message: string;
  danmakuId?: string;
  timestamp?: number;
}
```

#### 5.1.3 发送礼物感谢弹幕

```typescript
// 请求
interface SendThankRequest {
  roomId: string;
  giftId: string;
  userId: string;
  username: string;
  count: number;
  isCombo: boolean;
}

// 响应
interface SendThankResponse {
  success: boolean;
  message: string;
  danmakuId?: string;
}
```

#### 5.1.4 禁言用户

```typescript
// 请求
interface BanUserRequest {
  roomId: string;
  userId: string;
  duration: number;     // 禁言时长（分钟），0 表示永久
  reason: string;
}

// 响应
interface BanUserResponse {
  success: boolean;
  message: string;
}
```

### 5.2 AI 服务 API

#### 5.2.1 语音识别 (ASR)

```typescript
// 请求
interface ASRRequest {
  audioData: ArrayBuffer;  // 音频数据（PCM 或 WAV）
  language?: string;       // 'zh-CN' (默认)
  model?: string;          // 'whisper-1' (默认)
}

// 响应
interface ASRResponse {
  text: string;
  confidence: number;      // 0-1
  language: string;
  duration: number;        // 音频时长（秒）
}
```

#### 5.2.2 AI 对话生成

```typescript
// 请求
interface ChatRequest {
  messages: ChatMessage[];
  model: string;
  temperature?: number;    // 0-2，默认 0.7
  maxTokens?: number;      // 最大 token 数，默认 100
  systemPrompt?: string;   // 系统提示词
}

interface ChatMessage {
  role: 'system' | 'user' | 'assistant';
  content: string;
}

// 响应
interface ChatResponse {
  message: string;
  finishReason: string;
  usage: {
    promptTokens: number;
    completionTokens: number;
    totalTokens: number;
  };
}
```

### 5.3 本地数据 API

#### 5.3.1 配置管理

```typescript
// 获取配置
function getConfig(key: string): Promise<any>;

// 设置配置
function setConfig(key: string, value: any): Promise<void>;

// 批量获取配置
function getConfigs(keys: string[]): Promise<Record<string, any>>;

// 批量设置配置
function setConfigs(configs: Record<string, any>): Promise<void>;
```

#### 5.3.2 弹幕池管理

```typescript
// 获取弹幕池
function getDanmakuPool(): Promise<DanmakuPoolItem[]>;

// 添加弹幕
function addDanmaku(item: DanmakuPoolItem): Promise<void>;

// 更新弹幕
function updateDanmaku(id: number, item: Partial<DanmakuPoolItem>): Promise<void>;

// 删除弹幕
function deleteDanmaku(id: number): Promise<void>;

// 启用/禁用弹幕
function toggleDanmaku(id: number, enabled: boolean): Promise<void>;
```

#### 5.3.3 日志查询

```typescript
// 查询日志
function queryLogs(options: {
  level?: 'DEBUG' | 'INFO' | 'WARN' | 'ERROR';
  module?: string;
  startTime?: Date;
  endTime?: Date;
  limit?: number;
}): Promise<Log[]>;

// 清理旧日志
function cleanLogs(beforeDate: Date): Promise<number>;
```

---

## 6. 测试策略

### 6.1 测试金字塔

```
        /\
       /  \
      / E2E \        10% - 端到端测试
     /--------\
    /  集成测试  \     30% - API 和模块集成测试
   /--------------\
  /     单元测试     \   60% - 函数和组件单元测试
 /------------------\
```

### 6.2 单元测试

**覆盖目标**: 代码覆盖率 ≥ 80%

**测试框架**: Jest + Vue Test Utils

**关键测试模块**:

| 模块 | 测试重点 | 预期覆盖率 |
|------|----------|------------|
| 弹幕管理器 | 发送逻辑、触发词匹配、变量替换 | ≥ 90% |
| 效果渲染器 | Canvas 绘制、动画计算、队列管理 | ≥ 85% |
| AI 模块 | 上下文构建、敏感词过滤、冷却控制 | ≥ 90% |
| 配置管理 | 增删改查、验证逻辑、默认值 | ≥ 95% |
| 数据库 | CRUD 操作、索引查询、事务处理 | ≥ 85% |

**示例测试用例**:

```typescript
describe('弹幕管理器', () => {
  it('应正确替换变量', () => {
    const template = '感谢 {username} 在 {time} 送出的礼物！';
    const variables = { username: '测试用户', time: '12:00' };
    const result = replaceVariables(template, variables);
    expect(result).toBe('感谢 测试用户 在 12:00 送出的礼物！');
  });

  it('应在风控时自动降低发送频率', async () => {
    const manager = new DanmakuManager();
    await manager.send({ content: '测试弹幕' });
    // 模拟风控响应
    manager.handleRateLimit();
    expect(manager.getInterval()).toBeGreaterThan(manager.defaultInterval);
  });
});
```

### 6.3 集成测试

**测试框架**: Supertest + Jest

**测试场景**:

1. **弹幕接收与响应流程**
   - 模拟 Bilibili WebSocket 消息
   - 验证弹幕解析和存储
   - 验证触发词匹配和自动回复
   - 验证发送成功和错误处理

2. **AI 对话流程**
   - 模拟麦克风输入
   - 验证 ASR API 调用
   - 验证 LLM API 调用
   - 验证敏感词过滤
   - 验证弹幕发送决策

3. **特效触发流程**
   - 模拟舰长赠送事件
   - 验证特效队列管理
   - 验证 Canvas 渲染
   - 验证音效播放
   - 验证并发处理

### 6.4 端到端测试 (E2E)

**测试框架**: Playwright

**测试场景**:

1. **用户首次启动**
   - 打开应用
   - 显示欢迎界面
   - 引导配置 Bilibili 账号
   - 连接直播间成功
   - 显示主界面

2. **配置自动弹幕**
   - 导航到弹幕配置页面
   - 添加 3 条弹幕
   - 启用定时发送
   - 保存配置
   - 验证弹幕发送

3. **配置 AI 机器人**
   - 导航到 AI 配置页面
   - 输入 API Key
   - 选择人设
   - 启动测试
   - 验证回复生成

4. **高光时刻录制**
   - 模拟热度激增
   - 验证自动录制触发
   - 验证文件生成
   - 验证上传功能

### 6.5 性能测试

**测试工具**: k6 + Lighthouse

**测试指标**:

| 指标 | 目标值 | 测试方法 |
|------|--------|----------|
| 冷启动时间 | < 5s | 测量从双击图标到可用状态的时间 |
| 内存占用 (空闲) | < 500MB | 运行 1 小时后测量 |
| 内存占用 (运行) | < 1GB | 模拟 1000 条弹幕/分钟后测量 |
| CPU 占用 (空闲) | < 10% | 系统监控 10 分钟 |
| CPU 占用 (运行) | < 30% | 模拟 1000 条弹幕/分钟后监控 |
| 弹幕处理吞吐量 | ≥ 500 条/秒 | 压力测试 |
| 特效渲染帧率 | ≥ 30 FPS | 渲染 10 个并发特效时测量 |

### 6.6 安全测试

**测试工具**: OWASP ZAP + 代码审计

**测试项**:

- [ ] API Key 加密存储验证
- [ ] 敏感数据传输加密验证
- [ ] SQL 注入攻击测试
- [ ] XSS 攻击测试
- [ ] CSRF 攻击测试
- [ ] 权限控制测试
- [ ] 输入验证测试

### 6.7 兼容性测试

**测试环境**:

| 操作系统 | 版本 | 测试状态 |
|----------|------|----------|
| Windows | 10, 11 | ✅ 待测试 |
| macOS | 10.15+, 11, 12, 13 | ✅ 待测试 |
| Linux | Ubuntu 20.04, 22.04 | ✅ 待测试 |

### 6.8 用户验收测试 (UAT)

**测试人员**: 5-10 名真实主播

**测试周期**: 1 周

**验收标准**:

- [ ] 能够独立完成基础配置（15 分钟内）
- [ ] 能够连接直播间并接收弹幕
- [ ] 自动弹幕发送正常
- [ ] 礼物感谢功能正常
- [ ] 特效展示流畅
- [ ] 无致命 Bug
- [ ] 满意度评分 ≥ 4/5

---

## 7. 竞品分析

### 7.1 市场竞品概览

| 产品名称 | 开发者 | 主要功能 | 优势 | 劣势 | 价格 |
|----------|--------|----------|------|------|------|
| 弹幕姬 | 社区开源 | 弹幕接收、礼物感谢、简单特效 | 免费开源、功能简洁 | 缺少 AI 功能、UI 较老旧 | 免费 |
| 直播间助手 | 第三方 | 弹幕管理、数据统计、黑名单 | 功能全面、数据可视化 | 付费订阅、无 AI | ¥98/月 |
| Bilibili 直播姬 | 官方 | 推流、弹幕、礼物、互动 | 官方支持、稳定可靠 | 功能有限、无自动化 | 免费 |
| AI 直播助手 | 创业团队 | AI 互动、智能回复 | AI 功能强大、智能程度高 | 功能单一、价格昂贵 | ¥299/月 |

### 7.2 差异化优势

#### 7.2.1 对比弹幕姬

**我们的优势**:
- ✅ 集成 AI 智能回复功能
- ✅ 现代化 UI 设计，用户体验更好
- ✅ 支持语音识别和语音互动
- ✅ 完善的数据看板和分析功能

**弹幕姬的优势**:
- ✅ 完全免费开源
- ✅ 社区活跃，插件丰富
- ✅ 轻量级，资源占用低

#### 7.2.2 对比直播间助手

**我们的优势**:
- ✅ AI 智能互动功能
- ✅ 上舰特效系统
- ✅ 更灵活的配置选项
- ✅ 开源免费（MVP 版本）

**直播间助手的优势**:
- ✅ 功能更加全面
- ✅ 历史久，稳定性强
- ✅ 客服支持完善

#### 7.2.3 对比 Bilibili 直播姬

**我们的优势**:
- ✅ 高度自动化
- ✅ AI 智能互动
- ✅ 自定义特效
- ✅ 数据分析功能

**Bilibili 直播姬的优势**:
- ✅ 官方支持，不会封号
- ✅ 功能全面（推流、录制、互动）
- ✅ 稳定可靠，持续更新

### 7.3 竞争策略

1. **差异化定位**: 聚焦 AI 智能互动和自动化，避开与官方工具的直接竞争
2. **开源策略**: MVP 版本开源免费，积累用户和社区
3. **增值服务**: 高级功能（如本地 LLM、插件扩展）采用付费模式
4. **生态建设**: 提供插件系统和社区市场，鼓励用户贡献

---

## 8. 成功指标 (Success Metrics)

### 8.1 北极星指标 (North Star Metric)

**用户活跃度 (Daily Active Users, DAU)**

- **定义**: 每天至少使用 30 分钟的用户数
- **目标**: 
  - v1.0 发布后 1 个月: 100 DAU
  - v1.0 发布后 3 个月: 500 DAU
  - v1.0 发布后 6 个月: 2,000 DAU

### 8.2 关键绩效指标 (KPIs)

#### 8.2.1 用户增长指标

| 指标 | 目标 (v1.0 发布后) | 测量方法 |
|------|-------------------|----------|
| 下载量 | 1,000 次 | 下载统计 |
| 注册用户数 | 800 人 | 本地统计（不收集个人信息） |
| 新用户留存率 (D7) | ≥ 30% | Cohort 分析 |
| 新用户留存率 (D30) | ≥ 15% | Cohort 分析 |

#### 8.2.2 用户参与度指标

| 指标 | 目标 | 测量方法 |
|------|------|----------|
| 日均使用时长 | ≥ 60 分钟 | 使用时长统计 |
| 功能使用率 | ≥ 70% | 功能点击统计 |
| 自动弹幕使用率 | ≥ 60% | 功能使用统计 |
| AI 功能使用率 | ≥ 30% | API 调用统计 |

#### 8.2.3 产品质量指标

| 指标 | 目标 | 测量方法 |
|------|------|----------|
| 崩溃率 | < 0.1% | 错误日志统计 |
| 平均响应时间 | < 200ms | 性能监控 |
| 用户满意度 (NPS) | ≥ 40 | 用户调研 |
| Bug 报告数 | < 10 个/月 | Issue 统计 |

#### 8.2.4 社区指标

| 指标 | 目标 | 测量方法 |
|------|------|----------|
| GitHub Star 数 | 500 | GitHub API |
| Issue 解决率 | ≥ 90% | Issue 统计 |
| PR 合并率 | ≥ 80% | PR 统计 |
| 社区贡献者数 | ≥ 10 人 | 贡献者统计 |

### 8.3 业务指标 (如商业化)

**注意**: MVP 版本免费，商业化指标为 v2.0 规划

| 指标 | 目标 (v2.0) | 说明 |
|------|-------------|------|
| 付费转化率 | ≥ 5% | 免费用户转付费用户比例 |
| 月经常性收入 (MRR) | ¥10,000 | 月度订阅收入 |
| 客户获取成本 (CAC) | < ¥50 | 平均获客成本 |
| 客户生命周期价值 (LTV) | > ¥500 | 平均客户贡献 |
| 流失率 | < 5%/月 | 付费用户取消比例 |

### 8.4 技术指标

| 指标 | 目标 | 测量方法 |
|------|------|----------|
| 代码覆盖率 | ≥ 80% | Jest 测试报告 |
| API 响应时间 | P95 < 500ms | APM 监控 |
| 数据库查询时间 | P95 < 100ms | 数据库慢查询日志 |
| WebSocket 重连成功率 | ≥ 99% | 连接日志统计 |
| 内存泄漏检测 | 0 个 | Valgrind/Chrome DevTools |

### 8.5 指标追踪方案

**数据收集**:
- 本地日志记录（不上传）
- 可选的匿名使用统计（用户可关闭）
- GitHub Issues 和 PR 统计
- 用户调研和反馈收集

**数据可视化**:
- Grafana 仪表盘（内部使用）
- GitHub Insights（公开指标）
- 定期报告（月度/季度）

**数据审查**:
- 每周指标回顾
- 每月深度分析
- 每季度战略调整

---

## 9. 技术架构建议

### 9.1 技术栈选型
- **客户端框架**: Electron (跨平台桌面应用) 或 Tauri (更轻量)
- **前端框架**: Vue 3 + TypeScript + Pinia (状态管理)
- **后端协议**: WebSocket (连接 Bilibili 直播间)
- **语音识别**: Whisper (OpenAI) 或 阿里云/讯飞语音识别 API
- **AI 模型**: OpenAI API / Anthropic API / 本地 Ollama
- **特效渲染**: HTML5 Canvas + CSS3 Animations

### 9.2 系统架构图
```
┌─────────────────────────────────────────┐
│           Electron 主进程                  │
│  - 窗口管理  - 本地存储  - 系统托盘         │
└─────────────┬───────────────────────────┘
              │
┌─────────────┼───────────────────────────┐
│            渲染进程                      │
│  ┌──────────┐  ┌──────────┐  ┌────────┐│
│  │ 弹幕管理  │  │ 特效展示  │  │ AI模块  ││
│  └──────────┘  └──────────┘  └────────┘│
└──────┬─────────────────┬───────────────┘
       │                 │
       │ WebSocket       │ HTTP/REST
       │                 │
┌──────▼─────────┐  ┌────▼────────────┐
│ Bilibili 直播间 │  │ 第三方 AI 服务   │
│  (弹幕/礼物流)   │  │  (ASR/LLM)     │
└─────────────────┘  └─────────────────┘
```

### 9.3 数据流设计
1. **弹幕流**: Bilibili WebSocket -> 本地解析 -> 规则匹配 -> 弹幕池/特效触发
2. **语音流**: 麦克风输入 -> 音频缓冲 -> ASR API -> 文本 -> LLM API -> 弹幕输出
3. **配置流**: 用户界面操作 -> 本地 JSON/SQLite 存储 -> 运行时加载

---

## 10. 用户界面 (UI/UX) 设计

### 10.1 主界面布局
```
┌───────────────────────────────────────────────┐
│ 菜单栏 [文件][设置][关于]                       │
├─────────────┬─────────────────────────────────┤
│ 侧边栏       │   主工作区                       │
│ ┌─────────┐ │  ┌───────────────────────────┐ │
│ │弹幕配置  │ │  │  当前直播间信息             │ │
│ │上舰特效  │ │  │  房间号: 123456            │ │
│ │AI机器人  │ │  │  在线: 1,234              │ │
│ │数据看板  │ │  │  ───────────────────────  │ │
│ │高光录制  │ │  │  [OBS 源地址预览]          │ │
│ │设置      │ │  │  (用于特效预览)            │ │
│ └─────────┘ │  └───────────────────────────┘ │
│             │  ┌───────────────────────────┐ │
│             │  │  实时日志监控              │ │
│             │  │  > 收到礼物 x1             │ │
│             │  │  > AI 回复: 感谢大佬!      │ │
│             │  └───────────────────────────┘ │
└─────────────┴─────────────────────────────────┘
```

### 10.2 配置页面设计
- **弹幕配置**: 采用列表视图，支持拖拽排序弹幕池。
- **特效配置**: 提供实时预览窗口，边调整参数边看效果。
- **AI 配置**: 分步引导设置 (API Key -> 人设选择 -> 启动测试)。

---

## 11. 风险与挑战

### 11.1 技术风险
- **WebSocket 连接稳定性**: Bilibili 直播协议变更可能导致连接中断，需设计重连机制。
- **AI 延迟**: 语音识别和 LLM 生成存在延迟，可能导致互动不及时，需优化流式传输。
- **资源占用**: 特效渲染和语音处理对 CPU/GPU 占用较高，需提供性能优化选项。

### 11.2 合规风险
- **风控触发**: 自动弹幕频率过高可能导致账号被平台限流或封禁，需严格设置发送上限。
- **隐私保护**: 语音数据传输需加密，且不得私自保存用户语音。

---

## 12. 开发路线图

### 阶段一：MVP (最小可行性产品) - 4 周
- ✅ Bilibili 直播间连接与弹幕接收
- ✅ 基础自动弹幕发送 (定时/随机)
- ✅ 简单礼物感谢弹幕
- ✅ 基础 UI 界面

### 阶段二：核心功能完善 - 6 周
- ✅ 分级用户互动配置
- ✅ 上舰特效展示 (Canvas/CSS)
- ✅ 本地敏感词过滤
- ✅ 配置文件导入导出

### 阶段三：AI 与高级功能 - 8 周
- ❌ 语音识别接入 (Whisper)
- ❌ AI 对话生成 (LLM)
- ❌ 高光自动录制
- ✅ 直播数据看板

### 阶段四：优化与生态 - 持续迭代
- ❌ 性能优化 (降低 CPU 占用)
- ✅ 插件系统 (允许用户扩展)
- ❌ 多语言支持
---

## 13. 附录

### 13.1 参考资料
- [Bilibili Live Protocol (GitHub)](https://github.com/lovelyyoshino/Bilibili-Live-API)
- [Electron 官方文档](https://www.electronjs.org/docs)
- [OpenAI Whisper API](https://platform.openai.com/docs/guides/speech-to-text)
- [Vue 3 文档](https://vuejs.org/)
- [TypeScript 文档](https://www.typescriptlang.org/)

### 13.2 术语表
- **弹幕**: 直播间观众的实时评论。
- **舰长**: Bilibili 直播间的付费订阅用户。
- **ASR**: Automatic Speech Recognition (自动语音识别)。
- **LLM**: Large Language Model (大语言模型)。
- **MVP**: Minimum Viable Product (最小可行性产品)。
- **MoSCoW**: Must have, Should have, Could have, Won't have (需求优先级)。
- **DAU**: Daily Active Users (日活跃用户数)。
- **NPS**: Net Promoter Score (净推荐值)。
- **MTBF**: Mean Time Between Failures (平均故障间隔时间)。
- **MTTR**: Mean Time To Repair (平均修复时间)。
