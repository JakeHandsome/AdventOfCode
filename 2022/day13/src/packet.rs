#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Packet {
    Value(usize),
    Array(Vec<Packet>),
    Empty,
}
use std::fmt::Display;

use Packet::*;

impl Packet {
    // Parses a packet from an input string
    pub fn new(input: String) -> Self {
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

// Packet comarison
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

// Packet ToString, converts back to the original input
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
