use std::collections::HashMap;

use crate::animation::AnimationRecorder;
use crate::core::{Cell, Direction, Grid, Point};

use super::MazeSolver;

/// Wall follower (left-hand rule) maze solver.
///
/// This simple algorithm keeps one hand on a wall and follows it.
/// It works for all perfect mazes but may loop in mazes with cycles.
pub struct WallFollowerSolver;

impl WallFollowerSolver {
    pub fn new() -> Self {
        Self
    }
}

impl Default for WallFollowerSolver {
    fn default() -> Self {
        Self::new()
    }
}

impl MazeSolver for WallFollowerSolver {
    fn solve(&self, grid: &Grid, start: Point, end: Point) -> AnimationRecorder {
        let mut recorder = AnimationRecorder::from_grid(grid);
        let mut working = grid.clone();
        let mut came_from = HashMap::new();
        let mut current = start;
        let mut dir = Direction::South; // initial direction

        came_from.insert(start, None);
        working.set(start, Cell::Current).unwrap();
        recorder.record(&working, "Wall Follower: starting from entrance");

        let mut steps = 0;
        let max_steps = grid.width() * grid.height() * 2;

        while current != end && steps < max_steps {
            steps += 1;

            // Try turning left first
            let left_dir = dir.counter_clockwise();
            let left = current.neighbor(left_dir);
            let can_go_left = left.is_some_and(|p| matches!(grid.get(p), Some(Cell::Passage)));

            if can_go_left {
                dir = left_dir;
            } else {
                // Try going straight
                let straight = current.neighbor(dir);
                let can_go_straight =
                    straight.is_some_and(|p| matches!(grid.get(p), Some(Cell::Passage)));

                if !can_go_straight {
                    // Turn right
                    dir = dir.clockwise();
                    let right = current.neighbor(dir);
                    let can_go_right =
                        right.is_some_and(|p| matches!(grid.get(p), Some(Cell::Passage)));

                    if !can_go_right {
                        // Dead end, turn around
                        dir = dir.clockwise();
                    }
                }
            }

            if let Some(next) = current.neighbor(dir) {
                if matches!(grid.get(next), Some(Cell::Passage)) {
                    came_from.entry(next).or_insert(Some((current, dir)));
                    current = next;
                    working.set(current, Cell::Current).unwrap();
                    recorder.record(
                        &working,
                        format!("Wall Follower: at ({}, {})", current.x, current.y),
                    );
                }
            }
        }

        if current == end {
            recorder.record(&working, "Wall Follower: exit found!");
            super::common::reconstruct_path(&mut working, &came_from, end, &mut recorder);
        } else {
            recorder.record(&working, "Wall Follower: gave up (possible loop)");
        }

        recorder
    }

    fn name(&self) -> &'static str {
        "Wall Follower"
    }

    fn description(&self) -> &'static str {
        "Follows the left wall. Simple and works for perfect mazes."
    }
}
