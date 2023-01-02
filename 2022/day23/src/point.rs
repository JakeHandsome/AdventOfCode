use crate::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    pub const NORTH_WEST: Self = Self { x: -1, y: 1 };
    pub const NORTH: Self = Self { x: 0, y: 1 };
    pub const NORTH_EAST: Self = Self { x: 1, y: 1 };
    pub const EAST: Self = Self { x: 1, y: 0 };
    pub const SOUTH_EAST: Self = Self { x: 1, y: -1 };
    pub const SOUTH: Self = Self { x: 0, y: -1 };
    pub const SOUTH_WEST: Self = Self { x: -1, y: -1 };
    pub const WEST: Self = Self { x: -1, y: 0 };
    pub const fn get_adjacent_offsets() -> [Self; 8] {
        [
            Self::NORTH_WEST,
            Self::NORTH,
            Self::NORTH_EAST,
            Self::EAST,
            Self::SOUTH_EAST,
            Self::SOUTH,
            Self::SOUTH_WEST,
            Self::WEST,
        ]
    }

    pub const fn get_offset_for_direction(dir: Direction) -> [Self; 3] {
        match dir {
            Direction::North => [Self::NORTH_WEST, Self::NORTH, Self::NORTH_EAST],
            Direction::South => [Self::SOUTH_WEST, Self::SOUTH, Self::SOUTH_EAST],
            Direction::East => [Self::NORTH_EAST, Self::EAST, Self::SOUTH_EAST],
            Direction::West => [Self::SOUTH_WEST, Self::WEST, Self::NORTH_WEST],
        }
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
