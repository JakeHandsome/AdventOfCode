use common::*;

fn main() {
    let input = read_input_file_for_project_as_string!();
    println!("Part1: {}", part1(&input).unwrap());
    println!("Part2: {}", part2(&input).unwrap());
}

fn part1(input: &str) -> R<usize> {
    Err(Box::new(AdventOfCodeError::new("Not implemented")))
}

fn part2(input: &str) -> R<usize> {
    Err(Box::new(AdventOfCodeError::new("Not implemented")))
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#""#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 13140);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 0);
    }
}
