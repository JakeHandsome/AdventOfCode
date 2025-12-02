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
    let input = input.trim();
    let ranges = input.split(',');
    let mut count = 0;
    for (start, end) in ranges.map(|x| {
        let a = x
            .split_once('-')
            .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()));
        a.unwrap()
    }) {
        assert!(start < end);
        for i in start..=end {
            let as_str = i.to_string();
            // Asuumption, 2 repeating patterns
            if as_str.len() % 2 == 0 {
                let mid = as_str.len() / 2;
                if as_str[..mid] == as_str[mid..] {
                    count += i;
                }
            }
        }
    }
    Ok(count)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let input = input.trim();
    let ranges = input.split(',');
    let mut count = 0;
    for (start, end) in ranges.map(|x| {
        let a = x
            .split_once('-')
            .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()));
        a.unwrap()
    }) {
        for i in start..=end {
            let as_str = i.to_string();
            // Check if all digits are the same
            for k in 1..as_str.len() {
                if as_str.len() % k == 0 && as_str.as_bytes().chunks_exact(k).all_equal() {
                    count += i;
                    break;
                }
            }
        }
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 1227775554);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 4174379265);
    }
}
