use std::{collections::HashMap};

use common::*;
use rayon::prelude::*;

type MonkeyName = String;
type Yells = HashMap<MonkeyName, MonkeyYell>;
type Yells2 = HashMap<MonkeyName, MonkeyYellPt2>;

#[derive(Debug, Clone)]
enum MonkeyYell {
    Equation {
        lhs: MonkeyName,
        rhs: MonkeyName,
        operation: char,
    },
    Number(isize),
}

#[derive(Debug, Clone)]
enum MonkeyYellPt2 {
    Equation {
        lhs: MonkeyName,
        rhs: MonkeyName,
        operation: char,
    },
    Number(String),
    Human,
}

trait SolveYell {
    fn get_yell_value(&self, name: &MonkeyName) -> isize;
}

impl SolveYell for Yells {
    fn get_yell_value(&self, name: &MonkeyName) -> isize {
        let current = self.get(name).unwrap();

        match current {
            MonkeyYell::Equation { lhs, rhs, operation } => {
                let lhs = self.get_yell_value(lhs);
                let rhs = self.get_yell_value(rhs);
                if name == "root" {
                    println!("{}\n{}", lhs, rhs)
                }
                match *operation {
                    '*' => lhs * rhs,
                    '-' => lhs - rhs,
                    '+' => lhs + rhs,
                    '/' => lhs / rhs,
                    _ => unreachable!(),
                }
            }
            MonkeyYell::Number(x) => *x,
        }
    }
}
trait SolveYell2 {
    fn get_yell_value(&self, name: &MonkeyName) -> String;
}

impl SolveYell2 for Yells2 {
    fn get_yell_value(&self, name: &MonkeyName) -> String {
        let current = self.get(name).unwrap();

        match current {
            MonkeyYellPt2::Equation { lhs, rhs, operation } => {
                let lhs = self.get_yell_value(lhs);
                let rhs = self.get_yell_value(rhs);
                if !lhs.contains('x') && !rhs.contains('x') {
                    // We can reduce this
                    let lhs = lhs.parse::<isize>().unwrap();
                    let rhs = rhs.parse::<isize>().unwrap();
                    let val = match *operation {
                        '*' => lhs * rhs,
                        '-' => lhs - rhs,
                        '+' => lhs + rhs,
                        '/' => lhs / rhs,
                        _ => unreachable!(),
                    };
                    val.to_string()
                } else if *operation == '+' || *operation == '-' {
                    format!("({lhs}{operation}{rhs})")
                } else {
                    format!("{lhs}{operation}{rhs}")
                }
            }
            MonkeyYellPt2::Number(x) => x.to_owned(),
            MonkeyYellPt2::Human => "x".to_string(),
        }
    }
}

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

fn part1(input: &str) -> R<isize> {
    let mut yells: Yells = HashMap::new();
    for line in input.lines() {
        let split = line.split(':').collect::<Vec<_>>();
        let name: MonkeyName = split[0].into();
        let split2 = split[1].trim().split(' ').collect::<Vec<_>>();
        if split2.len() == 1 {
            yells.insert(name, MonkeyYell::Number(split2[0].parse()?));
        } else if split2.len() == 3 {
            yells.insert(
                name,
                MonkeyYell::Equation {
                    lhs: split2[0].into(),
                    rhs: split2[2].into(),
                    operation: split2[1].chars().next().unwrap(),
                },
            );
        }
    }
    Ok(yells.get_yell_value(&"root".to_string()))
}

fn part2(input: &str) -> R<String> {
    let mut yells: Yells2 = HashMap::new();
    for line in input.lines() {
        let split = line.split(':').collect::<Vec<_>>();
        let name: MonkeyName = split[0].into();
        let split2 = split[1].trim().split(' ').collect::<Vec<_>>();
        if split2.len() == 1 {
            yells.insert(name, MonkeyYellPt2::Number(split2[0].parse()?));
        } else if split2.len() == 3 {
            yells.insert(
                name,
                MonkeyYellPt2::Equation {
                    lhs: split2[0].into(),
                    rhs: split2[2].into(),
                    operation: split2[1].chars().next().unwrap(),
                },
            );
        }
    }

    let humn = yells.get_mut("humn").unwrap();
    *humn = MonkeyYellPt2::Human;
    let root = yells.get(&"root".to_string()).unwrap();
    let (lhs_name, rhs_name) = match root {
        MonkeyYellPt2::Equation { lhs, rhs, operation: _ } => (lhs, rhs),
        MonkeyYellPt2::Number(_) => todo!(),
        MonkeyYellPt2::Human => todo!(),
    };

    Ok(format!(
        "Take this equation to https://www.mathpapa.com/algebra-calculator.html and round to the nearest int \n{}={}",
        yells.get_yell_value(lhs_name),
        yells.get_yell_value(rhs_name)
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 152);
    }
    #[test]
    fn p2_test() {
        //assert_eq!(part2(SAMPLE1).unwrap(), 301);
    }
}
