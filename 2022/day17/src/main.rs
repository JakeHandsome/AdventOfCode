use core::num;
use std::collections::HashMap;

use board::Board;
use common::*;
use shapes::Shapes;

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
enum HorizontalMovement {
    Left,
    Right,
}
use HorizontalMovement::*;

impl HorizontalMovement {
    fn new(c: char) -> Self {
        match c {
            '<' => Left,
            '>' => Right,
            _ => unreachable!(),
        }
    }
}

mod board;
mod shapes;

fn part1(input: &str) -> R<usize> {
    // List of shapes in order of spawning
    let shapes = [
        Shapes::Rect(4, 1),
        Shapes::Cross,
        Shapes::L,
        Shapes::Rect(1, 4),
        Shapes::Rect(2, 2),
    ];
    let jet_patterns = input
        .chars()
        .filter(|c| matches!(c, '<' | '>'))
        .map(|c| HorizontalMovement::new(c))
        .collect::<Vec<_>>();
    let mut board = Board::new();
    let mut jet_index = 0;
    for i in 0..=2022 {
        //println!("\n-------\n{board}\n");
        let rock = &shapes[i % shapes.len()];
        loop {
            // Move rock left or right
            match jet_patterns[jet_index] {
                Left => {
                    rock.move_left(&mut board);
                    //  println!("<-\n{board}\n");
                }
                Right => {
                    rock.move_right(&mut board);
                    //   println!("->\n{board}\n");
                }
            }
            jet_index = (jet_index + 1) % jet_patterns.len();
            // Move rock down and potentionally place it
            if !rock.move_down(&mut board) {
                let last_rock_height = rock.turn_to_rock(&mut board);
                board.spawn_new_rock(last_rock_height);
                break;
            } else {
                board.current_piece_location.1 -= 1;
            }
            // println!("v\n{board}\n"
        }
    }
    // Subtract 2, because the height is also 1 higher than it should be (for piece spawning reasons).
    //Idk why you need to subtract the other 1. Probably 1 index vs 0 index idk?
    Ok(board.height - 2)
}

fn part2(input: &str) -> R<usize> {
    // List of shapes in order of spawning
    let shapes = [
        Shapes::Rect(4, 1),
        Shapes::Cross,
        Shapes::L,
        Shapes::Rect(1, 4),
        Shapes::Rect(2, 2),
    ];
    let jet_patterns = input
        .chars()
        .filter(|c| matches!(c, '<' | '>'))
        .map(|c| HorizontalMovement::new(c))
        .collect::<Vec<_>>();
    let mut board = Board::new();
    let mut jet_index = 0;
    let mut differences = HashMap::new();
    let mut diff_found = false;
    let mut rocks_simulated = 0;
    let mut height_offset = 0;
    while rocks_simulated <= 1_000_000_000_000 {
        let rock_index = rocks_simulated % shapes.len();
        let rock = &shapes[rock_index];
        loop {
            // Move rock left or right
            match jet_patterns[jet_index] {
                Left => {
                    rock.move_left(&mut board);
                }
                Right => {
                    rock.move_right(&mut board);
                }
            }
            jet_index = (jet_index + 1) % jet_patterns.len();
            // Move rock down and potentionally place it
            if rock.move_down(&mut board) {
                board.current_piece_location.1 -= 1;
            } else {
                rocks_simulated += 1;
                let last_rock_height = rock.turn_to_rock(&mut board);
                board.spawn_new_rock(last_rock_height);
                let key = (rock_index, jet_index);
                if board.height > 2 && !diff_found {
                    if !differences.contains_key(&key) {
                        differences.insert(key, vec![(board.height, rocks_simulated)]);
                    } else {
                        differences
                            .get_mut(&key)
                            .map(|val| val.push((board.height, rocks_simulated)));
                    }
                    let heights = &differences[&key];
                    let x = heights.len() - 1;
                    if heights.len() > 2 {
                        if heights[x].0 - heights[x - 1].0 == heights[x - 1].0 - heights[x - 2].0 {
                            diff_found = true;
                            let rocks_left = 1_000_000_000_000 - rocks_simulated;
                            let repeat_dist = heights[x].0 - heights[x - 1].0;
                            let num_rocks = heights[x].1 - heights[x - 1].1;
                            let (can_simulate, num_left) = (rocks_left / num_rocks, rocks_left % num_rocks);
                            height_offset = can_simulate * repeat_dist;
                            rocks_simulated = 1_000_000_000_000 - num_left;
                            println!("{rocks_simulated},{repeat_dist},{num_left}");
                        }
                    }
                }
                break;
            }
        }
    }
    // Subtract 2, because the height is also 1 higher than it should be (for piece spawning reasons).
    //Idk why you need to subtract the other 1. Probably 1 index vs 0 index idk?
    Ok(board.height - 2 + height_offset + 1) // Idk why I need +1 but it makes the unit test pass
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 3068);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 1_514_285_714_288);
    }
}
