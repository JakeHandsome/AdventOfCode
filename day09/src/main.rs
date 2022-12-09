use std::collections::HashSet;

use common::{read_input_file_for_project_as_string, R};

fn main() {
    let input = read_input_file_for_project_as_string!();
    println!("Part1: {:#?}", part1(&input).unwrap());
    println!("Part2: {:#?}", part2(&input).unwrap());
}

fn calculate_tail_movement(head: &(isize, isize), tail: &(isize, isize)) -> (isize, isize) {
    let mut tail = tail.to_owned();
    // If on the same column
    if head.1 == tail.1 {
        if head.0 - tail.0 == 2 {
            tail.0 += 1;
        }
        if -head.0 + tail.0 == 2 {
            tail.0 -= 1;
        }
    }
    // If on the same row
    else if head.0 == tail.0 {
        if head.1 - tail.1 == 2 {
            tail.1 += 1;
        }
        if -head.1 + tail.1 == 2 {
            tail.1 -= 1;
        }
    }
    // Differnt row and col
    else {
        // Determine if far enough away
        if (head.0 - tail.0).abs() + (head.1 - tail.1).abs() >= 3 {
            // Move diagonally
            if head.0 > tail.0 {
                tail.0 += 1;
            } else {
                tail.0 -= 1;
            }
            if head.1 > tail.1 {
                tail.1 += 1;
            } else {
                tail.1 -= 1;
            }
        }
    }
    tail
}

fn part1(input: &str) -> R<usize> {
    let mut tail_positions: HashSet<(isize, isize)> = HashSet::new();
    let mut head_position = (0isize, 0isize);
    let mut tail_position = (0isize, 0isize);
    let iter = input.lines();
    for command in iter {
        let split = command.split(' ').collect::<Vec<_>>();
        let direction = split.first().unwrap().to_owned();
        let count: usize = split.last().unwrap().parse()?;
        for _ in 0..count {
            // Move head
            match direction {
                "L" => head_position.0 -= 1,
                "R" => head_position.0 += 1,
                "U" => head_position.1 += 1,
                "D" => head_position.1 -= 1,
                _ => unreachable!("input should only be L R U D"),
            }
            // Move tail
            tail_position = calculate_tail_movement(&head_position, &tail_position);
            tail_positions.insert(tail_position);
        }
    }
    Ok(tail_positions.len())
}

fn part2(input: &str) -> R<usize> {
    let mut tail_positions: HashSet<(isize, isize)> = HashSet::new();
    let mut positions: Vec<(isize, isize)> = vec![(0, 0); 10];
    let iter = input.lines();
    for command in iter {
        let split = command.split(' ').collect::<Vec<_>>();
        let direction = split.first().unwrap().to_owned();
        let count: usize = split.last().unwrap().parse()?;
        // Move each piece in the direction count number of times
        for _ in 0..count {
            // Move head
            match direction {
                "L" => positions[0].0 -= 1,
                "R" => positions[0].0 += 1,
                "U" => positions[0].1 += 1,
                "D" => positions[0].1 -= 1,
                _ => unreachable!("input should only be L R U D"),
            }
            // Look at each knot besides the first and calculate its new positions based on the knot infront of it
            let number_of_segments = positions.len();
            for i in 1..number_of_segments {
                let segment_ahead = positions.get(i - 1).unwrap().to_owned();
                let current_segment = positions.get_mut(i).unwrap();
                // If on the same column
                *current_segment = calculate_tail_movement(&segment_ahead, current_segment);
                // If this is the last rope segment, save its position
                if i == number_of_segments - 1 {
                    tail_positions.insert(current_segment.to_owned());
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
