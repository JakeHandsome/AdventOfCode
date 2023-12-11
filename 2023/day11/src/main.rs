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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: isize,
    col: isize,
}

impl Point {
    fn from_index(index: usize, width: isize) -> Point {
        Point {
            row: (index as isize / width),
            col: (index as isize % width),
        }
    }
}

fn manhatten_distance(a: &Point, b: &Point) -> isize {
    (a.row - b.row).abs() + (a.col - b.col).abs()
}

fn part1(input: &str) -> anyhow::Result<isize> {
    let (mut galaxies, empty_rows, empty_cols) = parse_input(input);
    expand_space(&mut galaxies, empty_rows, empty_cols, 2);
    Ok(galaxies
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| manhatten_distance(&a, &b))
        .sum())
}

fn part2(input: &str) -> anyhow::Result<isize> {
    let (mut galaxies, empty_rows, empty_cols) = parse_input(input);
    #[cfg(test)]
    const MULTIPLE: isize = 10;
    #[cfg(not(test))]
    const MULTIPLE: isize = 1_000_000;
    expand_space(&mut galaxies, empty_rows, empty_cols, MULTIPLE);
    Ok(galaxies
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| manhatten_distance(&a, &b))
        .sum())
}

fn expand_space(galaxies: &mut Vec<Point>, empty_rows: Vec<isize>, empty_cols: Vec<isize>, empty_space_size: isize) {
    let multiple = empty_space_size - 1; // Each empty space is already 1 so increase it by size -1
    for galaxy in galaxies {
        galaxy.row += multiple * empty_rows.iter().filter(|x| **x < galaxy.row).count() as isize;
        galaxy.col += multiple * empty_cols.iter().filter(|x| **x < galaxy.col).count() as isize;
    }
}

fn parse_input(input: &str) -> (Vec<Point>, Vec<isize>, Vec<isize>) {
    let max_col = input.lines().next().unwrap().len() as isize;
    let max_row = input.lines().count() as isize;
    let flat_input = input.replace('\n', "");
    let galaxies = flat_input
        .match_indices('#')
        .map(|(i, _)| Point::from_index(i, max_col))
        .collect::<Vec<Point>>();
    let populated_rows = galaxies.iter().map(|p| p.row).collect::<BTreeSet<_>>();
    let populated_col = galaxies.iter().map(|p| p.col).collect::<BTreeSet<_>>();
    let empty_rows = (0..max_row).filter(|r| !populated_rows.contains(r)).collect_vec();
    let empty_cols = (0..max_col).filter(|c| !populated_col.contains(c)).collect_vec();
    (galaxies, empty_rows, empty_cols)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 374);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 1030);
    }
}
