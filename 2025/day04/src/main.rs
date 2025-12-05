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

fn part1(input: &str) -> anyhow::Result<usize> {
    let mut grid = Grid::new(input);
    let mut sum = 0;
    for c in 0..grid.cols {
        for r in 0..grid.rows {
            if grid.get_char(r, c) == Some('@') {
                let adjacent = grid.get_adjacent(r, c);
                let count = adjacent
                    .into_iter()
                    .map(|x| x.unwrap_or('.'))
                    .fold(0, |acc, x| if x == '@' { acc + 1 } else { acc });
                if count < 4 {
                    sum += 1;
                }
            }
        }
    }
    Ok(sum)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let mut grid = Grid::new(input);
    let mut sum = 0;
    loop {
        let mut removed = false;
        for c in 0..grid.cols {
            for r in 0..grid.rows {
                if grid.get_char(r, c) == Some('@') {
                    let adjacent = grid.get_adjacent(r, c);
                    let count = adjacent
                        .into_iter()
                        .map(|x| x.unwrap_or('.'))
                        .fold(0, |acc, x| if x == '@' { acc + 1 } else { acc });
                    if count < 4 {
                        sum += 1;
                        grid.set_char(r, c, '.');
                        removed = true;
                    }
                }
            }
        }
        if !removed {
            break;
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 13);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 43);
    }
}
