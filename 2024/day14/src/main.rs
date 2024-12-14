use std::{cmp::Ordering, collections::HashSet};

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
#[cfg(test)]
const WIDTH: isize = 11;
#[cfg(test)]
const HEIGHT: isize = 7;
#[cfg(not(test))]
const WIDTH: isize = 101;
#[cfg(not(test))]
const HEIGHT: isize = 103;

#[derive(Clone, Copy, Debug)]
struct Robot {
    pos: (isize, isize),
    vel: (isize, isize),
}

impl Robot {
    fn step(&mut self) {
        let mut new_x = self.pos.0 + self.vel.0;
        let mut new_y = self.pos.1 + self.vel.1;
        if new_x >= WIDTH {
            new_x -= WIDTH;
        } else if new_x < 0 {
            new_x += WIDTH;
        }
        if new_y >= HEIGHT {
            new_y -= HEIGHT;
        } else if new_y < 0 {
            new_y += HEIGHT;
        }
        self.pos = (new_x, new_y);
    }
}
fn part1(input: &str) -> anyhow::Result<usize> {
    Ok(input
        .lines()
        .map(|x| {
            // Convert string into robot
            // split pos and vel
            x.split(" ")
                .map(|s| {
                    // Trim off the first 2 chars and split , to get x,y as a tuple
                    s[2..]
                        .split(',')
                        .map(|x| x.parse::<isize>().expect("Should be a number"))
                        .collect_tuple()
                        .expect("Should be exactly 2 items")
                })
                // Collect 2 tuples ((posx,posy),(velx,vely))
                .collect_tuple()
                .map(|(pos, vel)| Robot { pos, vel })
                .expect("Should parse")
        })
        .map(|mut r| {
            // Advance robots 100 seconds
            (0..100).for_each(|_| r.step());
            r
        })
        // Check quadrants
        .map(|r| match (r.pos.0.cmp(&(WIDTH / 2)), r.pos.1.cmp(&(HEIGHT / 2))) {
            (Ordering::Less, Ordering::Less) => [1, 0, 0, 0],
            (Ordering::Less, Ordering::Greater) => [0, 1, 0, 0],
            (Ordering::Greater, Ordering::Less) => [0, 0, 1, 0],
            (Ordering::Greater, Ordering::Greater) => [0, 0, 0, 1],
            _ => [0, 0, 0, 0],
        })
        // Add quadrant counts together
        .fold([0, 0, 0, 0], |mut acc, x| {
            acc[0] += x[0];
            acc[1] += x[1];
            acc[2] += x[2];
            acc[3] += x[3];
            acc
        })
        .into_iter()
        .product())
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let mut robots = input
        .lines()
        .map(|x| {
            x.split(" ")
                .map(|s| {
                    s[2..]
                        .split(',')
                        .map(|x| x.parse::<isize>().expect("Should be a number"))
                        .collect_tuple()
                        .expect("Should be exactly 2 items")
                })
                .collect_tuple()
                .map(|(pos, vel)| Robot { pos, vel })
                .expect("Should parse")
        })
        .collect_vec();
    let mut time = 0;
    loop {
        // Hack: Assume all robots need to be in a unique position
        let set = robots.iter().map(|r| r.pos).collect::<HashSet<_>>();
        if set.len() == robots.len() {
            break;
        }
        robots.iter_mut().for_each(|x| x.step());
        time += 1;
    }

    Ok(time)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 12);
    }
}
