use rand::seq::SliceRandom;

use crate::animation::AnimationRecorder;
use crate::core::{Grid, Point};

use super::{init_grid, MazeGenerator};

/// 递归回溯迷宫生成器（随机化深度优先搜索）。
///
/// 该算法生成的迷宫具有长而蜿蜒的走廊，且死路较少。
/// 它是最简单、最常见的迷宫生成算法。
pub struct RecursiveBacktracker;

impl RecursiveBacktracker {
    pub fn new() -> Self {
        Self
    }

    fn carve(grid: &mut Grid, current: Point, recorder: &mut AnimationRecorder) {
        recorder.record(grid, format!("Visiting ({}, {})", current.x, current.y));

        let mut neighbors: Vec<_> = grid.passage_neighbors(current);
        neighbors.shuffle(&mut rand::thread_rng());

        for (next, dir) in neighbors {
            if matches!(grid.get(next), Some(crate::core::Cell::Wall)) {
                let _ = grid.carve_passage(current, dir);
                recorder.record(grid, format!("Carved passage to ({}, {})", next.x, next.y));
                Self::carve(grid, next, recorder);
            }
        }
    }
}

impl Default for RecursiveBacktracker {
    fn default() -> Self {
        Self::new()
    }
}

impl MazeGenerator for RecursiveBacktracker {
    fn generate(&self, width: usize, height: usize) -> AnimationRecorder {
        let mut recorder = AnimationRecorder::new(width, height);
        let (mut grid, start) = init_grid(width, height);
        Self::carve(&mut grid, start, &mut recorder);
        recorder.finish(grid);
        recorder
    }

    fn name(&self) -> &'static str {
        "Recursive Backtracker"
    }

    fn description(&self) -> &'static str {
        "A randomized DFS that creates long corridors with fewer dead ends."
    }
}
