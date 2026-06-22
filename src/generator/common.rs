use rand::seq::SliceRandom;
use rand::Rng;

use crate::core::{Cell, Grid, Point};

/// 随机打破额外的墙，为完美迷宫引入环路。
///
/// `loop_rate` 是 0.0 到 1.0 之间的值，表示要打破的合格墙的近似比例。
pub fn add_loops<R: Rng>(grid: &mut Grid, rng: &mut R, loop_rate: f64) -> usize {
    let mut candidates = Vec::new();

    // 查找所有分隔两个通道的墙（打破它们即可形成环路）
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

/// 从网格中随机选取一个通道单元格。
pub fn random_passage<R: Rng>(grid: &Grid, rng: &mut R) -> Option<Point> {
    let passages = grid.passages();
    passages.choose(rng).copied()
}

/// 计算入口（左上角通道）和出口（右下角通道）。
pub fn entrance_exit(grid: &Grid) -> (Point, Point) {
    let entrance = Point::new(1, 1);
    let exit = Point::new(grid.width() - 2, grid.height() - 2);
    (entrance, exit)
}
