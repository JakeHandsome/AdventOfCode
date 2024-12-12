use std::collections::{BTreeMap, HashSet};

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
    let grid = Grid::new(input.to_string());
    let mapping = parse_input(&grid);
    let mut count = HashSet::new();
    for (_, points) in mapping {
        for i in points.into_iter().combinations(2) {
            if let [(x1, y1), (x2, y2)] = i[..2] {
                let (x1, y1, x2, y2) = (x1 as isize, y1 as isize, x2 as isize, y2 as isize);
                let x_diff = x2 - x1;
                let y_diff = y2 - y1;
                if let Some(x) = grid.index(x1 - x_diff, y1 - y_diff) {
                    count.insert(x);
                }
                if let Some(x) = grid.index(x2 + x_diff, y2 + y_diff) {
                    count.insert(x);
                }
            }
        }
    }
    Ok(count.len())
}

// Convert input into Map of (Key = char, Val = Arr[Points])
fn parse_input(grid: &Grid) -> BTreeMap<char, Vec<(usize, usize)>> {
    let mut mapping = BTreeMap::new();
    grid.inner
        .chars()
        .enumerate()
        .filter(|(_, x)| *x != '.')
        .for_each(|(i, c)| {
            if let std::collections::btree_map::Entry::Vacant(e) = mapping.entry(c) {
                e.insert(vec![grid.index_to_row_col(i)]);
            } else {
                mapping.get_mut(&c).unwrap().push(grid.index_to_row_col(i));
            }
        });
    mapping
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let grid = Grid::new(input.to_string());
    let mapping = parse_input(&grid);
    let mut count = HashSet::new();
    for (_, points) in mapping {
        for i in points.into_iter().combinations(2) {
            if let [(x1, y1), (x2, y2)] = i[..2] {
                let (mut x1, mut y1, mut x2, mut y2) = (x1 as isize, y1 as isize, x2 as isize, y2 as isize);
                let x_diff = x2 - x1;
                let y_diff = y2 - y1;
                while let Some(x) = grid.index(x1 - x_diff, y1 - y_diff) {
                    count.insert(x);
                    x1 -= x_diff;
                    y1 -= y_diff;
                }
                while let Some(x) = grid.index(x2 + x_diff, y2 + y_diff) {
                    count.insert(x);
                    x2 += x_diff;
                    y2 += y_diff;
                }
                // Took me so long to figure out the original points counted in part 2. Why
                count.insert(grid.index(x1, y1).unwrap());
                count.insert(grid.index(x2, y2).unwrap());
            }
        }
    }
    Ok(count.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 14);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 34);
    }
}
