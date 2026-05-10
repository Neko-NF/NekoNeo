# NekoNeo — 前端 UI/UX 设计规范

> 版本：v1.1 | 日期：2026-05-10
> 设计方向：**Win10/11 Fluent 风格 · 简洁 · 现代 · 中性色调**

---

## 一、设计理念

### 1.1 核心原则

| 原则 | 说明 |
|------|------|
| **内容优先** | 数据和状态是主角，装饰元素服务于内容，不喧宾夺主 |
| **克制** | 不堆叠视觉效果；每个元素存在均有目的 |
| **中性色调** | 深色/浅色均使用中性灰色系（Win11 Mica 风格），避免大面积蓝色调 |
| **密度适中** | 工具型软件需要在有限空间展示足够信息，但避免拥挤 |
| **状态清晰** | 运行中/停止/错误/警告 必须有视觉区分，颜色语义严格 |
| **一致性** | 同类操作的视觉呈现统一，减少用户认知负担 |
| **Fluent 风格** | 参考 Win10/11 Fluent Design System：扁平层次、微妙阴影、清晰边框 |

### 1.2 禁止的风格方向

- ❌ 玻璃拟态（`backdrop-filter: blur`）
- ❌ 强阴影、大光晕
- ❌ 渐变背景（大面积）
- ❌ 动画过度（旋转、弹跳等非功能性动效）
- ❌ 圆角过大（>12px 的卡片圆角）

---

## 二、设计 Token（CSS 变量）

所有样式值必须来自以下变量，**禁止硬编码任何颜色、间距或圆角值**。

### 2.1 颜色系统

```css
/* src/styles/tokens.css */

:root {
  /* ── 主题色（由用户配置 seedColor 动态注入） ──────────────── */
  --color-primary:       #60cdff;   /* 深色默认：天蓝 */
  --color-primary-hover: #85d8ff;
  --color-primary-muted: rgba(96, 205, 255, 0.12);

  /* ── 语义状态色（固定，不跟随主题色变化） ─────────────────── */
  --color-success:       #6ccb5f;
  --color-success-muted: rgba(108, 203, 95, 0.10);
  --color-warning:       #f0c45a;
  --color-warning-muted: rgba(240, 196, 90, 0.10);
  --color-danger:        #f0656c;
  --color-danger-muted:  rgba(240, 101, 108, 0.10);
  --color-info:          #60a5fa;
  --color-info-muted:    rgba(96, 165, 250, 0.10);

  /* ── 背景层（Win11 中性灰深色模式） ────────────────────────── */
  --bg-app:      #1e1e1e;   /* 最底层：应用背景 */
  --bg-surface:  #2b2b2b;   /* 卡片/面板 */
  --bg-elevated: #333333;   /* 悬浮层/下拉/Tooltip */
  --bg-input:    #252525;   /* 输入框背景 */
  --bg-hover:    rgba(255, 255, 255, 0.04);
  --bg-active:   rgba(255, 255, 255, 0.07);

  /* ── 文字 ──────────────────────────────────────────────────── */
  --text-primary:   #e8e8e8;   /* 主要文字 */
  --text-secondary: #a0a0a0;   /* 辅助文字、描述 */
  --text-tertiary:  #6e6e6e;   /* 占位符、禁用 */
  --text-on-primary: #1e1e1e;  /* 主题色按钮上的文字 */

  /* ── 边框 ──────────────────────────────────────────────────── */
  --border-default: rgba(255, 255, 255, 0.07);
  --border-strong:  rgba(255, 255, 255, 0.12);
  --border-focus:   var(--color-primary);

  /* ── 阴影层级（Win11 Fluent 风格） ─────────────────────────── */
  --elevation-1: 0 1px 2px rgba(0, 0, 0, 0.24);
  --elevation-2: 0 2px 8px rgba(0, 0, 0, 0.32);

  /* ── Dock 导航栏 ──────────────────────────────────────────── */
  --dock-bg:          #2b2b2b;
  --dock-border:      rgba(255, 255, 255, 0.08);
  --dock-shadow:      0 2px 12px rgba(0, 0, 0, 0.40);
  --dock-height:      56px;
  --dock-item-size:   52px;
  --dock-gap:         var(--space-1);
  --dock-padding:     var(--space-2);
}

/* ── 浅色模式覆盖 ────────────────────────────────────────────── */
[data-theme="light"] {
  --color-primary:       #0078d4;   /* Fluent 蓝 */
  --color-primary-hover: #106ebe;
  --color-primary-muted: rgba(0, 120, 212, 0.10);

  --bg-app:      #f3f3f3;
  --bg-surface:  #ffffff;
  --bg-elevated: #f9f9f9;
  --bg-input:    #ffffff;
  --bg-hover:    rgba(0, 0, 0, 0.03);
  --bg-active:   rgba(0, 0, 0, 0.06);

  --text-primary:   #1a1a1a;
  --text-secondary: #5c5c5c;
  --text-tertiary:  #999999;
  --text-on-primary: #ffffff;

  --border-default: rgba(0, 0, 0, 0.07);
  --border-strong:  rgba(0, 0, 0, 0.14);

  --dock-bg:     #ffffff;
  --dock-border: rgba(0, 0, 0, 0.08);
  --dock-shadow: 0 2px 12px rgba(0, 0, 0, 0.12);

  --elevation-1: 0 1px 3px rgba(0, 0, 0, 0.08);
  --elevation-2: 0 4px 12px rgba(0, 0, 0, 0.10);
}
```

