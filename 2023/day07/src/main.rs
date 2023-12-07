use std::collections::BTreeMap;

use itertools::Itertools;

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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand<'a>(&'a str);

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Implement the Ord trait to allow sorting of the values
impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_type = self.hand_type();
        let other_type = other.hand_type();
        if self_type != other_type {
            self_type.cmp(&other_type)
        } else {
            for (self_char, other_char) in self.0.chars().zip(other.0.chars()) {
                if self_char != other_char {
                    return char_to_value(self_char).cmp(&char_to_value(other_char));
                }
            }
            panic!("None should be equal");
        }
    }
}

fn char_to_value(x: char) -> u8 {
    match x {
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'J' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => panic!("Unexpected char"),
    }
}

impl<'a> Hand<'a> {
    fn hand_type(&self) -> HandType {
        // Counts (from itertools) creates a hashmap, with the key being the character and the value being number of
        // occurances
        let counts = self.0.chars().counts();
        let unique_cards = counts.len();
        match unique_cards {
            // If there is only 1 type of card it is a five of a kind
            1 => HandType::FiveOfAKind,
            // If 2 types of card, either 4 of kind or full house
            2 => {
                // If any card has a count of 4, 4 of a kind
                if counts.values().any(|x| *x == 4) {
                    HandType::FourOfAKind
                } else {
                    // Otherwise full house
                    HandType::FullHouse
                }
            }
            // If 3 types of card, either 3 of kind or two pair
            3 => {
                if counts.values().any(|x| *x == 3) {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            // 4 unique cards means one pair
            4 => HandType::OnePair,
            // 5 unique is high card
            _ => HandType::HighCard,
        }
    }
}

fn part1(input: &str) -> R<usize> {
    // BTreeMap will sort the keys from lowest to highert
    let mut hands = BTreeMap::new();
    for line in input.lines() {
        let mut split = line.split_whitespace();
        let hand = Hand(split.next().unwrap());
        let bet = split.next().unwrap().parse::<usize>().unwrap();
        hands.insert(hand, bet);
    }
    let mut result = 0;
    for (i, bet) in hands.values().enumerate() {
        // Get the rank (starting from 1) and multiply the bet to the final result
        result += (i + 1) * bet;
    }
    Ok(result)
}

#[derive(Debug, PartialEq, Eq)]
struct HandPt2<'a>(&'a str);

impl<'a> PartialOrd for HandPt2<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for HandPt2<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_type = self.hand_type();
        let other_type = other.hand_type();
        // If the hand types are different sort by that
        if self_type != other_type {
            self_type.cmp(&other_type)
        } else {
            // Otherwise go character by character in each hand until one card is greater
            for (self_char, other_char) in self.0.chars().zip(other.0.chars()) {
                if self_char != other_char {
                    return char_to_value2(self_char).cmp(&char_to_value2(other_char));
                }
            }
            panic!("None should be equal");
        }
    }
}

impl<'a> HandPt2<'a> {
    fn hand_type(&self) -> HandType {
        let mut counts = self.0.chars().counts();
        // Before running the same hand_type as part1, replace all jokers with whatever the higest
        // count card is
        if let Some(joker_count) = counts.remove(&'J') {
            if joker_count == 5 {
                counts.insert('A', 5);
            } else {
                let max_count = counts.values_mut().max().unwrap();
                *max_count += joker_count;
            }
        }
        let len = counts.len();
        match len {
            // If there is only 1 type of card it is a five of a kind
            1 => HandType::FiveOfAKind,
            // If 2 types of card, either 4 of kind or full house
            2 => {
                // If any card has a count of 4, 4 of a kind
                if counts.values().any(|x| *x == 4) {
                    HandType::FourOfAKind
                } else {
                    // Otherwise full house
                    HandType::FullHouse
                }
            }
            3 => {
                if counts.values().any(|x| *x == 3) {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

// For part 2, a J is the lowest point value
fn char_to_value2(x: char) -> u8 {
    match x {
        'J' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => panic!("Unexpected char"),
    }
}
fn part2(input: &str) -> R<usize> {
    let mut hands = BTreeMap::new();
    for line in input.lines() {
        let mut split = line.split_whitespace();
        let hand = HandPt2(split.next().unwrap());
        let bet = split.next().unwrap().parse::<usize>().unwrap();
        hands.insert(hand, bet);
    }
    let mut result = 0;
    for (i, bet) in hands.values().enumerate() {
        result += (i + 1) * bet;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 6440);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 5905);
    }
}
