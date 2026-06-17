# Bilibili 直播助手 - 产品需求文档 (PRD)

**版本号**: v1.0
**创建日期**: 2026-06-03
**文档状态**: 初稿
**最后更新**: 2026-06-17（礼物音效系统功能完成）

---

## 实现进度总览

| 优先级 | 总计 | 已完成 | 完成率 |
|--------|------|--------|--------|
| Must Have (v1.0) | 6 | 6 | 100% |
| Should Have (v1.5) | 7 | 7 | 100% |
| Could Have (v2.0) | 6 | 6 | 100% |
| Won't Have (v3.0+) | 4 | 0 | 0% |
| **总计** | **23** | **19** | **83%** |

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
- ✅ 直播录制（全程/高光，含高光标记与片段提取）
- ✅ 多语言支持（中文/英文）
- ✅ 弹幕语音播报（edge-tts）
- ✅ 语音识别接入（whisper.cpp）
- ✅ AI 对话生成（LLM）
- ✅ 自动禁言与刷屏防护
- ✅ 礼物音效系统

**待实现功能**:
- ❌ 无

---

## 1. 产品概述

### 1.1 产品愿景
打造一款集自动化互动、智能回复、特效展示、直播录制于一体的 Bilibili 直播辅助工具，帮助主播降低直播运营成本，提升直播间活跃度与观众体验，同时满足内容存档和二次创作需求。

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
  - 直播内容无法存档，错过精彩瞬间无法回放
- **期望**:
  - AI 智能回复观众问题
  - 自动录制高光片段
  - 全程录制直播内容，便于后期剪辑
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

#### 场景 4：直播录制与高光剪辑
**背景**: 主播正在进行一场重要的游戏直播，需要录制全程内容并标记精彩瞬间
**流程**:
1. 主播连接直播间后，点击「开始录制」按钮
2. 系统开始录制直播流，状态栏显示录制状态和时长
3. 游戏进行中，弹幕数量激增（观众刷屏"666"）
4. 系统自动检测到热度上升，自动标记高光时刻
5. 主播按下快捷键 F9，手动标记另一个高光时刻
6. 直播结束后，点击「停止录制」
7. 系统生成完整的 MP4 录制文件和独立的高光片段
8. 主播在录制列表中查看文件，选择高光片段上传到 B 站动态

### 1.3.3 用户故事

| ID | 用户故事 | 优先级 | 验收标准 | 状态 |
|----|----------|--------|----------|------|
| US-001 | 作为全职主播，我想要自动感谢舰长，以便不遗漏任何感谢 | Must | 收到舰长后 3 秒内自动发送感谢弹幕 | ✅ |
| US-002 | 作为虚拟主播，我想要语音识别转写，以便 AI 能理解我的话 | Should | 语音识别准确率达到 90% 以上 | ✅ |
| US-003 | 作为兼职主播，我想要简单配置弹幕池，以便快速上手 | Must | 配置界面在 3 步内完成设置 | ✅ |
| US-004 | 作为运营者，我想要查看直播数据，以便优化直播策略 | Should | 数据看板实时更新，延迟 < 5 秒 | ✅ |
| US-005 | 作为主播，我想要自定义上舰特效，以便突出个人风格 | Could | 支持 10+ 预设模板和自定义颜色 | ✅ |
| US-006 | 作为主播，我想要自动禁言刷屏用户，以便维护直播间秩序 | Must | 检测到刷屏后 2 秒内自动禁言 | ✅ |
| US-007 | 作为主播，我想要录制直播内容，以便后期剪辑和存档 | Could | 录制文件可正常播放，CPU 占用增加 < 15% | ✅ |
| US-008 | 作为主播，我想要自动标记高光时刻，以便快速找到精彩片段 | Could | 高光触发响应时间 < 5 秒 | ✅ |
| US-009 | 作为主播，我想要弹幕语音播报，以便不看屏幕也能了解弹幕内容 | Should | 播报延迟 < 2 秒，准确率 > 95% | ✅ |

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
- ✅ 高光手动打点
- ✅ 弹幕语音播报

### Could Have (可以有，v2.0)
- ✅ 语音识别接入（Whisper）
- ✅ AI 对话生成（LLM）
- ✅ 直播录制（全程/高光，含高光标记与片段提取）
- ✅ 插件系统
- ✅ 多语言支持

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

### 3.2 敏感词过滤与风控 ✅
- ✅ **自动禁言**: 检测到违禁词（广告、引战）自动禁言用户。
- ✅ **刷屏防护**: 检测同一用户短时间内发送重复内容，自动禁言。
- ✅ **黑名单管理**: 跨直播间共享黑名单用户列表。

### 3.3 更新日志与版本管理 ✅

**功能描述**: 在应用内提供版本更新检测、更新日志查看及"新版本特性"展示能力，使用户及时了解产品迭代内容。

#### 3.3.1 自动检测更新
- **启动检测**: 应用启动时自动检查是否有新版本可用（可在设置中关闭）。
- **后台轮询**: 运行期间每隔固定周期（默认 4 小时）静默检查一次更新。
- **检查源**: 从配置的更新服务器（默认 GitHub Releases）获取最新版本信息，比对本地版本号（遵循 Semantic Versioning）。

