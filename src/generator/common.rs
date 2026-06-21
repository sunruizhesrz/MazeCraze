use rand::seq::SliceRandom;
use rand::Rng;

use crate::core::{Cell, Grid, Point};

/// Randomly break additional walls to introduce loops into a perfect maze.
///
/// `loop_rate` is a value between 0.0 and 1.0 representing the approximate
/// fraction of eligible walls to break.
pub fn add_loops<R: Rng>(grid: &mut Grid, rng: &mut R, loop_rate: f64) -> usize {
    let mut candidates = Vec::new();

    // Find all walls that separate two passages (breaking them creates a loop)
    for y in (2..grid.height() - 1).step_by(2) {
        for x in 1..grid.width() - 1 {
            let p = Point::new(x, y);
            if matches!(grid.get(p), Some(Cell::Wall)) {
                let above = Point::new(x, y - 1);
                let below = Point::new(x, y + 1);
                if is_passage_pair(grid, above, below) {
                    candidates.push(p);
                }
            }
        }
    }

    for x in (2..grid.width() - 1).step_by(2) {
        for y in 1..grid.height() - 1 {
            let p = Point::new(x, y);
            if matches!(grid.get(p), Some(Cell::Wall)) {
                let left = Point::new(x - 1, y);
                let right = Point::new(x + 1, y);
                if is_passage_pair(grid, left, right) {
                    candidates.push(p);
                }
            }
        }
    }

    let to_break = (candidates.len() as f64 * loop_rate).ceil() as usize;
    let mut count = 0;

    use rand::seq::IteratorRandom;
    for wall in candidates.iter().copied().choose_multiple(rng, to_break) {
        if grid.set(wall, Cell::Passage).is_ok() {
            count += 1;
        }
    }

    count
}

fn is_passage_pair(grid: &Grid, a: Point, b: Point) -> bool {
    matches!(grid.get(a), Some(Cell::Passage)) && matches!(grid.get(b), Some(Cell::Passage))
}

/// Pick a random passage cell from the grid.
pub fn random_passage<R: Rng>(grid: &Grid, rng: &mut R) -> Option<Point> {
    let passages = grid.passages();
    passages.choose(rng).copied()
}

/// Compute the entrance (top-left passage) and exit (bottom-right passage).
pub fn entrance_exit(grid: &Grid) -> (Point, Point) {
    let entrance = Point::new(1, 1);
    let exit = Point::new(grid.width() - 2, grid.height() - 2);
    (entrance, exit)
}
