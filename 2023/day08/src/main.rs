use std::collections::HashMap;

use common::*;
use num::integer::lcm;

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

fn part1(input: &str) -> R<usize> {
    let input = input.replace(['(', ')', ',', '='], "");
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().cycle();
    let mut map = HashMap::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let split = line.split_whitespace().collect::<Vec<_>>();
        map.insert(split[0], (split[1], split[2]));
    }

    let mut current = "AAA";
    let mut instructions_followed = 1;
    for char in instructions {
        if char == 'L' {
            current = map.get(current).unwrap().0;
        } else {
            current = map.get(current).unwrap().1;
        }
        if current == "ZZZ" {
            return Ok(instructions_followed);
        }
        instructions_followed += 1;
    }

    Err(Box::new(AdventOfCodeError::new("Not implemented")))
}

// Ouput will be cyclical. Find the answer for each starting number and then use least common
// multiple to find when they all sync up
fn part2(input: &str) -> R<usize> {
    let input = input.replace(['(', ')', ',', '='], "");
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().cycle();
    let mut map = HashMap::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let split = line.split_whitespace().collect::<Vec<_>>();
        map.insert(split[0], (split[1], split[2]));
    }

    let currents = map
        .keys()
        .filter(|x| x.ends_with('A'))
        .map(|mut current| {
            let current = &mut current;
            let instructions = instructions.clone();
            let mut result = 1;
            for char in instructions {
                if char == 'L' {
                    *current = &map.get(*current).unwrap().0;
                } else {
                    *current = &map.get(*current).unwrap().1;
                }
                if current.ends_with('Z') {
                    break;
                }
                result += 1;
            }
            result
        })
        .collect::<Vec<_>>();

    Ok(least_common_multiple_many(&currents))
}

// Finds the lcm of multiple numbers using recursion
fn least_common_multiple_many(numbers: &[usize]) -> usize {
    if numbers.len() == 2 {
        lcm(numbers[0], numbers[1])
    } else {
        let a = numbers[0];
        let b = least_common_multiple_many(&numbers[1..]);
        lcm(a, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

    const SAMPLE2: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 2);
        assert_eq!(part1(SAMPLE2).unwrap(), 6);
    }
    const SAMPLE3: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE3).unwrap(), 6);
    }
}
