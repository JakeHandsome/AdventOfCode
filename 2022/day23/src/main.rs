use common::*;
use elf::Elf;
use rayon::prelude::*;
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

mod elf;
mod point;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

fn parse_initial_conditions(input: &str) -> Vec<Elf> {
    let mut elves = vec![];
    // Reverse the input so positive Y is North, easier to wrap my head around
    for (y, line) in input.lines().rev().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                elves.push(Elf::new(x.try_into().unwrap(), y.try_into().unwrap()));
            }
        }
    }
    elves
}

fn part1(input: &str) -> R<usize> {
    let mut elves = parse_initial_conditions(input);
    let num_rounds = 10;
    for round in 0..num_rounds {
        let mut elf_positions = elves.iter().map(|x| x.location).collect::<Vec<_>>();
        elf_positions.sort();
        // First half of round
        let new_locations = elves
            .par_iter_mut()
            .map(|elf| {
                let proposal = elf.propose_movement(&elf_positions, round);
                elf.proposal = proposal;
                proposal
            })
            .flatten()
            .collect::<Vec<_>>();
        // Second half of round
        for elf in elves.iter_mut() {
            if let Some(proposal) = elf.proposal {
                // Make sure this is the only elf with this proposal
                if new_locations.iter().filter(|f| **f == proposal).count() == 1 {
                    // The elf gets to move
                    elf.move_location();
                }
                elf.proposal = None;
            }
        }
    }
    let xs = elves.iter().map(|elf| elf.location.x);
    let ys = elves.iter().map(|elf| elf.location.y);

    let max_x = xs.clone().max().unwrap();
    let min_x = xs.min().unwrap();
    let max_y = ys.clone().max().unwrap();
    let min_y = ys.min().unwrap();

    let area = (1 + max_x - min_x) * (1 + max_y - min_y);

    Ok(area as usize - elves.len())
}

fn part2(input: &str) -> R<usize> {
    let mut elves = parse_initial_conditions(input);
    let mut round = 0;
    loop {
        let mut elf_positions = elves.iter().map(|x| x.location).collect::<Vec<_>>();
        elf_positions.sort();
        // First half of round
        let new_locations = elves
            .par_iter_mut()
            .map(|elf| {
                let proposal = elf.propose_movement(&elf_positions, round);
                elf.proposal = proposal;
                proposal
            })
            .flatten()
            .collect::<Vec<_>>();
        // If no new locations where proposed, nothing needs to move
        if new_locations.is_empty() {
            break Ok(round + 1);
        }
        // Second half of round
        for elf in elves.iter_mut() {
            if let Some(proposal) = elf.proposal {
                // Make sure this is the only elf with this proposal
                if new_locations.iter().filter(|f| **f == proposal).count() == 1 {
                    // The elf gets to move
                    elf.move_location();
                }
                elf.proposal = None;
            }
        }
        round += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use point::Point;
    const SAMPLE1: &str = r#"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#.."#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 110);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 20);
    }
    #[test]
    fn small_example() {
        assert_eq!(
            part1(
                r#".....
..##.
..#..
.....
..##.
....."#
            )
            .unwrap(),
            25
        )
    }
    #[test]
    fn elf_isolated() {
        let elf = Elf::new(0, 0);
        let mut locations = vec![Point::new(0, 0)];
        locations.sort();
        assert_eq!(elf.propose_movement(&locations, 0), None);
    }

    #[test]
    fn elf_move_north() {
        let elf = Elf::new(0, 0);
        let mut locations = vec![Point::new(0, 0), Point::new(1, 0)];
        locations.sort();

        assert_eq!(elf.propose_movement(&locations, 0), Some(Point::NORTH));
    }

    #[test]
    fn elf_move_south() {
        let elf = Elf::new(0, 0);
        let mut locations = vec![Point::new(0, 0), Point::new(0, 1)];
        locations.sort();
        assert_eq!(elf.propose_movement(&locations, 0), Some(Point::SOUTH));
        let mut locations = vec![Point::new(0, 0), Point::new(-1, 1)];
        locations.sort();
        assert_eq!(elf.propose_movement(&locations, 0), Some(Point::SOUTH));
        let mut locations = vec![Point::new(0, 0), Point::new(1, 1)];
        locations.sort();
        assert_eq!(elf.propose_movement(&locations, 0), Some(Point::SOUTH));
    }

    #[test]
    fn elf_move_west() {
        let elf = Elf::new(0, 0);
        let mut locations = vec![Point::new(0, 0), Point::new(0, 1), Point::new(0, -1)];
        locations.sort();
        assert_eq!(elf.propose_movement(&locations, 0), Some(Point::WEST));
        let mut locations = vec![Point::new(0, 0), Point::new(0, 1), Point::new(1, -1)];
        locations.sort();
        assert_eq!(elf.propose_movement(&locations, 0), Some(Point::WEST));
    }
    #[test]
    fn elf_move_east() {
        let elf = Elf::new(0, 0);
        let mut locations = vec![Point::new(0, 0), Point::new(0, 1), Point::new(0, -1), Point::new(-1, 0)];
        locations.sort();

        assert_eq!(elf.propose_movement(&locations, 0), Some(Point::EAST));
    }
}
