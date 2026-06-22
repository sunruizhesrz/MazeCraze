//! 迷宫生成算法。
//!
//! 所有生成器都实现了 [`MazeGenerator`] trait，因此可以互换使用。
//! 每个生成器会产出一个 [`AnimationRecorder`]，用于记录生成过程中的每一步，
//! 以便后续可视化展示。

use crate::animation::AnimationRecorder;
use crate::core::{Grid, Point};

pub mod backtracker;
pub mod common;
pub mod kruskal;
pub mod prim;

pub use backtracker::RecursiveBacktracker;
pub use kruskal::RandomizedKruskal;
pub use prim::RandomizedPrim;

/// 迷宫生成器通过逐步开凿通道来生成迷宫。
pub trait MazeGenerator: Send + Sync {
    /// 生成迷宫，并记录每一步。
    fn generate(&self, width: usize, height: usize) -> AnimationRecorder;

    /// 算法的人类可读名称。
    fn name(&self) -> &'static str;

    /// 算法工作原理的简短描述。
    fn description(&self) -> &'static str;
}

/// 所有可用生成器的注册表。
pub fn all_generators() -> Vec<Box<dyn MazeGenerator>> {
    vec![
        Box::new(RecursiveBacktracker::new()),
        Box::new(RandomizedPrim::new()),
        Box::new(RandomizedKruskal::new()),
    ]
}

/// 按名称查找生成器（大小写不敏感，支持部分匹配）。
pub fn find_generator(name: &str) -> Option<Box<dyn MazeGenerator>> {
    let name_lower = name.to_lowercase();
    all_generators().into_iter().find(|g| {
        let g_name = g.name().to_lowercase();
        g.name().eq_ignore_ascii_case(name)
            || g_name.contains(&name_lower)
            || name_lower.contains(&g_name)
    })
}

/// 初始化网格，并在 (1, 1) 处开凿起始通道。
pub fn init_grid(width: usize, height: usize) -> (Grid, Point) {
    let mut grid = Grid::new(width, height).expect("valid dimensions");
    let start = Point::new(1, 1);
    grid.set(start, crate::core::Cell::Passage)
        .expect("start is in bounds");
    (grid, start)
}
