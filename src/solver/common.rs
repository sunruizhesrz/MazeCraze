use std::collections::HashMap;

use crate::animation::AnimationRecorder;
use crate::core::{Cell, Direction, Grid, Point};

/// Reconstruct the solution path from the `came_from` map and record each step.
pub fn reconstruct_path(
    working: &mut Grid,
    came_from: &HashMap<Point, Option<(Point, Direction)>>,
    end: Point,
    recorder: &mut AnimationRecorder,
) {
    let mut current = end;
    while let Some(Some((prev, _))) = came_from.get(&current) {
        let _ = working.set(current, Cell::Path);
        recorder.record(working, "Reconstructing solution path");
        current = *prev;
    }
    let _ = working.set(current, Cell::Path);
    recorder.record(working, "Path complete");
}

/// Manhattan distance heuristic for A*.
pub fn manhattan_distance(a: Point, b: Point) -> usize {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}
