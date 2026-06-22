//! 迷宫求解算法。
//!
//! 所有求解器都实现了 [`MazeSolver`] trait。它们在已存在的 [`Grid`] 上运行，
//! 并产出 [`AnimationRecorder`]，记录探索和求解过程。

use crate::animation::AnimationRecorder;
use crate::core::{Grid, Point};

pub mod astar;
pub mod bfs;
pub mod common;
pub mod dfs;
pub mod wall_follower;

pub use astar::AStarSolver;
pub use bfs::BfsSolver;
pub use dfs::DfsSolver;
pub use wall_follower::WallFollowerSolver;

/// 迷宫求解器在已有迷宫中寻找从起点到终点的路径。
pub trait MazeSolver: Send + Sync {
    /// 求解迷宫，并记录每一步。
    fn solve(&self, grid: &Grid, start: Point, end: Point) -> AnimationRecorder;

    /// 算法的人类可读名称。
    fn name(&self) -> &'static str;

    /// 算法的简短描述。
    fn description(&self) -> &'static str;
}

/// 所有可用求解器的注册表。
pub fn all_solvers() -> Vec<Box<dyn MazeSolver>> {
    vec![
        Box::new(BfsSolver::new()),
        Box::new(DfsSolver::new()),
        Box::new(AStarSolver::new()),
        Box::new(WallFollowerSolver::new()),
    ]
}

/// 按名称查找求解器（大小写不敏感，支持部分匹配和缩写）。
pub fn find_solver(name: &str) -> Option<Box<dyn MazeSolver>> {
    let name_lower = name.to_lowercase();
    // 将常见缩写映射到全称
    let search_term = match name_lower.as_str() {
        "bfs" => "breadth-first",
        "dfs" => "depth-first",
        "astar" | "a*" => "a* search",
        "wall-follower" | "wallfollower" => "wall follower",
        _ => &name_lower,
    };
    all_solvers().into_iter().find(|s| {
        let s_name = s.name().to_lowercase();
        s.name().eq_ignore_ascii_case(name)
            || s_name.contains(search_term)
            || search_term.contains(&s_name)
    })
}
