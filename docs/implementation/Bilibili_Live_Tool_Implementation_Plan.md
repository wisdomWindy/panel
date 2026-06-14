# Bilibili 直播助手 - 实施计划

**创建日期**: 2026-06-03
**基于文档**: PRD v1.0, Architecture v1.0
**目标**: 完成 MVP 阶段一功能 (4 周)

---

## Spec 验证 ✅

### Goals (目标)
打造一款集自动化互动、智能回复、特效展示于一体的 Bilibili 直播辅助工具

### Scope (范围) - MVP 阶段一
- Bilibili 直播间连接与弹幕接收
- 基础自动弹幕发送 (定时/随机)
- 简单礼物感谢弹幕
- 基础 UI 界面

### Non-goals (不包含)
- 语音识别 (ASR)
- AI 对话生成 (LLM)
- 上舰特效展示
- 高光录制
- 数据看板

### Risks (风险)
1. **技术风险**: WebSocket 连接稳定性、Bilibili 协议变更
2. **合规风险**: 自动弹幕风控、隐私保护
3. **资源风险**: CPU/GPU 占用过高

### Verification (验证)
- 单元测试覆盖率 ≥ 80%
- 集成测试覆盖核心流程
- E2E 测试覆盖关键用户场景
- 性能测试达标

---

## 实施计划

### 阶段一：基础设施 (当前周)

#### 任务 1.1: Bilibili 客户端
**文件**: `src/renderer/modules/bilibili/client.ts`
**工时**: 4 小时
**依赖**: 无
**验收标准**:
- ✅ 支持连接直播间 (WebSocket)
- ✅ 实现心跳保活机制
- ✅ 支持断线自动重连
- ✅ 支持接收原始消息

**实现步骤**:
1. 实现 WebSocket 连接逻辑
2. 实现心跳机制 (每 30 秒发送一次)
3. 实现断线重连 (指数退避)
4. 实现消息接收和解析

#### 任务 1.2: Bilibili 协议解析
**文件**: `src/renderer/modules/bilibili/protocol.ts`
**工时**: 3 小时
**依赖**: 任务 1.1
**验收标准**:
- ✅ 解析弹幕消息
- ✅ 解析礼物消息
- ✅ 解析在线人数消息
- ✅ 支持版本兼容

**实现步骤**:
1. 实现协议头解析
2. 实现消息体解码
3. 实现不同消息类型分发

#### 任务 1.3: Bilibili 类型定义
**文件**: `src/renderer/modules/bilibili/types.ts`
**工时**: 1 小时
**依赖**: 无
**验收标准**:
- ✅ 定义所有协议相关类型
- ✅ 完整的 TypeScript 类型

#### 任务 1.4: 弹幕发送器
**文件**: `src/renderer/modules/danmaku/sender.ts`
**工时**: 3 小时
**依赖**: 无
**验收标准**:
- ✅ 实现发送弹幕 API
- ✅ 实现频率限制 (最小 3 秒)
- ✅ 实现错误处理
- ✅ 支持风控检测

**实现步骤**:
1. 实现发送 API 调用
2. 实现频率限制器
3. 实现重试机制
4. 实现错误处理

#### 任务 1.5: 弹幕解析器
**文件**: `src/renderer/modules/danmaku/parser.ts`
**工时**: 2 小时
**依赖**: 任务 1.2
**验收标准**:
- ✅ 解析弹幕对象
- ✅ 提取用户信息
- ✅ 识别用户身份

**实现步骤**:
1. 实现弹幕数据解析
2. 实现用户信息提取
3. 实现身份识别逻辑

#### 任务 1.6: 变量替换工具
**文件**: `src/renderer/modules/utils/variable-replacer.ts`
**工时**: 1 小时
**依赖**: 无
**验收标准**:
- ✅ 支持基础变量替换
- ✅ 支持动态变量
- ✅ 错误处理

#### 任务 1.7: 敏感词过滤
**文件**: `src/renderer/modules/utils/sensitive-words.ts`
**工时**: 2 小时
**依赖**: 无
**验收标准**:
- ✅ 实现敏感词检测
- ✅ 支持自定义敏感词列表
- ✅ 性能优化 (Trie 树)

---

### 阶段二：弹幕管理 (第 2 周)

#### 任务 2.1: 完善 DanmakuManager
**文件**: `src/renderer/modules/danmaku/manager.ts`
**工时**: 4 小时
**依赖**: 阶段一所有任务
**验收标准**:
- ✅ 集成 Bilibili 客户端
- ✅ 实现定时发送逻辑
- ✅ 实现随机发送逻辑
- ✅ 实现触发词回复
- ✅ 事件驱动架构

#### 任务 2.2: Pinia Store - 直播间
**文件**: `src/renderer/stores/room.store.ts`
**工时**: 2 小时
**依赖**: 任务 2.1
**验收标准**:
- ✅ 管理直播间状态
- ✅ 管理连接状态
- ✅ 管理在线人数

#### 任务 2.3: Pinia Store - 弹幕
**文件**: `src/renderer/stores/danmaku.store.ts`
**工时**: 2 小时
**依赖**: 任务 2.1
**验收标准**:
- ✅ 管理弹幕列表
- ✅ 管理弹幕池
- ✅ 管理发送状态

#### 任务 2.4: Pinia Store - 配置
**文件**: `src/renderer/stores/config.store.ts`
**工时**: 2 小时
**依赖**: 无
**验收标准**:
- ✅ 管理全局配置
- ✅ 持久化配置

---

### 阶段三：UI 开发 (第 3 周)

