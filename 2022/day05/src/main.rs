use common::{read_input_file_for_project_as_string, R};
use std::{collections::VecDeque, str::Lines, vec};

fn main() {
    let input = read_input_file_for_project_as_string!();
    println!("Part1: {:#?}", part1(&input));
    println!("Part2: {:#?}", part2(&input));
}

fn part1(input: &str) -> R<String> {
    let mut iter = input.lines();
    let top_input = get_top_input(&mut iter);
    let mut state = parse_initial_state(top_input);
    for line in iter {
        // Parse out the quant,start and dest
        let (quantity, start, dest) = parse_instruction(line)?;
        for _ in 0..quantity {
            // Pop from the start and push to the end
            let intermediate = state[start].pop().unwrap();
            state[dest].push(intermediate);
        }
    }
    // convert char array into String
    Ok(state.iter().map(|x| x.last().unwrap()).collect())
}

fn part2(input: &str) -> R<String> {
    let mut iter = input.lines();
    let top_input = get_top_input(&mut iter);
    let mut state = parse_initial_state(top_input);
    for line in iter {
        let mut intermediate: VecDeque<char> = VecDeque::new();
        let (quantity, start, dest) = parse_instruction(line)?;
        for _ in 0..quantity {
            // pop to an intermediate vec_deque in the front to reverse the order
            intermediate.push_front(state[start].pop().unwrap());
        }
        // add each element
        for a in intermediate {
            state[dest].push(a);
        }
    }
    Ok(state.iter().map(|x| x.last().unwrap()).collect())
}
fn get_top_input(iter: &mut Lines) -> String {
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
    top_input
}

fn parse_instruction(instruction: &str) -> R<(usize, usize, usize)> {
    let mut split = instruction.split(' ');
    let quantity = split.nth(1).unwrap().parse::<usize>()?;
    let start = split.nth(1).unwrap().parse::<usize>()? - 1;
    let dest = split.nth(1).unwrap().parse::<usize>()? - 1;
    Ok((quantity, start, dest))
}

fn parse_initial_state(top_input: String) -> Vec<Vec<char>> {
    let mut iter = top_input.lines().rev().peekable();
    let line = iter.peek().unwrap();
    let width = (line.len() + 1) / 4;
    // Reverse so we read from the bottom of the stack to the top
    // Determine width to know how many vecs to create
    let mut initial_state = vec![vec![]; width];
    for line in iter {
        // If the line doesn't have [ its the index and can be ignored
        if !line.contains('[') {
            continue;
        }
        // Add an extra space so we can use chunk size of 4
        let line = line.to_owned() + " ";
        let chars: Vec<_> = line.chars().collect();
        for (index, a) in chars.chunks(4).enumerate() {
            // If we grab 4 chunks index 1 will always be the letter
            if a[1] != ' ' {
                initial_state[index].push(a[1]);
            }
        }
    }
    initial_state
}

#[cfg(test)]
mod day5_tests {
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
        assert_eq!(part1(SAMPLE).unwrap(), "CMZ")
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE).unwrap(), "MCD")
    }
}
