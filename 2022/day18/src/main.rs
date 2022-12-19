use std::{thread::Builder, vec};

use common::*;

fn main() {
    let input = read_input_file_for_project_as_string!();
    {
        let _timer = Timer::new("Part 1");
        println!("Part1: {}", part1(&input).unwrap());
    }
    // Make a bigger stack to recurse deeper :)
    Builder::new()
        .stack_size(1024 * 1024 * 10)
        .spawn(move || {
            let _timer = Timer::new("Part 2");
            println!("Part2: {}", part2(&input).unwrap());
        })
        .unwrap()
        .join()
        .unwrap();
}

type Cube = Vec<Vec<Vec<BlockType>>>;

fn get_from_cube(cube: &Cube, x: isize, y: isize, z: isize) -> Option<BlockType> {
    if x.is_negative() || y.is_negative() || z.is_negative() {
        None
    } else {
        let x = x as usize;
        let y = y as usize;
        let z = z as usize;
        if x >= cube.len() || y >= cube[0].len() || z >= cube[0][0].len() {
            None
        } else {
            Some(cube[x][y][z])
        }
    }
}

fn has_neighbor(cube: &Cube, x: isize, y: isize, z: isize) -> BlockType {
    if x.is_negative() || y.is_negative() || z.is_negative() {
        Air
    } else {
        let x = x as usize;
        let y = y as usize;
        let z = z as usize;
        if x >= cube.len() || y >= cube[0].len() || z >= cube[0][0].len() {
            Air
        } else {
            cube[x][y][z]
        }
    }
}
fn count_exposed_sides(cube: &Cube, x: isize, y: isize, z: isize) -> usize {
    let mut count = 0;
    for (x, y, z) in [
        (x + 1, y, z),
        (x - 1, y, z),
        (x, y + 1, z),
        (x, y - 1, z),
        (x, y, z + 1),
        (x, y, z - 1),
    ] {
        if has_neighbor(cube, x, y, z) != Rock {
            count += 1;
        }
    }
    count
}
fn part1(input: &str) -> R<usize> {
    let mut coords: Vec<(usize, usize, usize)> = vec![];
    let mut max_coord = 0;
    for line in input.lines() {
        let split = line.split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let x = split[0];
        let y = split[1];
        let z = split[2];
        let max = x.max(y).max(z);
        if max > max_coord {
            max_coord = max;
        }
        coords.push((x, y, z));
    }
    max_coord += 1;
    let mut cube = vec![vec![vec![Air; max_coord]; max_coord]; max_coord];
    for (x, y, z) in &coords {
        cube[*x][*y][*z] = Rock;
    }
    let mut answer = 0;
    for (x, y, z) in coords {
        answer += count_exposed_sides(&cube, x as isize, y as isize, z as isize);
    }
    Ok(answer)
}

#[derive(Copy, Debug, Clone, PartialEq)]
enum BlockType {
    Air,
    Water,
    Rock,
}
use BlockType::*;

fn part2(input: &str) -> R<usize> {
    let mut coords: Vec<(usize, usize, usize)> = vec![];
    let mut max_coord = 0;
    for line in input.lines() {
        let split = line.split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let x = split[0];
        let y = split[1];
        let z = split[2];
        let max = x.max(y).max(z);
        if max > max_coord {
            max_coord = max;
        }
        coords.push((x, y, z));
    }
    // Make it one bigger so the flood fill will go all the way around
    max_coord += 2;
    let mut cube = vec![vec![vec![Air; max_coord]; max_coord]; max_coord];
    for (x, y, z) in &coords {
        cube[*x][*y][*z] = Rock;
    }
    fill_with_water(&mut cube);
    let mut answer = 0;
    for (x, y, z) in coords {
        answer += count_exposed_sides(&cube, x as isize, y as isize, z as isize);
    }
    Ok(answer)
}
fn flood_fill(cube: &mut Cube, x: isize, y: isize, z: isize) {
    cube[x as usize][y as usize][z as usize] = Water;
    for (x2, y2, z2) in [
        (x - 1, y, z),
        (x, y - 1, z),
        (x, y, z - 1),
        (x + 1, y, z),
        (x, y + 1, z),
        (x, y, z + 1),
    ] {
        if let Some(block) = get_from_cube(cube, x2, y2, z2) {
            match block {
                Air => flood_fill(cube, x2, y2, z2),
                Water | Rock => (),
            }
        }
    }
}
#[allow(clippy::needless_range_loop)]
fn fill_with_water(cube: &mut Cube) {
    // Fill all external nodes with water
    flood_fill(cube, 0, 0, 0);
    let len = cube.len();
    for x in 0..len {
        for y in 0..len {
            for z in 0..len {
                match cube[x][y][z] {
                    // If an internal node is Air, turn it into rock
                    Air => cube[x][y][z] = Rock,
                    Water | Rock => (),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE2: &str = r#"1,1,1
2,1,1"#;
    const SAMPLE1: &str = r#"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE2).unwrap(), 10);
        assert_eq!(part1(SAMPLE1).unwrap(), 64);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 58);
    }
}
