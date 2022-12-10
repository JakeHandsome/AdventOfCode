use std::collections::HashSet;

use common::*;

fn main() {
    let input = read_input_file_for_project_as_string!();
    println!("Part1: {:#?}", part1(&input).unwrap());
    println!("Part2: \n{}", part2(&input).unwrap());
}

fn part1(input: &str) -> R<isize> {
    let mut solution = 0isize;
    let mut cycle = 0usize;
    let mut register = 1isize;
    for line in input.lines() {
        let mut split = line.split(' ');
        let command = split.next().unwrap();
        if command == "addx" {
            let value = split.next().unwrap().parse::<isize>()?;
            cycle += 1;
            if cycle % 40 == 20 {
                solution += cycle as isize * register;
            }
            cycle += 1;
            if cycle % 40 == 20 {
                solution += cycle as isize * register;
            }
            register += value;
        } else {
            cycle += 1;
            if cycle % 40 == 20 {
                solution += cycle as isize * register;
            }
        }
    }
    Ok(solution)
}

fn part2(input: &str) -> R<String> {
    let mut solution = vec![0; 240];
    let mut cycle = 0usize;
    let mut sprite_pos = 1isize;
    for line in input.lines() {
        let mut split = line.split(' ');
        let command = split.next().unwrap();
        if command == "addx" {
            let value = split.next().unwrap().parse::<isize>()?;
            if (cycle % 40) as isize >= sprite_pos - 1 && (cycle % 40) as isize <= sprite_pos + 1 {
                solution[cycle] = 1;
            }
            cycle += 1;
            if (cycle % 40) as isize >= sprite_pos - 1 && (cycle % 40) as isize <= sprite_pos + 1 {
                solution[cycle] = 1;
            }
            cycle += 1;
            sprite_pos += value;
        } else {
            if (cycle % 40) as isize >= sprite_pos - 1 && (cycle % 40) as isize <= sprite_pos + 1 {
                solution[cycle] = 1;
            }
            cycle += 1;
        }
    }
    Ok(solution
        .into_iter()
        .enumerate()
        .map(|(i, x)| {
            let mut str = "".to_owned();
            if x == 0 {
                str = str + ".";
            } else {
                str = str + "#";
            }
            if i % 40 == 39 {
                str = str + "\n";
            }
            str
        })
        .collect())
}

#[cfg(test)]
mod day9 {
    use super::*;
    const SAMPLE1: &str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;
    const SAMPLE2: &str = r#""#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 13140);
    }
    #[test]
    fn p2_test() {
        assert_eq!(
            part2(SAMPLE1).unwrap(),
            r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"#
        );
    }
}
