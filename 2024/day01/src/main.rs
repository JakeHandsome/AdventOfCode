use common::*;
use num::abs;

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
    let mut left = vec![];
    let mut right = vec![];
    for line in input.lines() {
        let mut a = line.split("   ");
        left.push(
            a.next()
                .expect("should be 2 elements")
                .parse::<i32>()
                .expect("should parse as a number"),
        );
        right.push(
            a.next()
                .expect("should be 2 elements")
                .parse::<i32>()
                .expect("should parse as a number"),
        );
    }
    left.sort();
    right.sort();
    let pairs = left.into_iter().zip(right);
    let mut sum = 0;
    for (l, r) in pairs {
        sum += abs(l - r);
    }

    Ok(sum as usize)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let mut left = vec![];
    let mut right = vec![];
    for line in input.lines() {
        let mut a = line.split("   ");
        left.push(
            a.next()
                .expect("should be 2 elements")
                .parse::<i32>()
                .expect("should parse as a number"),
        );
        right.push(
            a.next()
                .expect("should be 2 elements")
                .parse::<i32>()
                .expect("should parse as a number"),
        );
    }
    let mut sum = 0;
    for l in left {
        sum += l * right.iter().filter(|r| **r == l).count() as i32;
    }
    Ok(sum as usize)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 11);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 31);
    }
}
