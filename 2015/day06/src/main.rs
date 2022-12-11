use std::io::Split;

use common::*;

fn main() {
    let input = read_input_file_for_project_as_string!();
    println!("Part1: {}", part1(&input).unwrap());
    println!("Part2: {}", part2(&input).unwrap());
}

enum Action {
    On,
    Off,
    Toggle,
}

fn part1(input: &str) -> R<usize> {
    let mut lights = vec![false; 1000 * 1000];

    for line in input.lines() {
        let split = line.split(" ").collect::<Vec<_>>();
        let mut action = Action::Off;
        let mut start = (0, 0);
        let mut end = (0, 0);
        if split[0] == "toggle" {
            action = Action::Toggle;
            let start_split = split[1]
                .split(",")
                .map(|x| x.trim().parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            start = (start_split[0], start_split[1]);
            let end_split = split[3]
                .split(",")
                .map(|x| x.trim().parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            end = (end_split[0], end_split[1]);
        } else {
            if split[1] == "on" {
                action = Action::On;
            }
            let start_split = split[2]
                .split(",")
                .map(|x| x.trim().parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            start = (start_split[0], start_split[1]);
            let end_split = split[4]
                .split(",")
                .map(|x| x.trim().parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            end = (end_split[0], end_split[1]);
        }
        for x in start.0..=end.0 {
            for y in start.1..=end.1 {
                let index = y * 1000 + x;
                match action {
                    Action::On => lights[index] = true,
                    Action::Off => lights[index] = false,
                    Action::Toggle => lights[index] = !lights[index],
                }
            }
        }
    }
    Ok(lights.into_iter().filter(|x| *x).count())
}

fn part2(input: &str) -> R<usize> {
    let mut lights = vec![0usize; 1000 * 1000];

    for line in input.lines() {
        let split = line.split(" ").collect::<Vec<_>>();
        let mut action = Action::Off;
        let mut start = (0, 0);
        let mut end = (0, 0);
        if split[0] == "toggle" {
            action = Action::Toggle;
            let start_split = split[1]
                .split(",")
                .map(|x| x.trim().parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            start = (start_split[0], start_split[1]);
            let end_split = split[3]
                .split(",")
                .map(|x| x.trim().parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            end = (end_split[0], end_split[1]);
        } else {
            if split[1] == "on" {
                action = Action::On;
            }
            let start_split = split[2]
                .split(",")
                .map(|x| x.trim().parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            start = (start_split[0], start_split[1]);
            let end_split = split[4]
                .split(",")
                .map(|x| x.trim().parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            end = (end_split[0], end_split[1]);
        }
        for x in start.0..=end.0 {
            for y in start.1..=end.1 {
                let index = y * 1000 + x;
                match action {
                    Action::On => lights[index] += 1,
                    Action::Off => {
                        if lights[index] != 0 {
                            lights[index] -= 1
                        }
                    }
                    Action::Toggle => lights[index] += 2,
                }
            }
        }
    }
    Ok(lights.into_iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#""#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 13140);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 0);
    }
}
