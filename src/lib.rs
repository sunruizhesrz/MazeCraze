//! MazeCraze —— 一个 TUI 交互式迷宫生成与求解器。
//!
//! 本库提供以下能力：
//! - 多种迷宫生成算法（递归回溯、Prim、Kruskal）
//! - 多种迷宫求解算法（BFS、DFS、A*、沿墙行走）
//! - 实时逐步动画录制与回放
//! - Unicode 与 ASCII 两种渲染引擎
//! - 用于交互式探索的终端 UI

pub mod animation;
pub mod cli;
pub mod core;
pub mod export;
pub mod generator;
pub mod renderer;
pub mod solver;
pub mod tui;

/// 为便于使用，重新导出常用类型。
pub use core::{grid::Grid, point::Point};
