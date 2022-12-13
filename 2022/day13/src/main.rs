use std::{fmt::Display, vec};

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

#[derive(Debug, Eq, PartialEq, Clone)]
enum Packet {
    Value(usize),
    Array(Vec<Packet>),
    Empty,
}

impl Packet {
    fn new(input: String) -> Self {
        if input.starts_with('[') {
            let mut split = input[1..input.len() - 1].split(',').peekable();
            let mut elements: Vec<Packet> = vec![];
            let mut current = "".to_string();
            while split.peek().is_some() || current != *"" {
                if current.is_empty() {
                    current = split.next().unwrap().into()
                }
                let numopen = current.chars().filter(|c| *c == '[').count();
                let numclose = current.chars().filter(|c| *c == ']').count();
                if (current.contains('[')
                    && current.contains(']')
                        //Make sure the number of ] matches number of [
                && numopen == numclose)
                    || !current.contains('[') && !current.contains(']')
                {
                    // This is an element or a value
                    elements.push(Packet::new(current));
                    current = "".into();
                } else if numopen != numclose {
                    // Combine with next string
                    current = current.to_string() + "," + split.next().unwrap();
                }
            }
            Array(elements)
        } else {
            match input.parse() {
                Ok(x) => Value(x),
                Err(_) => Empty,
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let left = self;
        let right = other;
        use std::cmp::Ordering::*;
        match (left.to_owned(), right.to_owned()) {
            // Compare values
            (Value(left), Value(right)) => left.cmp(&right),
            // Convert value into array an compare
            (Value(left), Array(right)) => Array(vec![Value(left)]).cmp(&Array(right)),
            (Array(left), Value(right)) => Array(left).cmp(&Array(vec![Value(right)])),
            // Right side end early
            (Array(_), Empty) | (Value(_), Empty) => Greater,
            // Left side end early
            (Empty, Value(_)) | (Empty, Array(_)) => Less,
            (Empty, Empty) => Equal,
            (Array(left), Array(right)) => {
                let mut left = left.into_iter();
                let mut right = right.into_iter();
                match (left.next(), right.next()) {
                    (None, None) => Equal,
                    (None, Some(x)) => Empty.cmp(&x),
                    (Some(x), None) => x.cmp(&Empty),
                    (Some(l), Some(r)) => match l.cmp(&r) {
                        Equal => Array(left.collect()).cmp(&Array(right.collect())),
                        x => x,
                    },
                }
            }
        }
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value(x) => f.write_fmt(format_args!("{}", x)),
            Array(x) => f.write_fmt(format_args!(
                "[{}]",
                x.iter().map(|y| y.to_string()).collect::<Vec<_>>().join(",")
            )),
            Empty => Ok(()),
        }
    }
}

#[derive(Debug)]
struct Pair {
    left: Packet,
    right: Packet,
}

impl Pair {
    fn new(left: &str, right: &str) -> Self {
        Pair {
            left: Packet::new(left.into()),
            right: Packet::new(right.into()),
        }
    }
}

impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}\n{:?}\n", self.left, self.right))
    }
}

use Packet::*;

fn part1(input: &str) -> R<usize> {
    let mut result = 0usize;
    for (i, chunk) in input.lines().collect::<Vec<_>>().chunks(3).enumerate() {
        let pair = Pair::new(chunk[0], chunk[1]);
        match pair.left.cmp(&pair.right) {
            std::cmp::Ordering::Less => {
                result += i + 1;
                continue;
            }
            std::cmp::Ordering::Equal => unreachable!(),
            std::cmp::Ordering::Greater => continue,
        };
    }
    #[cfg(not(test))]
    {
        assert!(result > 5999);
    }
    Ok(result)
}

fn part2(input: &str) -> R<usize> {
    let mut packets = vec![Packet::new("[[2]]".into()), Packet::new("[[6]]".into())];
    for chunk in input.lines().collect::<Vec<_>>().chunks(3) {
        packets.push(Packet::new(chunk[0].into()));
        packets.push(Packet::new(chunk[1].into()));
    }
    packets.sort();
    let result = packets
        .into_iter()
        .enumerate()
        .filter(|x| format!("{}", x.1) == "[[2]]" || format!("{}", x.1) == "[[6]]")
        .map(|x| x.0 + 1)
        .product();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 13);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 140);
    }
}
