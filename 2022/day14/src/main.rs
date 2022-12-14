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
            Rock | Sand => true,
            Air | SandEmitter => false,
        }
    }
}

fn x_y_to_index(x: usize, y: usize) -> usize {
    y * 1000 + x
}

fn index_to_x_y(i: usize) -> (usize, usize) {
    (i % 1000, i / 1000)
}

fn sand_is_stable(cave_map: &mut Vec<CaveMatter>, x: usize, y: usize) -> bool {
    if x >= 999 || y >= 999 {
        // Fell off the map
        return false;
    }
    let index = x_y_to_index(x, y);
    // Part2, sand is not stable if original index is full
    if cave_map[index].is_blocking() {
        // Sand filled to top
        return false;
    }
    let down = x_y_to_index(x, y + 1);
    let down_left = x_y_to_index(x - 1, y + 1);
    let down_right = x_y_to_index(x + 1, y + 1);
    match cave_map[down].is_blocking() {
        true => match cave_map[down_left].is_blocking() {
            true => match cave_map[down_right].is_blocking() {
                true => {
                    // If everything is blocking, mark this node as Sand
                    cave_map[index] = Sand;
                    true
                }
                // Check the down_right location
                false => {
                    let (x, y) = index_to_x_y(down_right);
                    sand_is_stable(cave_map, x, y)
                }
            },
            // Check the down left location
            false => {
                let (x, y) = index_to_x_y(down_left);
                sand_is_stable(cave_map, x, y)
            }
        },
        // Check the down location
        false => {
            let (x, y) = index_to_x_y(down);
            sand_is_stable(cave_map, x, y)
        }
    }
}

fn part1(input: &str) -> R<usize> {
    let mut cave_map = vec![Air; 1000 * 1000];
    cave_map[500] = SandEmitter;
    for line in input.lines() {
        // Use a sliding window to find all the lines to draw
        for points in line.split("->").collect::<Vec<_>>().windows(2) {
            // Covert string "a,b" into vec![a,b]
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

            // Set each point in the line
            for x in min(start[0], end[0])..=max(start[0], end[0]) {
                for y in min(start[1], end[1])..=max(start[1], end[1]) {
                    cave_map[x_y_to_index(x, y)] = Rock;
                }
            }
        }
    }

    // Keep adding sand until no longer stable
    while sand_is_stable(&mut cave_map, 500, 0) {}
    #[cfg(test)]
    {
        // Print out the map in the test for sanity checking
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

fn part2(input: &str) -> R<usize> {
    let mut cave_map = vec![Air; 1000 * 1000];
    cave_map[500] = SandEmitter;
    // Find the greatest t for drawing the floor
    let mut greatest_y = 0;
    for line in input.lines() {
        // Use a sliding window to find all the lines to draw
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

            // Set each point in the line
            for x in min(start[0], end[0])..=max(start[0], end[0]) {
                for y in min(start[1], end[1])..=max(start[1], end[1]) {
                    if y > greatest_y {
                        greatest_y = y;
                    }
                    cave_map[x_y_to_index(x, y)] = Rock;
                }
            }
        }
    }
    // Draw the floor
    for x in 0..1000 {
        let y = greatest_y + 2;
        cave_map[x_y_to_index(x, y)] = Rock;
    }

    // Keep adding sand until no longer stable
    while sand_is_stable(&mut cave_map, 500, 0) {}
    #[cfg(test)]
    {
        // Print out the map in the test for sanity checking
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
