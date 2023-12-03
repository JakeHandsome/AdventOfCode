use std::{collections::HashSet, error::Error, fmt::Pointer};

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

// Parse the symbols, the numbers with their position in the matrix
fn parse_input(input: &str) -> Result<(Vec<Number>, Vec<Symbol>), Box<dyn Error>> {
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
    Ok((numbers, symbols))
}

#[derive(Debug)]
struct Number {
    value: usize,
    start: (isize, isize),
    len: usize,
}

impl Number {
    // Find if this number is adjacent to any of the symbols passed in
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
        // Reuse part1 to get all adjacent numbers to this symbol.
        let adjacent_numbers = numbers.iter().filter(|x| x.is_adjacent_to_symbol(&[*self]));
        if adjacent_numbers.clone().count() == 2 {
            // If there are exactly 2. Return the product
            Some(adjacent_numbers.map(|x| x.value).product::<usize>())
        } else {
            None
        }
    }
}

/// Part 1 i parsed the numbers and checked if each number was adjacent to a symbol. If it was it
/// was include in the sum. This made part 2 difficult since part 2 was much easier to find a
/// symbol and count the numbers
fn part1(input: &str) -> R<usize> {
    let (numbers, symbols) = parse_input(input)?;
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

// I made a gear_ratio function for symbol, that checked if any signal had 2 adjacent numbers and
// got the product. I realize I didnt not check specifically for the '*' symbol but it worked :)
// I was able to reuse a bit of part2 but it is a bit slow
fn part2(input: &str) -> R<usize> {
    let (numbers, symbols) = parse_input(input)?;
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
