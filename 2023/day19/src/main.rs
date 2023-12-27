use std::collections::HashMap;

use common::{
    winnow::{
        ascii::{alpha1, dec_uint},
        combinator::{separated, separated_pair},
        token::{one_of, take_until1},
        PResult, Parser,
    },
    *,
};

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

#[derive(Debug, PartialEq, Eq)]
struct Condition<'a> {
    destination: &'a str,
    comparison: Option<Comparison>,
}

impl<'a> Condition<'a> {
    fn matches(&self, part: &Part) -> bool {
        if let Some(comparison) = &self.comparison {
            let part_value = part.get(&comparison.key).unwrap();
            match comparison.operation {
                Operation::Greater => part_value > &comparison.value,
                Operation::Less => part_value < &comparison.value,
            }
        } else {
            true
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Comparison {
    key: char,
    operation: Operation,
    value: usize,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Operation {
    Greater,
    Less,
}
type Part = HashMap<char, usize>;

fn parse_workflow<'a>(input: &mut &'a str) -> PResult<(&'a str, Vec<Condition<'a>>)> {
    let name = take_until1("{").parse_next(input)?;
    let (_, conditions, _) = ('{', parse_conditions, '}').parse_next(input)?;
    Ok((name, conditions))
}

fn parse_conditions<'a>(input: &mut &'a str) -> PResult<Vec<Condition<'a>>> {
    separated(.., parse_condition, ",").parse_next(input)
}

fn parse_condition<'a>(input: &mut &'a str) -> PResult<Condition<'a>> {
    if input.contains(':') {
        let (comparison, destination) = separated_pair(parse_comparison, ':', alpha1).parse_next(input)?;
        Ok(Condition {
            destination,
            comparison: Some(comparison),
        })
    } else {
        let destination = alpha1.parse_next(input)?;
        Ok(Condition {
            destination,
            comparison: None,
        })
    }
}

fn parse_comparison(input: &mut &str) -> PResult<Comparison> {
    let (key, operation, value): (char, char, u32) =
        (one_of(['x', 'm', 'a', 's']), one_of(['>', '<']), dec_uint).parse_next(input)?;
    let operation = if operation == '>' {
        Operation::Greater
    } else {
        Operation::Less
    };
    Ok(Comparison {
        key,
        operation,
        value: value as usize,
    })
}

fn parse_part(input: &mut &str) -> PResult<Part> {
    let mut part = Part::new();
    // Get text between { }
    let (_, mut inner) = ("{", take_until1("}")).parse_next(input)?;
    // Parse the key,value separted by ,
    let a: Vec<(char, u32)> = separated(.., parse_part_key_value, ",").parse_next(&mut inner)?;
    for (key, value) in a {
        part.insert(key, value as usize);
    }

    Ok(part)
}

fn parse_part_key_value(a: &mut &str) -> PResult<(char, u32)> {
    // Extract key,value from key=value
    separated_pair(one_of(['x', 'm', 'a', 's']), "=", dec_uint).parse_next(a)
}

trait SolvePart {
    fn rating(&self, workflows: &HashMap<&str, Vec<Condition>>) -> usize;
    fn rating2(&self, workflows: &HashMap<String, Part2Condition>) -> usize;
}

impl SolvePart for Part {
    fn rating(&self, workflows: &HashMap<&str, Vec<Condition>>) -> usize {
        let mut current_workflow = START;
        loop {
            for condition in &workflows[current_workflow] {
                if condition.matches(self) {
                    current_workflow = condition.destination;
                    if current_workflow == "A" || current_workflow == "R" {
                        if current_workflow == "A" {
                            return self.values().sum::<usize>();
                        } else {
                            return 0;
                        }
                    }
                    break;
                }
            }
        }
    }

    fn rating2(&self, workflows: &HashMap<String, Part2Condition>) -> usize {
        let mut current_workflow = START.to_string();
        loop {
            let condition = &workflows[&current_workflow];
            if condition.matches(self) {
                current_workflow = condition.destination_if_true.clone();
            } else {
                current_workflow = condition.destination_else.clone();
            }
            if &current_workflow == "A" || &current_workflow == "R" {
                if &current_workflow == "A" {
                    return self.values().sum::<usize>();
                } else {
                    return 0;
                }
            }
        }
    }
}

const START: &str = "in";
fn part1(input: &str) -> anyhow::Result<usize> {
    let mut workflows = HashMap::new();
    let mut workflows_remap = HashMap::new();
    let mut parts = vec![];
    let mut empty_hit = false;
    for mut line in &mut input.lines() {
        if !empty_hit {
            if line.is_empty() {
                empty_hit = true;
                continue;
            }
            let (key, value) = parse_workflow(&mut line).map_err(|e| anyhow::anyhow!(e))?;
            let pairs = remap_pt2(key, &value, 0);
            workflows.insert(key, value);
            for (key, value) in pairs {
                workflows_remap.insert(key, value);
            }
        } else {
            let part = parse_part(&mut line).map_err(|e| anyhow::anyhow!(e))?;
            parts.push(part);
        }
    }

    let ans = parts
        .clone()
        .into_iter()
        .fold(0usize, |acc, part| acc + part.rating(&workflows));
    let ans_remap = parts
        .into_iter()
        .fold(0usize, |acc, part| acc + part.rating2(&workflows_remap));
    // Verify the same logic works for pt1 even with the remap
    assert_eq!(ans, ans_remap);
    Ok(ans)
}

#[derive(Debug)]
struct Part2Condition {
    key: char,
    operation: Operation,
    value: usize,
    destination_if_true: String,
    destination_else: String,
}

