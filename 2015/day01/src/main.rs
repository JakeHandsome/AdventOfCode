use common::*;

fn main() {
    let input = read_input_file_for_project_as_string!();
    println!("Part1: {}", part1(&input).unwrap());
    println!("Part2: {}", part2(&input).unwrap());
}

fn part1(input: &str) -> R<usize> {
    let ups = input.chars().filter(|c| *c == '(').count();
    let downs = input.chars().filter(|c| *c == ')').count();
    Ok(ups - downs)
}

fn part2(input: &str) -> R<usize> {
    let mut floor = 0isize;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => unreachable!(),
        }
        if floor.is_negative() {
            return Ok(i + 1);
        }
    }
    Err(Box::new(AdventOfCodeError::new("Never went in basement")))?
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"(())"#;
    const SAMPLE2: &str = "))(((((";
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 0);
        assert_eq!(part1(SAMPLE2).unwrap(), 3);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2("()())").unwrap(), 5);
    }
}

