use std::collections::HashSet;

use common::*;

fn main() {
    let input = read_input_file_for_project_as_string!();
    {
        let _timer = Timer::new("Part 1");
        println!("Part1: {}", part1(&input, 2_000_000).unwrap());
    }
    {
        let _timer = Timer::new("Part 2");
        println!("Part2: {}", part2(&input, 4_000_000).unwrap());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn calculate_manhatten_distance(&self, b: &Point) -> usize {
        self.x.abs_diff(b.x) + self.y.abs_diff(b.y)
    }
}

fn parse_input(input: &str) -> R<Vec<(Point, Point)>> {
    let mut points = vec![];
    for line in input.lines() {
        let split = line.split(':').collect::<Vec<_>>();
        let location = split[0].split(',').collect::<Vec<_>>();
        let location_x = location[0].split('=').last().unwrap().parse()?;
        let location_y = location[1].split('=').last().unwrap().parse()?;
        let beacon = split[1].split(',').collect::<Vec<_>>();
        let beacon_x = beacon[0].split('=').last().unwrap().parse()?;
        let beacon_y = beacon[1].split('=').last().unwrap().parse()?;
        points.push((
            Point {
                x: location_x,
                y: location_y,
            },
            Point {
                x: beacon_x,
                y: beacon_y,
            },
        ));
    }
    Ok(points)
}

fn part1(input: &str, y_val: isize) -> R<usize> {
    let points = parse_input(input)?;
    // These hash sets will be all the X coords in `y_val`
    let mut beacons = HashSet::new();
    let mut not_beacons = HashSet::new();
    for (sensor, beacon) in points {
        if beacon.y == y_val {
            beacons.insert(beacon.x);
        }
        let radius = sensor.calculate_manhatten_distance(&beacon) as isize;
        // For each Y difference the radious will be reduced by 1
        let width_at_y_coordinate = radius - sensor.y.abs_diff(y_val) as isize;
        if width_at_y_coordinate < 0 {
            // Not in the circle
            continue;
        }

        let first = sensor.x - width_at_y_coordinate;
        let last = sensor.x + width_at_y_coordinate;
        // Insert all confirmed not beacons known
        for x in first..=last {
            not_beacons.insert(x);
        }
    }
    // Return number of not_beacons minus number of confirmed beacons
    Ok(not_beacons.len() - beacons.len())
}

fn part2(input: &str, max_coord: isize) -> R<usize> {
    let points = parse_input(input)?;
    // Go through every point and find one not covered
    let mut x = 0;
    let mut y = 0;
    'next_coord: while y <= max_coord {
        // If x goes past max, move y down and reset x
        if x > max_coord {
            x = 0;
            y += 1;
        }
        for (sensor, beacon) in points.iter() {
            let radius = sensor.calculate_manhatten_distance(beacon) as isize;
            let width_at_y_coordinate = radius - sensor.y.abs_diff(y) as isize;
            if width_at_y_coordinate < 0 {
                // This sensor doesn't cover this line
                continue;
            }
            // This sensor does cover this line
            let first = sensor.x - width_at_y_coordinate;
            let last = sensor.x + width_at_y_coordinate;
            if first <= x && x <= last {
                // This point is covered go next
                x = last + 1; // Move x forward to the next not covered index save calculations
                continue 'next_coord;
            }
        }
        // If this point is not covered it wll fall through
        return Ok(x as usize * 4_000_000 + y as usize);
    }

    Err(Box::new(AdventOfCodeError::new("Not found")))
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1, 10).unwrap(), 26);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1, 20).unwrap(), 56_000_011);
    }
}
