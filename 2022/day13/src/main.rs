mod packet;

use packet::Packet;
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

#[derive(Debug)]
pub struct Pair {
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

fn part1(input: &str) -> R<usize> {
    let mut result = 0usize;
    for (i, chunk) in input.lines().collect::<Vec<_>>().chunks(3).enumerate() {
        let pair = Pair::new(chunk[0], chunk[1]);
        match pair.left.cmp(&pair.right) {
            std::cmp::Ordering::Less => {
                result += i + 1;
            }
            std::cmp::Ordering::Equal => unreachable!(),
            std::cmp::Ordering::Greater => (),
        };
    }
    #[cfg(not(test))]
    {
        // 5999 was first incorrect guess and it was too low
        assert!(result > 5999);
    }
    Ok(result)
}

fn part2(input: &str) -> R<usize> {
    // Add the marker packets
    let mut packets = vec![Packet::new("[[2]]".into()), Packet::new("[[6]]".into())];
    for chunk in input.lines().collect::<Vec<_>>().chunks(3) {
        packets.push(Packet::new(chunk[0].into()));
        packets.push(Packet::new(chunk[1].into()));
    }
    packets.sort();
    let result = packets
        .into_iter()
        .enumerate()
        .filter(|x| x.1.to_string() == "[[2]]" || x.1.to_string() == "[[6]]")
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
