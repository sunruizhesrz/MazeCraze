use rand::Rng;

/// The four cardinal directions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    /// All four directions.
    pub const ALL: [Self; 4] = [Self::North, Self::South, Self::West, Self::East];

    /// Return the opposite direction.
    pub const fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::West => Self::East,
            Self::East => Self::West,
        }
    }

    /// Rotate 90 degrees clockwise.
    pub const fn clockwise(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    /// Rotate 90 degrees counter-clockwise.
    pub const fn counter_clockwise(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
            Self::East => Self::North,
        }
    }

    /// Return a random direction.
    pub fn random<R: Rng>(rng: &mut R) -> Self {
        match rng.gen_range(0..4) {
            0 => Self::North,
            1 => Self::South,
            2 => Self::West,
            _ => Self::East,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opposite() {
        assert_eq!(Direction::North.opposite(), Direction::South);
        assert_eq!(Direction::South.opposite(), Direction::North);
        assert_eq!(Direction::East.opposite(), Direction::West);
        assert_eq!(Direction::West.opposite(), Direction::East);
    }

    #[test]
    fn test_clockwise() {
        assert_eq!(Direction::North.clockwise(), Direction::East);
        assert_eq!(Direction::East.clockwise(), Direction::South);
        assert_eq!(Direction::South.clockwise(), Direction::West);
        assert_eq!(Direction::West.clockwise(), Direction::North);
    }
}
