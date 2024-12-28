use std::{
    cmp::{min_by, Ordering},
    collections::{BTreeMap, HashMap},
    usize, vec,
};

use common::*;
use winnow::error::InputError;

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

#[derive(Clone, Debug)]
struct KeyPad {
    grid: Grid,
    char_positions: BTreeMap<char, (usize, usize)>,
}

impl KeyPad {
    fn find_all_shortest_paths(&self) -> HashMap<(char, char), Vec<Vec<char>>> {
        let mut map = HashMap::new();
        for perm in self.grid.inner.chars().filter(|c| *c != ' ').combinations(2) {
            let (a, b) = (perm[0], perm[1]);
            let mut paths = self.travel(self.grid.find_char(a).unwrap(), self.grid.find_char(b).unwrap(), vec![]);
            map.insert((a, b), paths.clone());
            {
                paths.iter_mut().for_each(|x| {
                    let a = x.pop().unwrap();
                    x.reverse();
                    x.iter_mut().for_each(|x| {
                        *x = match x {
                            'v' => '^',
                            '^' => 'v',
                            '<' => '>',
                            '>' => '<',
                            'A' => 'A',
                            _ => unreachable!("Invalid character"),
                        }
                    });
                    x.push(a);
                });
                map.insert((b, a), paths);
            }
        }
        map
    }
    fn check_new_point(&self, row: usize, col: usize) -> Option<(usize, usize)> {
        if let Some(' ') | None = self.grid.get_char(row, col) {
            None
        } else {
            Some((row, col))
        }
    }
    fn travel(&self, current: (usize, usize), dest: (usize, usize), mut path: Vec<char>) -> Vec<Vec<char>> {
        let neighbors = [
            match current.0.cmp(&dest.0) {
                // Need to add
                Ordering::Less => self.check_new_point(current.0 + 1, current.1).map(|x| ('v', x)),
                // Need to do nothing
                Ordering::Equal => None,
                // Need to sub
                Ordering::Greater => self
                    .check_new_point(current.0.wrapping_sub(1), current.1)
                    .map(|x| ('^', x)),
            },
            match current.1.cmp(&dest.1) {
                Ordering::Less => self.check_new_point(current.0, current.1 + 1).map(|x| ('>', x)),
                Ordering::Equal => None,
                Ordering::Greater => self
                    .check_new_point(current.0, current.1.wrapping_sub(1))
                    .map(|x| ('<', x)),
            },
        ];
        match neighbors {
            [None, None] => {
                assert_eq!(current, dest, "If there are no valid neighbors, must be at destination");
                path.push('A');
                vec![path]
            }
            [None, Some(x)] | [Some(x), None] => {
                path.push(x.0);
                self.travel(x.1, dest, path)
            }
            [Some(x), Some(y)] => {
                let mut path2 = path.clone();
                path2.push(x.0);
                let mut x = self.travel(x.1, dest, path2);
                path.push(y.0);
                x.append(&mut self.travel(y.1, dest, path));
                x
            }
        }
    }
}

impl KeyPad {
    fn new(input: &str) -> Self {
        let grid = Grid::new(input);
        Self {
            char_positions: grid.char_positions(),
            grid,
        }
    }
}

fn get_initial() -> (KeyPad, KeyPad, KeyPad) {
    let numpad = KeyPad::new(
        r#"789
456
123
 0A"#,
    );
    let keypad = KeyPad::new(
        r#" ^A
<v>"#,
    );

    (numpad, keypad.clone(), keypad)
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let mut res = 0;
    let (mut numpad, mut keypad1, mut keypad2) = get_initial();
    let numpad_paths = numpad.find_all_shortest_paths();
    let keypad_paths = keypad1.find_all_shortest_paths();
    println!("\n{keypad_paths:?}");
    for line in input.lines() {
        let mut out = vec![];
        let input = line.chars().collect::<Vec<_>>();
        let expanded = expand(input, &numpad_paths);
        for input in expanded {
            let expanded2 = expand(input, &keypad_paths);
            for input in expanded2 {
                let expanded3 = expand(input, &keypad_paths);
                out.push(expanded3);
            }
        }
        let mut out = out.into_iter().flatten().collect_vec();
        out.sort_by_key(|x1| x1.len());
        let min_len = out[0].len();
        let number: usize = line[..line.len() - 1].parse().unwrap();
        res += dbg!(min_len * number);
    }
    Ok(res)
}

fn expand(input: Vec<char>, a: &HashMap<(char, char), Vec<Vec<char>>>) -> Vec<Vec<char>> {
    let mut paths = vec![];
    let mut position = 'A';
    for new in input {
        let new_paths = if position == new {
            // Same character, just press A
            vec![vec!['A']]
        } else {
            a[&(position, new)].clone()
        };
        if paths.is_empty() {
            // Initialize the paths
            paths = new_paths;
        } else {
            // For each existing path, append each new path (and clone if there are more than 1)
            paths = paths
                .into_iter()
                .flat_map(|p| {
                    let mut new = vec![];
                    for mut np in new_paths.clone() {
                        let mut a = p.clone();
                        a.append(&mut np);
                        new.push(a);
                    }
                    new
                })
                .collect::<Vec<_>>();
        }
        position = new;
    }
    paths
}

fn part2(input: &str) -> anyhow::Result<usize> {
    Err(AdventOfCodeError::UnimplementedError)?
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"029A
980A
179A
456A
379A"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 126384);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 0);
    }
}
