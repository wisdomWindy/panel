# 排行榜功能规格说明

## 1. 功能概述

在现有直播数据看板（Dashboard）中新增排行榜区域，展示本场直播的送礼榜、弹幕榜和舰长贡献榜。

## 2. 功能范围

### 2.1 包含功能
- 送礼榜：展示本场直播送礼价值最高的用户列表
- 弹幕榜：展示本场直播发送弹幕数量最多的用户列表
- 舰长贡献榜：展示本场直播开通/续费舰长的用户列表
- Tab 切换：支持在三个排行榜之间切换
- 实时更新：排行榜数据每 5 秒自动更新
- 空状态：无数据时显示空状态提示

### 2.2 不包含功能
- 历史排行榜（跨直播场次）
- 排行榜导出功能
- 排行榜分享功能

## 3. 用户流程

1. 用户打开直播数据看板（Dashboard）
2. 滚动到排行榜区域
3. 默认显示送礼榜
4. 点击 Tab 切换到弹幕榜或舰长贡献榜
5. 排行榜数据自动更新

## 4. 模块设计

### 4.1 数据模块

#### 4.1.1 送礼榜数据
```typescript
interface GiftRankItem {
  rank: number;
  userId: string;
  username: string;
  totalValue: number; // 送礼总价值（电池）
}
```

**数据来源**：从 `danmakuStore.giftList` 聚合计算
**排序规则**：按 `totalValue` 降序
**显示数量**：前 20 名

#### 4.1.2 弹幕榜数据
```typescript
interface DanmakuRankItem {
  rank: number;
  userId: string;
  username: string;
  count: number; // 弹幕数量
}
```

**数据来源**：从 `danmakuStore.danmakuList` 聚合计算
**排序规则**：按 `count` 降序
**显示数量**：前 20 名

#### 4.1.3 舰长贡献榜数据
```typescript
interface CaptainRankItem {
  rank: number;
  userId: string;
  username: string;
  level: number; // 1=督察, 2=提督, 3=总督
  levelName: string; // 舰长等级名称
  timestamp: number; // 贡献时间
}
```

**数据来源**：从 `danmakuStore.captainList` 获取
**排序规则**：按 `level` 降序，同等级按 `timestamp` 升序
**显示数量**：全部显示

### 4.2 UI 组件

#### 4.2.1 排行榜容器
- 使用 `el-card` 组件作为容器
- 标题："排行榜"
- 内容区域使用 `el-tabs` 实现 Tab 切换

#### 4.2.2 Tab 组件
- 使用 `el-tabs` 和 `el-tab-pane`
- 三个 Tab：送礼榜、弹幕榜、舰长贡献榜
- 默认选中送礼榜

#### 4.2.3 排行榜列表
- 使用 `el-table` 组件展示排行榜数据
- 列配置：
  - 排名列：宽度 80px，居中显示
  - 用户名列：自适应宽度，左对齐
  - 数值列：宽度 150px，右对齐

#### 4.2.4 排名样式
- 第一名：🥇 金色背景
- 第二名：🥈 银色背景
- 第三名：🥉 铜色背景
- 其他：普通数字

#### 4.2.5 空状态
- 使用 `el-empty` 组件
- 提示文字："暂无排行榜数据"

## 5. API 和数据契约

### 5.1 内部数据接口

#### 5.1.1 获取送礼榜
```typescript
function getGiftRank(limit?: number): GiftRankItem[];
```

#### 5.1.2 获取弹幕榜
```typescript
function getDanmakuRank(limit?: number): DanmakuRankItem[];
```

#### 5.1.3 获取舰长贡献榜
```typescript
function getCaptainRank(): CaptainRankItem[];
```

### 5.2 数据更新机制

- 使用 `setInterval` 每 5 秒更新一次
- 组件卸载时清除定时器
- 使用 `computed` 属性自动计算排行榜数据

## 6. 边界情况

### 6.1 数据为空
- 无礼物数据：显示空状态
- 无弹幕数据：显示空状态
- 无舰长数据：显示空状态

### 6.2 数据量大
- 送礼榜和弹幕榜限制显示前 20 名
- 舰长贡献榜显示全部（通常数量较少）

### 6.3 数据更新
- 数据更新时无闪烁，平滑过渡
- 使用 Vue 的响应式机制自动更新

## 7. 验收标准

### 7.1 功能验收
- [ ] 排行榜区域正确显示在 Dashboard 页面
- [ ] 三个 Tab 可以正常切换
- [ ] 送礼榜显示前 20 名用户，按送礼价值降序
- [ ] 弹幕榜显示前 20 名用户，按弹幕数量降序
- [ ] 舰长贡献榜显示所有舰长，按等级降序
- [ ] 前三名使用特殊样式（金、银、铜）
- [ ] 无数据时显示空状态
- [ ] 数据每 5 秒自动更新

### 7.2 性能验收
- [ ] 排行榜渲染时间 < 100ms
- [ ] 数据更新无闪烁
- [ ] 内存占用增加 < 10MB

### 7.3 样式验收
- [ ] 使用 Tailwind CSS 实现所有样式
- [ ] 不使用 `<style>` 标签
- [ ] 不使用行内样式
- [ ] 遵循现有 Dashboard 的设计风格

## 8. 风险和依赖

### 8.1 风险
1. **数据量大**：如果直播时间很长，数据可能很多
   - 解决方案：限制显示数量，使用虚拟滚动（如果需要）

2. **性能问题**：排行榜数据更新频繁
   - 解决方案：使用防抖优化，避免频繁渲染

3. **舰长等级显示**：需要正确映射舰长等级名称
   - 解决方案：1=督察、2=提督、3=总督

### 8.2 依赖
- 依赖 `danmakuStore` 提供数据
- 依赖 Element Plus 组件库
- 依赖 Tailwind CSS 样式框架
