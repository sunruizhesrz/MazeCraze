use mazecraze::core::{grid::GridError, Cell, Direction, Grid, Point};

#[test]
fn test_grid_creation_and_dimensions() {
    let grid = Grid::new(5, 7).unwrap();
    assert_eq!(grid.width(), 5);
    assert_eq!(grid.height(), 7);
}

#[test]
fn test_invalid_dimensions() {
    assert_eq!(
        Grid::new(4, 5).unwrap_err(),
        GridError::InvalidDimensions(4, 5)
    );
    assert_eq!(
        Grid::new(5, 2).unwrap_err(),
        GridError::InvalidDimensions(5, 2)
    );
    assert_eq!(
        Grid::new(1, 1).unwrap_err(),
        GridError::InvalidDimensions(1, 1)
    );
}

#[test]
fn test_get_and_set() {
    let mut grid = Grid::new(5, 5).unwrap();
    let p = Point::new(1, 1);

    assert!(matches!(grid.get(p), Some(Cell::Wall)));
    grid.set(p, Cell::Passage).unwrap();
    assert!(matches!(grid.get(p), Some(Cell::Passage)));
}

#[test]
fn test_out_of_bounds() {
    let mut grid = Grid::new(5, 5).unwrap();
    assert_eq!(
        grid.set(Point::new(10, 10), Cell::Passage).unwrap_err(),
        GridError::OutOfBounds(10, 10)
    );
}

#[test]
fn test_passage_neighbors() {
    let mut grid = Grid::new(7, 7).unwrap();
    let p = Point::new(3, 3);
    grid.set(p, Cell::Passage).unwrap();

    let neighbors = grid.passage_neighbors(p);
    assert_eq!(neighbors.len(), 4);

    let expected = vec![
        (Point::new(3, 1), Direction::North),
        (Point::new(3, 5), Direction::South),
        (Point::new(1, 3), Direction::West),
        (Point::new(5, 3), Direction::East),
    ];
    for exp in &expected {
        assert!(neighbors.contains(exp), "Missing neighbor: {:?}", exp);
    }
}

#[test]
fn test_carve_passage() {
    let mut grid = Grid::new(5, 5).unwrap();
    let start = Point::new(1, 1);
    grid.set(start, Cell::Passage).unwrap();

    let end = grid.carve_passage(start, Direction::East).unwrap();
    assert_eq!(end, Point::new(3, 1));
    assert!(matches!(grid.get(Point::new(2, 1)), Some(Cell::Passage)));
    assert!(matches!(grid.get(end), Some(Cell::Passage)));
}

#[test]
fn test_reset_to_walls() {
    let mut grid = Grid::new(5, 5).unwrap();
    grid.set(Point::new(1, 1), Cell::Passage).unwrap();
    grid.reset_to_walls();
    assert!(matches!(grid.get(Point::new(1, 1)), Some(Cell::Wall)));
}

#[test]
fn test_clear_markers() {
    let mut grid = Grid::new(5, 5).unwrap();
    grid.set(Point::new(1, 1), Cell::Visited).unwrap();
    grid.set(Point::new(1, 3), Cell::Path).unwrap();
    grid.clear_markers();

    assert!(matches!(grid.get(Point::new(1, 1)), Some(Cell::Passage)));
    assert!(matches!(grid.get(Point::new(1, 3)), Some(Cell::Passage)));
}
