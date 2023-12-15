use std::{collections::VecDeque, fmt::Debug};

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

struct Input {
    inner: Vec<char>,
    rows: usize,
    cols: usize,
}

impl Debug for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for col in 0..self.cols {
            for row in 0..self.rows {
                write!(f, "{}", self.char_at(self.index(row, col)))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Input {
    fn new(input: &str) -> Self {
        let rows = input.lines().count();
        let cols = input.lines().next().unwrap().chars().count();
        let inner = input.replace('\n', "");
        let inner = inner.chars().collect_vec();
        Self { inner, rows, cols }
    }
}

impl Input {
    fn index(&self, x: usize, y: usize) -> usize {
        y * self.cols + x
    }

    fn index_to_row_col(&self, index: usize) -> (usize, usize) {
        (index / self.cols, index % self.cols)
    }

    fn char_at(&self, index: usize) -> char {
        self.inner[index]
    }

    fn shift_rocks_north(&mut self) {
        // Used to swap indexes around
        for row in 0..self.rows {
            let mut pivots = VecDeque::new();
            for col in 0..self.cols {
                self.swap_positions(row, col, &mut pivots);
            }
        }
    }
    fn shift_rocks_south(&mut self) {
        // Used to swap indexes around
        for row in 0..self.rows {
            let mut pivots = VecDeque::new();
            for col in (0..self.cols).rev() {
                self.swap_positions(row, col, &mut pivots);
            }
        }
    }
    fn shift_rocks_west(&mut self) {
        // Used to swap indexes around
        for col in 0..self.cols {
            let mut pivots = VecDeque::new();
            for row in 0..self.rows {
                self.swap_positions(row, col, &mut pivots);
            }
        }
    }
    fn shift_rocks_east(&mut self) {
        // Used to swap indexes around
        for col in 0..self.cols {
            let mut pivots = VecDeque::new();
            for row in (0..self.rows).rev() {
                self.swap_positions(row, col, &mut pivots);
            }
        }
    }

    fn swap_positions(&mut self, row: usize, col: usize, pivots: &mut VecDeque<usize>) {
        let index = self.index(row, col);
        match self.char_at(index) {
            // If a . is found add this to the pivot queue
            '.' => pivots.push_back(index),
            // If a '#' is found, clear the queue
            '#' => pivots.clear(),
            // If 'O' was found and the queue is not empty, swap the locations with the first pivot
            'O' => {
                if let Some(pivot) = pivots.pop_front() {
                    self.inner.swap(index, pivot);
                    pivots.push_back(index);
                }
            }

            _ => {}
        }
    }

    fn calc_result(&self) -> usize {
        self.inner
            .iter()
            .positions(|x| *x == 'O')
            .map(|x| self.rows - self.index_to_row_col(x).0)
            .sum()
    }
    fn spin_cycle(&mut self) {
        self.shift_rocks_north();
        self.shift_rocks_west();
        self.shift_rocks_south();
        self.shift_rocks_east();
    }
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let mut input = Input::new(input);
    input.shift_rocks_north();
    Ok(input.calc_result())
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let mut input = Input::new(input);
    // Assume a cycle forms in 1000 iterations
    let mut results = vec![];
    for _i in 0..1000 {
        input.spin_cycle();
        results.push(input.inner.clone());
        #[cfg(test)]
        if let 0..=2 = _i {
            println!("After {} cycles", _i + 1);
            println!("{input:?}");
        }
    }

    // Find how long the cycle is
    let mut cycle_len = 0;
    let last_result = &results[results.len() - 1];
    for (count, res) in results.iter().rev().enumerate() {
        cycle_len = dbg!(count);
        if res == last_result && count > 0 {
            break;
        }
    }
    let cycle_start = results.len() - cycle_len;
    // Do more iterations until a multiple of the end is found
    for _ in cycle_start..((1_000_000_000 - cycle_start) % cycle_len) {
        input.spin_cycle();
    }

    Ok(input.calc_result())
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 136);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 64);
    }
}