### 2.2 间距系统（4px 基准网格）

```css
:root {
  --space-1:  4px;
  --space-2:  8px;
  --space-3:  12px;
  --space-4:  16px;
  --space-5:  20px;
  --space-6:  24px;
  --space-8:  32px;
  --space-10: 40px;
  --space-12: 48px;
}
```

### 2.3 圆角系统

```css
:root {
  --radius-sm:   4px;    /* 输入框、小元素 */
  --radius-md:   6px;    /* 行内块元素 */
  --radius-lg:   8px;    /* 卡片、面板 */
  --radius-xl:   12px;   /* 模态框（最大值） */
  --radius-full: 999px;  /* 徽标、药丸形标签 */
}
```

> 注：Win11 风格偏向硬朗边角，因此将默认卡片圆角从 8px 收紧为 6px，面板圆角从 12px 收紧为 8px。新增 `--radius-xl: 12px` 仅用于模态框等最大圆角场景。

### 2.4 字体系统

```css
:root {
  /* 字体栈（--ui-font 由用户设置动态注入） */
  --font-ui: var(--ui-font, "Segoe UI"), system-ui, -apple-system, sans-serif;
  --font-mono: "JetBrains Mono", "Cascadia Code", "Consolas", monospace;

  /* 字号 */
  --text-xs:   11px;
  --text-sm:   12px;
  --text-base: 13px;   /* 正文基准（工具软件偏小） */
  --text-md:   14px;
  --text-lg:   15px;
  --text-xl:   17px;
  --text-2xl:  20px;

  /* 字重 */
  --fw-regular:  400;
  --fw-medium:   500;
  --fw-semibold: 600;
  --fw-bold:     700;

  /* 行高 */
  --leading-tight:  1.3;
  --leading-normal: 1.5;
  --leading-loose:  1.7;
}
```

### 2.5 动画 Token

```css
:root {
  --duration-fast:   120ms;
  --duration-base:   200ms;
  --duration-slow:   350ms;
  --ease-standard:   cubic-bezier(0.4, 0, 0.2, 1);
  --ease-decelerate: cubic-bezier(0, 0, 0.2, 1);
  --ease-accelerate: cubic-bezier(0.4, 0, 1, 1);
}
```

---

## 三、布局规范

### 3.1 整体骨架

