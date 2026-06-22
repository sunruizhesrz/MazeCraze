use rand::seq::IteratorRandom;

use crate::animation::AnimationRecorder;
use crate::core::{Cell, Point};

use super::{init_grid, MazeGenerator};

/// 随机化普里姆（Prim）算法迷宫生成器。
///
/// 该算法从起始单元格开始，通过随机选取前沿墙并开凿它来向外扩展迷宫。
/// 生成的迷宫走廊较短，结构更"有机"，且存在大量死路。
pub struct RandomizedPrim;

impl RandomizedPrim {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RandomizedPrim {
    fn default() -> Self {
        Self::new()
    }
}

impl MazeGenerator for RandomizedPrim {
    fn generate(&self, width: usize, height: usize) -> AnimationRecorder {
        let mut recorder = AnimationRecorder::new(width, height);
        let (mut grid, start) = init_grid(width, height);
        let mut rng = rand::thread_rng();

        let mut frontier: Vec<(Point, Point)> = Vec::new(); // (墙, 目标单元格)

        // 添加初始前沿墙
        for (neighbor, _dir) in grid.passage_neighbors(start) {
            let wall = Point::new((start.x + neighbor.x) / 2, (start.y + neighbor.y) / 2);
            frontier.push((wall, neighbor));
        }

        while let Some(idx) = (0..frontier.len()).choose(&mut rng) {
            let (wall, target) = frontier.swap_remove(idx);

            if matches!(grid.get(target), Some(Cell::Wall)) {
                grid.set(wall, Cell::Passage).unwrap();
                grid.set(target, Cell::Passage).unwrap();
                recorder.record(
                    &grid,
                    format!("Prim: carved to ({}, {})", target.x, target.y),
                );

                for (neighbor, _dir) in grid.passage_neighbors(target) {
                    if matches!(grid.get(neighbor), Some(Cell::Wall)) {
                        let new_wall =
                            Point::new((target.x + neighbor.x) / 2, (target.y + neighbor.y) / 2);
                        frontier.push((new_wall, neighbor));
                    }
                }
            }
        }

        recorder.finish(grid);
        recorder
    }

    fn name(&self) -> &'static str {
        "Randomized Prim"
    }

    fn description(&self) -> &'static str {
        "Grows the maze from a random frontier, producing short corridors and many dead ends."
    }
}
