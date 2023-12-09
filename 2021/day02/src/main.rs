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
#[derive(Default)]
struct Position {
    horizontal: isize,
    depth: isize,
    aim: isize,
}

fn part1(input: &str) -> anyhow::Result<isize> {
    let mut position = Position::default();
    for line in input.lines() {
        let mut split = line.split_whitespace();
        let command = split.next().unwrap();
        let value = split.next().unwrap().parse::<isize>()?;
        match command {
            "forward" => position.horizontal += value,
            "down" => position.depth += value,
            "up" => position.depth -= value,
            _ => return Err(AdventOfCodeError::UnimplementedError)?,
        }
    }
    Ok(position.horizontal * position.depth)
}

fn part2(input: &str) -> anyhow::Result<isize> {
    let mut position = Position::default();
    for line in input.lines() {
        let mut split = line.split_whitespace();
        let command = split.next().unwrap();
        let value = split.next().unwrap().parse::<isize>()?;
        match command {
            "forward" => {
                position.horizontal += value;
                position.depth += position.aim * value
            }
            "down" => position.aim += value,
            "up" => position.aim -= value,
            _ => return Err(AdventOfCodeError::UnimplementedError)?,
        }
    }
    Ok(position.horizontal * position.depth)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 150);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 900);
    }
}
