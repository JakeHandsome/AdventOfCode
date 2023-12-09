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
    Err(AdventOfCodeError::UnimplementedError)?
}

fn part2(input: &str) -> anyhow::Result<usize> {
    Err(AdventOfCodeError::UnimplementedError)?
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#""#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 0);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 0);
    }
}
