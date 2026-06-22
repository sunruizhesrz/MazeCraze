# MazeCraze Web 前端

[中文](README.md) | [English](README.en.md)

基于 React 18 + TypeScript + Vite 构建的浏览器端迷宫可视化界面，与 Rust 端形成"双端可视化"互补。

## 功能特性

- **三栏布局**：左侧控制面板、中央迷宫画布、右侧统计面板
- **3 种生成算法**：递归回溯、随机化 Prim、随机化 Kruskal
- **4 种求解算法**：BFS、DFS、A\*、沿墙行走
- **挑战模式**：可用方向键 / WASD 自行挑战迷宫，记录保存在 `localStorage`
- **动画控制**：播放 / 暂停、单步前进 / 后退、速度调节
- **Canvas 渲染**：高区分度配色，特殊单元格带发光效果
- **响应式尺寸**：根据迷宫大小自适应单元格像素

## 快速开始

```bash
# 进入前端目录
cd frontend

# 安装依赖
npm install

# 启动开发服务器
npm run dev

# 构建生产版本
npm run build

# 预览构建结果
npm run preview
```

## 目录结构

```
frontend/
├── index.html              # HTML 入口
├── package.json            # 依赖与脚本
├── vite.config.ts          # Vite 配置
├── eslint.config.js        # ESLint 配置
├── tsconfig.json           # TypeScript 配置
└── src/
    ├── main.tsx            # 应用挂载入口
    ├── App.tsx             # 根组件，管理全局状态与事件
    ├── App.css             # 全局样式
    ├── index.css           # 基础样式
    ├── core/
    │   ├── types.ts        # Cell、Direction、Point 等基础类型
    │   └── grid.ts         # Grid 网格类
    ├── algorithms/
    │   ├── generators.ts   # 三种迷宫生成算法
    │   └── solvers.ts      # 四种迷宫求解算法
    └── components/
        ├── Controls.tsx    # 控制面板组件
        ├── MazeCanvas.tsx  # Canvas 迷宫渲染组件
        └── Stats.tsx       # 统计信息组件
```

## 按键说明

| 按键 | 功能 |
|-----|------|
| `↑` / `W` | 向上移动（挑战模式） |
| `↓` / `S` | 向下移动（挑战模式） |
| `←` / `A` | 向左移动（挑战模式） |
| `→` / `D` | 向右移动（挑战模式） |

## 颜色图例

| 颜色 | 含义 |
|-----|------|
| 黑色 | 墙 |
| 白色 | 通道 |
| 橙色 | 已访问 |
| 品红 | 当前位置 |
| 亮绿 | 最终路径 |
| 青色 | 起点 |
| 纯红 | 终点 |

## ESLint 配置扩展

本项目基于 Vite 模板，包含基础 ESLint 规则。如需开启类型感知的规则，可参考以下配置：

```js
export default defineConfig([
  globalIgnores(['dist']),
  {
    files: ['**/*.{ts,tsx}'],
    extends: [
      // 其他配置...

      // 用以下替换 tseslint.configs.recommended
      tseslint.configs.recommendedTypeChecked,
      // 更严格的规则
      tseslint.configs.strictTypeChecked,
      // 风格相关规则
      tseslint.configs.stylisticTypeChecked,
    ],
    languageOptions: {
      parserOptions: {
        project: ['./tsconfig.node.json', './tsconfig.app.json'],
        tsconfigRootDir: import.meta.dirname,
      },
    },
  },
])
```

也可安装 [eslint-plugin-react-x](https://github.com/Rel1cx/eslint-react/tree/main/packages/plugins/eslint-plugin-react-x)
和 [eslint-plugin-react-dom](https://github.com/Rel1cx/eslint-react/tree/main/packages/plugins/eslint-plugin-react-dom)
以获得 React 专属的 lint 规则。

## 许可证

MIT
