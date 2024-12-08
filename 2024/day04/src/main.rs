use std::isize;

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

fn part1(input: &str) -> anyhow::Result<usize> {
    let mut result = 0;
    let mut array_2d = vec![];
    for line in input.lines() {
        // Get horizontal lines easily
        result += line.match_indices("XMAS").count();
        result += line.match_indices("SAMX").count();
        let chars = line.chars().collect_vec();
        array_2d.push(chars);
    }
    // Remove mutability
    let array_2d = array_2d;
    // Rotate array by transpose + reverse each row
    let mut rotate = array_2d.clone();
    (0..array_2d.len()).for_each(|x| {
        (0..array_2d[0].len()).for_each(|y| {
            rotate[x][y] = array_2d[y][x];
        });
    });
    rotate.iter_mut().for_each(|x| x.reverse());

    // Check for "vertical" items
    for row in &rotate {
        let a: String = row.iter().collect();
        //println!("{a}");
        result += a.match_indices("XMAS").count();
        result += a.match_indices("SAMX").count();
    }
    // Diagonal ???
    let mut diagonals = vec![];
    for arr in [array_2d, rotate] {
        (0..arr.len()).rev().for_each(|x| {
            diagonals.push(get_diagonal(&arr, x, 0));
        });
        (1..arr[0].len()).for_each(|y| {
            diagonals.push(get_diagonal(&arr, 0, y));
        });
    }
    for row in &diagonals {
        result += row.match_indices("XMAS").count();
        result += row.match_indices("SAMX").count();
    }

    Ok(result)
}

fn get_diagonal(arr: &[Vec<char>], mut x: usize, mut y: usize) -> String {
    let mut s = String::new();
    loop {
        if let Some(row) = arr.get(x) {
            if let Some(col) = row.get(y) {
                s.push(*col);
            } else {
                return s;
            }
        } else {
            return s;
        }
        x += 1;
        y += 1;
    }
}

#[derive(Debug, Clone)]
struct Crossword {
    inner: String,
    pub rows: usize,
    pub cols: usize,
}

impl Crossword {
    fn new(input: String) -> Self {
        let cols = input.lines().next().unwrap().len();
        let rows = input.lines().count();
        let inner = input.replace('\n', "");
        Self { inner, rows, cols }
    }
}

impl Crossword {
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
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let mut result = 0;
    let crossword = Crossword::new(input.to_string());
    (0..crossword.rows).for_each(|x| {
        (0..crossword.cols).for_each(|y| {
            if let Some('A') = crossword.get_char(x, y) {
                let left_up = crossword.get_char(x.wrapping_sub(1), y.wrapping_sub(1));
                let left_down = crossword.get_char(x.wrapping_sub(1), y + 1);
                let right_up = crossword.get_char(x + 1, y.wrapping_sub(1));
                let right_doon = crossword.get_char(x + 1, y + 1);
                if let (Some(lu), Some(ld), Some(ru), Some(rd)) = (left_up, left_down, right_up, right_doon) {
                    if ((lu == 'M' && rd == 'S') || (lu == 'S' && rd == 'M'))
                        && ((ru == 'M' && ld == 'S') || (ru == 'S' && ld == 'M'))
                    {
                        result += 1;
                    }
                }
            }
        });
    });
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 18);
    }
    const _SAMPLE1_CLEANED: &str = r#".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
.........."#;
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 9);
    }
}
