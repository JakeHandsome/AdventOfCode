use std::collections::BTreeMap;

use common::*;
use regex::Regex;

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
    let regex = Regex::new(r"(?m)mul\((\d+).(\d+)\)").unwrap();
    let mut result = 0;
    for mat in regex.captures_iter(input) {
        let a = mat
            .get(1)
            .expect("Group 1 should match")
            .as_str()
            .parse::<usize>()
            .expect("Should be number");
        let b = mat
            .get(2)
            .expect("Group 1 should match")
            .as_str()
            .parse::<usize>()
            .expect("Should be number");
        result += a * b;
    }
    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    // always start with a do
    let input = format!("do(){input}");
    // Convet dos and don'ts into and index with true or false
    let mut dos = input
        .match_indices("do()")
        .map(|(index, pat)| (index + pat.len(), true))
        .collect::<BTreeMap<_, _>>();
    let mut donts = input
        .match_indices("don't()")
        .map(|(index, pat)| (index + pat.len(), false))
        .collect::<BTreeMap<_, _>>();
    dos.append(&mut donts);
    let mut result = 0;
    let regex = Regex::new(r"(?m)mul\((\d+).(\d+)\)").unwrap();
    for mat in regex.captures_iter(&input) {
        let x = mat.get(0).expect("Should match");
        let mut start = x.start();
        // Walk backwards untill we find last do or dont
        while !dos.contains_key(&start) {
            start -= 1;
        }
        // Depending on if the last was a do or don't enable the same logic as step 1
        if *dos.get(&start).unwrap() {
            let a = mat
                .get(1)
                .expect("Group 1 should match")
                .as_str()
                .parse::<usize>()
                .expect("Should be number");
            let b = mat
                .get(2)
                .expect("Group 1 should match")
                .as_str()
                .parse::<usize>()
                .expect("Should be number");
            result += a * b;
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 161);
    }
    const SAMPLE2: &str = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE2).unwrap(), 48);
    }
}
