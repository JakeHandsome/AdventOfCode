use std::ops::Not;

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

#[derive(Default, Debug)]
struct Board {
    numbers: Vec<(usize, bool)>,
}

impl Board {
    fn mark_number(&mut self, number: usize) {
        for (tile_number, marked) in &mut self.numbers {
            if *tile_number == number {
                *marked = true;
                break;
            }
        }
    }
    fn is_winner(&self) -> bool {
        // Check rows
        for i in [0, 5, 10, 15, 20] {
            if self.numbers[i..i + 5].iter().all(|(_, marked)| *marked) {
                return true;
            }
        }
        for i in 0..5 {
            if self.numbers.iter().skip(i).step_by(5).all(|(_, marked)| *marked) {
                return true;
            }
        }
        false
    }
    fn score(&self, last_number: usize) -> usize {
        self.numbers
            .iter()
            .filter_map(|(x, marked)| if !marked { Some(*x) } else { None })
            .sum::<usize>()
            * last_number
    }
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let (mut numbers, mut boards) = parse_input(input);

    let mut last_number = 0;
    while boards.iter().any(|b| b.is_winner()).not() {
        last_number = numbers.next().unwrap();
        for board in &mut boards {
            board.mark_number(last_number);
        }
    }
    Ok(boards.into_iter().find(|b| b.is_winner()).unwrap().score(last_number))
}

fn parse_input(input: &str) -> (impl Iterator<Item = usize> + '_, Vec<Board>) {
    let mut iter = input.lines();
    let numbers = iter.next().unwrap().split(',').map(|s| s.parse::<usize>().unwrap());
    let _ = iter.next();
    // Throw out next line
    let mut boards = vec![];
    let mut current_board = Board::default();
    for line in iter {
        if line.is_empty() {
            boards.push(current_board);
            current_board = Board::default();
        } else {
            current_board
                .numbers
                .extend(line.split_whitespace().map(|s| (s.parse().unwrap(), false)));
        }
    }
    boards.push(current_board);
    (numbers, boards)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let (mut numbers, mut boards) = parse_input(input);

    let mut last_number = 0;
    while boards.len() > 1 {
        last_number = numbers.next().unwrap();
        boards.retain_mut(|x| {
            x.mark_number(last_number);
            !x.is_winner()
        })
    }
    let mut final_board = boards.pop().unwrap();
    while final_board.is_winner().not() {
        last_number = numbers.next().unwrap();
        final_board.mark_number(last_number)
    }
    Ok(final_board.score(last_number))
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 4512);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 1924);
    }
}
