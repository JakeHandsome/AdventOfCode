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

fn part1(input: &str) -> anyhow::Result<usize> {
    let total_inputs = input.lines().count();
    let mut ones: Vec<usize> = vec![];
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            if i >= ones.len() {
                ones.push(0);
            }
            if c == '1' {
                ones[i] += 1;
            }
        }
    }
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for one in ones {
        if one > total_inputs / 2 {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }
    let output = usize::from_str_radix(&gamma, 2)? * usize::from_str_radix(&epsilon, 2)?;
    Ok(output)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let numbers = input.lines().collect_vec();
    let a = find_number(numbers.clone(), true, 0);
    let b = find_number(numbers, false, 0);
    Ok(a * b)
}

fn find_number(numbers: Vec<&str>, most_common: bool, index: usize) -> usize {
    if numbers.len() == 1 {
        usize::from_str_radix(numbers[0], 2).unwrap()
    } else {
        let mut ones = vec![];
        let mut zeros = vec![];

        for number in numbers {
            if number.chars().nth(index).unwrap() == '1' {
                ones.push(number);
            } else if number.chars().nth(index).unwrap() == '0' {
                zeros.push(number);
            }
        }
        if ones.len() > zeros.len() {
            if most_common {
                find_number(ones, most_common, index + 1)
            } else {
                find_number(zeros, most_common, index + 1)
            }
        } else if ones.len() < zeros.len() {
            if most_common {
                find_number(zeros, most_common, index + 1)
            } else {
                find_number(ones, most_common, index + 1)
            }
        } else {
            if most_common {
                find_number(ones, most_common, index + 1)
            } else {
                find_number(zeros, most_common, index + 1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 198);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 230);
    }
}
