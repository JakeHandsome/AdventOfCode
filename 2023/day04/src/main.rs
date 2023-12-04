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

struct Card {
    winning_numbers: Vec<usize>,
    numbers: Vec<usize>,
    copies: usize,
}

impl Default for Card {
    fn default() -> Self {
        Self {
            winning_numbers: vec![],
            numbers: vec![],
            copies: 1,
        }
    }
}

impl Card {
    fn points(&self) -> usize {
        let mut points = 0;
        for win in &self.winning_numbers {
            if self.numbers.contains(win) {
                if points == 0 {
                    points += 1;
                } else {
                    points *= 2;
                }
            }
        }
        points
    }
    fn matches(&self) -> usize {
        let mut points = 0;
        for win in &self.winning_numbers {
            if self.numbers.contains(win) {
                points += 1;
            }
        }
        points
    }
}

fn part1(input: &str) -> R<usize> {
    let cards = parse(input);
    Ok(cards.into_iter().map(|card| card.points()).sum())
}

fn parse(input: &str) -> Vec<Card> {
    let mut cards = vec![];
    for line in input.lines() {
        let line = line.replace("  ", " ");
        let mut card = Card::default();
        let numbers = line.split(':').nth(1).unwrap();
        let mut split = numbers.split('|');
        card.winning_numbers = split
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .map(|x| x.trim().parse().unwrap())
            .collect::<Vec<_>>();
        card.numbers = split
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .map(|x| x.trim().parse().unwrap())
            .collect::<Vec<_>>();
        cards.push(card);
    }
    cards
}

fn part2(input: &str) -> R<usize> {
    let mut cards = parse(input);
    let len = cards.len();
    for i in 0..len {
        let current_card = cards.get(i).unwrap();
        let matches = current_card.matches();
        let copies = current_card.copies;
        for j in 1..=matches {
            let index = i + j;
            if index < len {
                let next_card = cards.get_mut(index).unwrap();
                next_card.copies += copies;
            }
        }
    }
    Ok(cards.into_iter().map(|x| x.copies).sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
 Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
 Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
 Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
 Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
 Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 13);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 30);
    }
}
