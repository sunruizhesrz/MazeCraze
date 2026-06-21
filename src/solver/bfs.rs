use std::collections::{HashMap, VecDeque};

use crate::animation::AnimationRecorder;
use crate::core::{Cell, Direction, Grid, Point};

use super::MazeSolver;

/// Breadth-First Search maze solver.
///
/// Guarantees the shortest path in an unweighted grid.
pub struct BfsSolver;

impl BfsSolver {
    pub fn new() -> Self {
        Self
    }
}

impl Default for BfsSolver {
    fn default() -> Self {
        Self::new()
    }
}

impl MazeSolver for BfsSolver {
    fn solve(&self, grid: &Grid, start: Point, end: Point) -> AnimationRecorder {
        let mut recorder = AnimationRecorder::from_grid(grid);
        let mut working = grid.clone();
        let mut visited = HashMap::new();
        let mut queue = VecDeque::new();

        queue.push_back(start);
        visited.insert(start, None);
        working.set(start, Cell::Current).unwrap();
        recorder.record(&working, "BFS: starting from entrance");

        while let Some(current) = queue.pop_front() {
            working.set(current, Cell::Visited).unwrap();

            if current == end {
                recorder.record(&working, "BFS: exit found!");
                super::common::reconstruct_path(&mut working, &visited, end, &mut recorder);
                return recorder;
            }

            for dir in Direction::ALL {
                if let Some(next) = current.neighbor(dir) {
                    if visited.contains_key(&next) {
                        continue;
                    }
                    if let Some(Cell::Passage) = grid.get(next) {
                        visited.insert(next, Some((current, dir)));
                        working.set(next, Cell::Current).unwrap();
                        queue.push_back(next);
                        recorder
                            .record(&working, format!("BFS: visiting ({}, {})", next.x, next.y));
                    }
                }
            }
        }

        recorder.record(&working, "BFS: no solution found");
        recorder
    }

    fn name(&self) -> &'static str {
        "Breadth-First Search"
    }

    fn description(&self) -> &'static str {
        "Explores level by level. Guarantees the shortest path."
    }
}
