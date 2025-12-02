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
    let mut pos = 50;
    let mut count = 0;
    for line in input.lines() {
        let dir = line
            .chars()
            .next()
            .map(|x| if x == 'L' { -1 } else { 1 })
            .expect("Input will be valid");
        let magnitude: i32 = line[1..].parse().expect("Line should be a valid int");
        pos += magnitude * dir;
        if pos % 100 == 0 {
            count += 1;
        }
    }
    Ok(count)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let mut pos = 50;
    let mut count = 0;
    for line in input.lines() {
        let dir = line
            .chars()
            .next()
            .map(|x| if x == 'L' { -1 } else { 1 })
            .expect("Input will be valid");
        let mut magnitude: i32 = line[1..].parse().expect("Line should be a valid int");
        if magnitude >= 100 {
            count += magnitude / 100;
            magnitude %= 100;
        }
        let double_count = pos == 0;
        // For multiple rotations count them multiple times
        pos += magnitude * dir;
        if (pos >= 100 || pos <= 0) && !double_count {
            count += 1;
        }
        pos %= 100;
        if pos < 0 {
            pos += 100;
        }
    }
    Ok(count as usize)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 3);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 6);
    }
    const SAMPLE2: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
R1000"#;
    #[test]
    fn p2_test2() {
        assert_eq!(part2(SAMPLE2).unwrap(), 16);
    }
}
