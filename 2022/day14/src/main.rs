use common::*;

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

#[derive(Clone, Debug, PartialEq)]
enum CaveMatter {
    Rock,
    Air,
    Sand,
    SandEmitter,
}
use CaveMatter::*;

impl CaveMatter {
    fn is_blocking(&self) -> bool {
        match self {
            Rock => true,
            Air => false,
            Sand => true,
            SandEmitter => false,
        }
    }
}

fn x_y_to_index(x: usize, y: usize) -> usize {
    y * 1000 + x
}

fn sand_is_stable(cave_map: &mut Vec<CaveMatter>, x: usize, y: usize) -> bool {
    if x >= 999 || y >= 999 {
        // Fell off the map
        return false;
    }
    let index = x_y_to_index(x, y);
    if cave_map[index].is_blocking() {
        // Sand filled to top
        return false;
    }
    let down = x_y_to_index(x, y + 1);
    let down_left = x_y_to_index(x - 1, y + 1);
    let down_right = x_y_to_index(x + 1, y + 1);
    if cave_map[down].is_blocking() {
        if cave_map[down_left].is_blocking() {
            if cave_map[down_right].is_blocking() {
                cave_map[index] = Sand;
                true
            } else {
                sand_is_stable(cave_map, x + 1, y + 1)
            }
        } else {
            sand_is_stable(cave_map, x - 1, y + 1)
        }
    } else {
        sand_is_stable(cave_map, x, y + 1)
    }
}

fn part2(input: &str) -> R<usize> {
    let mut cave_map = vec![Air; 1000 * 1000];
    cave_map[500] = SandEmitter;
    let mut greatest_y = 0;
    for line in input.lines() {
        for points in line.split("->").collect::<Vec<_>>().windows(2) {
            let start = points[0]
                .trim()
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let end = points[1]
                .trim()
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            use std::cmp::{max, min};

            for x in min(start[0], end[0])..=max(start[0], end[0]) {
                for y in min(start[1], end[1])..=max(start[1], end[1]) {
                    let index = x_y_to_index(x, y);
                    if y > greatest_y {
                        greatest_y = y;
                    }
                    cave_map[index] = Rock;
                }
            }
        }
    }
    for x in 0..1000 {
        let y = greatest_y + 2;
        cave_map[x_y_to_index(x, y)] = Rock;
    }

    // Keep adding sand until no longer stable
    while sand_is_stable(&mut cave_map, 500, 0) {}
    #[cfg(test)]
    {
        for (i, cave_spot) in cave_map.iter().enumerate() {
            let c = match cave_spot {
                Rock => '#',
                Air => '.',
                Sand => 'o',
                SandEmitter => '+',
            };
            print!("{}", c);
            if i % 1000 == 999 {
                println!();
            }
        }
    }
    Ok(cave_map.into_iter().filter(|x| *x == Sand).count())
}

fn part1(input: &str) -> R<usize> {
    let mut cave_map = vec![Air; 1000 * 1000];
    cave_map[500] = SandEmitter;
    for line in input.lines() {
        for points in line.split("->").collect::<Vec<_>>().windows(2) {
            let start = points[0]
                .trim()
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let end = points[1]
                .trim()
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            use std::cmp::{max, min};

            for x in min(start[0], end[0])..=max(start[0], end[0]) {
                for y in min(start[1], end[1])..=max(start[1], end[1]) {
                    let index = y * 1000 + x;
                    cave_map[index] = Rock;
                }
            }
        }
    }
    // Keep adding sand until no longer stable
    while sand_is_stable(&mut cave_map, 500, 0) {}
    #[cfg(test)]
    {
        for (i, cave_spot) in cave_map.iter().enumerate() {
            let c = match cave_spot {
                Rock => '#',
                Air => '.',
                Sand => 'o',
                SandEmitter => '+',
            };
            print!("{}", c);
            if i % 1000 == 999 {
                println!();
            }
        }
    }
    Ok(cave_map.into_iter().filter(|x| *x == Sand).count())
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 24);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 93);
    }
}