```
┌──────────────────────────────────────────────┐
│  顶部拖拽区（自定义无边框标题栏，高度 36px）   │
├──────────────────────────────────────────────┤
│                                              │
│              内容区（全屏）                   │
│          RouterView（可滚动）                 │
│                                              │
│                                              │
│  ┌────────────────────────────────────────┐  │
│  │     底部 Dock 浮动导航栏（悬浮）         │  │
│  └────────────────────────────────────────┘  │
└──────────────────────────────────────────────┘
```

- **内容区**：全屏铺满，无侧边栏，视野最大化
- **内容区内边距**：`var(--space-6)`（24px），底部额外留 `var(--dock-height) + var(--space-6)` 避免被 Dock 遮挡
- **Dock 位置**：`position: fixed`，吸附在内容区**底部中央**，不跨越标题栏
- **最小窗口尺寸**：800 × 560px（Tauri 配置）

### 3.2 Dashboard 布局（关键信息 + 快捷开关）

Dashboard **不展示图表**，只显示最关键的状态摘要与一组快捷功能开关：

```
┌──────────────────────────────────────────────┐
│  ┌─────────────────────────────────────────┐ │
│  │  状态摘要卡                              │ │
│  │  ● 在线                 2s 前上报        │ │
│  │  chrome.exe · 电量 85% ⚡                │ │
│  └─────────────────────────────────────────┘ │
│                                              │
│  ┌─────────────────────────────────────────┐ │
│  │  快捷开关区                              │ │
│  │  截图上报  ●──  │  隐身模式  ○──         │ │
│  │  接收通知  ●──  │  勿扰模式  ○──         │ │
│  └─────────────────────────────────────────┘ │
│                                              │
│  ┌─────────────────────────────────────────┐ │
│  │  系统简报                               │ │
│  │  CPU   12.4%  ████░░░░░░               │ │
│  │  内存  45.2%  ██████░░░░               │ │
│  │  延迟  42ms  ·  下行 1.2MB/s            │ │
│  └─────────────────────────────────────────┘ │
│                                              │
│      [ 仪 表 盘 ·  设 置 · 隐 私 · 日 志 · 关 于 ]      │
└──────────────────────────────────────────────┘
```

**Dashboard 三分区语义：**

| 分区 | 内容 | 备注 |
|------|------|------|
| **状态摘要** | 在线/离开状态 · 最后上报时间 · 当前前台应用 · 电池状态 | 实时刷新 |
| **快捷开关** | 截图上报 / 隐身模式 / 接收通知 / 勿扰模式（2×2 布局） | 直接写入配置 |
| **系统简报** | CPU% · 内存% · 网络延迟 · 下行速度（仅数字+细进度条） | 无图表 |

---

## 四、组件视觉规范

### 4.1 状态徽标（NBadge）

| 语义 | 颜色变量 | 使用场景 |
|------|---------|---------|
| `running` | `--color-primary` | 上报服务运行中 |
| `success` | `--color-success` | 上报成功 |
| `warning` | `--color-warning` | 连续失败/看门狗触发 |
| `danger`  | `--color-danger`  | 服务停止/密钥错误 |
| `neutral` | `--text-tertiary` | 未知/初始状态 |

```css
/* 徽标基础样式 */
.n-badge {
  display:       inline-flex;
  align-items:   center;
  gap:           var(--space-1);
  padding:       2px var(--space-2);
  border-radius: var(--radius-full);
  font-size:     var(--text-xs);
  font-weight:   var(--fw-medium);
  line-height:   var(--leading-tight);
}

.n-badge--running { background: var(--color-primary-muted); color: var(--color-primary); }
.n-badge--success { background: var(--color-success-muted); color: var(--color-success); }
.n-badge--warning { background: var(--color-warning-muted); color: var(--color-warning); }
.n-badge--danger  { background: var(--color-danger-muted);  color: var(--color-danger); }
```

### 4.2 按钮（NButton）

