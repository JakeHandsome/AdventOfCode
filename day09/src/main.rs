use std::{collections::HashSet, hash::Hash};

use common::{read_input_file_for_project_as_string, AdventOfCodeError, R};

fn main() {
    let input = read_input_file_for_project_as_string!();
    println!("Part1: {:#?}", part1(&input).unwrap());
    println!("Part2: {:#?}", part2(&input).unwrap());
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn char_to_direction(c: &str) -> Direction {
    use Direction::*;
    match c {
        "R" => Right,
        "L" => Left,
        "U" => Up,
        "D" => Down,
        _ => todo!(),
    }
}

fn part1(input: &str) -> R<usize> {
    let mut tail_positions: HashSet<(isize, isize)> = HashSet::new();
    let mut head_position = (0isize, 0isize);
    let mut tail_position = (0isize, 0isize);
    let mut iter = input.lines().into_iter();
    while let Some(command) = iter.next() {
        let split = command.split(" ").collect::<Vec<_>>();
        let direction = char_to_direction(split.first().unwrap());
        let count: usize = split.last().unwrap().parse()?;
        for _ in 0..count {
            // Move head
            match direction {
                Direction::Left => head_position.0 = head_position.0 - 1,
                Direction::Right => head_position.0 = head_position.0 + 1,
                Direction::Up => head_position.1 = head_position.1 + 1,
                Direction::Down => head_position.1 = head_position.1 - 1,
            }
            // Move tail
            // If on the same column
            if head_position.1 == tail_position.1 {
                if head_position.0 - tail_position.0 == 2 {
                    tail_position.0 += 1;
                }
                if -head_position.0 + tail_position.0 == 2 {
                    tail_position.0 -= 1;
                }
            }
            // If on the same row
            else if head_position.0 == tail_position.0 {
                if head_position.1 - tail_position.1 == 2 {
                    tail_position.1 += 1;
                }
                if -head_position.1 + tail_position.1 == 2 {
                    tail_position.1 -= 1;
                }
            }
            // Differnt row and col
            else {
                // Determine if far enough away
                if (head_position.0 - tail_position.0).abs() + (head_position.1 - tail_position.1).abs() == 3 {
                    // Move diagonally
                    if head_position.0 > tail_position.0 {
                        tail_position.0 += 1;
                    } else {
                        tail_position.0 -= 1;
                    }
                    if head_position.1 > tail_position.1 {
                        tail_position.1 += 1;
                    } else {
                        tail_position.1 -= 1;
                    }
                }
            }
            tail_positions.insert(tail_position);
        }
    }
    Ok(tail_positions.len())
}

fn part2(input: &str) -> R<usize> {
    let mut tail_positions: HashSet<(isize, isize)> = HashSet::new();
    let mut positions: Vec<(isize, isize)> = vec![(0, 0); 10];
    let mut iter = input.lines().into_iter();
    while let Some(command) = iter.next() {
        let split = command.split(" ").collect::<Vec<_>>();
        let direction = char_to_direction(split.first().unwrap());
        let count: usize = split.last().unwrap().parse()?;
        for _ in 0..count {
            // Move head
            match direction {
                Direction::Left => positions[0].0 = positions[0].0 - 1,
                Direction::Right => positions[0].0 = positions[0].0 + 1,
                Direction::Up => positions[0].1 = positions[0].1 + 1,
                Direction::Down => positions[0].1 = positions[0].1 - 1,
            }
            // Move tail
            for i in 1..positions.len() {
                let head = positions.get(i - 1).unwrap().to_owned();
                let mut tail_position = positions.get_mut(i).unwrap();
                // If on the same column
                if head.1 == tail_position.1 {
                    if head.0 - tail_position.0 == 2 {
                        tail_position.0 += 1;
                    }
                    if -head.0 + tail_position.0 == 2 {
                        tail_position.0 -= 1;
                    }
                }
                // If on the same row
                else if head.0 == tail_position.0 {
                    if head.1 - tail_position.1 == 2 {
                        tail_position.1 += 1;
                    }
                    if -head.1 + tail_position.1 == 2 {
                        tail_position.1 -= 1;
                    }
                }
                // Differnt row and col
                else {
                    // Determine if far enough away
                    if (head.0 - tail_position.0).abs() + (head.1 - tail_position.1).abs() >= 3 {
                        // Move diagonally
                        if head.0 > tail_position.0 {
                            tail_position.0 += 1;
                        } else {
                            tail_position.0 -= 1;
                        }
                        if head.1 > tail_position.1 {
                            tail_position.1 += 1;
                        } else {
                            tail_position.1 -= 1;
                        }
                    }
                }
                if i == 10 - 1 {
                    tail_positions.insert(tail_position.to_owned());
                }
            }
        }
    }

    Ok(tail_positions.len())
}
#[cfg(test)]
mod day9 {
    use super::*;
    const SAMPLE1: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;
    const SAMPLE2: &str = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 13);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 1);
        assert_eq!(part2(SAMPLE2).unwrap(), 36);
    }
}
