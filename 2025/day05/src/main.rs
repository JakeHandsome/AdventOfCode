use std::{cmp::Ordering, collections::HashSet, ops::RangeInclusive};

use common::{anyhow::bail, winnow::Parser, *};

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

#[derive(Default)]
struct Database {
    ranges: Vec<RangeInclusive<usize>>,
    ingredients: Vec<usize>,
}

fn parse_input(input: &str) -> Database {
    let mut parse_ranges = true;
    let mut db = Database::default();
    for line in input.lines() {
        if line.is_empty() {
            parse_ranges = false;
            continue;
        }
        if parse_ranges {
            let (min, max) = line.split_once('-').expect("Range has a -");
            let (min, max) = (min.parse::<usize>().unwrap(), max.parse().unwrap());
            db.ranges.push(min..=max);
        } else {
            db.ingredients.push(line.parse().unwrap());
        }
    }
    db
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let Database { ranges, ingredients } = parse_input(input);
    let mut sum = 0;
    for id in ingredients {
        for range in &ranges {
            if range.contains(&id) {
                sum += 1;
                break;
            }
        }
    }
    Ok(sum)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let Database {
        mut ranges,
        ingredients: _,
    } = parse_input(input);
    let mut sum = 0;
    ranges.sort_by(|a, b| (a.start(), a.end()).cmp(&(b.start(), b.end())));
    dbg!(&ranges);
    let mut end = 0;
    for mut range in ranges {
        if *range.start() <= end {
            range = (end + 1)..=*range.end();
        }
        if !range.is_empty() {
            sum += range.end() - range.start() + 1;
        }
        end = *range.end();
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 3);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 14);
    }
}
