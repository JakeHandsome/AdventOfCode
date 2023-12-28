use common::{
    winnow::{ascii::float, seq, PResult, Parser},
    *,
};

fn main() {
    let input = read_input_file_for_project_as_string!();
    {
        let _timer = Timer::new("Part 1");
        println!(
            "Part1: {}",
            part1(
                &input,
                &Bounds {
                    min: 200_000_000_000_000f64,
                    max: 400_000_000_000_000f64,
                }
            )
            .unwrap()
        );
    }
    {
        let _timer = Timer::new("Part 2");
        println!("Part2: {}", part2(&input).unwrap());
    }
}
#[derive(Debug, PartialEq, Clone, Copy)]
struct Hailstone {
    px: f64,
    py: f64,
    pz: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

impl Hailstone {
    fn slope_2d(&self) -> f64 {
        self.vy / self.vx
    }
    fn y_intercept(&self) -> f64 {
        self.py - (self.slope_2d()) * self.px
    }
    fn x_in_future(&self, x: f64) -> bool {
        self.vx > 0.0 && x > self.px || self.vx < 0.0 && x < self.px
    }
    fn y_in_future(&self, y: f64) -> bool {
        self.vy > 0.0 && y > self.py || self.vy < 0.0 && y < self.py
    }
    fn intersection_point_within_bounds_in_future(&self, other: &Self, bounds: &Bounds) -> Option<(f64, f64)> {
        let m1 = self.slope_2d();
        let m2 = other.slope_2d();
        // Slopes are same, cannot intercept
        if m1 == m2 {
            None
        } else {
            let b1 = self.y_intercept();
            let b2 = other.y_intercept();
            // Calculate x position of intersection
            let x = (b2 - b1) / (m1 - m2);
            // Make sure X is within bounds and in future
            if x >= bounds.min && x <= bounds.max && self.x_in_future(x) && other.x_in_future(x) {
                // Calculate y pos
                let y = m1 * x + b1;
                // Make sure y is within bounds and in future
                if y >= bounds.min && y <= bounds.max && self.y_in_future(y) && other.y_in_future(y) {
                    Some((x, y))
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
    fn intersects_within_bounds_in_future(&self, other: &Self, bounds: &Bounds) -> bool {
        self.intersection_point_within_bounds_in_future(other, bounds).is_some()
    }
}

fn hailstone(input: &mut &str) -> PResult<Hailstone> {
    seq!(Hailstone{
        px:float,
        _:", ",
        py:float,
        _:", ",
        pz:float,
        _: " @ ",
        vx:float,
        _: ", ",
        vy:float,
        _: ", ",
        vz:float,
    })
    .parse_next(input)
}

#[derive(Debug)]
struct Bounds {
    min: f64,
    max: f64,
}

fn part1(input: &str, bounds: &Bounds) -> anyhow::Result<usize> {
    let hailstones = input.lines().map(|mut line| hailstone(&mut line).unwrap());

    let count = hailstones
        .tuple_combinations()
        .filter(|(a, b)| a.intersects_within_bounds_in_future(b, bounds))
        .count();
    Ok(count)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    Err(AdventOfCodeError::UnimplementedError)?
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"19, 13, 30 @ -2, 1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @ 1, -5, -3"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1, &Bounds { min: 7f64, max: 27f64 }).unwrap(), 2);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 0);
    }
    #[test]
    fn parse_input() {
        let a = hailstone(&mut "19, 13, 30 @ -2, 1, -2").unwrap();
        assert_eq!(
            a,
            Hailstone {
                px: 19.0,
                py: 13.0,
                pz: 30.0,
                vx: -2.0,
                vy: 1.0,
                vz: -2.0
            }
        );
    }
}
