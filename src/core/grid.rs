use thiserror::Error;

use super::{Direction, Point};

/// 迷宫网格中的单元格。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    /// 墙壁单元格。
    Wall,
    /// 可通行单元格。
    Passage,
    /// 求解过程中已访问的单元格。
    Visited,
    /// 最终求解路径的一部分。
    Path,
    /// 当前活动单元格（用于动画高亮显示）。
    Current,
}

/// 网格操作过程中可能出现的错误。
#[derive(Error, Debug, PartialEq)]
pub enum GridError {
    #[error("无效的网格尺寸：宽度 ({0}) 和高度 ({1}) 必须为奇数且 >= 3")]
    InvalidDimensions(usize, usize),
    #[error("点 ({0}, {1}) 超出边界")]
    OutOfBounds(usize, usize),
    #[error("点 ({0}, {1}) 不是有效的单元格坐标（必须为奇数索引）")]
    InvalidCellCoordinate(usize, usize),
}

/// 迷宫网格。
///
/// 网格采用"墙-通道"编码方式：
/// - 奇数索引 (1, 3, 5...) 为潜在的通道
/// - 偶数索引 (0, 2, 4...) 为墙壁
///
///   这种编码方式可保证网格拥有自然的边界，且墙体厚度一致。
#[derive(Clone, Debug, PartialEq)]
pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    /// 创建一个全部由墙壁填充的新网格。
    ///
    /// 宽度和高度必须为奇数且至少为 3。
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

    /// 网格宽度。
    pub fn width(&self) -> usize {
        self.width
    }

    /// 网格高度。
    pub fn height(&self) -> usize {
        self.height
    }

    /// 获取指定点处的单元格。
    pub fn get(&self, p: Point) -> Option<&Cell> {
        self.cells.get(p.y)?.get(p.x)
    }

    /// 获取单元格的可变引用。
    pub fn get_mut(&mut self, p: Point) -> Option<&mut Cell> {
        self.cells.get_mut(p.y)?.get_mut(p.x)
    }

    /// 设置单元格的值。
    pub fn set(&mut self, p: Point, cell: Cell) -> Result<(), GridError> {
        let c = self.get_mut(p).ok_or(GridError::OutOfBounds(p.x, p.y))?;
        *c = cell;
        Ok(())
    }

    /// 检查点是否在边界内。
    pub fn contains(&self, p: Point) -> bool {
        p.x < self.width && p.y < self.height
    }

    /// 获取单元格的有效通道邻居（相距 2 格，中间有墙）。
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

    /// 获取两个通道单元格之间的单元格（即墙壁位置）。
    pub fn wall_between(&self, a: Point, dir: Direction) -> Option<Point> {
        a.moved(dir, 1).filter(|p| self.contains(*p))
    }

    /// 从 `from` 沿 `dir` 方向开凿通道，将中间的墙和目标单元格同时变为通道。
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

    /// 检查两个相邻通道单元格之间是否存在通道。
    pub fn is_connected(&self, a: Point, b: Point) -> bool {
        let dx = a.x.abs_diff(b.x);
        let dy = a.y.abs_diff(b.y);

        if dx + dy != 2 || (dx != 0 && dy != 0) {
            return false;
        }

        let mid = Point::new((a.x + b.x) / 2, (a.y + b.y) / 2);
        matches!(self.get(mid), Some(Cell::Passage))
    }

    /// 返回所有通道单元格的坐标。
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

    /// 将所有非墙单元格重置为墙（仅保留物理墙体）。
    pub fn reset_to_walls(&mut self) {
        for row in &mut self.cells {
            for cell in row {
                *cell = Cell::Wall;
            }
        }
    }

    /// 清除可视化标记（Visited、Path、Current），将它们还原为 Passage。
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
