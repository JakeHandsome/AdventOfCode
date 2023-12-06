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

fn part1(input: &str) -> R<usize> {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect::<Vec<_>>();
    let distance = lines
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let mut wins_per_round = Vec::with_capacity(time.len());

    for i in 0..time.len() {
        let mut win_count = 0;
        let max_time = time[i];
        let distance_to_beat = distance[i];
        for selected_time in 0..max_time {
            let speed = selected_time;
            let time_remaining = max_time - selected_time;
            if speed * time_remaining > distance_to_beat {
                win_count += 1;
            }
        }
        wins_per_round.push(win_count);
    }
    let ans: usize = wins_per_round.iter().product();

    Ok(ans)
}

fn part2(input: &str) -> R<usize> {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("")
        .parse::<usize>()
        .unwrap();
    let distance = lines
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("")
        .parse::<usize>()
        .unwrap();
    let mut win_count = 0;
    for selected_time in 0..time {
        let speed = selected_time;
        let time_remaining = time - selected_time;
        if speed * time_remaining > distance {
            win_count += 1;
        }
    }
    Ok(win_count)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 288);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 71503);
    }
}
