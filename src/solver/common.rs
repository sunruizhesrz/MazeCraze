use std::collections::HashMap;

use crate::animation::AnimationRecorder;
use crate::core::{Cell, Direction, Grid, Point};

/// 根据 `came_from` 映射重建求解路径，并记录每一步。
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

/// A* 算法使用的曼哈顿距离启发式函数。
pub fn manhattan_distance(a: Point, b: Point) -> usize {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}
