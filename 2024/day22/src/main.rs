use std::collections::{vec_deque, HashMap, VecDeque};

use common::*;
use num::pow;

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
    Ok(input
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .map(|mut x| {
            (0..2000).for_each(|_| x = hash1(x));
            x
        })
        .sum())
}

#[inline]
fn hash1(mut value: usize) -> usize {
    value ^= value * 64;
    value %= 16777216;
    value ^= value / 32;
    value %= 16777216;
    value ^= value * 2048;
    value %= 16777216;
    value
}

fn part2(input: &str) -> anyhow::Result<usize> {
    Ok(*input
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .map(|mut x| {
            // Create a map of the last 4 changes the first time its found with the value
            let mut map = HashMap::new();
            let mut counter = VecDeque::new();
            (0..2000).for_each(|_| {
                let old = x % 10;
                x = hash1(x);
                let new = x % 10;
                let diff = old as isize - new as isize;
                counter.push_back(diff);
                if counter.len() == 4 {
                    if !map.contains_key(&counter) {
                        map.insert(counter.clone(), new);
                    }
                    let _ = counter.pop_front();
                }
            });
            map
        })
        .fold(HashMap::new(), |mut acc, x| {
            // Combine all the hashmaps into a single one, adding the values
            for (k, v) in x {
                if let Some(x) = acc.get_mut(&k) {
                    *x += v;
                } else {
                    acc.insert(k, v);
                }
            }
            acc
        })
        // Find the highest value
        .values()
        .max()
        .unwrap())
}

// Reading the problem issue. Its asking for least digit not most digit
#[allow(unused)]
fn most_significant_digit(x: usize) -> usize {
    let num_digits = (x.checked_ilog10().unwrap_or(0) + 1) as usize;
    if num_digits == 1 {
        x
    } else {
        let divisor = pow(10, num_digits - 1);
        x / divisor
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"1
10
100
2024"#;
    const SAMPLE2: &str = r#"1
2
3
2024"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 37327623);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE2).unwrap(), 23);
    }
    #[test]
    fn hash_test() {
        let mut init = 123;
        let seq = vec![
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432, 5908254,
        ];
        for s in seq {
            init = hash1(init);
            assert_eq!(s, init);
        }
    }
    #[test]
    fn msd() {
        assert_eq!(9, most_significant_digit(92398));
        assert_eq!(2, most_significant_digit(239112938));
        assert_eq!(4, most_significant_digit(4432));
        assert_eq!(3, most_significant_digit(329123));
        assert_eq!(9, most_significant_digit(92318888882318));
        assert_eq!(1, most_significant_digit(123));
        assert_eq!(3, most_significant_digit(392319));
        assert_eq!(0, most_significant_digit(0));
    }
}
