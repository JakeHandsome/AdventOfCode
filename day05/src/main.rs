use common::read_input_file_for_project_as_string;
fn main() {
    let input = read_input_file_for_project_as_string!();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &String) -> String {
    let mut iter = input.lines();
    let mut top_input = "".to_string();
    for line in iter.by_ref() {
        if line.is_empty() {
            // remove final new line
            top_input.remove(top_input.len() - 1usize);
            break;
        }
        top_input += line;
        top_input += "\n";
    }
    let mut state = parse_initial_state(top_input);
    for line in iter {
        let mut split = line.split(' ');
        let quantity = split.nth(1).unwrap().parse::<usize>().unwrap();
        let start = split.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let dest = split.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        for _ in 0..quantity {
            let intermediate = state[start].pop().unwrap();
            state[dest].push(intermediate);
        }
    }
    state.iter().map(|x| x.last().unwrap()).collect()
}

fn part2(input: &String) -> String {
    let mut iter = input.lines();
    let mut top_input = "".to_string();
    for line in iter.by_ref() {
        if line.is_empty() {
            // remove final new line
            top_input.remove(top_input.len() - 1usize);
            break;
        }
        top_input += line;
        top_input += "\n";
    }
    let mut state = parse_initial_state(top_input);
    for line in iter {
        let mut split = line.split(' ');
        let quantity = split.nth(1).unwrap().parse::<usize>().unwrap();
        let start = split.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let dest = split.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let mut intermediate = vec![];
        for _ in 0..quantity {
            intermediate.push(state[start].pop().unwrap());
        }
        intermediate.reverse();
        for a in intermediate {
            state[dest].push(a);
        }
    }
    state.iter().map(|x| x.last().unwrap()).collect()
}
fn parse_initial_state(top_input: String) -> Vec<Vec<char>> {
    let mut iter = top_input.lines().peekable();
    let line = iter.peek().unwrap();
    let width = (line.len() + 1) / 4;
    println!("width = {}", width);
    let mut initial_state = vec![vec![]; width];
    for line in iter {
        if !line.contains('[') {
            continue;
        }
        let line = line.to_owned() + " ";
        let chars: Vec<_> = line.chars().collect();
        for (index, a) in chars.chunks(4).enumerate() {
            if a[1] != ' ' {
                initial_state[index].push(a[1]);
            }
        }
    }
    for stack in &mut initial_state {
        stack.reverse();
    }
    initial_state
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(&(SAMPLE).to_string()), "CMZ")
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(&(SAMPLE).to_string()), "MCD")
    }
}
