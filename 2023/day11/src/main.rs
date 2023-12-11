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
    for galaxy in &mut galaxies {
        galaxy.row += empty_rows.iter().filter(|x| **x < galaxy.row).count() as isize;
        galaxy.col += empty_cols.iter().filter(|x| **x < galaxy.col).count() as isize;
    }
    Ok(galaxies
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| manhatten_distance(&a, &b))
        .sum())
}

fn part2(input: &str) -> anyhow::Result<isize> {
    let (mut galaxies, empty_rows, empty_cols) = parse_input(input);
    for galaxy in &mut galaxies {
        #[cfg(test)]
        const MULTIPLE: isize = 10 - 1; //Idk why -1 is needed but it works
        #[cfg(not(test))]
        const MULTIPLE: isize = 1_000_000 - 1; //Idk why -1 is needed but it works
        galaxy.row += MULTIPLE * empty_rows.iter().filter(|x| **x < galaxy.row).count() as isize;
        galaxy.col += MULTIPLE * empty_cols.iter().filter(|x| **x < galaxy.col).count() as isize;
    }
    Ok(galaxies
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| manhatten_distance(&a, &b))
        .sum())
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
