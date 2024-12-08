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

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn walk_or_turn(
    grid: &Grid,
    current_position: (usize, usize),
    current_direction: Direction,
) -> Option<((usize, usize), Direction)> {
    match current_direction {
        Direction::Up => {
            let front = (current_position.0.wrapping_sub(1), current_position.1);
            grid.get_char(front.0, front.1).map(|char| {
                if char == '#' {
                    (current_position, Direction::Right)
                } else {
                    (front, current_direction)
                }
            })
        }
        Direction::Right => {
            let front = (current_position.0, current_position.1 + 1);
            grid.get_char(front.0, front.1).map(|char| {
                if char == '#' {
                    (current_position, Direction::Down)
                } else {
                    (front, current_direction)
                }
            })
        }
        Direction::Down => {
            let front = (current_position.0 + 1, current_position.1);
            grid.get_char(front.0, front.1).map(|char| {
                if char == '#' {
                    (current_position, Direction::Left)
                } else {
                    (front, current_direction)
                }
            })
        }
        Direction::Left => {
            let front = (current_position.0, current_position.1.wrapping_sub(1));
            grid.get_char(front.0, front.1).map(|char| {
                if char == '#' {
                    (current_position, Direction::Up)
                } else {
                    (front, current_direction)
                }
            })
        }
    }
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let grid = Grid::new(input.to_string());
    let start_index = grid.inner.chars().position(|x| x == '^').expect("Input should have ^");
    let mut current_position = grid.index_to_row_col(start_index);
    let mut current_direction = Direction::Up;
    let mut marked_tiles: BTreeSet<(usize, usize)> = BTreeSet::new();
    marked_tiles.insert(current_position);
    while let Some((position, direction)) = walk_or_turn(&grid, current_position, current_direction) {
        current_position = position;
        current_direction = direction;
        marked_tiles.insert(current_position);
    }

    Ok(marked_tiles.len())
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let grid = Grid::new(input.to_string());
    let start_index = grid.inner.chars().position(|x| x == '^').expect("Input should have ^");
    let loops = (0..grid.inner.len())
        .into_par_iter()
        .fold(
            || 0usize,
            |loops, i| {
                let obsticle_pos = grid.index_to_row_col(i);
                if let Some('.') = grid.get_char(obsticle_pos.0, obsticle_pos.1) {
                    let mut grid = grid.clone();
                    grid.inner.replace_range(i..=i, "#");

                    let mut current_position = grid.index_to_row_col(start_index);
                    let mut current_direction = Direction::Up;
                    let mut visited = BTreeSet::new();
                    visited.insert((current_position, current_direction));
                    while let Some((position, direction)) = walk_or_turn(&grid, current_position, current_direction) {
                        current_position = position;
                        current_direction = direction;
                        if visited.contains(&(current_position, current_direction)) {
                            return loops + 1;
                        }
                        visited.insert((current_position, current_direction));
                    }
                }
                loops
            },
        )
        .sum();

    Ok(loops)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 41);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 6);
    }
}
