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
    Ok(input
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect_vec()
        .windows(2)
        .filter(|x| x[1] > x[0])
        .count())
}

fn part2(input: &str) -> anyhow::Result<usize> {
    Ok(input
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect_vec()
        .windows(3)
        .map(|x| x.iter().sum::<usize>())
        .collect_vec()
        .windows(2)
        .filter(|x| x[1] > x[0])
        .count())
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"199
200
208
210
200
207
240
269
260
263"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 7);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 5);
    }
}
