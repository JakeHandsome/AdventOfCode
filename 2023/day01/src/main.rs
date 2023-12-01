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

/// Save the first at last number and sum up the results
fn part1(input: &str) -> R<usize> {
    let mut calibration_values = vec![];
    for line in input.lines() {
        let mut first_digit = None;
        let mut last_digit = None;
        for char in line.chars() {
            if char.is_numeric() {
                let digit = char.to_digit(10).expect("Is numeric") as usize;
                first_digit.get_or_insert(digit);
                last_digit = Some(digit);
            }
        }
        calibration_values.push(first_digit.unwrap() * 10 + last_digit.unwrap());
    }
    println!("{:?}", calibration_values);
    Ok(calibration_values.into_iter().sum())
}

/// Convert the input to just digits and then call part1
fn part2(input: &str) -> R<usize> {
    let mut index = 0;
    let mut fixed = String::new();
    'outer: while index < input.len() {
        for (digit, char) in [
            ("one", '1'),
            ("two", '2'),
            ("three", '3'),
            ("four", '4'),
            ("five", '5'),
            ("six", '6'),
            ("seven", '7'),
            ("eight", '8'),
            ("nine", '9'),
        ] {
            if index + digit.len() < input.len() && input[index..index + digit.len()] == *digit {
                fixed.push(char);
                index += 1;
                continue 'outer;
            }
        }
        fixed.push(input.as_bytes()[index] as char);
        index += 1;
    }
    part1(&fixed)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 142);
    }
    const SAMPLE2: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE2).unwrap(), 281);
    }
}
