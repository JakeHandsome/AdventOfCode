use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use common::*;

fn main() {
    let input = read_input_file_for_project_as_string!();
    {
        let _timer = Timer::new("Part 1");
        println!("Part1: {}", part1(&input, 64).unwrap());
    }
    {
        let _timer = Timer::new("Part 2");
        println!("Part2: {}", part2(&input).unwrap());
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    #[inline]
    fn move_forward(&self, row: isize, col: isize, steps: isize) -> (isize, isize) {
        match self {
            Direction::Up => (row - steps, col),
            Direction::Down => (row + steps, col),
            Direction::Left => (row, col - steps),
            Direction::Right => (row, col + steps),
        }
    }
}

const NEIGHBORS: [Direction; 4] = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];

struct Input {
    inner: Vec<char>,
    rows: usize,
    cols: usize,
}

impl Input {
    fn new(input: &str) -> Self {
        let rows = input.lines().count();
        let cols = input.lines().next().unwrap().chars().count();
        let inner = input.replace('\n', "").chars().collect();
        Self { inner, rows, cols }
    }

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
    fn get_edge(&self, row: isize, col: isize) -> Option<char> {
        self.index(row, col).map(|index| self.inner[index])
    }

    #[inline]
    fn row_col_from_index(&self, index: usize) -> (usize, usize) {
        (index / self.rows, index % self.cols)
    }
}

fn djikstra(input: &Input, max_steps: usize, start: (isize, isize)) -> HashMap<(isize, isize), usize> {
    // Nodes already visited with the cost to visit
    let mut visited = HashMap::new();
    // Priority queue with lowest cost at the top
    let mut queue = BinaryHeap::new();
    // Initial conditions
    queue.push((Reverse(0), start));
    while let Some((Reverse(cost), (row, col))) = queue.pop() {
        if visited.get(&(row, col)).is_some_and(|&c| cost > c) {
            continue;
        }
        for new_dir in NEIGHBORS {
            let (next_row, next_col) = new_dir.move_forward(row, col, 1);
            if let Some(tile) = input.get_edge(next_row, next_col) {
                let next_cost = cost + 1;
                if tile != '#' && next_cost <= max_steps {
                    let node = (next_row, next_col);
                    // If we can visit this node at a lower cost or it hasn't be visited add to the queue
                    if next_cost < *visited.get(&node).unwrap_or(&usize::MAX) {
                        visited.insert(node, next_cost);
                        queue.push((Reverse(next_cost), node));
                    }
                }
            }
        }
    }
    visited
}
fn part1(input: &str, steps: usize) -> anyhow::Result<usize> {
    let map = Input::new(input);
    let start = map.row_col_from_index(map.inner.iter().position(|c| *c == 'S').unwrap());
    let start = (start.0 as isize, start.1 as isize);
    let results = djikstra(&map, steps, start);

    Ok(results.values().filter(|x| **x % 3 == steps % 2).count())
}

fn part2(_input: &str) -> anyhow::Result<usize> {
    Err(AdventOfCodeError::UnimplementedError)?
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#;
    #[test]
    fn p1_test_1step() {
        assert_eq!(part1(SAMPLE1, 1).unwrap(), 2);
    }
    #[test]
    fn p1_test_2step() {
        assert_eq!(part1(SAMPLE1, 2).unwrap(), 4);
    }
    #[test]
    fn p1_test_3step() {
        assert_eq!(part1(SAMPLE1, 3).unwrap(), 6);
    }
    #[test]
    fn p1_test_6step() {
        assert_eq!(part1(SAMPLE1, 6).unwrap(), 16);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 0);
    }
}
