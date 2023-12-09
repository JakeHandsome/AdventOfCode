use std::collections::VecDeque;

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

fn solve_line_p1(line: &str) -> isize {
    let mut rounds = vec![];
    let initial = line
        .split_whitespace()
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    rounds.push(initial);
    while !rounds.last().unwrap().iter().all(|x| *x == 0) {
        let last_round = rounds.last().unwrap();
        let next_round = last_round.windows(2).map(|x| x[1] - x[0]).collect();
        rounds.push(next_round);
    }
    let mut last_add = 0;
    for x in rounds.iter_mut().rev() {
        last_add += x.last().unwrap();
        x.push(last_add)
    }
    *rounds.first().unwrap().last().unwrap()
}

fn part1(input: &str) -> R<isize> {
    Ok(input.lines().map(solve_line_p1).sum())
}
fn solve_line_p2(line: &str) -> isize {
    let mut rounds = VecDeque::new();
    let initial = line
        .split_whitespace()
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<VecDeque<_>>();
    rounds.push_back(initial);
    while !rounds.back().unwrap().iter().all(|x| *x == 0) {
        let last_round = rounds.back_mut().unwrap().make_contiguous();
        let next_round = last_round.windows(2).map(|x| x[1] - x[0]).collect();
        rounds.push_back(next_round);
    }

    let mut last_sub = 0;
    for x in rounds.iter_mut().rev() {
        last_sub = x.front().unwrap() - last_sub;
        x.push_front(last_sub)
    }
    *rounds.front().unwrap().front().unwrap()
}

fn part2(input: &str) -> R<isize> {
    Ok(input.lines().map(solve_line_p2).sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
    #[test]
    fn p1_testl1() {
        assert_eq!(solve_line_p1(SAMPLE1.lines().next().unwrap()), 18);
    }
    #[test]
    fn p1_testl2() {
        assert_eq!(solve_line_p1(SAMPLE1.lines().nth(1).unwrap()), 28);
    }
    #[test]
    fn p1_testl3() {
        assert_eq!(solve_line_p1(SAMPLE1.lines().nth(2).unwrap()), 68);
    }

    #[test]
    fn p2_testl3() {
        assert_eq!(solve_line_p2(SAMPLE1.lines().nth(2).unwrap()), 5);
    }
    /*
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 114);
    }*/
}
