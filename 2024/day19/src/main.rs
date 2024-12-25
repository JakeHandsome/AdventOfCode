use std::collections::{BTreeSet, HashMap};

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
    let mut lines = input.lines();
    let available_towels = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.trim().to_string())
        .enumerate()
        .collect::<BTreeSet<(usize, String)>>();
    // Ignore empty line
    let _ = lines.next();
    let mut count = 0;
    for line in lines {
        println!("{}", &line);
        let mut memo = HashMap::new();
        if let Some(x) = check(line, 0, &available_towels, &mut memo) {
            count += 1;
            println!("  Pass {x:?}");
        } else {
            println!("  Fail");
        }
    }
    Ok(count)
}

fn check(
    line: &str,
    index: usize,
    available_towels: &BTreeSet<(usize, String)>,
    memo: &mut HashMap<(usize, usize), Option<Vec<String>>>,
) -> Option<Vec<String>> {
    if index == line.len() {
        return Some(vec![]);
    }
    let possible_towels = available_towels
        .iter()
        .filter(|(_, t)| {
            let len = t.len();
            (index + len) <= line.len() && &line[index..index + len] == t
        })
        .collect_vec();
    let mut res = None;
    for (i, t) in possible_towels {
        if res.is_some() {
            break;
        }
        if memo.contains_key(&(index, *i)) {
            res = memo.get(&(index, *i)).unwrap().to_owned();
        } else if let Some(mut x) = check(line, index + t.len(), available_towels, memo) {
            x.push(t.to_owned());
            res = Some(x);
            memo.insert((index, *i), res.clone());
            break;
        } else {
            memo.insert((index, *i), None);
        }
    }

    res
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let mut lines = input.lines();
    let available_towels = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.trim().to_string())
        .enumerate()
        .collect::<BTreeSet<(usize, String)>>();
    // Ignore empty line
    let _ = lines.next();
    let mut count = 0;
    for line in lines {
        println!("{}", &line);
        let mut memo = HashMap::new();
        count += check2(line, 0, &available_towels, &mut memo);
    }
    Ok(count)
}

fn check2(
    line: &str,
    index: usize,
    available_towels: &BTreeSet<(usize, String)>,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if index == line.len() {
        return 1;
    }
    let possible_towels = available_towels
        .iter()
        .filter(|(_, t)| {
            let len = t.len();
            (index + len) <= line.len() && &line[index..index + len] == t
        })
        .collect_vec();
    let mut res = 0;
    for (i, t) in possible_towels {
        if memo.contains_key(&(index, *i)) {
            res += memo.get(&(index, *i)).unwrap().to_owned();
        } else {
            let x = check2(line, index + t.len(), available_towels, memo);
            res += x;
            memo.insert((index, *i), x);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 6);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 16);
    }
}
