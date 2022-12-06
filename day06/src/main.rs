use common::AdventOfCodeError;
use common::{read_input_file_for_project_as_string, R};
use itertools::enumerate;
use itertools::peek_nth;
use itertools::Itertools;
use std::vec;

fn main() {
    let input = read_input_file_for_project_as_string!();
    println!("Part1: {:#?}", part1(&input).unwrap());
    println!("Part2: {:#?}", part2(&input).unwrap());
}

fn part1(input: &str) -> R<usize> {
    find_index_of_unique_char(input, 4)
}

fn part2(input: &str) -> R<usize> {
    find_index_of_unique_char(input, 14)
}

fn find_index_of_unique_char(input: &str, consecutive_chars: usize) -> R<usize> {
    let mut iter = peek_nth(enumerate(input.chars()));
    while let Some((i, current)) = iter.next() {
        let mut chars = vec![current];
        for index in 0..consecutive_chars - 1 {
            chars.push(iter.peek_nth(index).unwrap().1);
        }
        if chars.into_iter().unique().count() == consecutive_chars {
            let result = i + consecutive_chars;
            return Ok(result);
        }
    }
    Err(Box::new(AdventOfCodeError::new(format!(
        "Failed to find sequence of {} unique chars",
        consecutive_chars
    ))))
}

#[cfg(test)]
mod day6_tests {
    use super::*;
    const SAMPLE1: &str = r#"mjqjpqmgbljsphdztnvjfqwrcgsmlb"#;
    const SAMPLE2: &str = r#"bvwbjplbgvbhsrlpgdmjqwftvncz"#;
    const SAMPLE3: &str = r#"nppdvjthqldpwncqszvftbrmjlhg"#;
    const SAMPLE4: &str = r#"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"#;
    const SAMPLE5: &str = r#"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 7);
        assert_eq!(part1(SAMPLE2).unwrap(), 5);
        assert_eq!(part1(SAMPLE3).unwrap(), 6);
        assert_eq!(part1(SAMPLE4).unwrap(), 10);
        assert_eq!(part1(SAMPLE5).unwrap(), 11);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 19);
        assert_eq!(part2(SAMPLE2).unwrap(), 23);
        assert_eq!(part2(SAMPLE3).unwrap(), 23);
        assert_eq!(part2(SAMPLE4).unwrap(), 29);
        assert_eq!(part2(SAMPLE5).unwrap(), 26);
    }
}
