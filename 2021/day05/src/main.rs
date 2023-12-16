use std::collections::HashMap;

use common::{
    winnow::{
        ascii::digit1,
        combinator::{repeat, separated_pair},
        PResult, Parser,
    },
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

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct LineSegment {
    start: Point,
    end: Point,
}

impl LineSegment {
    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }
    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }
}

fn line_segment(input: &mut &str) -> PResult<LineSegment> {
    let (start, _, end) = (point, " -> ", point).parse_next(input)?;
    Ok(LineSegment { start, end })
}

fn point(input: &mut &str) -> PResult<Point> {
    let (x, y) = separated_pair(digit1.try_map(str::parse), ',', digit1.try_map(str::parse)).parse_next(input)?;
    Ok(Point { x, y })
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let mut line_segments = vec![];
    for mut line in input.lines() {
        let a = line_segment.parse_next(&mut line).map_err(|e| anyhow::anyhow!(e))?;
        line_segments.push(a);
    }
    line_segments.retain(|l| l.is_horizontal() || l.is_vertical());
    let mut marked_count = HashMap::new();
    for line in line_segments {
        for x in line.start.x.min(line.end.x)..=line.start.x.max(line.end.x) {
            for y in line.start.y.min(line.end.y)..=line.start.y.max(line.end.y) {
                if let Some(count) = marked_count.get_mut(&(x, y)) {
                    *count += 1;
                } else {
                    marked_count.insert((x, y), 1);
                }
            }
        }
    }
    marked_count.retain(|_, v| *v > 1);
    Ok(marked_count.keys().count())
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let mut line_segments = vec![];
    for mut line in input.lines() {
        let a = line_segment.parse_next(&mut line).map_err(|e| anyhow::anyhow!(e))?;
        line_segments.push(a);
    }
    let mut marked_count = HashMap::new();
    for line in line_segments {
        if line.is_horizontal() || line.is_vertical() {
            for x in line.start.x.min(line.end.x)..=line.start.x.max(line.end.x) {
                for y in line.start.y.min(line.end.y)..=line.start.y.max(line.end.y) {
                    if let Some(count) = marked_count.get_mut(&(x, y)) {
                        *count += 1;
                    } else {
                        marked_count.insert((x, y), 1);
                    }
                }
            }
        } else {
            let mut current = line.start;
            while current != line.end {
                if let Some(count) = marked_count.get_mut(&(current.x, current.y)) {
                    *count += 1;
                } else {
                    marked_count.insert((current.x, current.y), 1);
                }
                if line.start.x < line.end.x {
                    current.x += 1;
                } else {
                    current.x -= 1;
                }
                if line.start.y < line.end.y {
                    current.y += 1;
                } else {
                    current.y -= 1;
                }
            }
            if let Some(count) = marked_count.get_mut(&(current.x, current.y)) {
                *count += 1;
            } else {
                marked_count.insert((current.x, current.y), 1);
            }
        }
    }
    marked_count.retain(|_, v| *v > 1);
    Ok(marked_count.keys().count())
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 5);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 12);
    }
    #[test]
    fn point_winnow() {
        let mut a = "0,9";
        let a = point.parse_next(&mut a).unwrap();
        assert_eq!(a, (Point { x: 0, y: 9 }));
    }
    #[test]
    fn line_segment_winnow() {
        let mut a = "0,9 -> 5,9";
        let a = line_segment.parse_next(&mut a).unwrap();
        assert_eq!(
            a,
            LineSegment {
                start: Point { x: 0, y: 9 },
                end: Point { x: 5, y: 9 }
            }
        );
    }
}