```css
/* 主要操作按钮 */
.n-btn {
  display:         inline-flex;
  align-items:     center;
  gap:             var(--space-2);
  padding:         var(--space-2) var(--space-4);
  border-radius:   var(--radius-sm);
  font-size:       var(--text-base);
  font-weight:     var(--fw-medium);
  border:          1px solid transparent;
  cursor:          pointer;
  transition:      background var(--duration-fast) var(--ease-standard),
                   border-color var(--duration-fast) var(--ease-standard);
  user-select:     none;
}

/* 变体 */
.n-btn--primary  { background: var(--color-primary); color: var(--text-on-primary); }
.n-btn--primary:hover { background: var(--color-primary-hover); }

.n-btn--secondary { background: var(--bg-elevated); color: var(--text-primary); border-color: var(--border-default); }
.n-btn--secondary:hover { background: var(--bg-active); }

.n-btn--danger { background: var(--color-danger-muted); color: var(--color-danger); border-color: var(--color-danger); }

.n-btn--ghost { background: transparent; color: var(--text-secondary); }
.n-btn--ghost:hover { background: var(--bg-hover); color: var(--text-primary); }

/* 尺寸 */
.n-btn--sm { padding: var(--space-1) var(--space-3); font-size: var(--text-sm); }
.n-btn--lg { padding: var(--space-3) var(--space-6); font-size: var(--text-md); }

/* 禁用 */
.n-btn:disabled { opacity: 0.4; cursor: not-allowed; }
```

### 4.3 开关（NSwitch）

唯一允许的开关实现方案（div-based，不用 checkbox）：

```vue
<!-- NSwitch.vue -->
<script setup lang="ts">
interface Props {
  modelValue: boolean
  disabled?: boolean
}
const props = withDefaults(defineProps<Props>(), { disabled: false })
const emit  = defineEmits<{ 'update:modelValue': [boolean] }>()
const toggle = () => { if (!props.disabled) emit('update:modelValue', !props.modelValue) }
</script>

<template>
  <div
    class="n-switch"
    :class="{ 'n-switch--on': modelValue, 'n-switch--disabled': disabled }"
    role="switch"
    :aria-checked="modelValue"
    :tabindex="disabled ? -1 : 0"
    @click="toggle"
    @keydown.space.prevent="toggle"
  />
</template>

<style scoped>
.n-switch {
  position:      relative;
  width:         36px;
  height:        20px;
  border-radius: var(--radius-full);
  background:    var(--border-strong);
  cursor:        pointer;
  transition:    background var(--duration-fast) var(--ease-standard);
  flex-shrink:   0;
}
.n-switch::after {
  content:       '';
  position:      absolute;
  top:           3px;
  left:          3px;
  width:         14px;
  height:        14px;
  border-radius: var(--radius-full);
  background:    #fff;
  transition:    transform var(--duration-fast) var(--ease-standard);
}
.n-switch--on { background: var(--color-primary); }
.n-switch--on::after { transform: translateX(16px); }
.n-switch--disabled { opacity: 0.4; cursor: not-allowed; }
</style>
```

### 4.4 设置行（SettingsRow）

```vue
<!-- 通用设置行布局 -->
<template>
  <div class="settings-row">
    <div class="settings-row__info">
      <span class="settings-row__label">{{ label }}</span>
      <span v-if="description" class="settings-row__desc">{{ description }}</span>
    </div>
    <slot />   <!-- 右侧控件（Switch / Select / Button 等） -->
  </div>
</template>

<style scoped>
.settings-row {
  display:         flex;
  align-items:     center;
  justify-content: space-between;
  padding:         var(--space-3) var(--space-4);
  border-bottom:   1px solid var(--border-default);
  min-height:      52px;
  gap:             var(--space-4);
}
.settings-row:last-child { border-bottom: none; }

.settings-row__info { flex: 1; min-width: 0; }

.settings-row__label {
  display:     block;
  font-size:   var(--text-base);
  font-weight: var(--fw-medium);
  color:       var(--text-primary);
}

.settings-row__desc {
  display:     block;
  margin-top:  2px;
  font-size:   var(--text-sm);
  font-weight: var(--fw-regular);
  color:       var(--text-secondary);
}
</style>
```

