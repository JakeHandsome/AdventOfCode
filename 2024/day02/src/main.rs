use std::{cmp::Ordering, usize};

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

enum IsLevelSafe {
    Yes,
    // Supposed to grab the index that needed to be removed, but it could be 2 possible indicies
    No(usize),
}

fn is_level_safe<T: Iterator<Item = usize>>(levels: T) -> IsLevelSafe {
    let mut direction = None;
    for (i, (l, r)) in levels.tuple_windows().enumerate() {
        match l.cmp(&r) {
            Ordering::Less => {
                let diff = r - l;
                match direction {
                    Some(Ordering::Less) | None => {
                        direction = Some(Ordering::Less);
                        if diff > 3 {
                            return IsLevelSafe::No(i);
                        }
                    }
                    Some(_) => return IsLevelSafe::No(i),
                }
            }
            std::cmp::Ordering::Greater => {
                let diff = l - r;
                match direction {
                    Some(Ordering::Greater) | None => {
                        direction = Some(Ordering::Greater);
                        if diff > 3 {
                            return IsLevelSafe::No(i);
                        }
                    }
                    Some(_) => return IsLevelSafe::No(i),
                }
            }
            std::cmp::Ordering::Equal => return IsLevelSafe::No(i),
        }
    }
    IsLevelSafe::Yes
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let mut safe_count = 0;
    for line in input.lines() {
        let levels = line.split(' ').map(|s| s.parse::<usize>().expect("Should be a number"));
        if matches!(is_level_safe(levels), IsLevelSafe::Yes) {
            safe_count += 1;
        }
    }
    Ok(safe_count)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let mut safe_count = 0;
    for line in input.lines() {
        let levels = line.split(' ').map(|s| s.parse::<usize>().expect("Should be a number"));
        match is_level_safe(levels.clone()) {
            IsLevelSafe::Yes => safe_count += 1,
            IsLevelSafe::No(_) => {
                // tried to remove just the invalid index, realized I don't know if the first or
                // last caused it, Just re-run all possible combinations
                let levels = levels.collect_vec();
                for i in 0..levels.len() {
                    let mut clone = levels.clone();
                    clone.remove(i);
                    if matches!(is_level_safe(clone.into_iter()), IsLevelSafe::Yes) {
                        safe_count += 1;
                        break;
                    }
                }
            }
        }
    }
    Ok(safe_count)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 2);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 4);
    }
}
