use std::ops::Add;

use super::Direction;

/// 迷宫网格中的一个点。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    /// 创建一个新的点。
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// 沿指定方向移动指定距离。若越界则返回 None。
    pub fn moved(&self, dir: Direction, dist: usize) -> Option<Self> {
        match dir {
            Direction::North => self.y.checked_sub(dist).map(|y| Self::new(self.x, y)),
            Direction::South => Some(Self::new(self.x, self.y + dist)),
            Direction::West => self.x.checked_sub(dist).map(|x| Self::new(x, self.y)),
            Direction::East => Some(Self::new(self.x + dist, self.y)),
        }
    }

    /// 沿指定方向移动 1 格（获取邻居点）。
    pub fn neighbor(&self, dir: Direction) -> Option<Self> {
        self.moved(dir, 1)
    }
}

impl Add<Direction> for Point {
    type Output = Option<Point>;

    fn add(self, rhs: Direction) -> Self::Output {
        self.neighbor(rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_new() {
        let p = Point::new(3, 5);
        assert_eq!(p.x, 3);
        assert_eq!(p.y, 5);
    }

    #[test]
    fn test_point_moved() {
        let p = Point::new(2, 2);
        assert_eq!(p.moved(Direction::North, 1), Some(Point::new(2, 1)));
        assert_eq!(p.moved(Direction::South, 1), Some(Point::new(2, 3)));
        assert_eq!(p.moved(Direction::West, 1), Some(Point::new(1, 2)));
        assert_eq!(p.moved(Direction::East, 1), Some(Point::new(3, 2)));
        assert_eq!(p.moved(Direction::North, 3), None); // 下溢
    }
}