impl Part2Condition {
    fn matches(&self, part: &Part) -> bool {
        match self.operation {
            Operation::Greater => part.get(&self.key) > Some(&self.value),
            Operation::Less => part.get(&self.key) < Some(&self.value),
        }
    }
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let mut workflows = HashMap::new();
    for mut line in &mut input.lines() {
        if line.is_empty() {
            break;
        }
        let (key, value) = parse_workflow(&mut line).map_err(|e| anyhow::anyhow!(e))?;
        // Rework inputs so each map only has a single if/else
        let pairs = remap_pt2(key, &value, 0);
        for (key, value) in pairs {
            workflows.insert(key, value);
        }
    }
    let start_limits = HashMap::from([('x', (1, 4000)), ('m', (1, 4000)), ('a', (1, 4000)), ('s', (1, 4000))]);
    let results = solve_pt2(&workflows, start_limits, START.to_string());
    let mut ans = 0;
    // For each limit calculate product of the valid range of values and sum them up
    for result in results {
        let mut values = 1usize;
        for x in ['x', 'm', 'a', 's'] {
            if let Some((min, max)) = result.get(&x) {
                values *= max - min + 1;
            }
        }
        ans += values;
    }
    Ok(ans)
}

/// The idea here is to walk all decisions trees because the hashmap now forms a binary tree. At
/// each step we calculate the min and max for each key. If the tree ends ins "A" we return those
/// limits. Otherwise empty is returned.
fn solve_pt2(
    workflows: &HashMap<String, Part2Condition>,
    current_limits: HashMap<char, (usize, usize)>,
    key: String,
) -> Vec<HashMap<char, (usize, usize)>> {
    if key == "R" {
        vec![]
    } else if key == "A" {
        vec![current_limits]
    } else {
        let workflow = &workflows[&key];
        let mut true_limit = current_limits.clone();
        let mut false_limit = current_limits;
        if workflow.operation == Operation::Greater {
            let t = true_limit.get_mut(&workflow.key).unwrap();
            t.0 = t.0.max(workflow.value + 1);
            let f = false_limit.get_mut(&workflow.key).unwrap();
            f.1 = f.1.min(workflow.value);
        } else {
            let t = true_limit.get_mut(&workflow.key).unwrap();
            t.1 = t.1.min(workflow.value - 1);
            let f = false_limit.get_mut(&workflow.key).unwrap();
            f.0 = f.0.max(workflow.value);
        }
        // Continue down the true and path, updating limits as we go
        let mut results = solve_pt2(workflows, true_limit, workflow.destination_if_true.clone());
        results.append(&mut solve_pt2(
            workflows,
            false_limit,
            workflow.destination_else.clone(),
        ));
        results
    }
}

fn remap_pt2(key: &str, values: &[Condition], depth: usize) -> Vec<(String, Part2Condition)> {
    if values.len() == 2 {
        let cond = Part2Condition {
            key: values[0].comparison.unwrap().key,
            operation: values[0].comparison.unwrap().operation,
            value: values[0].comparison.unwrap().value,
            destination_if_true: values[0].destination.to_string(),
            destination_else: values[1].destination.to_string(),
        };
        let key = key.to_string();
        vec![(key, cond)]
    } else {
        let mut ret = vec![];
        let key = key.to_string();
        let next = format!("{}{}", key, depth + 1);
        let cond = Part2Condition {
            key: values[0].comparison.unwrap().key,
            operation: values[0].comparison.unwrap().operation,
            value: values[0].comparison.unwrap().value,
            destination_if_true: values[0].destination.to_string(),
            destination_else: next.clone(),
        };
        ret.push((key, cond));
        ret.append(&mut remap_pt2(next.as_str(), &values[1..], depth + 1));
        ret
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    const SAMPLE1: &str = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 19114);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 167409079868000);
    }
    #[test]
    fn parse_comparison_test() {
        assert_eq!(
            parse_comparison(&mut "x>123").unwrap(),
            Comparison {
                key: 'x',
                operation: Operation::Greater,
                value: 123
            }
        );
        assert_eq!(
            parse_comparison(&mut "x<123").unwrap(),
            Comparison {
                key: 'x',
                operation: Operation::Less,
                value: 123
            }
        );
    }
    #[test]
    fn parse_condition_test() {
        assert_eq!(
            parse_condition(&mut "x<123:B").unwrap(),
            Condition {
                destination: "B",
                comparison: Some(Comparison {
                    key: 'x',
                    operation: Operation::Less,
                    value: 123
                })
            }
        );
        assert_eq!(
            parse_condition(&mut "A").unwrap(),
            Condition {
                destination: "A",
                comparison: None
            }
        );
    }
    #[test]
    fn parse_conditions_test() {
        assert_eq!(
            parse_conditions(&mut "a>1716:R,A").unwrap(),
            vec![
                Condition {
                    destination: "R",
                    comparison: Some(Comparison {
                        key: 'a',
                        operation: Operation::Greater,
                        value: 1716
                    })
                },
                Condition {
                    destination: "A",
                    comparison: None
                }
            ]
        )
    }
    #[test]
    fn parse_workflow_test() {
        assert_eq!(
            parse_workflow(&mut "pv{a>1716:R,A}").unwrap(),
            (
                "pv",
                vec![
                    Condition {
                        destination: "R",
                        comparison: Some(Comparison {
                            key: 'a',
                            operation: Operation::Greater,
                            value: 1716
                        })
                    },
                    Condition {
                        destination: "A",
                        comparison: None
                    }
                ]
            )
        )
    }
}
