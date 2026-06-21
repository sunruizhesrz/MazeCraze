use rand::seq::SliceRandom;

use crate::animation::AnimationRecorder;
use crate::core::{Cell, Point};

use super::{init_grid, MazeGenerator};

/// Union-Find (Disjoint Set Union) data structure for Kruskal's algorithm.
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return false;
        }
        match self.rank[root_x].cmp(&self.rank[root_y]) {
            std::cmp::Ordering::Less => self.parent[root_x] = root_y,
            std::cmp::Ordering::Greater => self.parent[root_y] = root_x,
            std::cmp::Ordering::Equal => {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
        }
        true
    }
}

/// Randomized Kruskal's algorithm maze generator.
///
/// Treats the maze as a graph and finds a spanning tree by randomly selecting
/// walls to remove, using union-find to avoid cycles. Produces a very uniform
/// distribution of corridors.
pub struct RandomizedKruskal;

impl RandomizedKruskal {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RandomizedKruskal {
    fn default() -> Self {
        Self::new()
    }
}

impl MazeGenerator for RandomizedKruskal {
    fn generate(&self, width: usize, height: usize) -> AnimationRecorder {
        let mut recorder = AnimationRecorder::new(width, height);
        let (mut grid, _start) = init_grid(width, height);
        let mut rng = rand::thread_rng();

        // Collect all passage cells and all walls between them
        let mut passages: Vec<Point> = Vec::new();
        let mut walls: Vec<(Point, Point, Point)> = Vec::new(); // (wall, cell_a, cell_b)

        for y in (1..height).step_by(2) {
            for x in (1..width).step_by(2) {
                let p = Point::new(x, y);
                grid.set(p, Cell::Passage).unwrap();
                passages.push(p);

                // East neighbor
                if x + 2 < width {
                    let wall = Point::new(x + 1, y);
                    let neighbor = Point::new(x + 2, y);
                    walls.push((wall, p, neighbor));
                }
                // South neighbor
                if y + 2 < height {
                    let wall = Point::new(x, y + 1);
                    let neighbor = Point::new(x, y + 2);
                    walls.push((wall, p, neighbor));
                }
            }
        }

        walls.shuffle(&mut rng);

        let mut uf = UnionFind::new(passages.len());
        let index_of = |p: Point| -> usize { passages.iter().position(|&q| q == p).unwrap() };

        for (wall, a, b) in walls {
            let idx_a = index_of(a);
            let idx_b = index_of(b);
            if uf.union(idx_a, idx_b) {
                grid.set(wall, Cell::Passage).unwrap();
                recorder.record(
                    &grid,
                    format!("Kruskal: removed wall at ({}, {})", wall.x, wall.y),
                );
            }
        }

        recorder.finish(grid);
        recorder
    }

    fn name(&self) -> &'static str {
        "Randomized Kruskal"
    }

    fn description(&self) -> &'static str {
        "Uses union-find to randomly build a spanning tree, producing very uniform mazes."
    }
}
