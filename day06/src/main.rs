use std::vec;

use common::{read_input_file_for_project_as_string, R};
use itertools::enumerate;
use itertools::peek_nth;
use itertools::Itertools;

fn main() {
    let input = read_input_file_for_project_as_string!();
    println!("Part1: {:#?}", part1(&input).unwrap());
    println!("Part2: {:#?}", part2(&input).unwrap());
}

fn part1(input: &str) -> R<usize> {
    let mut iter = peek_nth(enumerate(input.chars()));
    while let Some((i, current)) = iter.next() {
        let next1 = iter.peek_nth(0).unwrap().1;
        let next2 = iter.peek_nth(1).unwrap().1;
        let next3 = iter.peek_nth(2).unwrap().1;
        let a = vec![current, next1, next2, next3]
            .into_iter()
            .unique()
            .collect::<Vec<_>>();

        if a.len() == 4 {
            let result = i + 4;
            return Ok(result);
        }
    }
    todo!();
}

fn part2(input: &str) -> R<usize> {
    let mut iter = peek_nth(enumerate(input.chars()));
    while let Some((i, current)) = iter.next() {
        let next1 = iter.peek_nth(0).unwrap().1;
        let next2 = iter.peek_nth(1).unwrap().1;
        let next3 = iter.peek_nth(2).unwrap().1;
        let next4 = iter.peek_nth(3).unwrap().1;
        let next5 = iter.peek_nth(4).unwrap().1;
        let next6 = iter.peek_nth(5).unwrap().1;
        let next7 = iter.peek_nth(6).unwrap().1;
        let next8 = iter.peek_nth(7).unwrap().1;
        let next9 = iter.peek_nth(8).unwrap().1;
        let next10 = iter.peek_nth(9).unwrap().1;
        let next11 = iter.peek_nth(10).unwrap().1;
        let next12 = iter.peek_nth(11).unwrap().1;
        let next13 = iter.peek_nth(12).unwrap().1;
        let a = vec![
            current, next1, next2, next3, next4, next5, next6, next7, next8, next9, next10, next11, next12, next13,
        ]
        .into_iter()
        .unique()
        .collect::<Vec<_>>();

        if a.len() == 14 {
            let result = i + 14;
            return Ok(result);
        }
    }
    todo!()
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
