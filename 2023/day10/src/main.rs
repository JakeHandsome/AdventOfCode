/// This one is messy pt2 got me good
use std::collections::BTreeSet;

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

fn determine_new_direction(previous_dir: Direction, c: char) -> Option<Direction> {
    match (previous_dir, c) {
        (Direction::North, '|') => Some(Direction::North),
        (Direction::North, '7') => Some(Direction::West),
        (Direction::North, 'F') => Some(Direction::East),
        (Direction::East, '-') => Some(Direction::East),
        (Direction::East, 'J') => Some(Direction::North),
        (Direction::East, '7') => Some(Direction::South),
        (Direction::South, '|') => Some(Direction::South),
        (Direction::South, 'L') => Some(Direction::East),
        (Direction::South, 'J') => Some(Direction::West),
        (Direction::West, '-') => Some(Direction::West),
        (Direction::West, 'F') => Some(Direction::South),
        (Direction::West, 'L') => Some(Direction::North),

        _ => None,
    }
}

fn move_forward(dir: Direction, coord: (usize, usize)) -> (usize, usize) {
    let (row, col) = coord;
    match dir {
        Direction::North => {
            // Handle overflows
            if row == 0 {
                (row, col)
            } else {
                (row - 1, col)
            }
        }
        Direction::South => (row + 1, col),
        Direction::East => (row, col + 1),
        Direction::West => {
            if col == 0 {
                (row, col)
            } else {
                (row, col - 1)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn coord_from_index(index: usize, max_cols: usize) -> (usize, usize) {
    (index / max_cols, index % max_cols)
}

fn index_from_coord(row: usize, col: usize, max_cols: usize) -> usize {
    row * max_cols + col
}

trait Day10 {
    fn adjancent_coord(&self) -> [(usize, usize); 4];
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let max_rows = input.lines().count();
    let max_cols = input.lines().next().unwrap().len();
    let input = input.replace('\n', "");

    let start = coord_from_index(input.find('S').unwrap(), max_cols);
    let mut current_directions = vec![];
    for direction in [Direction::North, Direction::South, Direction::East, Direction::West] {
        let adjacent_spot = move_forward(direction, start);
        let adjacent_char =
            input.as_bytes()[index_from_coord(adjacent_spot.0, adjacent_spot.1, max_cols) as usize] as char;
        if (determine_new_direction(direction, adjacent_char)).is_some() {
            current_directions.push(direction);
        }
    }
    let mut current_positions = [start, start];
    let mut solution = 0;
    while solution == 0 || current_positions[0] != current_positions[1] {
        for i in 0..2 {
            current_positions[i] = move_forward(current_directions[i], current_positions[i]);
            let next_char = input.as_bytes()
                [index_from_coord(current_positions[i].0, current_positions[i].1, max_cols) as usize]
                as char;
            let new_direction = determine_new_direction(current_directions[i], next_char);
            if let Some(x) = new_direction {
                current_directions[i] = x;
            }
        }
        solution += 1
    }
    Ok(solution)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let max_cols = input.lines().next().unwrap().len();
    let input = input.replace('\n', "");

    let start = coord_from_index(input.find('S').unwrap(), max_cols);
    let mut current_directions = vec![];
    for direction in [Direction::North, Direction::South, Direction::East, Direction::West] {
        let adjacent_spot = move_forward(direction, start);
        let adjacent_char =
            input.as_bytes()[index_from_coord(adjacent_spot.0, adjacent_spot.1, max_cols) as usize] as char;
        if (determine_new_direction(direction, adjacent_char)).is_some() {
            current_directions.push(direction);
        }
    }
    let mut current_positions = [start, start];
    let mut loop_points = BTreeSet::new();
    while loop_points.is_empty() || current_positions[0] != current_positions[1] {
        for i in 0..2 {
            loop_points.insert(current_positions[i]);
            current_positions[i] = move_forward(current_directions[i], current_positions[i]);
            let next_char = input.as_bytes()
                [index_from_coord(current_positions[i].0, current_positions[i].1, max_cols) as usize]
                as char;
            let new_direction = determine_new_direction(current_directions[i], next_char);
            if let Some(x) = new_direction {
                current_directions[i] = x;
            }
        }
    }
    loop_points.insert(current_positions[0]);
    let mut inside_points = vec![];
    let mut inside = false;
    for (i, c) in input.chars().enumerate() {
        let (row, col) = coord_from_index(i, max_cols);
        if col == 0 {
            // Start of the row, we are not inside
            println!();
            inside = false;
        }
        // If this point is on the loop it is not inside it
        if loop_points.contains(&(row, col)) {
            print!("{c}");
            // If F | or 7 is found, invert the inside. I consider these "Moving Up" tiles. Should
            // also work for "Moving Down" tiles L | J
            // HACK: For my input 'S' is a '|'
            if let 'F' | '|' | '7' | 'S' = c {
                inside = !inside
            }
        } else if inside {
            // If we are inside the loop and this point is not part of the loop, it is inside
            print!("I");
            inside_points.push((row, col));
        } else {
            // Otherwise is is an outside point
            print!("O");
        }
    }
    println!();
    Ok(inside_points.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#".....
.S-7.
.|.|.
.L-J.
....."#;
    const SAMPLE2: &str = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 4);
        assert_eq!(part1(SAMPLE2).unwrap(), 8);
    }

    const SAMPLE3: &str = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#;
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE3).unwrap(), 10);
    }
}
