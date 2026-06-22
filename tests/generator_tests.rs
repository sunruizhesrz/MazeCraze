use std::collections::VecDeque;

use mazecraze::core::{Cell, Grid, Point};
use mazecraze::generator::MazeGenerator;
use mazecraze::generator::{RandomizedKruskal, RandomizedPrim, RecursiveBacktracker};

/// 使用 BFS 检查所有通道单元格是否都能从入口到达。
fn is_fully_connected(grid: &Grid) -> bool {
    let mut visited = vec![vec![false; grid.width()]; grid.height()];
    let mut queue = VecDeque::new();
    let start = Point::new(1, 1);

    queue.push_back(start);
    visited[start.y][start.x] = true;

    while let Some(current) = queue.pop_front() {
        for (next, _) in grid.passage_neighbors(current) {
            if grid.is_connected(current, next) && !visited[next.y][next.x] {
                visited[next.y][next.x] = true;
                queue.push_back(next);
            }
        }
    }

    // 统计可达通道数
    let reachable = grid.passages().iter().filter(|p| visited[p.y][p.x]).count();
    reachable == grid.passages().len()
}

/// 检查迷宫是否为完美迷宫（无环路）。
/// 完美迷宫满足：
/// - 所有通道单元格均存在（通道数 == 单元格数）
/// - 所有通道相互连通
/// - 已开凿的墙数 == 单元格数 - 1（生成树性质，无环路）
fn is_perfect_maze(grid: &Grid) -> bool {
    let cells = ((grid.width() - 1) / 2) * ((grid.height() - 1) / 2);
    let passages = grid.passages().len();

    // 统计已开凿的墙（偶数 y、奇数 x 处的水平墙；奇数 y、偶数 x 处的垂直墙）
    let mut carved_walls = 0usize;
    for y in (2..grid.height() - 1).step_by(2) {
        for x in 1..grid.width() - 1 {
            if matches!(grid.get(Point::new(x, y)), Some(Cell::Passage)) {
                carved_walls += 1;
            }
        }
    }
    for x in (2..grid.width() - 1).step_by(2) {
        for y in 1..grid.height() - 1 {
            if matches!(grid.get(Point::new(x, y)), Some(Cell::Passage)) {
                carved_walls += 1;
            }
        }
    }

    passages == cells && carved_walls == cells - 1 && is_fully_connected(grid)
}

#[test]
fn test_backtracker_generates_valid_maze() {
    let gen = RecursiveBacktracker::new();
    let recorder = gen.generate(21, 11);
    let final_grid = &recorder.frames().last().unwrap().grid;

    assert!(is_fully_connected(final_grid));
    assert!(is_perfect_maze(final_grid));
}

#[test]
fn test_prim_generates_valid_maze() {
    let gen = RandomizedPrim::new();
    let recorder = gen.generate(21, 11);
    let final_grid = &recorder.frames().last().unwrap().grid;

    assert!(is_fully_connected(final_grid));
    assert!(is_perfect_maze(final_grid));
}

#[test]
fn test_kruskal_generates_valid_maze() {
    let gen = RandomizedKruskal::new();
    let recorder = gen.generate(21, 11);
    let final_grid = &recorder.frames().last().unwrap().grid;

    assert!(is_fully_connected(final_grid));
    assert!(is_perfect_maze(final_grid));
}

#[test]
fn test_generators_produce_different_mazes() {
    // 基础检查：用两种不同算法运行两次，应当
    // 生成不同的墙体结构（概率上几乎必然不同）。
    use mazecraze::renderer::{AsciiRenderer, Renderer};

    let backtracker = RecursiveBacktracker::new();
    let prim = RandomizedPrim::new();

    let bt_recorder = backtracker.generate(15, 15);
    let prim_recorder = prim.generate(15, 15);

    let bt_grid = &bt_recorder.frames().last().unwrap().grid;
    let prim_grid = &prim_recorder.frames().last().unwrap().grid;

    let renderer = AsciiRenderer::new();
    let bt_rendered = renderer.render(bt_grid);
    let prim_rendered = renderer.render(prim_grid);

    // 它们应当在结构上不同（不雷同）
    assert_ne!(
        bt_rendered, prim_rendered,
        "Two different algorithms produced identical mazes (unlikely but possible)"
    );
}
