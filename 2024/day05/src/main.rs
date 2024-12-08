use std::{
    cmp::Ordering,
    collections::{
        btree_map::{self, Entry},
        BTreeMap,
    },
};

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
    let (printing_rules, page_updates) = parse_input(input);
    let mut result = 0;

    for update in &page_updates {
        if check_update(update, &printing_rules) {
            result += update[(update.len()) / 2];
        }
    }
    Ok(result)
}

fn parse_input(input: &str) -> (BTreeMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    let mut parse_first = true;
    let mut printing_rules = BTreeMap::new();
    let mut page_updates = vec![];
    for line in input.lines() {
        if parse_first {
            if line.is_empty() {
                parse_first = false;
                continue;
            }
            let mut split = line.split('|');
            let left = split
                .next()
                .expect("Should have item")
                .parse::<usize>()
                .expect("Should be a number");
            let right = split
                .next()
                .expect("Should have item")
                .parse::<usize>()
                .expect("Should be a number");
            if let Entry::Vacant(e) = printing_rules.entry(right) {
                e.insert(vec![left]);
            } else {
                let a = printing_rules.get_mut(&right).expect("Should exist");
                a.push(left);
            }
        } else {
            page_updates.push(
                line.split(',')
                    .map(|x| x.parse::<usize>().expect("Should be a number"))
                    .collect_vec(),
            );
        }
    }
    (printing_rules, page_updates)
}

fn check_update(update: &[usize], printing_rules: &BTreeMap<usize, Vec<usize>>) -> bool {
    for (i, number) in update.iter().enumerate() {
        if let Some(prereqs) = printing_rules.get(number) {
            // If update contains any prereqs
            let required_prereqs = prereqs.iter().filter(|&x| update.contains(x)).collect_vec();
            if !required_prereqs.iter().all(|x| update[..i].contains(x)) {
                //                println!( "{} Expected {:?} in {:?}, remaining {:?}", number, required_prereqs, update[..i].iter(), update[i..].iter());
                return false;
            }
        }
    }
    true
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let (printing_rules, page_updates) = parse_input(input);
    let mut result = 0;
    let mut failing_updates = vec![];

    for update in page_updates {
        if !check_update(&update, &printing_rules) {
            failing_updates.push(update);
        }
    }
    for update in &mut failing_updates {
        update.sort_by(|a, b| {
            if let Some(others) = printing_rules.get(a) {
                if others.contains(b) {
                    return Ordering::Greater;
                }
            }
            if let Some(others) = printing_rules.get(b) {
                if others.contains(a) {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        });
    }

    for update in failing_updates {
        if !check_update(&update, &printing_rules) {
            todo!("Oops")
        }
        result += update[(update.len()) / 2];
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 143);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 123);
    }
}
