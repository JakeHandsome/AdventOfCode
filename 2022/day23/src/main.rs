use common::*;
use point::Point;

fn main() {
    let input = read_input_file_for_project_as_string!();
    {
        let _timer = Timer::new("Part 1");
        println!("Part1: {}", part1(&input).unwrap());
    }
    {
        let _timer = Timer::new("Part 2");
        println!("Part2: {}", part2(&input).unwrap());
    }
}

mod point {
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
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

struct Elf {
    // Where this elf is
    location: Point,
    // How this elf wants to move
    proposal: Option<Point>,
}

impl Elf {
    fn new(x: isize, y: isize) -> Self {
        Self {
            location: Point { x, y },
            proposal: None,
        }
    }
    /// Move this elf to the desired position if it is not none
    fn move_location(&mut self) {
        if let Some(new_loc) = self.proposal {
            self.location = new_loc;
        }
    }

    fn propose_movement(&self, elf_locations: Vec<Point>, index: usize) -> Option<Point> {
        use Direction::*;
        let directions = [North, South, West, East];

        // Check the directions in order, based on the index (round number)
        // Ex: Round1 N S W E
        //     Round2 S W E N
        let direction1 = directions[index % 4];
        let direction2 = directions[(index + 1) % 4];
        let direction3 = directions[(index + 2) % 4];
        let direction4 = directions[(index + 3) % 4];

        // Check if is isolated
        if Point::get_adjacent_offsets()
            .iter()
            .all(|p| !elf_locations.contains(&(*p + self.location)))
        {
            return None;
        }
        let mut direction = None;
        if Point::get_offset_for_direction(direction1)
            .iter()
            .all(|p| !elf_locations.contains(&(*p + self.location)))
        {
            direction = Some(direction1);
        } else if Point::get_offset_for_direction(direction2)
            .iter()
            .all(|p| !elf_locations.contains(&(*p + self.location)))
        {
            direction = Some(direction2);
        } else if Point::get_offset_for_direction(direction3)
            .iter()
            .all(|p| !elf_locations.contains(&(*p + self.location)))
        {
            direction = Some(direction3);
        } else if Point::get_offset_for_direction(direction4)
            .iter()
            .all(|p| !elf_locations.contains(&(*p + self.location)))
        {
            direction = Some(direction4);
        }
        match direction {
            Some(North) => Some(self.location + Point::new(0, 1)),
            Some(South) => Some(self.location + Point::new(0, -1)),
            Some(East) => Some(self.location + Point::new(1, 0)),
            Some(West) => Some(self.location + Point::new(-1, 0)),
            None => None,
        }
    }
}

fn part1(input: &str) -> R<usize> {
    let mut elves = vec![];
    for (y, line) in input.lines().rev().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                elves.push(Elf::new(x.try_into().unwrap(), y.try_into().unwrap()));
            }
        }
    }
    let num_rounds = 10;
    for round in 0..num_rounds {
        let elf_positions = elves.iter().map(|x| x.location).collect::<Vec<_>>();
        let mut new_locations = vec![];
        // Round 1
        for elf in elves.iter_mut() {
            let proposal = elf.propose_movement(elf_positions.clone(), round);
            elf.proposal = proposal;
            if let Some(move_point) = proposal {
                new_locations.push(move_point);
            }
        }
        // Round 2
        for elf in elves.iter_mut() {
            if let Some(proposal) = elf.proposal {
                // Make sure this is the only elf with this proposal
                if new_locations.iter().filter(|f| **f == proposal).count() == 1 {
                    // The elf gets to move
                    elf.move_location();
                }
                elf.proposal = None;
            }
        }
    }
    let xs = elves.iter().map(|elf| elf.location.x);
    let ys = elves.iter().map(|elf| elf.location.y);

    let max_x = xs.clone().max().unwrap();
    let min_x = xs.min().unwrap();
    let max_y = ys.clone().max().unwrap();
    let min_y = ys.min().unwrap();

    let area = (1 + max_x - min_x) * (1 + max_y - min_y);

    Ok(area as usize - elves.len())
}

fn part2(input: &str) -> R<usize> {
    let mut elves = vec![];
    for (y, line) in input.lines().rev().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                elves.push(Elf::new(x.try_into().unwrap(), y.try_into().unwrap()));
            }
        }
    }
    let mut round = 0;
    loop {
        let elf_positions = elves.iter().map(|x| x.location).collect::<Vec<_>>();
        let mut new_locations = vec![];
        // Round 1
        for elf in elves.iter_mut() {
            let proposal = elf.propose_movement(elf_positions.clone(), round);
            elf.proposal = proposal;
            if let Some(move_point) = proposal {
                new_locations.push(move_point);
            }
        }
        // Round 2
        if new_locations.len() == 0 {
            break Ok(round + 1);
        }
        for elf in elves.iter_mut() {
            if let Some(proposal) = elf.proposal {
                // Make sure this is the only elf with this proposal
                if new_locations.iter().filter(|f| **f == proposal).count() == 1 {
                    // The elf gets to move
                    elf.move_location();
                }
                elf.proposal = None;
            }
        }
        round += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#.."#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 110);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 20);
    }
    #[test]
    fn small_example() {
        assert_eq!(
            part1(
                r#".....
..##.
..#..
.....
..##.
....."#
            )
            .unwrap(),
            25
        )
    }
    #[test]
    fn elf_isolated() {
        let elf = Elf::new(0, 0);
        let locations = vec![Point::new(0, 0)];

        assert_eq!(elf.propose_movement(locations, 0), None);
    }

    #[test]
    fn elf_move_north() {
        let elf = Elf::new(0, 0);
        let locations = vec![Point::new(0, 0), Point::new(1, 0)];

        assert_eq!(elf.propose_movement(locations, 0), Some(Point::NORTH));
    }

    #[test]
    fn elf_move_south() {
        let elf = Elf::new(0, 0);
        let locations = vec![Point::new(0, 0), Point::new(0, 1)];
        assert_eq!(elf.propose_movement(locations, 0), Some(Point::SOUTH));
        let locations = vec![Point::new(0, 0), Point::new(-1, 1)];
        assert_eq!(elf.propose_movement(locations, 0), Some(Point::SOUTH));
        let locations = vec![Point::new(0, 0), Point::new(1, 1)];
        assert_eq!(elf.propose_movement(locations, 0), Some(Point::SOUTH));
    }

    #[test]
    fn elf_move_west() {
        let elf = Elf::new(0, 0);
        let locations = vec![Point::new(0, 0), Point::new(0, 1), Point::new(0, -1)];
        assert_eq!(elf.propose_movement(locations, 0), Some(Point::WEST));
        let locations = vec![Point::new(0, 0), Point::new(0, 1), Point::new(1, -1)];
        assert_eq!(elf.propose_movement(locations, 0), Some(Point::WEST));
    }
    #[test]
    fn elf_move_east() {
        let elf = Elf::new(0, 0);
        let locations = vec![Point::new(0, 0), Point::new(0, 1), Point::new(0, -1), Point::new(-1, 0)];

        assert_eq!(elf.propose_movement(locations, 0), Some(Point::EAST));
    }
}
