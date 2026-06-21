use mazecraze::core::{Cell, Point};
use mazecraze::generator::{MazeGenerator, RecursiveBacktracker};
use mazecraze::solver::{AStarSolver, BfsSolver, DfsSolver, MazeSolver, WallFollowerSolver};

/// Create a simple 5x5 maze for deterministic tests.
fn make_simple_maze() -> mazecraze::core::Grid {
    use mazecraze::core::Grid;
    let mut grid = Grid::new(5, 5).unwrap();
    // Create a simple path: (1,1) → (3,1) → (3,3)
    grid.set(Point::new(1, 1), Cell::Passage).unwrap();
    grid.set(Point::new(2, 1), Cell::Passage).unwrap();
    grid.set(Point::new(3, 1), Cell::Passage).unwrap();
    grid.set(Point::new(3, 2), Cell::Passage).unwrap();
    grid.set(Point::new(3, 3), Cell::Passage).unwrap();
    grid
}

/// Count Path cells in the entire grid.
fn count_path_cells(grid: &mazecraze::core::Grid) -> usize {
    (0..grid.height())
        .flat_map(|y| (0..grid.width()).map(move |x| Point::new(x, y)))
        .filter(|p| matches!(grid.get(*p), Some(Cell::Path)))
        .count()
}

#[allow(dead_code)]
fn is_valid_path(grid: &mazecraze::core::Grid, path: &[Point]) -> bool {
    if path.is_empty() {
        return false;
    }
    // Check start and end
    if path[0] != Point::new(1, 1) {
        return false;
    }
    if *path.last().unwrap() != Point::new(3, 3) {
        return false;
    }
    // Check consecutive points are adjacent and passable
    for window in path.windows(2) {
        let a = window[0];
        let b = window[1];
        let dx = a.x.abs_diff(b.x);
        let dy = a.y.abs_diff(b.y);
        if dx + dy != 1 {
            return false;
        }
        if !matches!(grid.get(b), Some(Cell::Passage | Cell::Path)) {
            return false;
        }
    }
    true
}

#[test]
fn test_bfs_finds_path() {
    let grid = make_simple_maze();
    let solver = BfsSolver::new();
    let recorder = solver.solve(&grid, Point::new(1, 1), Point::new(3, 3));

    let final_frame = recorder.frames().last().unwrap();
    let path_count = count_path_cells(&final_frame.grid);
    assert!(path_count > 0, "BFS should find a path");
}

#[test]
fn test_dfs_finds_path() {
    let grid = make_simple_maze();
    let solver = DfsSolver::new();
    let recorder = solver.solve(&grid, Point::new(1, 1), Point::new(3, 3));

    let final_frame = recorder.frames().last().unwrap();
    let path_count = count_path_cells(&final_frame.grid);
    assert!(path_count > 0, "DFS should find a path");
}

#[test]
fn test_astar_finds_shortest_path() {
    let grid = make_simple_maze();
    let solver = AStarSolver::new();
    let recorder = solver.solve(&grid, Point::new(1, 1), Point::new(3, 3));

    let final_frame = recorder.frames().last().unwrap();
    let path_count = count_path_cells(&final_frame.grid);

    // Shortest path in this simple maze is 5 cells: (1,1) (2,1) (3,1) (3,2) (3,3)
    assert_eq!(path_count, 5, "A* should find the shortest path");
}

#[test]
fn test_wall_follower_on_perfect_maze() {
    let gen = RecursiveBacktracker::new();
    let recorder = gen.generate(15, 15);
    let grid = recorder.frames().last().unwrap().grid.clone();

    let solver = WallFollowerSolver::new();
    let solve_recorder = solver.solve(&grid, Point::new(1, 1), Point::new(13, 13));

    let final_frame = solve_recorder.frames().last().unwrap();
    let path_count = count_path_cells(&final_frame.grid);
    assert!(path_count > 0, "Wall follower should solve a perfect maze");
}

#[test]
fn test_bfs_astar_same_path_length_on_perfect_maze() {
    let gen = RecursiveBacktracker::new();
    let recorder = gen.generate(21, 21);
    let grid = recorder.frames().last().unwrap().grid.clone();
    let start = Point::new(1, 1);
    let end = Point::new(19, 19);

    let bfs = BfsSolver::new();
    let astar = AStarSolver::new();

    let bfs_recorder = bfs.solve(&grid, start, end);
    let astar_recorder = astar.solve(&grid, start, end);

    let bfs_path_len = count_path_cells(&bfs_recorder.frames().last().unwrap().grid);
    let astar_path_len = count_path_cells(&astar_recorder.frames().last().unwrap().grid);

    assert_eq!(
        bfs_path_len, astar_path_len,
        "BFS and A* should find paths of the same length on an unweighted grid"
    );
}
