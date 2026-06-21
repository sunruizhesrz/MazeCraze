use thiserror::Error;

use super::{Direction, Point};

/// A cell in the maze grid.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    /// A wall cell.
    Wall,
    /// A passable cell.
    Passage,
    /// A cell that has been visited during solving.
    Visited,
    /// Part of the final solution path.
    Path,
    /// The current active cell (for animation highlighting).
    Current,
}

/// Errors that can occur when working with a grid.
#[derive(Error, Debug, PartialEq)]
pub enum GridError {
    #[error("invalid grid dimensions: width ({0}) and height ({1}) must be odd and >= 3")]
    InvalidDimensions(usize, usize),
    #[error("point ({0}, {1}) is out of bounds")]
    OutOfBounds(usize, usize),
    #[error("point ({0}, {1}) is not a valid cell coordinate (must be odd indices)")]
    InvalidCellCoordinate(usize, usize),
}

/// The maze grid.
///
/// The grid uses a "wall-and-passage" encoding where:
/// - Odd indices (1, 3, 5...) are potential passages
/// - Even indices (0, 2, 4...) are walls
///
///   This ensures a natural border and consistent wall thickness.
#[derive(Clone, Debug, PartialEq)]
pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    /// Create a new grid filled entirely with walls.
    ///
    /// Width and height must be odd and at least 3.
    pub fn new(width: usize, height: usize) -> Result<Self, GridError> {
        if width < 3 || height < 3 || width.is_multiple_of(2) || height.is_multiple_of(2) {
            return Err(GridError::InvalidDimensions(width, height));
        }

        let cells = vec![vec![Cell::Wall; width]; height];
        Ok(Self {
            width,
            height,
            cells,
        })
    }

    /// Width of the grid.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Height of the grid.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Get a cell at the given point.
    pub fn get(&self, p: Point) -> Option<&Cell> {
        self.cells.get(p.y)?.get(p.x)
    }

    /// Get a mutable reference to a cell.
    pub fn get_mut(&mut self, p: Point) -> Option<&mut Cell> {
        self.cells.get_mut(p.y)?.get_mut(p.x)
    }

    /// Set a cell value.
    pub fn set(&mut self, p: Point, cell: Cell) -> Result<(), GridError> {
        let c = self.get_mut(p).ok_or(GridError::OutOfBounds(p.x, p.y))?;
        *c = cell;
        Ok(())
    }

    /// Check if a point is within bounds.
    pub fn contains(&self, p: Point) -> bool {
        p.x < self.width && p.y < self.height
    }

    /// Get the valid passage neighbors of a cell (2 steps away, with wall in between).
    pub fn passage_neighbors(&self, p: Point) -> Vec<(Point, Direction)> {
        let mut neighbors = Vec::with_capacity(4);
        for dir in Direction::ALL {
            if let Some(neighbor) = p.moved(dir, 2) {
                if self.contains(neighbor) {
                    neighbors.push((neighbor, dir));
                }
            }
        }
        neighbors
    }

    /// Get the cell between two passage cells.
    pub fn wall_between(&self, a: Point, dir: Direction) -> Option<Point> {
        a.moved(dir, 1).filter(|p| self.contains(*p))
    }

    /// Carve a passage from `from` in direction `dir`, turning both the wall and target into passages.
    pub fn carve_passage(&mut self, from: Point, dir: Direction) -> Result<Point, GridError> {
        let wall = self
            .wall_between(from, dir)
            .ok_or(GridError::OutOfBounds(from.x, from.y))?;
        let to = from
            .moved(dir, 2)
            .ok_or(GridError::OutOfBounds(from.x, from.y))?;

        self.set(wall, Cell::Passage)?;
        self.set(to, Cell::Passage)?;
        Ok(to)
    }

    /// Check if a passage exists between two adjacent passage cells.
    pub fn is_connected(&self, a: Point, b: Point) -> bool {
        let dx = a.x.abs_diff(b.x);
        let dy = a.y.abs_diff(b.y);

        if dx + dy != 2 || (dx != 0 && dy != 0) {
            return false;
        }

        let mid = Point::new((a.x + b.x) / 2, (a.y + b.y) / 2);
        matches!(self.get(mid), Some(Cell::Passage))
    }

    /// Return all passage cell coordinates.
    pub fn passages(&self) -> Vec<Point> {
        let mut result = Vec::new();
        for y in (1..self.height).step_by(2) {
            for x in (1..self.width).step_by(2) {
                if matches!(self.get(Point::new(x, y)), Some(Cell::Passage)) {
                    result.push(Point::new(x, y));
                }
            }
        }
        result
    }

    /// Reset all non-wall cells to Wall (preserving only the physical walls).
    pub fn reset_to_walls(&mut self) {
        for row in &mut self.cells {
            for cell in row {
                *cell = Cell::Wall;
            }
        }
    }

    /// Clear visualization markers (Visited, Path, Current) back to Passage.
    pub fn clear_markers(&mut self) {
        for row in &mut self.cells {
            for cell in row.iter_mut() {
                if matches!(cell, Cell::Visited | Cell::Path | Cell::Current) {
                    *cell = Cell::Passage;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_creation() {
        let grid = Grid::new(5, 5).unwrap();
        assert_eq!(grid.width(), 5);
        assert_eq!(grid.height(), 5);
        assert!(matches!(grid.get(Point::new(0, 0)), Some(Cell::Wall)));
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
}
