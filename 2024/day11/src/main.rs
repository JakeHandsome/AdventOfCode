use std::collections::HashMap;

use common::*;
use num::{pow, Integer};

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
    let mut new = input
        .trim()
        .split(" ")
        .map(|x| x.parse::<usize>().expect("Should be a number"))
        .collect_vec();
    for _ in 0..25 {
        new = new
            .into_iter()
            .flat_map(|x| {
                if x == 0 {
                    return vec![1];
                }
                let num_digits = (x.checked_ilog10().expect("Number cannot be 0") + 1) as usize;
                if num_digits % 2 == 0 {
                    let divisor = pow(10, num_digits / 2);
                    let (l, r) = x.div_rem(&divisor);
                    return vec![l, r];
                }
                vec![x * 2024]
            })
            .collect_vec();
    }
    Ok(new.len())
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let mut memo = HashMap::new();
    let len = input
        .trim()
        .split(" ")
        .map(|x| x.parse::<usize>().expect("Should be a number"))
        .fold(0, |acc, x| acc + process_number(x, 75, &mut memo));
    Ok(len)
}

// Takes in one number, runs the algorith a certain number of times returning the final length
fn process_number(x: usize, steps: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if memo.contains_key(&(x, steps)) {
        return memo[&(x, steps)];
    }
    if steps == 0 {
        return 1;
    }
    let next = {
        if x == 0 {
            vec![1]
        } else {
            let num_digits = (x.checked_ilog10().expect("Number cannot be 0") + 1) as usize;
            if num_digits % 2 == 0 {
                let divisor = pow(10, num_digits / 2);
                let (l, r) = x.div_rem(&divisor);
                vec![l, r]
            } else {
                vec![x * 2024]
            }
        }
    };
    let sum = next
        .into_iter()
        .fold(0, |acc, x| acc + process_number(x, steps - 1, memo));
    memo.insert((x, steps), sum);
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"125 17"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 55312);
    }
    #[test]
    fn p2_test() {
        // No example for part 2
        //assert_eq!(part2(SAMPLE1).unwrap(), 0);
    }
}
