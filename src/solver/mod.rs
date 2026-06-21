//! Maze solving algorithms.
//!
//! All solvers implement the [`MazeSolver`] trait. They operate on an existing
//! [`Grid`] and produce an [`AnimationRecorder`] capturing the exploration and
//! solution process.

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

/// A maze solver finds a path from start to end in an existing maze.
pub trait MazeSolver: Send + Sync {
    /// Solve the maze, recording every step.
    fn solve(&self, grid: &Grid, start: Point, end: Point) -> AnimationRecorder;

    /// Human-readable name of the algorithm.
    fn name(&self) -> &'static str;

    /// Short description.
    fn description(&self) -> &'static str;
}

/// Registry of all available solvers.
pub fn all_solvers() -> Vec<Box<dyn MazeSolver>> {
    vec![
        Box::new(BfsSolver::new()),
        Box::new(DfsSolver::new()),
        Box::new(AStarSolver::new()),
        Box::new(WallFollowerSolver::new()),
    ]
}

/// Find a solver by its name (case-insensitive, supports partial match and abbreviations).
pub fn find_solver(name: &str) -> Option<Box<dyn MazeSolver>> {
    let name_lower = name.to_lowercase();
    // Map common abbreviations to full names
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