#### 3.3.2 更新日志查看器
- **入口**: 通过菜单栏「帮助 → 更新日志」或设置页面「关于 → 查看更新日志」打开。
- **内容格式**: 以时间倒序展示历史版本记录，每条记录包含：
  - 版本号（如 `v1.2.0`）
  - 发布日期
  - 变更分类（新功能 / 改进 / 修复 / 破坏性变更），遵循 [Keep a Changelog](https://keepachangelog.com/) 规范
  - 变更条目列表
- **数据来源**: 本地打包的 `CHANGELOG.md` 文件，启动时解析并缓存。

#### 3.3.3 "新版本特性"弹窗
- **触发条件**: 应用检测到本地版本号发生变更（即用户完成更新后首次启动）。
- **展示内容**: 当前版本的更新日志摘要，突出显示新功能和重要改进。
- **交互**: 用户点击「知道了」关闭弹窗，勾选「不再提醒此版本」后该版本不再弹出。
- **降级**: 弹窗展示失败时静默跳过，不影响正常启动流程。

#### 3.3.4 更新通知与下载
- **通知方式**: 检测到新版本后，在状态栏或菜单栏显示更新提示图标（红点），不强制打断用户操作。
- **更新详情**: 点击提示后展示新版本的更新日志内容及「前往下载」按钮。
- **下载跳转**: 跳转至对应版本的下载页面（GitHub Releases 或官网），暂不支持应用内自动安装。

#### 3.3.5 功能规格

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

### 3.4 直播录制 ✅

**功能描述**: 提供直播流录制能力，支持全程录制和高光片段录制，满足主播内容存档、二次剪辑和内容复用需求。

**实现状态**: 已完成（2026-06-16）

**已实现功能**:
- ✅ 全程录制（MP4/FLV 格式，可选质量：原始/720p/1080p）
- ✅ 分段录制（按文件大小自动分段）
- ✅ 高光自动标记（弹幕激增/高价值礼物/舰长赠送）
- ✅ 高光手动标记（F9 快捷键）
- ✅ 录制管理页面（文件列表、删除、打开目录）
- ✅ 录制状态栏（实时显示状态/时长/大小）
- ✅ 设置页面录制配置
- ✅ FFmpeg 集成

**技术实现**:
- 主进程模块：`src/main/modules/recording/`
- 渲染进程组件：`Recording.vue`, `RecordingStatusBar.vue`, `RecordingList.vue`, `HighlightList.vue`
- 状态管理：`stores/recording.ts`
- 服务层：`services/recording-service.ts`

**测试覆盖**:
- 单元测试：117 个测试用例全部通过
- 性能测试：弹幕处理 584ms/10000条，内存泄漏检测通过

#### 3.4.1 全程录制
- **自动开始**: 连接直播间后自动开始录制直播流（可在设置中关闭）。
- **格式支持**: 支持 MP4（H.264+AAC）和 FLV 格式输出。
- **质量配置**: 支持选择录制质量（原始流质量 / 720p / 1080p），默认使用原始流质量。
- **分段录制**: 支持按文件大小（如 2GB）或时长（如 1 小时）自动分段，避免单文件过大。
- **存储管理**: 支持自定义存储路径，可设置最大存储空间和自动清理策略。

#### 3.4.2 高光录制
- **自动触发**: 当直播间热度激增（弹幕刷屏、高价值礼物、舰长赠送）时，自动标记高光时刻。
- **手动打点**: 支持主播按下快捷键（默认 F9）手动标记高光时刻。
- **片段提取**: 自动提取高光时刻前后 N 秒（默认 30 秒）的录制片段。
- **独立保存**: 高光片段单独保存为 MP4 或 GIF 文件，支持直接上传 B 站动态，便于后续剪辑和分享。

#### 3.4.3 录制管理
- **录制列表**: 展示所有录制文件，按时间倒序排列，显示文件名、时长、大小、录制时间。
- **播放预览**: 支持在应用内预览录制文件（使用系统默认播放器或内嵌播放器）。
- **文件操作**: 支持重命名、删除、打开文件所在目录。
- **导出功能**: 支持导出为 MP4 格式，可直接上传到 B 站动态或其他平台。

#### 3.4.4 录制状态显示
- **状态栏指示**: 在主界面状态栏显示录制状态（录制中 / 已暂停 / 已停止）。
- **录制时长**: 实时显示当前录制时长。
- **文件大小**: 实时显示当前录制文件大小。
- **快捷操作**: 支持一键开始/暂停/停止录制。

#### 3.4.5 功能规格

**输入**:
- 直播流 URL（从 Bilibili 直播间获取）
- 录制配置（格式、质量、分段大小、存储路径）
- 高光触发规则（热度阈值、礼物价值、舰长事件）
- 快捷键配置

**输出**:
- 录制文件（MP4/FLV）
- 高光片段（MP4）
- 录制元数据（JSON 格式，包含录制时间、直播间信息、时长等）

**前置条件**:
- 已连接 Bilibili 直播间
- 已配置存储路径（有足够磁盘空间）
- 直播流可访问

**后置条件**:
- 录制文件已保存到指定路径
- 录制元数据已记录
- 高光片段已独立保存（如有）

**异常处理**:
- 直播流中断：自动重连，录制暂停，重连成功后继续录制
- 磁盘空间不足：停止录制，提示用户清理空间
- 录制文件损坏：记录错误日志，尝试修复或重新录制
- 网络波动：使用缓冲机制，避免录制卡顿

**验收标准**:
- 录制启动延迟 < 3 秒
- 录制文件与直播流同步误差 < 1 秒
- 录制过程中 CPU 占用增加 < 15%
- 录制过程中内存占用增加 < 200MB
- 高光触发响应时间 < 5 秒
- 录制文件可正常使用主流播放器播放
- 分段录制无丢帧或卡顿

#### 3.4.6 数据模型

```sql
-- 录制记录表
CREATE TABLE recordings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    room_id TEXT NOT NULL,
    room_name TEXT,
    file_path TEXT NOT NULL,
    file_name TEXT NOT NULL,
    file_size INTEGER DEFAULT 0,
    duration INTEGER DEFAULT 0,  -- 秒
    format TEXT DEFAULT 'mp4',   -- 'mp4', 'flv'
    quality TEXT DEFAULT 'original',  -- 'original', '720p', '1080p'
    status TEXT DEFAULT 'recording',  -- 'recording', 'paused', 'completed', 'failed'
    is_highlight BOOLEAN DEFAULT FALSE,
    highlight_reason TEXT,  -- 'auto', 'manual', 'gift', 'captain'
    started_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    ended_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 高光时刻表
CREATE TABLE highlights (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    recording_id INTEGER,
    room_id TEXT NOT NULL,
    timestamp INTEGER NOT NULL,  -- 相对于录制开始的秒数
    reason TEXT NOT NULL,  -- 'danmaku_surge', 'high_value_gift', 'captain', 'manual'
    description TEXT,
    clip_path TEXT,  -- 高光片段文件路径
    clip_duration INTEGER DEFAULT 30,  -- 片段时长（秒）
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (recording_id) REFERENCES recordings(id)
);
```

#### 3.4.7 API 设计

```typescript
// 开始录制
function startRecording(options: RecordingOptions): Promise<RecordingSession>;

// 暂停录制
function pauseRecording(sessionId: string): Promise<void>;

// 恢复录制
function resumeRecording(sessionId: string): Promise<void>;

// 停止录制
function stopRecording(sessionId: string): Promise<RecordingResult>;

// 标记高光时刻
function markHighlight(sessionId: string, reason: string): Promise<Highlight>;

// 获取录制列表
function getRecordings(options?: QueryOptions): Promise<Recording[]>;

// 获取高光列表
function getHighlights(recordingId?: string): Promise<Highlight[]>;

// 删除录制
function deleteRecording(recordingId: string): Promise<void>;

// 录制配置
interface RecordingOptions {
  roomId: string;
  format: 'mp4' | 'flv';
  quality: 'original' | '720p' | '1080p';
  segmentSize?: number;  // MB
  segmentDuration?: number;  // 秒
  outputPath: string;
  autoStart: boolean;
  highlightEnabled: boolean;
  highlightConfig?: {
    danmakuThreshold: number;  // 弹幕数量阈值
    giftValueThreshold: number;  // 礼物价值阈值
    clipDuration: number;  // 高光片段时长（秒）
  };
}

// 录制会话
interface RecordingSession {
  id: string;
  roomId: string;
  status: 'recording' | 'paused' | 'stopped';
  startedAt: Date;
  filePath: string;
  currentSize: number;
  currentDuration: number;
}

// 录制结果
interface RecordingResult {
  sessionId: string;
  filePath: string;
  fileSize: number;
  duration: number;
  highlightCount: number;
}
```

#### 3.4.8 技术实现建议

- **录制引擎**: 使用 FFmpeg 或 fluent-ffmpeg 进行直播流录制和转码。
- **流媒体协议**: 支持 RTMP、HLS、FLV 协议的直播流录制。
- **硬件加速**: 支持 NVIDIA NVENC、Intel Quick Sync、AMD VCE 硬件加速编码。
- **内存缓冲**: 使用环形缓冲区处理网络波动，避免录制卡顿。
- **文件分段**: 在关键帧处进行分段，确保分段无缝衔接。

---

### 3.5 弹幕语音播报 ✅

**功能描述**: 将直播间的弹幕内容通过语音合成（TTS）技术实时播报为主播可听到的语音，帮助主播在不查看屏幕的情况下了解弹幕内容，提升互动体验和直播效率。

**实现状态**: 已完成（2026-06-16）

**已实现功能**:
- ✅ 集成 edge-tts 开源工具（Microsoft Edge TTS 服务）
- ✅ 支持 Web Speech API 作为备选引擎
- ✅ 18 种高质量中文语音选择
- ✅ 收到弹幕时自动朗读弹幕内容
- ✅ 支持按用户身份过滤播报对象
- ✅ 自动跳过敏感词、广告、刷屏内容
- ✅ 弹幕进入播报队列，按顺序播放
- ✅ 支持礼物播报和舰长播报
- ✅ 支持快捷开关、音量控制、暂停功能
- ✅ 播报配置持久化
- ✅ 引擎自动降级机制

**技术实现**:
- TTS 模块：`src/renderer/modules/tts/`
- TTS 服务：`src/renderer/services/tts-service.ts`
- TTS 状态管理：`src/renderer/stores/tts.ts`
- 主进程 TTS 模块：`src/main/modules/tts/index.ts`
- 设置页面：`src/renderer/views/Settings.vue`
- 开源工具：edge-tts（https://github.com/rany2/edge-tts）

**TTS 引擎对比**:

| 引擎 | 语音质量 | 中文语音数量 | 离线支持 | 费用 |
|------|----------|--------------|----------|------|
| edge-tts | 高（微软神经网络语音） | 18 种 | 否 | 免费 |
| Web Speech API | 中等 | 有限 | 是 | 免费 |

#### 3.5.1 核心功能

- **实时播报**: 收到弹幕时自动朗读弹幕内容（如"用户A说：主播你好"）。
- **选择性播报**: 支持按用户身份（舰长、房管、粉丝等级）过滤播报对象。
- **智能过滤**: 自动跳过敏感词、广告、刷屏内容，仅播报有效弹幕。
- **播报队列**: 弹幕进入播报队列，按顺序播放，避免多条弹幕同时播报造成混乱。

#### 3.5.2 TTS 引擎集成

- **edge-tts（推荐）**: 集成 Microsoft Edge 的在线 TTS 服务，提供高质量神经网络语音。
  - 开源项目：https://github.com/rany2/edge-tts
  - 支持 18 种中文语音（晓晓、云希、云健等）
  - 无需 API Key，免费使用
  - 支持语速、音调调节
- **Web Speech API（备选）**: 浏览器内置 TTS 引擎，支持离线使用。
- **自动降级**: edge-tts 不可用时自动切换到 Web Speech API。
- **语音选择**: 支持选择不同语音（男声/女声/童声）、语速、音调调节。
- **多语言支持**: 支持中文、英文等多语言语音播报。

#### 3.5.3 播报规则配置

- **播报对象**: 配置哪些用户的弹幕需要播报（全部/舰长/房管/指定等级以上）。
- **播报内容**: 配置播报格式（如"{用户名}说：{弹幕内容}"）。
- **播报间隔**: 设置最小播报间隔（如 2 秒），避免过于频繁。
- **静音时段**: 支持设置静音时段（如夜间自动关闭）。
- **关键词过滤**: 配置不播报的关键词列表。

#### 3.5.4 礼物播报

- **礼物感谢播报**: 收到礼物时自动播报感谢语（如"感谢 用户A 送出的 小心心"）。
- **舰长播报**: 舰长开通时播报欢迎语（如"欢迎 用户A 开通舰长"）。
- **价值阈值**: 可配置仅播报超过指定价值的礼物。

#### 3.5.5 用户交互

- **快捷开关**: 支持一键开启/关闭语音播报。
- **音量控制**: 独立控制播报音量，不影响其他音频。
- **播报暂停**: 支持临时暂停播报（如主播需要安静时）。
- **历史回放**: 支持回放最近 N 条播报记录。

#### 3.5.6 功能规格

**输入**:
- 弹幕消息流（来自 Bilibili WebSocket）
- 礼物消息流
- 播报配置（规则、语音、过滤）

**输出**:
- 语音音频输出（通过系统音频设备）
- 播报日志记录

**前置条件**:
- 已连接 Bilibili 直播间
- TTS 引擎可用（本地或云端）
- 系统音频设备正常

**后置条件**:
- 弹幕已播报
- 播报记录已保存

**异常处理**:
- TTS 引擎不可用：降级为文本提示，记录警告日志
- 网络不可用（云端 TTS）：切换到本地 TTS 或暂停播报
- 播报队列满：丢弃最旧的弹幕，优先播报新弹幕
- 音频设备异常：暂停播报，提示用户检查设备

**验收标准**:
- 播报延迟 < 2 秒（从弹幕收到到语音输出）
- 播报准确率 > 95%（正确朗读弹幕内容）
- CPU 占用增加 < 5%
- 内存占用增加 < 50MB
- 支持同时处理 100+ 条弹幕/分钟的播报队列
- 播报音量可独立调节

#### 3.5.7 数据模型

```sql
-- 播报配置表
CREATE TABLE tts_config (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    enabled BOOLEAN DEFAULT FALSE,
    engine TEXT DEFAULT 'local',  -- 'local', 'google', 'azure', 'baidu'
    voice TEXT DEFAULT 'default',  -- 语音名称
    speed REAL DEFAULT 1.0,  -- 语速 (0.5-2.0)
    pitch REAL DEFAULT 1.0,  -- 音调 (0.5-2.0)
    volume REAL DEFAULT 0.8,  -- 音量 (0-1)
    min_interval INTEGER DEFAULT 2000,  -- 最小播报间隔（毫秒）
    format_template TEXT DEFAULT '{username}说：{content}',
    filter_keywords TEXT,  -- 过滤关键词（JSON 数组）
    broadcast_gifts BOOLEAN DEFAULT TRUE,
    gift_value_threshold INTEGER DEFAULT 0,  -- 礼物价值阈值
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 播报日志表
CREATE TABLE tts_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    message_type TEXT NOT NULL,  -- 'danmaku', 'gift', 'captain'
    username TEXT,
    content TEXT,
    broadcast_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

#### 3.5.8 API 设计

```typescript
// TTS 配置接口
interface TTSConfig {
  enabled: boolean;
  engine: 'local' | 'google' | 'azure' | 'baidu';
  voice: string;
  speed: number;  // 0.5-2.0
  pitch: number;  // 0.5-2.0
  volume: number;  // 0-1
  minInterval: number;  // 毫秒
  formatTemplate: string;
  filterKeywords: string[];
  broadcastGifts: boolean;
  giftValueThreshold: number;
}

// 开始播报
function startBroadcast(): Promise<void>;

// 停止播报
function stopBroadcast(): Promise<void>;

// 暂停播报
function pauseBroadcast(): Promise<void>;

// 恢复播报
function resumeBroadcast(): Promise<void>;

// 更新配置
function updateTTSConfig(config: Partial<TTSConfig>): Promise<void>;

// 获取配置
function getTTSConfig(): Promise<TTSConfig>;

// 获取播报日志
function getTTSLogs(options?: QueryOptions): Promise<TTSLog[]>;

// 手动播报文本
function speak(text: string): Promise<void>;
```

#### 3.5.9 技术实现建议

- **本地 TTS**: 使用 Web Speech API (SpeechSynthesis) 或系统原生 TTS API。
- **云端 TTS**: 使用 HTTP REST API 调用云端服务，支持流式返回。
- **音频队列**: 使用 AudioContext API 实现音频队列管理和播放。
- **弹幕缓冲**: 使用环形缓冲区处理弹幕高峰期，避免丢失。
- **语音缓存**: 缓存常用语音片段（如"说"、"感谢"），减少 TTS 调用。
- **异步处理**: 弹幕处理和 TTS 调用使用异步队列，不阻塞主线程。

---

## 3.6 非功能性需求 (Non-Functional Requirements)

### 3.6.1 性能要求

| 指标 | 要求 | 测量方法 |
|------|------|----------|
| 弹幕接收延迟 | < 100ms | 从 Bilibili 发送弹幕到本地接收的时间差 |
| 弹幕发送延迟 | < 500ms | 从触发发送到弹幕显示在直播间的时间差 |
| 特效渲染帧率 | ≥ 30 FPS | Canvas 动画帧率监控 |
| AI 语音识别延迟 | < 3s | 从说话结束到转写文本返回的时间 |
| AI 生成延迟 | < 5s | 从收到弹幕到生成回复的时间 |
| 内存占用 | < 500MB (空闲) / < 1GB (运行) | 系统资源监控 |
| CPU 占用 | < 10% (空闲) / < 30% (运行) / < 45% (录制中) | 系统资源监控 |
| 冷启动时间 | < 5s | 从启动到可用状态的时间 |
| 连接恢复时间 | < 10s | 网络断开后自动重连的时间 |
| 录制启动延迟 | < 3s | 从点击录制到开始录制的时间 |
| 录制同步误差 | < 1s | 录制文件与直播流的时间差 |
| TTS 播报延迟 | < 2s | 从弹幕收到到语音输出的时间 |
| TTS CPU 占用 | < 5% | 播报功能开启时的额外 CPU 占用 |

### 3.6.2 可靠性要求

| 指标 | 要求 | 说明 |
|------|------|------|
| 可用性 | ≥ 99.5% | 系统正常运行时间占比 |
| MTBF (平均故障间隔) | ≥ 72 小时 | 两次故障之间的平均时间 |
| MTTR (平均恢复时间) | ≤ 5 分钟 | 故障恢复的平均时间 |
| 数据丢失率 | 0% | 配置和日志数据不丢失 |
| 异常处理率 | ≥ 95% | 捕获并处理异常的比例 |

### 3.6.3 安全性要求

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
| 录制内容保护 | 录制文件存储在用户指定目录，不上传到云端 |

### 3.6.4 兼容性要求

| 类别 | 要求 |
|------|------|
| 操作系统 | Windows 10+、macOS 10.15+、Ubuntu 20.04+ |
| Node.js 版本 | ≥ 18.0.0 |
| 内存要求 | ≥ 4GB RAM |
| 磁盘空间 | ≥ 500MB 可用空间（基础）/ ≥ 10GB（录制功能） |
| 网络要求 | 稳定的互联网连接（≥ 10 Mbps） |
| Bilibili 账号 | 支持实名认证账号 |

### 3.6.5 可扩展性要求

| 需求 | 描述 |
|------|------|
| 插件系统 | ✅ 支持第三方插件扩展功能 |
| 多语言 | 预留国际化接口，支持多语言 |
| 配置导入导出 | 支持 JSON/YAML 格式的配置导入导出 |
| 模块化架构 | 核心功能模块化，便于单独升级 |
| API 扩展 | 支持自定义 AI 模型接入（自定义 Endpoint） |
| 录制扩展 | 支持自定义录制格式和编码器 |

### 3.6.6 可维护性要求

| 需求 | 描述 |
|------|------|
| 日志系统 | 结构化日志，支持日志级别（DEBUG/INFO/WARN/ERROR） |
| 错误追踪 | 自动收集错误堆栈和上下文信息 |
| 诊断模式 | 提供诊断工具帮助排查问题 |
| 更新机制 | ✅ 支持自动检测和安装更新 |
| 文档完善 | 提供完整的用户文档和开发者文档 |
| 录制诊断 | 提供录制状态监控和故障诊断工具 |

### 3.6.7 可用性要求

| 需求 | 描述 |
|------|------|
| 学习曲线 | 新用户能在 15 分钟内完成基础配置 |
| 错误提示 | 错误信息清晰易懂，提供解决方案 |
| 快捷键支持 | 常用操作支持快捷键（包括录制控制） |
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
('ai.model', 'gpt-4', 'string', 'AI 模型选择'),
('recording.enabled', 'false', 'boolean', '录制功能开关'),
('recording.format', 'mp4', 'string', '录制格式'),
('recording.quality', 'original', 'string', '录制质量'),
('recording.outputPath', '', 'string', '录制输出路径'),
('recording.autoStart', 'false', 'boolean', '自动开始录制'),
('recording.highlightEnabled', 'true', 'boolean', '高光标记功能开关');
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

#### 4.1.8 录制记录表 (recordings)

```sql
CREATE TABLE recordings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    room_id TEXT NOT NULL,
    room_name TEXT,
    file_path TEXT NOT NULL,
    file_name TEXT NOT NULL,
    file_size INTEGER DEFAULT 0,
    duration INTEGER DEFAULT 0,  -- 秒
    format TEXT DEFAULT 'mp4',   -- 'mp4', 'flv'
    quality TEXT DEFAULT 'original',  -- 'original', '720p', '1080p'
    status TEXT DEFAULT 'recording',  -- 'recording', 'paused', 'completed', 'failed'
    is_highlight BOOLEAN DEFAULT FALSE,
    highlight_reason TEXT,  -- 'auto', 'manual', 'gift', 'captain'
    started_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    ended_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 索引
CREATE INDEX idx_recordings_room_id ON recordings(room_id);
CREATE INDEX idx_recordings_started_at ON recordings(started_at);
CREATE INDEX idx_recordings_status ON recordings(status);
```

#### 4.1.9 高光时刻表 (highlights)

```sql
CREATE TABLE highlights (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    recording_id INTEGER,
    room_id TEXT NOT NULL,
    timestamp INTEGER NOT NULL,  -- 相对于录制开始的秒数
    reason TEXT NOT NULL,  -- 'danmaku_surge', 'high_value_gift', 'captain', 'manual'
    description TEXT,
    clip_path TEXT,  -- 高光片段文件路径
    clip_duration INTEGER DEFAULT 30,  -- 片段时长（秒）
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (recording_id) REFERENCES recordings(id)
);

-- 索引
CREATE INDEX idx_highlights_recording_id ON highlights(recording_id);
CREATE INDEX idx_highlights_room_id ON highlights(room_id);
CREATE INDEX idx_highlights_created_at ON highlights(created_at);
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

#### 4.2.5 录制配置对象 (RecordingConfig)

```typescript
interface RecordingConfig {
  enabled: boolean;
  autoStart: boolean;
  format: 'mp4' | 'flv';
  quality: 'original' | '720p' | '1080p';
  segmentSize: number;  // MB，0 表示不分段
  segmentDuration: number;  // 秒，0 表示不分段
  outputPath: string;
  maxStorageSize: number;  // GB，0 表示不限制
  autoCleanup: boolean;
  cleanupDays: number;  // 自动清理天数
  highlightEnabled: boolean;
  highlightConfig: {
    danmakuThreshold: number;  // 弹幕数量阈值
    giftValueThreshold: number;  // 礼物价值阈值
    captainTrigger: boolean;  // 舰长触发
    clipDuration: number;  // 高光片段时长（秒）
    beforeSeconds: number;  // 高光前录制秒数
    afterSeconds: number;  // 高光后录制秒数
  };
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

#### 4.2.6 录制配置对象 (RecordingConfig)

```typescript
interface RecordingConfig {
  enabled: boolean;
  autoStart: boolean;
  format: 'mp4' | 'flv';
  quality: 'original' | '720p' | '1080p';
  segmentSize: number;  // MB，0 表示不分段
  segmentDuration: number;  // 秒，0 表示不分段
  outputPath: string;
  maxStorageSize: number;  // GB，0 表示不限制
  autoCleanup: boolean;
  cleanupDays: number;  // 自动清理天数
  highlightEnabled: boolean;
  highlightConfig: {
    danmakuThreshold: number;  // 弹幕数量阈值
    giftValueThreshold: number;  // 礼物价值阈值
    captainTrigger: boolean;  // 舰长触发
    clipDuration: number;  // 高光片段时长（秒）
    beforeSeconds: number;  // 高光前录制秒数
    afterSeconds: number;  // 高光后录制秒数
  };
}
```

#### 4.2.7 录制会话对象 (RecordingSession)

```typescript
interface RecordingSession {
  id: string;
  roomId: string;
  roomName: string;
  status: 'recording' | 'paused' | 'stopped' | 'completed' | 'failed';
  filePath: string;
  fileName: string;
  fileSize: number;
  duration: number;
  format: string;
  quality: string;
  startedAt: Date;
  endedAt?: Date;
  highlightCount: number;
}
```

#### 4.2.8 高光时刻对象 (Highlight)

```typescript
interface Highlight {
  id: string;
  recordingId: string;
  roomId: string;
  timestamp: number;  // 相对于录制开始的秒数
  reason: 'danmaku_surge' | 'high_value_gift' | 'captain' | 'manual';
  description: string;
  clipPath?: string;
  clipDuration: number;
  createdAt: Date;
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

### 5.4 录制 API

#### 5.4.1 录制控制

```typescript
// 开始录制
function startRecording(options: RecordingOptions): Promise<RecordingSession>;

// 暂停录制
function pauseRecording(sessionId: string): Promise<void>;

// 恢复录制
function resumeRecording(sessionId: string): Promise<void>;

// 停止录制
function stopRecording(sessionId: string): Promise<RecordingResult>;

// 获取当前录制状态
function getRecordingStatus(sessionId: string): Promise<RecordingStatus>;

// 获取所有录制会话
function getRecordingSessions(): Promise<RecordingSession[]>;
```

#### 5.4.2 高光管理

```typescript
// 标记高光时刻
function markHighlight(sessionId: string, reason: string, description?: string): Promise<Highlight>;

// 获取高光列表
function getHighlights(recordingId?: string): Promise<Highlight[]>;

// 生成高光片段
function generateHighlightClip(highlightId: string): Promise<string>;

// 删除高光
function deleteHighlight(highlightId: string): Promise<void>;
```

#### 5.4.3 录制文件管理

```typescript
// 获取录制列表
function getRecordings(options?: QueryOptions): Promise<Recording[]>;

// 获取录制详情
function getRecording(recordingId: string): Promise<Recording>;

// 删除录制
function deleteRecording(recordingId: string): Promise<void>;

// 打开录制文件所在目录
function openRecordingFolder(recordingId: string): Promise<void>;

// 获取录制文件路径
function getRecordingPath(recordingId: string): Promise<string>;
```

#### 5.4.4 数据结构

```typescript
// 录制选项
interface RecordingOptions {
  roomId: string;
  format: 'mp4' | 'flv';
  quality: 'original' | '720p' | '1080p';
  segmentSize?: number;  // MB
  segmentDuration?: number;  // 秒
  outputPath: string;
  autoStart: boolean;
  highlightEnabled: boolean;
  highlightConfig?: {
    danmakuThreshold: number;
    giftValueThreshold: number;
    captainTrigger: boolean;
    clipDuration: number;
    beforeSeconds: number;
    afterSeconds: number;
  };
}

// 录制会话
interface RecordingSession {
  id: string;
  roomId: string;
  roomName: string;
  status: 'recording' | 'paused' | 'stopped' | 'completed' | 'failed';
  filePath: string;
  fileName: string;
  fileSize: number;
  duration: number;
  format: string;
  quality: string;
  startedAt: Date;
  endedAt?: Date;
  highlightCount: number;
}

// 录制结果
interface RecordingResult {
  sessionId: string;
  filePath: string;
  fileSize: number;
  duration: number;
  highlightCount: number;
  highlights: Highlight[];
}

// 录制状态
interface RecordingStatus {
  sessionId: string;
  status: 'recording' | 'paused' | 'stopped' | 'completed' | 'failed';
  currentSize: number;
  currentDuration: number;
  highlightCount: number;
  lastHighlightAt?: Date;
}

// 录制查询选项
interface QueryOptions {
  roomId?: string;
  status?: string;
  startDate?: Date;
  endDate?: Date;
  limit?: number;
  offset?: number;
  sortBy?: string;
  sortOrder?: 'asc' | 'desc';
}
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
| 录制模块 | 开始/暂停/停止录制、分段、高光标记 | ≥ 85% |

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

4. **录制流程**
   - 模拟直播流输入
   - 验证录制开始/暂停/停止
   - 验证文件生成和格式
   - 验证分段录制
   - 验证高光标记和片段生成
   - 验证录制状态同步

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

5. **直播录制**
   - 连接直播间
   - 开始录制
   - 验证录制文件生成
   - 暂停/恢复录制
   - 停止录制
   - 验证录制文件完整性
   - 标记高光时刻
   - 验证高光片段生成

### 6.5 性能测试

**测试工具**: k6 + Lighthouse

**测试指标**:

| 指标 | 目标值 | 测试方法 |
|------|--------|----------|
| 冷启动时间 | < 5s | 测量从双击图标到可用状态的时间 |
| 内存占用 (空闲) | < 500MB | 运行 1 小时后测量 |
| 内存占用 (运行) | < 1GB | 模拟 1000 条弹幕/分钟后测量 |
| 内存占用 (录制中) | < 1.2GB | 录制直播流时测量 |
| CPU 占用 (空闲) | < 10% | 系统监控 10 分钟 |
| CPU 占用 (运行) | < 30% | 模拟 1000 条弹幕/分钟后监控 |
| CPU 占用 (录制中) | < 45% | 录制直播流时监控 |
| 弹幕处理吞吐量 | ≥ 500 条/秒 | 压力测试 |
| 特效渲染帧率 | ≥ 30 FPS | 渲染 10 个并发特效时测量 |
| 录制启动延迟 | < 3s | 测量从点击录制到开始录制的时间 |
| 录制文件写入速度 | ≥ 10MB/s | 测量录制文件写入性能 |
| 高光标记响应时间 | < 5s | 测量从触发到标记完成的时间 |

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
- [ ] 能够开始/暂停/停止录制
- [ ] 录制文件可正常播放
- [ ] 高光标记功能正常
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
- ✅ 直播录制和高光剪辑功能

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
- ✅ 直播录制和高光剪辑

**Bilibili 直播姬的优势**:
- ✅ 官方支持，不会封号
- ✅ 功能全面（推流、录制、互动）
- ✅ 稳定可靠，持续更新

### 7.3 竞争策略

1. **差异化定位**: 聚焦 AI 智能互动、自动化和直播录制，避开与官方工具的直接竞争
2. **开源策略**: MVP 版本开源免费，积累用户和社区
3. **增值服务**: 高级功能（如本地 LLM、插件扩展、高级录制功能）采用付费模式
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
| 录制功能使用率 | ≥ 40% | 录制会话统计 |

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
| 录制文件完整性 | 100% | 录制文件校验 |
| 录制成功率 | ≥ 99% | 录制会话统计 |

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
- **录制引擎**: FFmpeg (直播流录制和转码)
- **录制封装**: fluent-ffmpeg (Node.js FFmpeg 封装库)

### 9.2 系统架构图
```
┌─────────────────────────────────────────┐
│           Electron 主进程                  │
│  - 窗口管理  - 本地存储  - 系统托盘         │
│  - 录制引擎 (FFmpeg)                      │
└─────────────┬───────────────────────────┘
              │
┌─────────────┼───────────────────────────┐
│            渲染进程                      │
│  ┌──────────┐  ┌──────────┐  ┌────────┐│
│  │ 弹幕管理  │  │ 特效展示  │  │ AI模块  ││
│  └──────────┘  └──────────┘  └────────┘│
│  ┌──────────┐  ┌──────────┐           │
│  │ 录制管理  │  │ 高光标记  │           │
│  └──────────┘  └──────────┘           │
└──────┬─────────────────┬───────────────┘
       │                 │
       │ WebSocket       │ HTTP/REST
       │                 │
┌──────▼─────────┐  ┌────▼────────────┐
│ Bilibili 直播间 │  │ 第三方 AI 服务   │
│  (弹幕/礼物流)   │  │  (ASR/LLM)     │
│  (直播流录制)    │  │                │
└─────────────────┘  └─────────────────┘
```

### 9.3 数据流设计
1. **弹幕流**: Bilibili WebSocket -> 本地解析 -> 规则匹配 -> 弹幕池/特效触发
2. **语音流**: 麦克风输入 -> 音频缓冲 -> ASR API -> 文本 -> LLM API -> 弹幕输出
3. **配置流**: 用户界面操作 -> 本地 JSON/SQLite 存储 -> 运行时加载
4. **录制流**: 直播流 URL -> FFmpeg 录制 -> 分段/转码 -> 本地存储 -> 高光标记
5. **高光流**: 弹幕/礼物事件 -> 热度分析 -> 高光标记 -> 片段提取 -> 独立保存

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
│ │数据看板  │ │  │  录制状态: ● 录制中 01:23:45│ │
│ │直播录制  │ │  │  文件大小: 1.2GB           │ │
│ │录制管理  │ │  │  高光标记: 3 个            │ │
│ │插件管理  │ │  │  ───────────────────────  │ │
│ │设置      │ │  │  [OBS 源地址预览]          │ │
│ └─────────┘ │  │  (用于特效预览)            │ │
│             │  └───────────────────────────┘ │
│             │  ┌───────────────────────────┐ │
│             │  │  实时日志监控              │ │
│             │  │  > 收到礼物 x1             │ │
│             │  │  > AI 回复: 感谢大佬!      │ │
│             │  │  > 高光时刻已标记 (弹幕刷屏)│ │
│             │  └───────────────────────────┘ │
└─────────────┴─────────────────────────────────┘
```

### 10.2 配置页面设计
- **弹幕配置**: 采用列表视图，支持拖拽排序弹幕池。
- **特效配置**: 提供实时预览窗口，边调整参数边看效果。
- **AI 配置**: 分步引导设置 (API Key -> 人设选择 -> 启动测试)。
- **录制配置**: 提供录制格式、质量、存储路径选择，支持录制测试和磁盘空间检查。
  - **基本设置**: 录制格式 (MP4/FLV)、质量 (原始/720p/1080p)、存储路径
  - **分段设置**: 分段大小 (MB)、分段时长 (秒)、是否启用分段
  - **存储管理**: 最大存储空间 (GB)、自动清理天数、当前使用空间
  - **高光设置**: 启用高光标记、弹幕阈值、礼物价值阈值、舰长触发、片段时长
  - **快捷键**: 开始/暂停/停止录制、标记高光
- **录制管理**: 展示录制文件列表，支持播放、删除、打开目录、导出操作。
  - **文件列表**: 按时间倒序展示，显示文件名、时长、大小、录制时间、状态
  - **操作按钮**: 播放、删除、打开目录、导出为 MP4
  - **高光列表**: 展示高光时刻，支持生成片段、删除操作
  - **存储统计**: 显示总存储空间、已使用空间、剩余空间

---

## 11. 风险与挑战

### 11.1 技术风险
- **WebSocket 连接稳定性**: Bilibili 直播协议变更可能导致连接中断，需设计重连机制。
- **AI 延迟**: 语音识别和 LLM 生成存在延迟，可能导致互动不及时，需优化流式传输。
- **资源占用**: 特效渲染、语音处理和直播录制对 CPU/GPU 占用较高，需提供性能优化选项。
- **录制稳定性**: 直播流中断或网络波动可能导致录制卡顿或文件损坏，需实现缓冲和断点续录机制。
- **存储空间**: 长时间录制占用大量磁盘空间，需实现自动清理和存储管理策略。

### 11.2 合规风险
- **风控触发**: 自动弹幕频率过高可能导致账号被平台限流或封禁，需严格设置发送上限。
- **隐私保护**: 语音数据传输需加密，且不得私自保存用户语音。
- **录制合规**: 录制直播内容需遵守平台规定，不得录制他人隐私内容。
- **版权风险**: 录制内容可能涉及版权问题，需提示用户遵守相关法律法规。

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
- ✅ 语音识别接入 (Whisper)
- ✅ AI 对话生成 (LLM)
- ✅ 直播录制（全程/高光，含高光标记与片段提取）
- ✅ 直播数据看板

### 阶段四：优化与生态 - 持续迭代
- ❌ 性能优化 (降低 CPU 占用)
- ✅ 插件系统 (允许用户扩展)
- ✅ 多语言支持
---

## 13. 附录

### 13.1 参考资料
- [Bilibili Live Protocol (GitHub)](https://github.com/lovelyyoshino/Bilibili-Live-API)
- [Electron 官方文档](https://www.electronjs.org/docs)
- [OpenAI Whisper API](https://platform.openai.com/docs/guides/speech-to-text)
- [Vue 3 文档](https://vuejs.org/)
- [TypeScript 文档](https://www.typescriptlang.org/)
- [FFmpeg 官方文档](https://ffmpeg.org/documentation.html)
- [fluent-ffmpeg 文档](https://github.com/fluent-ffmpeg/node-fluent-ffmpeg)

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
- **FFmpeg**: 开源音视频处理工具，用于直播流录制和转码。
- **高光时刻**: 直播中特别精彩或值得关注的瞬间。
- **分段录制**: 将长时间录制分割成多个小文件的功能。