### 4.5 指标行（MetricRow）

```css
/* 单项系统指标展示行 */
.metric-row {
  display:     grid;
  grid-template-columns: 1fr auto;
  align-items: center;
  gap:         var(--space-2);
  padding:     var(--space-2) 0;
}

.metric-row__label {
  font-size:   var(--text-sm);
  font-weight: var(--fw-medium);
  color:       var(--text-secondary);
}

.metric-row__value {
  font-size:   var(--text-sm);
  font-weight: var(--fw-semibold);
  color:       var(--text-primary);
  font-family: var(--font-mono);
  text-align:  right;
}

/* 进度条（用于 CPU/内存使用率） */
.metric-row__bar {
  grid-column: 1 / -1;
  height:      3px;
  border-radius: var(--radius-full);
  background:  var(--border-default);
  overflow:    hidden;
}
.metric-row__bar-fill {
  height:     100%;
  border-radius: var(--radius-full);
  background: var(--color-primary);
  transition: width var(--duration-slow) var(--ease-standard);
}
.metric-row__bar-fill--warning { background: var(--color-warning); }
.metric-row__bar-fill--danger  { background: var(--color-danger); }
```

---

### 4.6 图标（NIcon）

不使用外部图标库，采用内联 SVG 组件 `NIcon.vue`。支持以下图标名：

| 图标名 | 用途 | 位置 |
|--------|------|------|
| `house` | 仪表盘 | Dock 导航 |
| `gear` | 设置 | Dock 导航 |
| `shield` | 隐私 | Dock 导航 |
| `terminal` | 日志 | Dock 导航 |
| `info` | 关于 | Dock 导航 |
| `sun` | 浅色模式 | TopBar 主题切换 |
| `moon` | 深色模式 | TopBar 主题切换 |

```vue
<!-- 使用示例 -->
<NIcon name="house" :size="20" />
```

所有图标均为 24×24 viewBox，`stroke-width="2"`，颜色继承 `currentColor`。

### 4.7 主题切换按钮（TopBar）

TopBar 右侧内置主题切换按钮，点击在深色/浅色之间即时切换。图标按当前主题状态显示：
- 深色模式 → 太阳图标（暗示可切换到浅色）
- 浅色模式 → 月亮图标（暗示可切换到深色）

```css
.top-bar__theme-btn {
  width:        28px;
  height:       28px;
  border:       none;
  border-radius: var(--radius-sm);
  background:   transparent;
  color:        var(--text-tertiary);
}

.top-bar__theme-btn:hover {
  background: var(--bg-hover);
  color:      var(--text-primary);
}
```

### 4.8 外观设置区（Settings 页新增）

Settings 页新增「外观」设置区块，集中管理：
- **主题模式**：深色 / 浅色 / 跟随系统（通过 `themeMode` 配置项）
- **主题色**：`<input type="color">` 取色器 + 文本输入框（对应 `seedColor`）
- **界面缩放**：数值输入（80%–200%，步长 5%，对应 `uiScale`）
- **界面字体**：文本输入（对应 `uiFont`，默认 Segoe UI）

所有外观变更通过 `useTheme().applyTheme()` 即时生效，无需刷新页面。

---

## 五、Dock 导航栏规范

### 5.1 外观与定位

Dock 是一条**固定在内容区底部中央**的浮动胶囊形导航栏：

- 固定宽度由内容决定（`fit-content`），不撑满全宽
- 圆角为 `var(--radius-full)`，形成胶囊形
- 背景使用 `--dock-bg`（实色，无模糊），加顶部细边框和投影
- 每个导航项：图标（20px）+ 文字标签（11px），上下排列
- 导航项共 5 个：仪表盘 / 设置 / 隐私 / 日志 / 关于

### 5.2 CSS 规范

