use std::collections::{HashMap, HashSet};

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

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct MirrorMaze {
    inner: String,
    rows: usize,
    cols: usize,
}

impl MirrorMaze {
    fn new(input: &str) -> Self {
        let rows = input.lines().count();
        let cols = input.lines().next().unwrap().chars().count();
        let inner = input.replace('\n', "");
        Self { inner, rows, cols }
    }
}

impl MirrorMaze {
    #[inline]
    fn index(&self, row: isize, col: isize) -> Option<usize> {
        if row < 0 || col < 0 || row >= self.rows as isize || col >= self.cols as isize {
            None
        } else {
            let index = (row * self.cols as isize + col) as usize;
            debug_assert!(index < self.inner.len(), "{},r{},c{}", index, row, col);
            Some(index)
        }
    }

    fn index_to_row_col(&self, index: usize) -> (usize, usize) {
        (index / self.cols, index % self.cols)
    }

    #[inline]
    fn get_char(&self, row: isize, col: isize) -> Option<char> {
        self.index(row as isize, col as isize)
            .map(|index| self.inner.as_bytes()[index] as char)
    }
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let maze = MirrorMaze::new(input);
    let mut map = HashSet::new();
    walk_maze(&maze, 0, 0, Direction::Right, &mut map);
    let unique_locations = map.into_iter().map(|(r, c, _)| (r, c)).collect::<HashSet<_>>();

    Ok(unique_locations.len())
}

fn walk_maze(maze: &MirrorMaze, row: isize, col: isize, dir: Direction, map: &mut HashSet<(isize, isize, Direction)>) {
    if map.contains(&(row, col, dir)) {
        // Already been here
        return;
    }
    if let Some(x) = maze.get_char(row, col) {
        map.insert((row, col, dir));
        match (x, dir) {
            // Keep going same direction
            ('.', _) | ('-', Direction::Left | Direction::Right) | ('|', Direction::Up | Direction::Down) => {
                let (next_row, next_col) = match dir {
                    Direction::Up => (row - 1, col),
                    Direction::Down => (row + 1, col),
                    Direction::Left => (row, col - 1),
                    Direction::Right => (row, col + 1),
                };
                walk_maze(maze, next_row, next_col, dir, map)
            }
            // Splitters
            ('-', Direction::Up | Direction::Down) => {
                walk_maze(maze, row, col + 1, Direction::Right, map);
                walk_maze(maze, row, col - 1, Direction::Left, map);
            }
            ('|', Direction::Left | Direction::Right) => {
                walk_maze(maze, row + 1, col, Direction::Down, map);
                walk_maze(maze, row - 1, col, Direction::Up, map);
            }
            // Reflections
            ('/', Direction::Up) => walk_maze(maze, row, col + 1, Direction::Right, map),
            ('/', Direction::Down) => walk_maze(maze, row, col - 1, Direction::Left, map),
            ('/', Direction::Left) => walk_maze(maze, row + 1, col, Direction::Down, map),
            ('/', Direction::Right) => walk_maze(maze, row - 1, col, Direction::Up, map),
            ('\\', Direction::Up) => walk_maze(maze, row, col - 1, Direction::Left, map),
            ('\\', Direction::Down) => walk_maze(maze, row, col + 1, Direction::Right, map),
            ('\\', Direction::Left) => walk_maze(maze, row - 1, col, Direction::Up, map),
            ('\\', Direction::Right) => walk_maze(maze, row + 1, col, Direction::Down, map),
            _ => unreachable!(),
        }
    }
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let maze = MirrorMaze::new(input);
    let mut starting_conditions = vec![];
    for col in 0..maze.cols as isize {
        starting_conditions.push((0, col, Direction::Down));
        starting_conditions.push((maze.rows as isize - 1, col, Direction::Up));
    }
    for row in 0..maze.rows as isize {
        starting_conditions.push((row, 0, Direction::Right));
        starting_conditions.push((row, maze.cols as isize - 1, Direction::Left));
    }
    let sol = starting_conditions
        .into_par_iter()
        .map(|(row, col, dir)| {
            let mut map = HashSet::new();
            walk_maze(&maze, row, col, dir, &mut map);
            map.into_iter().map(|(r, c, _)| (r, c)).collect::<HashSet<_>>().len()
        })
        .max()
        .unwrap();

    Ok(sol)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 46);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 51);
    }
}
