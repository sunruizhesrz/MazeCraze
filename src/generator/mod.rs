//! Maze generation algorithms.
//!
//! All generators implement the [`MazeGenerator`] trait, allowing them to be
//! used interchangeably. Each generator produces an [`AnimationRecorder`] that
//! captures every step of the generation process for visualization.

use crate::animation::AnimationRecorder;
use crate::core::{Grid, Point};

pub mod backtracker;
pub mod common;
pub mod kruskal;
pub mod prim;

pub use backtracker::RecursiveBacktracker;
pub use kruskal::RandomizedKruskal;
pub use prim::RandomizedPrim;

/// A maze generator produces a maze by progressively carving passages.
pub trait MazeGenerator: Send + Sync {
    /// Generate a maze, recording every step.
    fn generate(&self, width: usize, height: usize) -> AnimationRecorder;

    /// Human-readable name of the algorithm.
    fn name(&self) -> &'static str;

    /// Short description of how the algorithm works.
    fn description(&self) -> &'static str;
}

/// Registry of all available generators.
pub fn all_generators() -> Vec<Box<dyn MazeGenerator>> {
    vec![
        Box::new(RecursiveBacktracker::new()),
        Box::new(RandomizedPrim::new()),
        Box::new(RandomizedKruskal::new()),
    ]
}

/// Find a generator by its name (case-insensitive, supports partial match).
pub fn find_generator(name: &str) -> Option<Box<dyn MazeGenerator>> {
    let name_lower = name.to_lowercase();
    all_generators().into_iter().find(|g| {
        let g_name = g.name().to_lowercase();
        g.name().eq_ignore_ascii_case(name)
            || g_name.contains(&name_lower)
            || name_lower.contains(&g_name)
    })
}

/// Initialize a grid with the starting passage carved at (1, 1).
pub fn init_grid(width: usize, height: usize) -> (Grid, Point) {
    let mut grid = Grid::new(width, height).expect("valid dimensions");
    let start = Point::new(1, 1);
    grid.set(start, crate::core::Cell::Passage)
        .expect("start is in bounds");
    (grid, start)
}