```css
/* Dock 容器 */
.dock {
  position:      fixed;
  bottom:        var(--space-4);
  left:          50%;
  transform:     translateX(-50%);
  z-index:       100;
  display:       flex;
  align-items:   center;
  gap:           var(--dock-gap);
  padding:       var(--dock-padding);
  background:    var(--dock-bg);
  border:        1px solid var(--dock-border);
  border-radius: var(--radius-full);
  box-shadow:    var(--dock-shadow);
  user-select:   none;
}

/* Dock 导航项 */
.dock-item {
  display:        flex;
  flex-direction: column;
  align-items:    center;
  justify-content: center;
  gap:            3px;
  width:          var(--dock-item-size);
  height:         var(--dock-item-size);
  border-radius:  var(--radius-md);
  cursor:         pointer;
  color:          var(--text-tertiary);
  transition:     background var(--duration-fast) var(--ease-standard),
                  color var(--duration-fast) var(--ease-standard);
}

.dock-item:hover {
  background: var(--bg-hover);
  color:      var(--text-secondary);
}

/* 激活状态：主题色背景块 + 主题色文字 */
.dock-item--active {
  background: var(--color-primary-muted);
  color:      var(--color-primary);
}

/* 文字标签 */
.dock-item__label {
  font-size:   var(--text-xs);   /* 11px */
  font-weight: var(--fw-medium);
  line-height: 1;
}
```

### 5.3 Vue 组件结构

```vue
<!-- src/components/layout/Dock.vue -->
<script setup lang="ts">
import { useRouter, useRoute } from 'vue-router'
import NIcon from '@/components/base/NIcon.vue'

const router = useRouter()
const route  = useRoute()

const navItems = [
  { name: 'dashboard', icon: 'house',    label: '仪表盘' },
  { name: 'settings',  icon: 'gear',     label: '设置'   },
  { name: 'privacy',   icon: 'shield',   label: '隐私'   },
  { name: 'console',   icon: 'terminal', label: '日志'   },
  { name: 'about',     icon: 'info',     label: '关于'   },
]
</script>

<template>
  <nav class="dock" role="navigation" aria-label="主导航">
    <button
      v-for="item in navItems"
      :key="item.name"
      class="dock-item"
      :class="{ 'dock-item--active': route.name === item.name }"
      :aria-label="item.label"
      :aria-current="route.name === item.name ? 'page' : undefined"
      @click="router.push({ name: item.name })"
    >
      <NIcon :name="item.icon" :size="20" />
      <span class="dock-item__label">{{ item.label }}</span>
    </button>
  </nav>
</template>
```

### 5.4 内容区底部留白

为防止 Dock 遮挡页面内容，`PageContainer` 组件需在底部添加留白：

```css
.page-container {
  padding:        var(--space-6);
  padding-bottom: calc(var(--dock-height) + var(--space-6) + var(--space-4));
  overflow-y:     auto;
  height:         100%;
  box-sizing:     border-box;
}
```

---

## 六、主题系统实现

### 6.1 主题色注入

```typescript
// src/composables/useTheme.ts

import { watch } from 'vue'
import { useConfigStore } from '@/stores/config'

export function useTheme() {
  const configStore = useConfigStore()

  function applyTheme(config: { themeMode: string; seedColor: string; uiScale: number; uiFont: string }) {
    const root = document.documentElement

    // 深浅模式
    if (config.themeMode === 'system') {
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
      root.setAttribute('data-theme', prefersDark ? 'dark' : 'light')
    } else {
      root.setAttribute('data-theme', config.themeMode)
    }

    // 主题色（动态注入 CSS 变量）
    if (/^#[0-9a-f]{6}$/i.test(config.seedColor)) {
      root.style.setProperty('--color-primary', config.seedColor)
      root.style.setProperty('--color-primary-hover', adjustColor(config.seedColor, -15))
      root.style.setProperty('--color-primary-muted', hexToRgba(config.seedColor, 0.12))
    }

    // 界面缩放
    root.style.setProperty('--ui-scale', `${config.uiScale}%`)
    ;(document.body as HTMLElement).style.zoom = `${config.uiScale}%`

    // 字体
    if (config.uiFont) {
      root.style.setProperty('--ui-font', `"${config.uiFont}"`)
    }
  }

  // 跟随系统深色模式变化
  function watchSystemTheme() {
    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
      if (configStore.config?.themeMode === 'system') {
        applyTheme(configStore.config)
      }
    })
  }

  return { applyTheme, watchSystemTheme }
}
```

