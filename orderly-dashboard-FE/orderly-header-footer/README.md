# Orderly Header & Footer — 迁移包

## 文件结构

```
orderly-header-footer/
├── components/
│   ├── SiteHeader.tsx     ← Header 组件（MorphingHeader, HeaderLogo）
│   └── SiteFooter.tsx     ← Footer 组件（SiteFooter, BuyOrderModal）
├── shared/
│   └── orderly.ts         ← 所有导航链接、社交链接数据
├── imports/
│   ├── IcSocialYoutubeS24.tsx
│   ├── UniswapLogo.tsx
│   └── CamelotLogo.tsx
├── styles/
│   ├── fonts.css          ← Atyp BL Variable 字体声明（必须）
│   └── theme.css          ← 字体映射 CSS（必须，否则字体显示异常）
└── public/
    └── images/
        ├── svg/           ← EthBadge, ArbBadge, SolBadge, PancakeLogo, WOOFiLogo
        └── logos/
            └── raydium.png
```

## 迁移步骤

### 1. 复制文件

按以下对应关系复制到新项目：

| 本包路径 | 新项目目标路径 |
|---------|--------------|
| `components/*.tsx` | `src/app/components/` (或你的组件目录) |
| `shared/orderly.ts` | `src/app/shared/orderly.ts` |
| `imports/*.tsx` | `src/imports/` |
| `styles/fonts.css` | `src/styles/fonts.css` |
| `styles/theme.css` | `src/styles/theme.css` |
| `public/images/` | `public/images/` |

### 2. 安装依赖

```bash
npm install motion posthog-js
```

> 如果不需要 PostHog 埋点，删除 `SiteHeader.tsx` 中所有 `posthog.capture(...)` 调用，并移除 `import posthog` 这行。

### 3. 引入字体 CSS

在全局 CSS 或 layout 文件中引入：

```css
@import './styles/fonts.css';
@import './styles/theme.css';
```

### 4. 使用组件

```tsx
import { MorphingHeader } from '@/app/components/SiteHeader';
import { SiteFooter }     from '@/app/components/SiteFooter';

export default function Layout({ children }) {
  return (
    <>
      {/* Header 需要 fixed/sticky 定位，放在页面顶部 */}
      <div style={{ position: 'fixed', top: 0, left: 0, right: 0, zIndex: 50, display: 'flex', justifyContent: 'center' }}>
        <MorphingHeader />
      </div>

      <main>{children}</main>

      <SiteFooter />
    </>
  );
}
```

### 5. 修改路径别名（如需要）

`SiteHeader.tsx` 和 `SiteFooter.tsx` 中使用 `@/` 指向 `src/`。  
如果新项目的别名不同，全局替换 `@/app/shared/orderly` 和 `@/imports/` 为对应路径。

## 导出的组件

| 组件 | 来源文件 | 说明 |
|------|---------|------|
| `MorphingHeader` | SiteHeader.tsx | 主 Header，滚动时自动隐藏/显示 |
| `HeaderLogo` | SiteHeader.tsx | 单独的 Orderly Logo 链接 |
| `CampaignIcon` | SiteHeader.tsx | Campaigns 火焰图标 |
| `SiteFooter` | SiteFooter.tsx | 完整 Footer |
| `BuyOrderModal` | SiteFooter.tsx | Buy ORDER 弹窗，可单独复用 |
