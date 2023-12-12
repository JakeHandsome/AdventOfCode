use std::collections::BTreeMap;

use common::*;

fn main() {
    let input = read_input_file_for_project_as_string!();
    {
        let _timer = Timer::new("Part 1");
        println!("Part1(original algorithm): {}", part1(&input, true).unwrap());
    }
    {
        let _timer = Timer::new("Part 1");
        println!("Part1(new algorithm): {}", part1(&input, false).unwrap());
    }
    {
        let _timer = Timer::new("Part 2");
        println!("Part2: {}", part2(&input).unwrap());
    }
    // Make a graphical ouput to explain the solution
    let input = "??#?###?????? 1,5,2"; // This is the input used to generate the graph

    // let input = unfold_line(input);
    let mut split = input.split(' ');


    let mut puzzle = split.next().unwrap().to_string();
    // End all lines with '.' to fix end cases
    puzzle.push('.');
    let key = split
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect_vec();
    let mut map = BTreeMap::new();
    _ = get_solutions(&mut map, &puzzle, &key, 0, 0, 0);
    day12::create_visual_graph(map, puzzle, key);
}


fn part1(input: &str, part1_algo: bool) -> anyhow::Result<usize> {
    if part1_algo {
        Ok(input.lines().map(solve_line).sum())
    } else {
        Ok(input.lines().map(solve_line2).sum())
    }
}

// This is a brute force that checks every possible combination, too slow for part 2
fn solve_line(input: &str) -> usize {
    let mut split = input.split(' ');
    let puzzle = split.next().unwrap();
    let num_unknown = puzzle.chars().filter(|x| *x == '?').count();
    let key = split
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect_vec();
    let mut solutions = 0;
    for i in 0..=(2usize.pow(num_unknown as u32) - 1) {
        let mut puzzle = String::from(puzzle);
        for j in (0..num_unknown).rev() {
            if (i >> j) & 1 == 1 {
                puzzle = puzzle.replacen('?', "#", 1);
            } else {
                puzzle = puzzle.replacen('?', ".", 1);
            }
        }
        let a = is_solution(puzzle.as_str(), &key);
        if a {
            solutions += 1;
        }
    }

    solutions
}