### 6.2 颜色工具函数

```typescript
// src/utils/color.ts

/** 16进制颜色 → rgba 字符串 */
export function hexToRgba(hex: string, alpha: number): string {
  const r = parseInt(hex.slice(1, 3), 16)
  const g = parseInt(hex.slice(3, 5), 16)
  const b = parseInt(hex.slice(5, 7), 16)
  return `rgba(${r}, ${g}, ${b}, ${alpha})`
}

/** 调整颜色亮度（lightness） */
export function adjustColor(hex: string, amount: number): string {
  // 简单实现：在 L* 轴偏移
  // 完整实现可用 hsl 转换
  const r = Math.max(0, Math.min(255, parseInt(hex.slice(1, 3), 16) + amount))
  const g = Math.max(0, Math.min(255, parseInt(hex.slice(3, 5), 16) + amount))
  const b = Math.max(0, Math.min(255, parseInt(hex.slice(5, 7), 16) + amount))
  return `#${r.toString(16).padStart(2, '0')}${g.toString(16).padStart(2, '0')}${b.toString(16).padStart(2, '0')}`
}
```

---

## 七、动效规范

### 7.1 允许的动效

| 场景 | 属性 | Duration | Easing |
|------|------|---------|--------|
| 悬停颜色变化 | `background`, `color` | `--duration-fast` | `--ease-standard` |
| 指标数值/进度条更新 | `width`, `opacity` | `--duration-slow` | `--ease-standard` |
| 页面切换 | `opacity` + `transform: translateY(8px)` | `--duration-base` | `--ease-decelerate` |
| 展开/折叠面板 | `height` (或 `max-height`) | `--duration-base` | `--ease-standard` |
| 开关滑动 | `transform` | `--duration-fast` | `--ease-standard` |
| Toast/通知出现 | `opacity` + `transform: translateY(-8px)` | `--duration-base` | `--ease-decelerate` |

### 7.2 页面切换动画（Vue Router）

```css
/* src/styles/animations.css */

.page-enter-active,
.page-leave-active {
  transition: opacity var(--duration-base) var(--ease-standard),
              transform var(--duration-base) var(--ease-standard);
}
.page-enter-from {
  opacity:   0;
  transform: translateY(6px);
}
.page-leave-to {
  opacity:   0;
  transform: translateY(-6px);
}
```

```vue
<!-- App.vue - RouterView 包裹动画 -->
<RouterView v-slot="{ Component }">
  <Transition name="page" mode="out-in">
    <component :is="Component" :key="$route.name" />
  </Transition>
</RouterView>
```

### 7.3 禁止的动效

- ❌ `animation: spin` / 旋转动画（loading 除外）
- ❌ `transition-duration > 400ms`（感知迟钝）
- ❌ `bounce`、`elastic` 等弹性曲线（不符合工具软件气质）
- ❌ 为纯装饰目的添加动画

---

## 八、日志控制台视觉规范

```css
/* 日志条目颜色 */
.log-entry { font-family: var(--font-mono); font-size: var(--text-sm); line-height: 1.6; }
.log-entry--info    { color: var(--text-secondary); }
.log-entry--success { color: var(--color-success); }
.log-entry--warn    { color: var(--color-warning); }
.log-entry--error   { color: var(--color-danger); }

