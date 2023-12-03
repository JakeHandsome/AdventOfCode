use std::{collections::HashSet, fmt::Pointer};

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

#[derive(Debug)]
struct Number {
    value: usize,
    start: (isize, isize),
    len: usize,
}

impl Number {
    fn is_adjacent_to_symbol(&self, symbols: &[Symbol]) -> bool {
        let mut points = HashSet::new();
        for i in 0..self.len {
            points.insert((self.start.0, self.start.1 + i as isize));
        }
        for (y, x) in points {
            let adjacent_points = vec![
                (y - 1, x - 1),
                (y - 1, x),
                (y - 1, x + 1),
                (y, x - 1),
                (y, x + 1),
                (y + 1, x - 1),
                (y + 1, x),
                (y + 1, x + 1),
            ];
            for (y, x) in adjacent_points {
                if symbols.iter().any(|s| *s == Symbol(y, x)) {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Symbol(isize, isize);

impl Symbol {
    fn gear_ratio(&self, numbers: &[Number]) -> Option<usize> {
        let (y, x) = (self.0, self.1);
        let adjacent_numbers = numbers.iter().filter(|x| x.is_adjacent_to_symbol(&[*self]));
        if adjacent_numbers.clone().count() == 2 {
            Some(adjacent_numbers.map(|x| x.value).product::<usize>())
        } else {
            None
        }
    }
}

fn part1(input: &str) -> R<usize> {
    let mut numbers = vec![];
    let mut symbols = vec![];
    let mut index = 0;
    let mut number_start_index = None;
    let width = input.find('\n').unwrap();
    let input = input.replace('\n', "");
    while index < input.len() {
        let current_char = input.as_bytes()[index];
        if number_start_index.is_none() && current_char.is_ascii_digit() {
            number_start_index = Some(index);
        } else if number_start_index.is_some() && !current_char.is_ascii_digit() {
            let start_index = number_start_index.take().unwrap();
            let start = ((start_index / width) as isize, (start_index % width) as isize);
            let number = Number {
                value: input[start_index..index].parse()?,
                start,
                len: index - start_index,
            };
            numbers.push(number);
        }
        if !current_char.is_ascii_digit() && current_char != b'.' {
            symbols.push(Symbol((index / width) as isize, (index % width) as isize));
        }
        index += 1;
    }
    Ok(numbers
        .into_iter()
        .filter_map(|x| {
            if x.is_adjacent_to_symbol(&symbols) {
                Some(x.value)
            } else {
                None
            }
        })
        .sum())
}

fn part2(input: &str) -> R<usize> {
    let mut numbers = vec![];
    let mut symbols = vec![];
    let mut index = 0;
    let mut number_start_index = None;
    let width = input.find('\n').unwrap();
    let input = input.replace('\n', "");
    while index < input.len() {
        let current_char = input.as_bytes()[index];
        if number_start_index.is_none() && current_char.is_ascii_digit() {
            number_start_index = Some(index);
        } else if number_start_index.is_some() && !current_char.is_ascii_digit() {
            let start_index = number_start_index.take().unwrap();
            let start = ((start_index / width) as isize, (start_index % width) as isize);
            let number = Number {
                value: input[start_index..index].parse()?,
                start,
                len: index - start_index,
            };
            numbers.push(number);
        }
        if !current_char.is_ascii_digit() && current_char != b'.' {
            symbols.push(Symbol((index / width) as isize, (index % width) as isize));
        }
        index += 1;
    }
    Ok(symbols.into_iter().filter_map(|x| x.gear_ratio(&numbers)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 4361);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 467835);
    }
}