#### 任务 3.1: 弹幕配置页面
**文件**: `src/renderer/components/DanmakuConfig/index.vue`
**工时**: 6 小时
**依赖**: 任务 2.3, 2.4
**验收标准**:
- ✅ 弹幕池管理 (增删改查)
- ✅ 触发词配置
- ✅ 定时发送配置
- ✅ 表单验证

**子组件**:
- `src/renderer/components/DanmakuConfig/components/DanmakuPool.vue` (2h)
- `src/renderer/components/DanmakuConfig/components/TriggerConfig.vue` (2h)
- `src/renderer/components/DanmakuConfig/components/TimedConfig.vue` (2h)

#### 任务 3.2: 设置页面
**文件**: `src/renderer/components/Settings/index.vue`
**工时**: 4 小时
**依赖**: 任务 2.4
**验收标准**:
- ✅ 账号配置
- ✅ 通用设置
- ✅ 配置导入导出

#### 任务 3.3: 日志监控页面
**文件**: `src/renderer/components/LogViewer/index.vue`
**工时**: 4 小时
**依赖**: 无
**验收标准**:
- ✅ 实时日志显示
- ✅ 日志过滤
- ✅ 日志级别控制

#### 任务 3.4: 完善 Dashboard
**文件**: `src/renderer/App.vue`
**工时**: 2 小时
**依赖**: 任务 3.1, 3.2, 3.3
**验收标准**:
- ✅ 集成所有页面
- ✅ 路由管理
- ✅ 响应式布局

---

### 阶段四：测试与优化 (第 4 周)

#### 任务 4.1: 单元测试
**文件**: `tests/unit/`
**工时**: 8 小时
**依赖**: 所有功能代码
**验收标准**:
- ✅ 代码覆盖率 ≥ 80%
- ✅ 所有核心函数有测试
- ✅ 测试通过率 100%

#### 任务 4.2: 集成测试
**文件**: `tests/integration/`
**工时**: 6 小时
**依赖**: 任务 4.1
**验收标准**:
- ✅ 测试核心流程
- ✅ 测试错误处理
- ✅ 测试边界情况

#### 任务 4.3: E2E 测试
**文件**: `tests/e2e/`
**工时**: 6 小时
**依赖**: 任务 4.2
**验收标准**:
- ✅ 测试用户场景
- ✅ 测试关键路径
- ✅ 测试回归

#### 任务 4.4: 性能优化
**文件**: 多个文件
**工时**: 6 小时
**依赖**: 所有代码
**验收标准**:
- ✅ 内存占用 < 500MB (空闲)
- ✅ CPU 占用 < 10% (空闲)
- ✅ 弹幕接收延迟 < 100ms
- ✅ 弹幕发送延迟 < 500ms

#### 任务 4.5: 文档完善
**文件**: 多个文档
**工时**: 4 小时
**依赖**: 所有代码
**验收标准**:
- ✅ 用户文档
- ✅ 开发文档
- ✅ API 文档

---

## 工作包拆分

### 工作包 A: Bilibili 客户端
**负责人**: Subagent 1
**文件**: 
- `src/renderer/modules/bilibili/client.ts`
- `src/renderer/modules/bilibili/protocol.ts`
- `src/renderer/modules/bilibili/types.ts`

**写入规则**: 仅修改上述文件

### 工作包 B: 弹幕管理
**负责人**: Subagent 2
**文件**:
- `src/renderer/modules/danmaku/sender.ts`
- `src/renderer/modules/danmaku/parser.ts`
- `src/renderer/modules/danmaku/manager.ts`

**写入规则**: 仅修改上述文件

### 工作包 C: 工具模块
**负责人**: Subagent 3
**文件**:
- `src/renderer/modules/utils/variable-replacer.ts`
- `src/renderer/modules/utils/sensitive-words.ts`

**写入规则**: 仅修改上述文件

### 工作包 D: Pinia Store
**负责人**: Subagent 4
**文件**:
- `src/renderer/stores/room.store.ts`
- `src/renderer/stores/danmaku.store.ts`
- `src/renderer/stores/config.store.ts`

**写入规则**: 仅修改上述文件

### 工作包 E: UI 组件
**负责人**: Subagent 5
**文件**:
- `src/renderer/components/DanmakuConfig/`
- `src/renderer/components/Settings/`
- `src/renderer/components/LogViewer/`
- `src/renderer/App.vue`

**写入规则**: 仅修改上述文件和目录

### 工作包 F: 测试
**负责人**: Subagent 6
**文件**:
- `tests/unit/`
- `tests/integration/`
- `tests/e2e/`

**写入规则**: 仅修改上述目录

---

## 验证命令

### 构建验证
```bash
npm run build
```

### 测试验证
```bash
npm run test:unit
npm run test:integration
npm run test:e2e
```

### 代码质量验证
```bash
npm run lint
npm run type-check
```

### 性能验证
```bash
npm run test:performance
```

---

## 风险应对

### 风险 1: Bilibili 协议变更
**应对**: 
- 使用版本检测
- 提供降级方案
- 定期更新协议解析

### 风险 2: 依赖安装失败
**应对**:
- 使用镜像源
- 提供离线安装包
- 文档说明替代方案

### 风险 3: 性能不达标
**应对**:
- 提前性能测试
- 使用虚拟滚动
- 优化渲染逻辑

---

## 下一步行动

1. ✅ 立即开始工作包 A (Bilibili 客户端)
2. ✅ 并行启动工作包 B、C (弹幕管理、工具模块)
3. ✅ 完成后启动工作包 D、E (Store、UI)
4. ✅ 最后进行工作包 F (测试)

---

**实施计划完成 ✅**