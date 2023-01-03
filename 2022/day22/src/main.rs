use std::{iter::Map, vec};

use common::*;

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
#[derive(Debug)]
enum LR {
    L,
    R,
}

#[derive(Debug)]
enum Instruction {
    Move(u64),
    Turn(LR),
}

#[derive(Debug, Clone, Copy)]
enum Heading {
    Up = 3,
    Down = 1,
    Left = 2,
    Right = 0,
}

#[derive(Debug, Clone)]
struct Player {
    position: (usize, usize),
    heading: Heading,
}
impl Player {
    fn new(x: usize, y: usize) -> Self {
        Self {
            position: (x, y),
            heading: Heading::Right,
        }
    }

    // Gets the position in front of the player handling wrapping
    fn get_next_position(&self, map: &TileMap) -> (usize, usize) {
        let mut next = match self.heading {
            Heading::Up => (
                self.position.0,
                // If we try to subtract from 0, wrap to the bottom
                self.position.1.checked_sub(1).unwrap_or_else(|| map.height() - 1),
            ),
            Heading::Down => (self.position.0, self.position.1 + 1),
            Heading::Left => (
                self.position.0.checked_sub(1).unwrap_or_else(|| map.width - 1),
                self.position.1,
            ),
            Heading::Right => (self.position.0 + 1, self.position.1),
        };
        // Handle wrapping
        if next.0 >= map.width {
            next.0 = 0;
        }
        if next.1 >= map.height() {
            next.1 = 0;
        }
        next
    }

    fn move_forward(&mut self, num: u64, map: &TileMap) {
        for _ in 0..num {
            let mut ghost = self.clone();
            let next_position = loop {
                // Find the next position that isn't a nothing block
                let (x, y) = ghost.get_next_position(map);
                assert!(x < map.width);
                assert!(y < map.height());
                ghost.position = (x, y);
                if map.get(x, y) != MapTile::Nothing {
                    break (x, y);
                }
            };
            if map.get(next_position.0, next_position.1) == MapTile::Tile {
                self.position = next_position;
            } else {
                break;
            }
        }
    }

    fn turn(&mut self, dir: LR) {
        use Heading::*;
        self.heading = match (dir, self.heading) {
            (LR::L, Heading::Up) => Left,
            (LR::L, Heading::Down) => Right,
            (LR::L, Heading::Left) => Down,
            (LR::L, Heading::Right) => Up,
            (LR::R, Heading::Up) => Right,
            (LR::R, Heading::Down) => Left,
            (LR::R, Heading::Left) => Up,
            (LR::R, Heading::Right) => Down,
        }
    }
}

fn part1(input: &str) -> R<usize> {
    let mapstr = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|s| format!("{:}\n", s))
        .collect::<String>();
    println!("{:}", mapstr);
    let map = TileMap::new(mapstr);
    let instructions = parse_instructions(input.lines().last().unwrap())?;

    let (initial_x, initial_y) = map.index_to_x_y(map.tiles.iter().position(|x| *x == MapTile::Tile).unwrap());

    let mut player = Player::new(initial_x, initial_y);

    for instruction in instructions {
        match instruction {
            Instruction::Move(num) => player.move_forward(num, &map),
            Instruction::Turn(dir) => player.turn(dir),
        }
    }
    let final_col = player.position.0 + 1;
    let final_row = player.position.1 + 1;
    Ok(final_row * 1000 + final_col * 4 + player.heading as usize)
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum MapTile {
    Tile,
    Wall,
    Nothing,
}

struct TileMap {
    tiles: Vec<MapTile>,
    width: usize,
}

impl TileMap {
    fn new(mapstr: String) -> Self {
        let mut s = Self {
            tiles: vec![],
            width: 0,
        };
        s.width = mapstr.lines().map(|str| str.len()).max().unwrap();
        for line in mapstr.lines() {
            let mut count = 0;
            for char in line.chars() {
                match char {
                    '.' => s.tiles.push(MapTile::Tile),
                    '#' => s.tiles.push(MapTile::Wall),
                    ' ' => s.tiles.push(MapTile::Nothing),
                    _ => unreachable!(),
                }
                count += 1;
            }
            while count < s.width {
                s.tiles.push(MapTile::Nothing);
                count += 1;
            }
        }
        s
    }

    fn height(&self) -> usize {
        self.tiles.len() / self.width
    }

    fn get(&self, x: usize, y: usize) -> MapTile {
        self.tiles[y * self.width + x]
    }

    fn index_to_x_y(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }
}

fn part2(input: &str) -> R<usize> {
    Err(Box::new(AdventOfCodeError::new("Not implemented")))
}

fn parse_instructions(line: &str) -> R<Vec<Instruction>> {
    let mut instructions = vec![];
    let mut numbers = vec![];
    for char in line.chars() {
        if char.is_alphabetic() {
            // Convert the vec to a string and parse it to a number and reset vec
            instructions.push(Instruction::Move(numbers.iter().collect::<String>().parse()?));
            numbers.clear();
            // Add the L/R instruction to the queue
            instructions.push(Instruction::Turn(match char {
                'L' => LR::L,
                'R' => LR::R,
                _ => unreachable!("Should only be L and R"),
            }));
        } else {
            // collect the characters into a vec
            numbers.push(char);
        }
    }
    // If there are still numbers in the vec, add it to the end of the instructions list
    if !numbers.is_empty() {
        instructions.push(Instruction::Move(numbers.iter().collect::<String>().parse()?));
    }
    Ok(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 6032);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), todo!());
    }
}
