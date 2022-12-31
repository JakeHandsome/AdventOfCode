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

fn part1(input: &str) -> R<String> {
    Ok(decimal_to_snafu(input.lines().map(|s| snafu_to_decimal(s)).sum()))
}

fn part2(input: &str) -> R<usize> {
    Err(Box::new(AdventOfCodeError::new("Not implemented")))
}

fn snafu_to_decimal(input: &str) -> isize {
    let mut iter = input.chars();
    let mut position = 5isize.pow(input.len() as u32 - 1);
    let mut result = 0;
    while let Some(c) = iter.next() {
        match c {
            '0' | '1' | '2' => {
                result += position * c.to_digit(10).unwrap() as isize;
            }
            '=' => result += position * -2,
            '-' => result += position * -1,
            _ => unreachable!(),
        }
        position /= 5;
    }
    result
}

const SNAFU_CHARS: [char; 5] = ['=', '-', '0', '1', '2'];

fn decimal_to_snafu(x: isize) -> String {
    if x > 0 {
        let (remainder, current_char) = ((x + 2) / 5, (x + 2) % 5);
        // Calculate the rest of the nubmers and append the remainder encoded as snafu
        format!("{}{}", decimal_to_snafu(remainder), SNAFU_CHARS[current_char as usize])
    } else {
        // Return an empty string, no more characters left
        return "".to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), "2=-1=0");
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 0);
    }
    #[test]
    fn basic_snafu() {
        assert_eq!(snafu_to_decimal("1"), 1, "1");
        assert_eq!(snafu_to_decimal("2"), 2, "2'");
        assert_eq!(snafu_to_decimal("1="), 3, "3");
        assert_eq!(snafu_to_decimal("1-"), 4, "4");
        assert_eq!(snafu_to_decimal("10"), 5, "5'");
        assert_eq!(snafu_to_decimal("11"), 6);
        assert_eq!(snafu_to_decimal("12"), 7);
        assert_eq!(snafu_to_decimal("2="), 8);
        assert_eq!(snafu_to_decimal("2-"), 9);
        assert_eq!(snafu_to_decimal("20"), 10);
        assert_eq!(snafu_to_decimal("1=0"), 15);
        assert_eq!(snafu_to_decimal("1-0"), 20);
        assert_eq!(snafu_to_decimal("1=11-2"), 2022);
        assert_eq!(snafu_to_decimal("1-0---0"), 12345);
        assert_eq!(snafu_to_decimal("1121-1110-1=0"), 314159265);
    }
    #[test]
    fn snafu_to1() {
        assert_eq!(snafu_to_decimal("1"), 1);
    }
    #[test]
    fn snafu_to2() {
        assert_eq!(snafu_to_decimal("2"), 2);
    }
    #[test]
    fn snafu_to3() {
        assert_eq!(snafu_to_decimal("1="), 3);
    }
    #[test]
    fn snafu_to4() {
        assert_eq!(snafu_to_decimal("1-"), 4);
    }
    #[test]
    fn snafu_to5() {
        assert_eq!(snafu_to_decimal("10"), 5);
    }
    #[test]
    fn snafu_to6() {
        assert_eq!(snafu_to_decimal("11"), 6);
    }
    #[test]
    fn snafu_to7() {
        assert_eq!(snafu_to_decimal("12"), 7);
    }
    #[test]
    fn snafu_to8() {
        assert_eq!(snafu_to_decimal("2="), 8);
    }
    #[test]
    fn snafu_to9() {
        assert_eq!(snafu_to_decimal("2-"), 9);
    }
    #[test]
    fn snafu_to10() {
        assert_eq!(snafu_to_decimal("20"), 10);
    }
    #[test]
    fn snafu_to15() {
        assert_eq!(snafu_to_decimal("1=0"), 15);
    }
    #[test]
    fn snafu_to20() {
        assert_eq!(snafu_to_decimal("1-0"), 20);
    }
    #[test]
    fn snafu_to2022() {
        assert_eq!(snafu_to_decimal("1=11-2"), 2022);
    }
    #[test]
    fn snafu_to12345() {
        assert_eq!(snafu_to_decimal("1-0---0"), 12345);
    }
    #[test]
    fn snafu_to314159265() {
        assert_eq!(snafu_to_decimal("1121-1110-1=0"), 314159265);
    }
    #[test]
    fn decimal_1_to_snafu() {
        assert_eq!(decimal_to_snafu(1), "1".to_string());
    }
    #[test]
    fn decimal_2_to_snafu() {
        assert_eq!(decimal_to_snafu(2), "2".to_string());
    }
    #[test]
    fn decimal_3_to_snafu() {
        assert_eq!(decimal_to_snafu(3), "1=".to_string());
    }
    #[test]
    fn decimal_4_to_snafu() {
        assert_eq!(decimal_to_snafu(4), "1-".to_string());
    }
    #[test]
    fn decimal_5_to_snafu() {
        assert_eq!(decimal_to_snafu(5), "10".to_string());
    }
    #[test]
    fn decimal_6_to_snafu() {
        assert_eq!(decimal_to_snafu(6), "11".to_string());
    }
    #[test]
    fn decimal_7_to_snafu() {
        assert_eq!(decimal_to_snafu(7), "12".to_string());
    }
    #[test]
    fn decimal_8_to_snafu() {
        assert_eq!(decimal_to_snafu(8), "2=".to_string());
    }
    #[test]
    fn decimal_9_to_snafu() {
        assert_eq!(decimal_to_snafu(9), "2-".to_string());
    }
    #[test]
    fn decimal_10_to_snafu() {
        assert_eq!(decimal_to_snafu(10), "20".to_string());
    }
    #[test]
    fn decimal_15_to_snafu() {
        assert_eq!(decimal_to_snafu(15), "1=0".to_string());
    }
    #[test]
    fn decimal_20_to_snafu() {
        assert_eq!(decimal_to_snafu(20), "1-0".to_string());
    }
    #[test]
    fn decimal_2022_to_snafu() {
        assert_eq!(decimal_to_snafu(2022), "1=11-2".to_string());
    }
    #[test]
    fn decimal_12345_to_snafu() {
        assert_eq!(decimal_to_snafu(12345), "1-0---0".to_string());
    }
    #[test]
    fn decimal_314159265_to_snafu() {
        assert_eq!(decimal_to_snafu(314159265), "1121-1110-1=0".to_string());
    }
}