/* 日志时间戳 */
.log-entry__time {
  color:       var(--text-tertiary);
  margin-right: var(--space-2);
  user-select: none;
}
```

---

## 九、响应式与窗口尺寸规范

| 窗口宽度 | 布局变化 |
|---------|---------|
| `< 900px` | 禁止（Tauri 设置 minWidth） |
| `900px ~ 1100px` | 侧边栏自动收起为图标模式（56px） |
| `≥ 1100px` | 侧边栏展开（200px） |

---

## 十、无障碍规范（基础）

| 元素 | 要求 |
|------|------|
| 交互元素 | 必须有 `tabindex` 支持键盘聚焦 |
| 图标按钮 | 必须有 `aria-label` |
| 开关 | 使用 `role="switch"` + `aria-checked` |
| 颜色对比度 | 正文文字 ≥ 4.5:1（WCAG AA） |
| 状态变化 | 不仅依赖颜色，同时提供文字描述 |

---

## 十一、禁止行为清单

```
❌ 在任何 .vue 文件中写 style="color: xxx" 或 style="font-size: xxx"
❌ 在 CSS 中硬编码颜色值（应改用 var(--token-name)）
❌ 在 CSS 中硬编码间距数值（如 padding: 16px，应改用 var(--space-4)）
❌ 圆角超过 var(--radius-lg)（12px）
❌ 使用 backdrop-filter: blur（玻璃效果）
❌ 添加大面积渐变背景
❌ transition-duration 超过 400ms
❌ 组件内重复定义已在 tokens.css 中声明的 CSS 变量
❌ 在 <style scoped> 中覆盖全局 Token（会造成主题失效）
```

## 2026-05-10 Win11 风格重构

- [x] 已完成：设计 Token 全面改为 Win10/11 Fluent 中性灰色调（深色 #1e1e1e / 浅色 #f3f3f3）
- [x] 已完成：移除 AppShell 和 Settings 页的所有渐变背景，改为纯色平面布局
- [x] 已完成：新增 `NIcon.vue` 内联 SVG 图标组件（house / gear / shield / terminal / info / sun / moon），替代 Phosphor Icons 外部依赖
- [x] 已完成：Dock 导航栏图标从文字 emoji 替换为 NIcon SVG 矢量图标
- [x] 已完成：TopBar 新增深色/浅色即时切换按钮
- [x] 已完成：Settings 页新增「外观」区块（主题模式 / 主题色 / 界面缩放 / 字体），替代原 hero 渐变卡片
- [x] 已完成：圆角系统收紧（md: 6px, lg: 8px）适配 Win11 硬朗风格，新增 --radius-xl: 12px
- [x] 已完成：新增 --elevation-1 / --elevation-2 阴影层级变量
- [x] 已完成：NSwitch 放大至 40×22px，滑块 16px 带微阴影
- [x] 已完成：NBadge 新增语义色小圆点指示器
- [x] 已完成：NInput/NSelect 高度收窄至 34px，hover 边框浮现，focus 带色环
- [x] 已完成：NButton 主按钮 hover/active 使用 filter: brightness() 反馈
- [x] 已完成：所有页面（Dashboard / Settings / Privacy / Console / About）卡片样式统一更新

## 2026-05-10 五大页面 UI 优化

- [x] 已完成：Dashboard — Hero 服务状态栏（状态指示点动画 + 相对时间 + 4 项快速统计）、双栏布局（快捷开关 + 系统指标）、截图中心（预览 + 元信息合并卡片）
- [x] 已完成：Settings — 水平分段导航栏（5 个锚点 pill 按钮 + 平滑滚动）、移除所有渐变卡片、外观区块（主题色取色器 + 缩放 + 字体）
- [x] 已完成：Privacy — Hero 描述横幅、窗口列表搜索过滤、已排除窗口视觉弱化、可视化选择器搜索 + 已排除标记
- [x] 已完成：Console — 日志级别筛选工具栏、自动滚动开关、日志计数 + 清空按钮、自适应填满页面高度
- [x] 已完成：About — 品牌 Hero（图标 + 版本 + 描述）、应用/系统信息双栏 KV 卡片、更新状态区 + 快捷导航入口
