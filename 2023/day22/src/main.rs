use std::collections::HashSet;

use common::{
    winnow::{ascii::dec_uint, prelude::*, seq, PResult},
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: u64,
    y: u64,
    z: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Brick {
    start: Position,
    end: Position,
}

fn brick_from_str(input: &mut &str) -> PResult<Brick> {
    seq!(Brick{
        start: position_from_str,
        _:'~',
        end: position_from_str,
    })
    .parse_next(input)
}

fn position_from_str(input: &mut &str) -> PResult<Position> {
    seq!(Position{
        x:dec_uint,
        _:',',
        y:dec_uint,
        _:',',
        z:dec_uint
    })
    .parse_next(input)
}

fn bricks_fall_single_step(bricks: &[Brick]) -> (bool, Vec<Brick>) {
    let mut fell = false;

    // Get occupied spaces, only care about the top most Z tiles, since we cannot fall through a
    // brick
    let mut occupied_spaces = HashSet::new();
    for Brick { start, end } in bricks {
        for x in start.x..=end.x {
            for y in start.y..=end.y {
                occupied_spaces.insert((x, y, end.z));
            }
        }
    }

    let mut new_bricks = vec![];
    for Brick { start, end } in bricks {
        let mut supported = false;
        'outer: for x in start.x..=end.x {
            for y in start.y..=end.y {
                let solid = {
                    // Ground is always solid
                    if start.z - 1 == 0 {
                        true
                    } else {
                        // if space is occupied, it's solid
                        occupied_spaces.contains(&(x, y, start.z - 1))
                    }
                };
                if solid {
                    supported = true;
                    break 'outer;
                }
            }
        }
        if !supported {
            fell = true;
            new_bricks.push(Brick {
                start: Position {
                    x: start.x,
                    y: start.y,
                    z: start.z - 1,
                },
                end: Position {
                    x: end.x,
                    y: end.y,
                    z: end.z - 1,
                },
            });
        } else {
            new_bricks.push(Brick {
                start: *start,
                end: *end,
            });
        }
    }

    (fell, new_bricks)
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let settled_bricks = parse_input_and_settle_bricks(input);
    let mut solution = 0;
    // Go through every brick and remove one, run a single step of falling and see if any fell
    for i in 0..settled_bricks.len() {
        let mut copy = settled_bricks.clone();
        copy.remove(i);
        if !bricks_fall_single_step(&copy).0 {
            solution += 1;
        }
    }

    Ok(solution)
}

fn parse_input_and_settle_bricks(input: &str) -> Vec<Brick> {
    let mut initial_bricks = input
        .lines()
        .map(|mut line| brick_from_str(&mut line).unwrap())
        .collect_vec();
    // Sort so lowest Z is first for more efficient iterating
    initial_bricks.sort_by_key(|b| b.start.z);
    settle_bricks(&initial_bricks)
}

fn settle_bricks(initial_bricks: &[Brick]) -> Vec<Brick> {
    let (mut fell, mut new_bricks) = bricks_fall_single_step(initial_bricks);
    // Keep running steps until no bricks fall anymore, is is then settled
    while fell {
        let res = bricks_fall_single_step(&new_bricks);
        fell = res.0;
        new_bricks = res.1;
    }
    new_bricks
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let settled_bricks = parse_input_and_settle_bricks(input);
    let mut solution = 0;
    // Go through each and remove a brick, but this time run until settles. Then compare the
    // original copy with the new settled to determine how many changed
    for i in 0..settled_bricks.len() {
        let mut copy = settled_bricks.clone();
        copy.remove(i);
        let new_settled_bricks = settle_bricks(&copy);
        for (old, new) in copy.into_iter().zip(new_settled_bricks.into_iter()) {
            if old != new {
                solution += 1;
            }
        }
    }

    Ok(solution)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 5);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 7);
    }
}
