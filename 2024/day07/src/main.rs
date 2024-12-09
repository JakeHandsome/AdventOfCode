use std::iter::repeat_n;

use common::*;
use num::pow::Pow;

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
#[derive(Debug, Clone)]
struct Equation {
    test_value: usize,
    operators: Vec<usize>,
}
#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Sub,
    Cat,
}

impl Equation {
    fn test(&self) -> bool {
        // In this mask us a bit 0 for add, bit 1 for subtract
        for mask in 0..2.pow(self.operators.len() - 1) {
            let mut index = 0;
            let mut iter = self.operators.iter();
            let first = iter.next().expect("Should have multiple items");

            let out = iter.fold(*first, |acc, &x| {
                let add = mask & (1 << index) == 0;
                index += 1;
                if add {
                    acc + x
                } else {
                    acc * x
                }
            });

            if out == self.test_value {
                return true;
            }
        }
        false
    }
    fn test2(&self) -> bool {
        // cant use binary bits :(
        // repeat_n(iter,k).multi_cartesian_product will get all permuations with repeats https://docs.rs/itertools/latest/itertools/trait.Itertools.html#method.permutations
        for perm in repeat_n(
            [Operator::Add, Operator::Sub, Operator::Cat].iter(),
            self.operators.len() - 1,
        )
        .multi_cartesian_product()
        {
            let mut index = 0;
            let mut iter = self.operators.iter();
            let first = iter.next().expect("Should have multiple items");

            let out = iter.fold(*first, |acc, &x| {
                let res = match perm[index] {
                    Operator::Add => acc + x,
                    Operator::Sub => acc * x,
                    Operator::Cat => format!("{}{}", acc, x).parse().expect("Will be a number"),
                };
                index += 1;
                res
            });

            if out == self.test_value {
                return true;
            }
        }
        false
    }
}

fn parse_input(x: &str) -> Equation {
    let mut a = x.split(':');
    let test_value = a
        .next()
        .expect(": should be input")
        .parse()
        .expect("Should be a number");
    let operators = a
        .next()
        .expect("Second half of split")
        .trim()
        .split(" ")
        .map(|x| x.parse().expect("Should be a number"))
        .collect_vec();
    Equation { test_value, operators }
}
fn part1(input: &str) -> anyhow::Result<usize> {
    Ok(input
        .lines()
        .map(parse_input)
        .filter(|x| x.test())
        .map(|x| x.test_value)
        .sum())
}

fn part2(input: &str) -> anyhow::Result<usize> {
    Ok(input
        .lines()
        .map(parse_input)
        .filter(|x| x.test2())
        .map(|x| x.test_value)
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 3749);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 11387);
    }
}
