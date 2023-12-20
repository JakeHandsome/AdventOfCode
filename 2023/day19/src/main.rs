use std::{any, collections::HashMap};

use common::{
    winnow::{
        ascii::{alpha1, dec_uint},
        combinator::{rest, separated, separated_pair},
        stream::AsChar,
        token::{any, one_of, take_till, take_until0, take_until1, take_while},
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

#[derive(Debug, Eq, PartialEq)]
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
    let (_, conditions, _) = ("{", parse_conditions, "}").parse_next(input)?;
    Ok((name, conditions))
}

fn parse_conditions<'a>(input: &mut &'a str) -> PResult<Vec<Condition<'a>>> {
    let conditions: Vec<Condition<'a>> = separated(.., parse_condition, ",").parse_next(input)?;
    Ok(conditions)
}

fn parse_condition<'a>(input: &mut &'a str) -> PResult<Condition<'a>> {
    if input.contains(':') {
        let (comparison, destination) =
            separated_pair(parse_comparison, ":", take_while(.., AsChar::is_alpha)).parse_next(input)?;
        Ok(Condition {
            destination,
            comparison: Some(comparison),
        })
    } else {
        let destination = take_while(.., AsChar::is_alpha).parse_next(input)?;
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
    let a = separated_pair(one_of(['x', 'm', 'a', 's']), "=", dec_uint).parse_next(a)?;
    Ok(a)
}

trait SolvePart {
    fn rating(&self, workflows: &HashMap<&str, Vec<Condition>>) -> usize;
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
}

const START: &str = "in";
fn part1(input: &str) -> anyhow::Result<usize> {
    let mut workflows = HashMap::new();
    let mut parts = vec![];
    let mut empty_hit = false;
    for mut line in &mut input.lines() {
        if !empty_hit {
            if line.is_empty() {
                empty_hit = true;
                continue;
            }
            let (key, value) = parse_workflow(&mut line).map_err(|e| anyhow::anyhow!(e))?;
            workflows.insert(key, value);
        } else {
            let part = parse_part(&mut line).map_err(|e| anyhow::anyhow!(e))?;
            parts.push(part);
        }
    }
    let ans = parts
        .into_iter()
        .fold(0usize, |acc, part| acc + part.rating(&workflows));
    Ok(ans)
}
fn part2(input: &str) -> anyhow::Result<usize> {
    let mut workflows = HashMap::new();
    for mut line in &mut input.lines() {
        if line.is_empty() {
            break;
        }
        let (key, value) = parse_workflow(&mut line).map_err(|e| anyhow::anyhow!(e))?;
        workflows.insert(key, value);
    }
    Err(AdventOfCodeError::UnimplementedError)?
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
