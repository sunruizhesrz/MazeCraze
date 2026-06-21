use std::collections::HashMap;

use crate::animation::AnimationRecorder;
use crate::core::{Cell, Direction, Grid, Point};

use super::MazeSolver;

/// Depth-First Search maze solver.
///
/// Explores as deep as possible before backtracking. Does not guarantee
/// the shortest path.
pub struct DfsSolver;

impl DfsSolver {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DfsSolver {
    fn default() -> Self {
        Self::new()
    }
}

impl MazeSolver for DfsSolver {
    fn solve(&self, grid: &Grid, start: Point, end: Point) -> AnimationRecorder {
        let mut recorder = AnimationRecorder::from_grid(grid);
        let mut working = grid.clone();
        let mut visited = HashMap::new();
        let mut stack = vec![start];

        visited.insert(start, None);
        working.set(start, Cell::Current).unwrap();
        recorder.record(&working, "DFS: starting from entrance");

        while let Some(current) = stack.pop() {
            if current == end {
                recorder.record(&working, "DFS: exit found!");
                super::common::reconstruct_path(&mut working, &visited, end, &mut recorder);
                return recorder;
            }

            let mut found_next = false;
            for dir in Direction::ALL {
                if let Some(next) = current.neighbor(dir) {
                    if visited.contains_key(&next) {
                        continue;
                    }
                    if let Some(Cell::Passage) = grid.get(next) {
                        visited.insert(next, Some((current, dir)));
                        working.set(next, Cell::Current).unwrap();
                        // Push current back so we can backtrack if next is a dead end
                        stack.push(current);
                        stack.push(next);
                        recorder
                            .record(&working, format!("DFS: exploring ({}, {})", next.x, next.y));
                        found_next = true;
                        break; // DFS: go deep immediately
                    }
                }
            }

            if !found_next {
                // Dead end: mark as visited and let stack pop back to parent
                working.set(current, Cell::Visited).unwrap();
            }
        }

        recorder.record(&working, "DFS: no solution found");
        recorder
    }

    fn name(&self) -> &'static str {
        "Depth-First Search"
    }

    fn description(&self) -> &'static str {
        "Explores deep corridors first. Fast but does not guarantee shortest path."
    }
}