fn is_solution(as_str: &str, key: &[usize]) -> bool {
    let mut last_char = '.';
    let mut key_index = 0;
    let mut current_count = 0;
    // Make sure every sequence ends with '.'
    for char in as_str.chars().chain(['.'].into_iter()) {
        match (last_char, char) {
            // Empty space to empty space do nothing
            ('.', '.') => (),
            // Filled to empty,
            ('#', '.') => {
                // If the key exists and matches, continue
                if let Some(count) = key.get(key_index) {
                    if current_count == *count {
                        current_count = 0;
                        key_index += 1;
                    } else {
                        return false;
                    }
                } else {
                    // If a match fais return false
                    return false;
                }
            }
            // next is filled, increment current count, make sure it doesn't excede max count
            (_, '#') => {
                current_count += 1;
                if let Some(count) = key.get(key_index) {
                    if current_count > *count {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => unreachable!(),
        }
        last_char = char;
    }
    // Make sure we matched all keys
    key_index >= key.len()
}

fn part2(input: &str) -> anyhow::Result<usize> {
    Ok(input.lines().map(|x| solve_line2(&unfold_line(x))).sum())
}

// This solution steps through each char as a tree and looks for repeated solutions so it doesn't
// need to re calculate
fn solve_line2(input: &str) -> usize {
    let mut split = input.split(' ');
    let mut puzzle = split.next().unwrap().to_string();
    // End all lines with '.' to fix end cases
    puzzle.push('.');
    let key = split
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect_vec();
    let mut map = BTreeMap::new();
    get_solutions(&mut map, &puzzle, &key, 0, 0, 0)
}

/// Recursive function that steps through the puzzle like a tree. If a '.' or '#' is found there is
/// a single path, but it diverges if a '?' is found. If the function finds a similar state, it will
/// use the previously calculated value.
///
/// `map` HashMap of solved locations
/// `puzzle` the string input of #/./?
/// `key` the puzzle key ex: `[2,2]`
/// `puzzle_index` the index into the puzzle
/// `key_index` the index into the key
/// `current_count` the number of `#` in a row
///
/// The map contains 3 parts: index into puzzle, index into key, current number of `#` in a row.
/// If all of these variables are the same, the calculation will be the same so it can be saved in
/// map
///
/// There are also optimaztions to end the puzzle cheking early if it is impossible to find a
/// solution given the current input
///
/// # Example
/// ```text
/// Given the puzzle:
/// ???.## 2,2
///
/// We can be at position 4 at two ways
/// ##..##
///    ^
/// .##.##
///    ^
/// In both these cases, puzzle index is 3, key index is 2, current count is 0 so the answer can be
/// re-used
///
/// In the other case
/// #.#.##
///  ^
/// The current_count(1) != key[0](2) when there was a transition from # to . so it is impossible for this string
/// to have a solution so it exists early. Similar optimization exists when key_index > key.len()
/// ```
pub(crate) fn get_solutions(
    map: &mut BTreeMap<(usize, usize, usize), usize>,
    puzzle: &str,
    key: &[usize],
    puzzle_index: usize,
    key_index: usize,
    current_count: usize,
) -> usize {
    if map.contains_key(&(puzzle_index, key_index, current_count)) {
        *map.get(&(puzzle_index, key_index, current_count)).unwrap()
    } else {
        if puzzle_index == puzzle.len() {
            // If we are at the last character, the key index is at the end and current count is 0, we
            // found a solution
            if key_index == key.len() && current_count == 0 {
                map.insert((puzzle_index, key_index, current_count), 1);
                return 1;
            } else {
                map.insert((puzzle_index, key_index, current_count), 0);
                // Last index, no solution return 0
                return 0;
            }
        }
        let current_char = puzzle.as_bytes()[puzzle_index] as char;
        let mut solution = match current_char {
            // case for '#'
            '#' | '?' => {
                let current_count = current_count + 1;
                if let Some(max_count_for_index) = key.get(key_index) {
                    if current_count <= *max_count_for_index {
                        // Keep going
                        get_solutions(map, puzzle, key, puzzle_index + 1, key_index, current_count)
                    } else {
                        // No more solutions found because the current count is too low, not enough
                        // consecutive `#`
                        0
                    }
                } else {
                    // No more solutions found, key_index is out of range
                    0
                }
            }
            _ => 0,
        };
        solution += match current_char {
            // case for .
            '.' | '?' => {
                // If current count > 0 last char was '#'
                if current_count != 0 {
                    if let Some(expected_count_for_index) = key.get(key_index) {
                        // If the current count was max for this index, we could still have a match
                        // so increment the key_index and continue
                        if current_count == *expected_count_for_index {
                            let key_index = key_index + 1;
                            let current_count = 0;
                            get_solutions(map, puzzle, key, puzzle_index + 1, key_index, current_count)
                        } else {
                            //  The current count is not same as the expected there are no more
                            //  solutions in this path
                            0
                        }
                    } else {
                        // key_index is too far, no more solutions
                        0
                    }
                } else {
                    get_solutions(map, puzzle, key, puzzle_index + 1, key_index, current_count)
                }
            }
            _ => 0,
        };
        map.insert((puzzle_index, key_index, current_count), solution);
        solution
    }
}

fn unfold_line(input: &str) -> String {
    let mut split = input.split(' ');
    let puzzle = split.next().unwrap();
    let key = split.next().unwrap();
    format!("{puzzle}?{puzzle}?{puzzle}?{puzzle}?{puzzle} {key},{key},{key},{key},{key}")
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1, true).unwrap(), 21);
    }
    #[test]
    fn p1_testl1() {
        assert_eq!(solve_line(SAMPLE1.lines().nth(0).unwrap()), 1);
    }
    #[test]
    fn p1_testl2() {
        assert_eq!(solve_line(SAMPLE1.lines().nth(1).unwrap()), 4);
    }
    #[test]
    fn p1_testl3() {
        assert_eq!(solve_line(SAMPLE1.lines().nth(2).unwrap()), 1);
    }
    #[test]
    fn p1_testl4() {
        assert_eq!(solve_line(SAMPLE1.lines().nth(3).unwrap()), 1);
    }
    #[test]
    fn p1_testl5() {
        assert_eq!(solve_line(SAMPLE1.lines().nth(4).unwrap()), 4);
    }
    #[test]
    fn p1_testl6() {
        assert_eq!(solve_line(SAMPLE1.lines().nth(5).unwrap()), 10);
    }
    #[test]
    fn p1_2_testl1() {
        assert_eq!(solve_line2(SAMPLE1.lines().nth(0).unwrap()), 1);
    }
    #[test]
    fn p1_2_testl2() {
        assert_eq!(solve_line2(SAMPLE1.lines().nth(1).unwrap()), 4);
    }
    #[test]
    fn p1_2_testl3() {
        assert_eq!(solve_line2(SAMPLE1.lines().nth(2).unwrap()), 1);
    }
    #[test]
    fn p1_2_testl4() {
        assert_eq!(solve_line2(SAMPLE1.lines().nth(3).unwrap()), 1);
    }
    #[test]
    fn p1_2_testl5() {
        assert_eq!(solve_line2(SAMPLE1.lines().nth(4).unwrap()), 4);
    }
    #[test]
    fn p1_2_testl6() {
        assert_eq!(solve_line2(SAMPLE1.lines().nth(5).unwrap()), 10);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 525152);
    }
    #[test]
    fn p2_testl1() {
        let a = unfold_line(SAMPLE1.lines().nth(0).unwrap());
        assert_eq!(
            &a,
            "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3"
        );
        assert_eq!(solve_line2(&a), 1);
    }
    #[test]
    fn p2_testl2() {
        assert_eq!(solve_line2(&unfold_line(SAMPLE1.lines().nth(1).unwrap())), 16384);
    }
    #[test]
    fn p2_testl3() {
        assert_eq!(solve_line2(&unfold_line(SAMPLE1.lines().nth(2).unwrap())), 1);
    }
    #[test]
    fn p2_testl4() {
        assert_eq!(solve_line2(&unfold_line(SAMPLE1.lines().nth(3).unwrap())), 16);
    }
    #[test]
    fn p2_testl5() {
        assert_eq!(solve_line2(&unfold_line(SAMPLE1.lines().nth(4).unwrap())), 2500);
    }
    #[test]
    fn p2_testl6() {
        assert_eq!(solve_line2(&unfold_line(SAMPLE1.lines().nth(5).unwrap())), 506250);
    }
}
