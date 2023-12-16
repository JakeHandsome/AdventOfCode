use std::collections::HashSet;

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

    #[inline]
    fn get_char(&self, row: isize, col: isize) -> Option<char> {
        self.index(row, col).map(|index| self.inner.as_bytes()[index] as char)
    }
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let maze = MirrorMaze::new(input);
    let mut map = HashSet::new();
    walk_maze(&maze, 0, 0, Direction::Right, &mut map);
    // Convert the hashset from row,col,direction to just row,col This will be the total number of unique tiles
    Ok(map.into_iter().map(|(r, c, _)| (r, c)).collect::<HashSet<_>>().len())
}

// Walk the maze recursively stopping if this node and direction was already hit
fn walk_maze(maze: &MirrorMaze, row: isize, col: isize, dir: Direction, map: &mut HashSet<(isize, isize, Direction)>) {
    if map.contains(&(row, col, dir)) {
        // Already been here
        return;
    }
    if let Some(char) = maze.get_char(row, col) {
        // Mark that we have been here
        map.insert((row, col, dir));
        // This match statement covers all possible reflections
        match (char, dir) {
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
    // Determine all possible starting possitions and run maze for each of them in parallel finding
    // the max result
    let mut starting_conditions = vec![];
    for col in 0..maze.cols as isize {
        // Start at top row moving down for each column
        starting_conditions.push((0, col, Direction::Down));
        // Bottom row each column moving up
        starting_conditions.push((maze.rows as isize - 1, col, Direction::Up));
    }
    for row in 0..maze.rows as isize {
        // Left most column for reach row moving right
        starting_conditions.push((row, 0, Direction::Right));
        starting_conditions.push((row, maze.cols as isize - 1, Direction::Left));
    }
    Ok(starting_conditions
        // Parallel not really required heres, changes execution from 250ms to 50ms. I assumed it
        // would be harder to brute force
        .into_par_iter()
        .map(|(row, col, dir)| {
            let mut map = HashSet::new();
            walk_maze(&maze, row, col, dir, &mut map);
            // Convert the hashset from row,col,direction to just row,col This will be the total number of unique tiles
            map.into_iter().map(|(r, c, _)| (r, c)).collect::<HashSet<_>>().len()
        })
        .max()
        .unwrap())
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
