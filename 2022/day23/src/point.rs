use crate::Direction;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
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
    pub fn get_adjacent_offsets() -> [Point; 8] {
        [
            Point::NORTH_WEST,
            Point::NORTH,
            Point::NORTH_EAST,
            Point::EAST,
            Point::SOUTH_EAST,
            Point::SOUTH,
            Point::SOUTH_WEST,
            Point::WEST,
        ]
    }

    pub fn get_offset_for_direction(dir: Direction) -> [Point; 3] {
        match dir {
            Direction::North => [Point::NORTH_WEST, Point::NORTH, Point::NORTH_EAST],
            Direction::South => [Point::SOUTH_WEST, Point::SOUTH, Point::SOUTH_EAST],
            Direction::East => [Point::NORTH_EAST, Point::EAST, Point::SOUTH_EAST],
            Direction::West => [Point::SOUTH_WEST, Point::WEST, Point::NORTH_WEST],
        }
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
