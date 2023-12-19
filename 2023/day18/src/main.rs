use hashbrown::HashMap;
use hashbrown::HashSet;

use common::*;

fn main() {
    let input = read_input_file_for_project_as_string!();
    {
        let _timer = Timer::new("Part 1");
        println!("Part1 original: {}", part1(&input, false).unwrap());
    }
    {
        let _timer = Timer::new("Part 1");
        println!("Part1 new: {}", part1(&input, true).unwrap());
    }
    {
        let _timer = Timer::new("Part 2");
        println!("Part2: {}", part2(&input).unwrap());
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    row: isize,
    col: isize,
}

fn part1(input: &str, part2_algo: bool) -> anyhow::Result<usize> {
    let mut pool = HashSet::new();
    let mut points = vec![];
    let mut location = Point { row: 0, col: 0 };
    let mut perimiter = 0;
    for line in input.lines() {
        let mut split = line.split_whitespace();
        let dir = split.next().unwrap();
        let dist = split.next().unwrap().parse::<isize>()?;
        perimiter += dist as usize;
        match dir {
            "U" => {
                for row in (location.row - dist)..=location.row {
                    pool.insert((row, location.col));
                }
                location.row -= dist;
            }
            "D" => {
                for row in (location.row)..(location.row + dist) {
                    pool.insert((row, location.col));
                }
                location.row += dist;
            }
            "L" => {
                for col in (location.col - dist)..=location.col {
                    pool.insert((location.row, col));
                }
                location.col -= dist;
            }
            "R" => {
                for col in (location.col)..(location.col + dist) {
                    pool.insert((location.row, col));
                }
                location.col += dist;
            }
            _ => unreachable!(),
        }
        points.push(location);
    }
    if !part2_algo {
        solve(pool)
    } else {
        let area = shoelace(&points);
        // Pick's Theorem
        // Area = inner_points + (perimiter/2) -1)
        // Solution to this problem is I + P
        // A = I + (P/2) - 1
        // I = A - (P/2) + 1
        // I + P = A + (P/2) + 1
        Ok(area + perimiter / 2 + 1)
    }
}

// This was the fastest I could make thise solution but it is too slow for pt2
fn solve(pool: HashSet<(isize, isize)>) -> Result<usize, anyhow::Error> {
    let mut max = Point {
        row: isize::MIN,
        col: isize::MIN,
    };
    let mut min = Point {
        row: isize::MAX,
        col: isize::MAX,
    };
    for (row, col) in &pool {
        max.row = max.row.max(*row);
        min.row = min.row.min(*row);
        max.col = max.col.max(*col);
        min.col = min.col.min(*col);
    }
    // Translate pool into day 10 format
    let pool = pool
        .par_iter()
        .map(|(row, col)| {
            if pool.contains(&(*row + 1, *col)) && pool.contains(&(*row - 1, *col)) {
                ((*row, *col), '|')
            } else if pool.contains(&(*row, *col + 1)) && pool.contains(&(*row, *col - 1)) {
                ((*row, *col), '-')
            } else if pool.contains(&(*row + 1, *col)) && pool.contains(&(*row, *col + 1)) {
                ((*row, *col), 'F')
            } else if pool.contains(&(*row - 1, *col)) && pool.contains(&(*row, *col + 1)) {
                ((*row, *col), 'L')
            } else if pool.contains(&(*row + 1, *col)) && pool.contains(&(*row, *col - 1)) {
                ((*row, *col), '7')
            } else if pool.contains(&(*row - 1, *col)) && pool.contains(&(*row, *col - 1)) {
                ((*row, *col), 'J')
            } else {
                unreachable!();
            }
        })
        .collect::<HashMap<(isize, isize), char>>();

    // Reword a double for loop using iterators for par_iter. This function runs each row in
    // parallel, not fast enough for pt2
    let count = (min.row..=max.row)
        .into_par_iter()
        .fold(
            || 0,
            |count, r| {
                let mut inside = false;
                (min.col..=max.col).fold(count, |mut count, c| {
                    if let Some(x) = pool.get(&(r, c)) {
                        if let 'F' | '|' | '7' = x {
                            inside = !inside;
                        }
                        count += 1;
                    } else if inside {
                        count += 1;
                    }
                    count
                })
            },
        )
        .reduce(|| 0, |a, b| a + b);
    Ok(count)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let mut pool = Vec::new();
    let mut location = Point { row: 0, col: 0 };
    let mut perimiter = 0;
    for line in input.lines() {
        let split = line.split_whitespace();
        let color = split.last().unwrap();
        let color = &color[2..color.len() - 1];
        let dist = isize::from_str_radix(&color[..color.len() - 1], 16)?;
        perimiter += dist as usize;
        let dir = color
            .chars()
            .last()
            .map(|x| match x {
                '0' => "R",
                '1' => "D",
                '2' => "L",
                '3' => "U",
                _ => unreachable!(),
            })
            .unwrap();
        match dir {
            "U" => {
                location.row -= dist;
            }
            "D" => {
                location.row += dist;
            }
            "L" => {
                location.col -= dist;
            }
            "R" => {
                location.col += dist;
            }
            _ => unreachable!(),
        }
        pool.push(location);
    }
    let area = shoelace(&pool);
    // Pick's Theorem
    // Area = inner_points + (perimiter/2) -1)
    // Solution to this problem is I + P
    // A = I + (P/2) - 1
    // I = A - (P/2) + 1
    // I + P = A + (P/2) + 1
    Ok(area + perimiter / 2 + 1)
}

// https://en.wikipedia.org/wiki/Shoelace_formula
fn shoelace(points: &[Point]) -> usize {
    let mut sum = 0;
    for x in 0..points.len() - 1 {
        let y = x + 1;
        sum += points[x].row * points[y].col - points[y].row * points[x].col;
    }
    // Don't forget final point comparing last to first
    sum += points[points.len() - 1].row * points[0].col - points[0].row * points[points.len() - 1].col;
    (sum.abs() / 2) as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1, true).unwrap(), 62);
        assert_eq!(part1(SAMPLE1, false).unwrap(), 62);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 952408144115);
    }
}
