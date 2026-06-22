use std::collections::HashMap;

use crate::animation::AnimationRecorder;
use crate::core::{Cell, Direction, Grid, Point};

use super::MazeSolver;

/// 沿墙行走（左手法则）迷宫求解器。
///
/// 这个简单的算法始终将一只手放在墙上并顺着墙前进。
/// 对所有完美迷宫都有效，但在带环路的迷宫中可能陷入循环。
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
        let mut dir = Direction::South; // 初始方向

        came_from.insert(start, None);
        working.set(start, Cell::Current).unwrap();
        recorder.record(&working, "Wall Follower: starting from entrance");

        let mut steps = 0;
        let max_steps = grid.width() * grid.height() * 2;

        while current != end && steps < max_steps {
            steps += 1;

            // 优先尝试左转
            let left_dir = dir.counter_clockwise();
            let left = current.neighbor(left_dir);
            let can_go_left = left.is_some_and(|p| matches!(grid.get(p), Some(Cell::Passage)));

            if can_go_left {
                dir = left_dir;
            } else {
                // 尝试直行
                let straight = current.neighbor(dir);
                let can_go_straight =
                    straight.is_some_and(|p| matches!(grid.get(p), Some(Cell::Passage)));

                if !can_go_straight {
                    // 右转
                    dir = dir.clockwise();
                    let right = current.neighbor(dir);
                    let can_go_right =
                        right.is_some_and(|p| matches!(grid.get(p), Some(Cell::Passage)));

                    if !can_go_right {
                        // 死路，调头
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
