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
    let mut sum = 0;
    for line in input.lines() {
        let mut largest = (0, 0);
        let digits_pos = line
            .chars()
            .map(|c| c.to_digit(10).expect("Input should only be numbers"))
            .enumerate()
            .collect_vec();
        for (i, c) in digits_pos.iter().take(digits_pos.len() - 1) {
            if *c > largest.0 {
                largest = (*c, *i);
            }
            if largest.0 == 9 {
                break;
            }
        }
        let mut second_largest = (0, 0);
        for (i, c) in digits_pos.iter().skip(largest.1 + 1) {
            if *c > second_largest.0 {
                second_largest = (*c, *i);
            }
            if second_largest.0 == 9 {
                break;
            }
        }
        sum += largest.0 * 10;
        sum += second_largest.0;
    }
    Ok(sum as usize)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let mut sum = 0;
    for line in input.lines() {
        let digits_pos = line
            .chars()
            .map(|c| c.to_digit(10).expect("Input should only be numbers") as usize)
            .enumerate()
            .collect_vec();
        let digit_len = digits_pos.len();
        let mut digits = [(0, 0); 12];
        let mut digits_left = 12;
        //Loop through making sure we have 12 digits left and select the max for each
        // Ex check all but last 12 for largest digit,
        // then check remaining slice - last 11 for next digit continue until all 12 digits are found
        // If only digit_left remain, take them all
        while digits_left > 0 {
            let index = 12 - digits_left;
            let take = digit_len - digits_left;
            let skip = if index == 0 { 0 } else { digits[index - 1].1 + 1 };
            let current_largest = digits.get_mut(index).unwrap();
            for (i, c) in &digits_pos[skip..=take] {
                if *c > current_largest.0 {
                    *current_largest = (*c, *i);
                }
                if current_largest.0 == 9 {
                    break;
                }
            }
            digits_left -= 1;
        }
        let mut val = 0;
        val += digits[11].0;
        val += digits[10].0 * 10;
        val += digits[9].0 * 100;
        val += digits[8].0 * 1_000;
        val += digits[7].0 * 10_000;
        val += digits[6].0 * 100_000;
        val += digits[5].0 * 1_000_000;
        val += digits[4].0 * 10_000_000;
        val += digits[3].0 * 100_000_000;
        val += digits[2].0 * 1_000_000_000;
        val += digits[1].0 * 10_000_000_000;
        val += digits[0].0 * 100_000_000_000;
        sum += val;
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 357);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 3121910778619);
    }
    #[test]
    fn p22_test() {
        assert_eq!(part2("234234234234278").unwrap(), 3121910778619);
    }
}
