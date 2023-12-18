use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

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

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Any,
}

impl Direction {
    #[inline]
    fn move_forward(&self, row: isize, col: isize, steps: isize) -> (isize, isize) {
        match self {
            Direction::Up => (row - steps, col),
            Direction::Down => (row + steps, col),
            Direction::Left => (row, col - steps),
            Direction::Right => (row, col + steps),
            Direction::Any => unreachable!(),
        }
    }
    #[inline]
    fn neighbors(&self) -> Vec<Self> {
        match self {
            Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
            Direction::Left | Direction::Right => vec![Direction::Up, Direction::Down],
            Direction::Any => vec![Direction::Left, Direction::Right, Direction::Up, Direction::Down],
        }
    }
}

struct Input {
    inner: Vec<usize>,
    rows: usize,
    cols: usize,
}

impl Input {
    fn new(input: &str) -> Self {
        let rows = input.lines().count();
        let cols = input.lines().next().unwrap().chars().count();
        let inner = input
            .replace('\n', "")
            .chars()
            .map(|x| x.to_digit(10).unwrap() as usize)
            .collect();
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
    fn get_edge(&self, row: isize, col: isize) -> Option<usize> {
        self.index(row, col).map(|index| self.inner[index])
    }
}

/// https://www.redblobgames.com/pathfinding/a-star/introduction.html
fn djikstra(input: &Input) -> usize {
    // Nodes already visited with the cost to visit
    let mut visited = HashMap::new();
    // Priority queue with lowest cost at the top
    let mut queue = BinaryHeap::new();
    // Initial conditions
    queue.push((Reverse(0), (0, 0, Direction::Any)));
    while let Some((Reverse(cost), (row, col, dir))) = queue.pop() {
        if (row, col) == (input.rows - 1, input.cols - 1) {
            return cost;
        }
        if visited.get(&(row, col, dir)).is_some_and(|&c| cost > c) {
            continue;
        }
        // For each neigbhor add connections for 1-3 next nodes.
        for new_dir in dir.neighbors() {
            let mut next_cost = cost;
            for steps in 1..=3 {
                let (next_row, next_col) = new_dir.move_forward(row as isize, col as isize, steps);
                if let Some(cost) = input.get_edge(next_row, next_col) {
                    next_cost += cost;
                    let node = (next_row as usize, next_col as usize, new_dir);
                    // If we can visit this node at a lower cost or it hasn't be visited add to the queue
                    if next_cost < *visited.get(&node).unwrap_or(&usize::MAX) {
                        visited.insert(node, next_cost);
                        queue.push((Reverse(next_cost), node));
                    }
                }
            }
        }
    }
    unreachable!()
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let input = Input::new(input);
    Ok(djikstra(&input))
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let input = Input::new(input);
    Ok(djikstra2(&input))
}
fn djikstra2(input: &Input) -> usize {
    // Nodes already visited with the cost to visit
    let mut visited = HashMap::new();
    // Priority queue with lowest cost at the top
    let mut queue = BinaryHeap::new();
    // Initial conditions
    queue.push((Reverse(0), (0, 0, Direction::Any)));
    while let Some((Reverse(cost), (row, col, dir))) = queue.pop() {
        if (row, col) == (input.rows - 1, input.cols - 1) {
            return cost;
        }
        if visited.get(&(row, col, dir)).is_some_and(|&c| cost > c) {
            continue;
        }
        // For each neigbhor add connections for 1-3 next nodes.
        for new_dir in dir.neighbors() {
            let mut next_cost = cost;
            for steps in 1..=10 {
                let (next_row, next_col) = new_dir.move_forward(row as isize, col as isize, steps);
                if let Some(cost) = input.get_edge(next_row, next_col) {
                    next_cost += cost;
                    let node = (next_row as usize, next_col as usize, new_dir);
                    if steps < 4 {
                        // Dont add this node until 4 steps happened
                        continue;
                    }
                    // If we can visit this node at a lower cost or it hasn't be visited add to the queue
                    if next_cost < *visited.get(&node).unwrap_or(&usize::MAX) {
                        visited.insert(node, next_cost);
                        queue.push((Reverse(next_cost), node));
                    }
                }
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 102);
    }
    const SAMPLE2: &str = r#"111111111111
999999999991
999999999991
999999999991
999999999991"#;
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 94);
        assert_eq!(part2(SAMPLE2).unwrap(), 71);
    }
}
