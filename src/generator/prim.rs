use rand::seq::IteratorRandom;

use crate::animation::AnimationRecorder;
use crate::core::{Cell, Point};

use super::{init_grid, MazeGenerator};

/// Randomized Prim's algorithm maze generator.
///
/// This algorithm grows the maze outward from the starting cell by randomly
/// selecting a frontier wall and carving it. It produces short corridors and
/// a more "organic" maze structure with many dead ends.
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

        let mut frontier: Vec<(Point, Point)> = Vec::new(); // (wall, target)

        // Add initial frontier walls
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
