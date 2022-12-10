use std::collections::HashSet;

use common::*;

fn main() {
    let input = read_input_file_for_project_as_string!();
    println!("Part1: {}", part1(&input).unwrap());
    println!("Part2: {}", part2(&input).unwrap());
}

fn part1(input: &str) -> R<usize> {
    let mut position = (0isize, 0isize);
    let mut positions: HashSet<(isize, isize)> = HashSet::new();
    positions.insert(position);

    for c in input.chars() {
        match c {
            'v' => position.1 -= 1,
            '^' => position.1 += 1,
            '<' => position.0 -= 1,
            '>' => position.0 += 1,
            _ => unreachable!(),
        }
        positions.insert(position);
    }
    Ok(positions.len())
}

fn part2(input: &str) -> R<usize> {
    let mut santa_pos = (0isize, 0isize);
    let mut robo_pos = (0isize, 0isize);
    let mut positions: HashSet<(isize, isize)> = HashSet::new();
    positions.insert(santa_pos);

    for (i, c) in input.chars().enumerate() {
        let a = {
            if i % 2 == 0 {
                &mut santa_pos
            } else {
                &mut robo_pos
            }
        };
        match c {
            'v' => a.1 -= 1,
            '^' => a.1 += 1,
            '<' => a.0 -= 1,
            '>' => a.0 += 1,
            _ => unreachable!(),
        }
        positions.insert(*a);
    }
    Ok(positions.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#""#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(">").unwrap(), 2);
        assert_eq!(part1("^>v<").unwrap(), 4);
        assert_eq!(part1("^v^v^v^v^v").unwrap(), 2);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2("^v").unwrap(), 3);
        assert_eq!(part2("^>v<").unwrap(), 3);
        assert_eq!(part2("^v^v^v^v^v").unwrap(), 11);
    }
}
