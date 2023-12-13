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

#[derive(Debug)]
struct Mirror {
    inner: String,
    rows: usize,
    cols: usize,
}

impl Mirror {
    fn new(input: String) -> Self {
        let cols = input.lines().next().unwrap().len();
        let rows = input.lines().count();
        let inner = input.replace('\n', "");
        Self { inner, rows, cols }
    }
}

impl Mirror {
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
    fn get_char(&self, row: usize, col: usize) -> Option<char> {
        self.index(row as isize, col as isize)
            .map(|index| self.inner.as_bytes()[index] as char)
    }

    fn find_reflection(&self) -> usize {
        for row in (0..self.rows).collect_vec().windows(2) {
            if self.check_row(row[0], row[1]) {
                return row[1] * 100;
            }
        }
        for col in (0..self.cols).collect_vec().windows(2) {
            if self.check_col(col[0], col[1]) {
                return col[1];
            }
        }
        unreachable!();
    }

    fn check_row(&self, row1: usize, row2: usize) -> bool {
        for (r1, r2) in (0..=row1).rev().zip(row2..self.rows) {
            for col in 0..self.cols {
                match (self.get_char(r1, col), self.get_char(r2, col)) {
                    (Some(a), Some(b)) => {
                        if a != b {
                            return false;
                        }
                    }
                    (None, None) => return dbg!(true),
                    _ => return dbg!(true),
                }
            }
        }
        true
    }
    fn check_col(&self, col1: usize, col2: usize) -> bool {
        for (c1, c2) in (0..=col1).rev().zip(col2..self.cols) {
            for row in 0..self.rows {
                match (self.get_char(row, c1), self.get_char(row, c2)) {
                    (Some(a), Some(b)) => {
                        if a != b {
                            return false;
                        }
                    }
                    (None, None) => return dbg!(true),
                    _ => {
                        dbg!(self);
                        return true;
                    }
                }
            }
        }
        true
    }
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let mut mirrors = vec![];
    let mut current_mirror = Some(String::new());
    for line in input.lines() {
        if line.is_empty() {
            mirrors.push(Mirror::new(current_mirror.take().unwrap()));
            current_mirror = Some(String::new());
        } else if let Some(mirror) = &mut current_mirror {
            mirror.push_str(line);
            mirror.push('\n');
        } else {
            unreachable!();
        }
    }
    mirrors.push(Mirror::new(current_mirror.take().unwrap()));
    Ok(mirrors.into_iter().map(|x| x.find_reflection()).sum())
}

fn part2(input: &str) -> anyhow::Result<usize> {
    Err(AdventOfCodeError::UnimplementedError)?
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 405);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 0);
    }
}
