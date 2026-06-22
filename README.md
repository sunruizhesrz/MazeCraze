# MazeCraze

[中文](README.md) | [English](README.en.md)

> 一个支持实时算法动画的 TUI 交互式迷宫生成与求解器。

## 功能特性

- **3 种迷宫生成算法**：递归回溯（Recursive Backtracker）、随机化 Prim、随机化 Kruskal
- **4 种迷宫求解算法**：BFS（最短路径）、DFS、A\*（启发式）、沿墙行走（Wall Follower）
- **实时动画**：可观看生成与求解的每一步，支持播放 / 暂停 / 单步控制
- **双渲染模式**：Unicode 制表符（美观）与 ASCII（兼容性更强）
- **交互式 TUI**：基于 `ratatui`，提供流畅的终端体验
- **命令行批处理模式**：可在命令行直接生成并求解迷宫
- **导出功能**：可将迷宫保存为文本文件
- **挑战模式**：玩家可用方向键 / WASD 自行挑战迷宫，记录本地最佳成绩

## 快速开始

```bash
# 进入项目目录
cd mazecraze

# 构建
cargo build --release

# 以 TUI 模式运行（默认）
cargo run

# 命令行批处理模式
cargo run -- --generate backtracker --solve astar --size 31x31 --export maze.txt
```

## 按键说明

| 按键 | 功能 |
|-----|------|
| `g` | 生成新迷宫 |
| `s` | 求解当前迷宫 |
| `Space` | 播放 / 暂停动画 |
| `→` / `l` | 单步前进 |
| `←` / `h` | 单步后退 |
| `+` / `]` | 提升播放速度 |
| `r` | 重新开始当前动画 |
| `p` | 开始挑战模式 |
| `↑↓←→` / `WASD` | 挑战模式下移动玩家 |
| `h` | 显示帮助 |
| `q` / `Esc` | 退出 / 返回菜单 |

## 项目结构

```
src/
├── main.rs              # 程序入口
├── lib.rs               # 公共 API
├── core/                # 网格、单元格、坐标、方向
├── generator/           # 迷宫生成算法及 trait
├── solver/              # 迷宫求解算法及 trait
├── renderer/            # Unicode 与 ASCII 渲染器
├── animation/           # 帧记录器与播放器
├── tui/                 # 终端 UI（ratatui）
├── export/              # 文本导出
└── cli/                 # 命令行参数
tests/
├── grid_tests.rs        # 网格单元测试
├── generator_tests.rs   # 生成器测试
├── solver_tests.rs      # 求解器测试
├── integration_tests.rs # 集成测试
└── challenge_tests.rs   # 挑战模式测试
frontend/                # React + TypeScript Web 前端
```

## 涉及的 Rust 特性

- **所有权与借用**：严格区分可变的生成过程与不可变的渲染过程
- **Trait 系统**：通过 `MazeGenerator`、`MazeSolver`、`Renderer` trait 实现算法可插拔
- **泛型**：`AnimationRecorder` 可与任意算法配合使用
- **错误处理**：自定义 `GridError`，使用 `Result` 传播错误，极少使用 `unwrap`
- **模式匹配**：对 `Cell`、`Direction`、`AppState` 进行穷尽 `match`
- **测试**：单元测试 + 集成测试，覆盖所有算法组合
- **工程化**：使用 `cargo fmt`、`cargo clippy`，模块化工作空间

## Web 前端

项目另含一个 React + TypeScript + Vite 的 Web 前端（位于 `frontend/` 目录），
使用 Canvas 渲染迷宫，支持与 TUI 类似的生成、求解、挑战功能。
详见 `frontend/README.md`。

## 许可证

MIT
