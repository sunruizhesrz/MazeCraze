use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use crate::animation::AnimationRecorder;
use crate::core::{Cell, Direction, Grid, Point};

use super::MazeSolver;

/// 优先队列中使用的 A* 搜索节点。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Node {
    point: Point,
    cost: usize,     // g(n)：从起点到当前点的代价
    priority: usize, // f(n) = g(n) + h(n)
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority) // 按 priority 构造小顶堆
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A* 迷宫求解器。
///
/// 使用曼哈顿距离作为启发式函数。可保证找到最短路径，
/// 且通常比 BFS 探索更少的单元格。
pub struct AStarSolver;

impl AStarSolver {
    pub fn new() -> Self {
        Self
    }
}

impl Default for AStarSolver {
    fn default() -> Self {
        Self::new()
    }
}

impl MazeSolver for AStarSolver {
    fn solve(&self, grid: &Grid, start: Point, end: Point) -> AnimationRecorder {
        let mut recorder = AnimationRecorder::from_grid(grid);
        let mut working = grid.clone();
        let mut came_from = HashMap::new();
        let mut cost_so_far = HashMap::new();
        let mut heap = BinaryHeap::new();

        came_from.insert(start, None);
        cost_so_far.insert(start, 0);
        heap.push(Node {
            point: start,
            cost: 0,
            priority: super::common::manhattan_distance(start, end),
        });

        working.set(start, Cell::Current).unwrap();
        recorder.record(&working, "A*: starting from entrance");

        while let Some(current) = heap.pop() {
            working.set(current.point, Cell::Visited).unwrap();

            if current.point == end {
                recorder.record(&working, "A*: exit found!");
                super::common::reconstruct_path(&mut working, &came_from, end, &mut recorder);
                return recorder;
            }

            for dir in Direction::ALL {
                if let Some(next) = current.point.neighbor(dir) {
                    if let Some(Cell::Passage) = grid.get(next) {
                        let new_cost = current.cost + 1;
                        let old_cost = cost_so_far.get(&next).copied().unwrap_or(usize::MAX);

                        if new_cost < old_cost {
                            cost_so_far.insert(next, new_cost);
                            came_from.insert(next, Some((current.point, dir)));
                            heap.push(Node {
                                point: next,
                                cost: new_cost,
                                priority: new_cost + super::common::manhattan_distance(next, end),
                            });
                            working.set(next, Cell::Current).unwrap();
                            recorder.record(
                                &working,
                                format!("A*: evaluating ({}, {})", next.x, next.y),
                            );
                        }
                    }
                }
            }
        }

        recorder.record(&working, "A*: no solution found");
        recorder
    }

    fn name(&self) -> &'static str {
        "A* Search"
    }

    fn description(&self) -> &'static str {
        "Heuristic-guided search. Finds shortest path with minimal exploration."
    }
}
