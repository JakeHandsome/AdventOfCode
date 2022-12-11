use common::*;

fn main() {
    let input = read_input_file_for_project_as_string!();
    println!("Part1: {}", part1(&input).unwrap());
    println!("Part2: {}", part2(&input).unwrap());
}

#[derive(Debug)]
enum OperationType {
    Add,
    Muliply,
}

#[derive(Debug)]
enum OperationValue {
    Number(usize),
    Old,
}

#[derive(Debug)]
struct Operation {
    operation_type: OperationType,
    value: OperationValue,
}
impl Operation {
    fn new(input: &str) -> Self {
        let split = input.trim().split(' ').collect::<Vec<_>>();
        let operation_type = match split.get(4).unwrap().to_owned() {
            "*" => OperationType::Muliply,
            "+" => OperationType::Add,
            _ => unreachable!(),
        };
        let value = match split.last() {
            Some(x) if x.parse::<usize>().is_ok() => OperationValue::Number(x.parse::<usize>().unwrap()),
            Some(_) => OperationValue::Old,
            None => unreachable!(),
        };
        Operation { operation_type, value }
    }
}

#[derive(Debug)]
struct Monkey {
    _index: usize,
    held_items: Vec<usize>,
    operation: Operation,
    divisor: usize,
    if_true: usize,
    if_false: usize,
    inspections: usize,
}

impl Monkey {
    fn new(monkey: &[&str]) -> R<Self> {
        let index = monkey[0][7..=7].parse::<usize>()?;
        let starting_items = monkey[1]
            .trim()
            .split(':')
            .last()
            .unwrap()
            .split(',')
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let operation = Operation::new(monkey[2]);
        let divisor = monkey[3].trim().split(' ').last().unwrap().parse::<usize>()?;
        let if_true = monkey[4].trim().split(' ').last().unwrap().parse::<usize>()?;
        let if_false = monkey[5].trim().split(' ').last().unwrap().parse::<usize>()?;
        Ok(Monkey {
            _index: index,
            held_items: starting_items,
            operation,
            divisor,
            if_true,
            if_false,
            inspections: 0,
        })
    }
}

fn execute_round(monkeys: &mut Vec<Monkey>, common_divisor: Option<usize>) {
    for i in 0..monkeys.len() {
        let mut val_to_push = vec![];
        let mut monkey_to_receive = vec![];
        {
            let monkey = monkeys.get_mut(i).unwrap();
            for item in monkey.held_items.iter_mut() {
                monkey.inspections += 1;
                let value = match monkey.operation.value {
                    OperationValue::Number(x) => x,
                    OperationValue::Old => *item,
                };
                match monkey.operation.operation_type {
                    OperationType::Add => *item += value,
                    OperationType::Muliply => *item *= value,
                };
                match common_divisor {
                    // Reduce all items by the common divisor if it is set
                    Some(x) => *item %= x,
                    None => *item /= 3,
                }
                val_to_push.push(*item);
                if *item % monkey.divisor == 0 {
                    monkey_to_receive.push(monkey.if_true);
                } else {
                    monkey_to_receive.push(monkey.if_false);
                }
            }
            monkey.held_items.clear();
        }
        for (i, val) in val_to_push.into_iter().enumerate() {
            monkeys.get_mut(monkey_to_receive[i]).unwrap().held_items.push(val);
        }
    }
}

fn part1(input: &str) -> R<usize> {
    let mut vec = input.lines().collect::<Vec<&str>>();
    vec.push("");
    let iter = vec.chunks(7);
    let mut monkeys = vec![];
    for monkey in iter {
        monkeys.push(Monkey::new(monkey)?);
    }

    for _round in 0..20 {
        execute_round(&mut monkeys, None);
    }
    let mut inspections = monkeys.into_iter().map(|x| x.inspections).collect::<Vec<_>>();
    inspections.sort_by(|a, b| b.cmp(a));
    Ok(inspections[0] * inspections[1])
}

fn part2(input: &str) -> R<usize> {
    let mut vec = input.lines().collect::<Vec<&str>>();
    vec.push("");
    let iter = vec.chunks(7);
    let mut monkeys = vec![];
    for monkey in iter {
        monkeys.push(Monkey::new(monkey)?);
    }
    // All the divisors multiplied together
    let common_divisor: usize = monkeys.iter().map(|m| m.divisor).product();
    for _round in 0..10_000 {
        execute_round(&mut monkeys, Some(common_divisor));
    }
    let mut inspections = monkeys.into_iter().map(|x| x.inspections).collect::<Vec<_>>();
    inspections.sort_by(|a, b| b.cmp(a));
    Ok(inspections[0] * inspections[1])
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 10605);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 2713310158);
    }
}
