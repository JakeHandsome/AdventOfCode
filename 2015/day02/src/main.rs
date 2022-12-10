use common::*;

fn main() {
    let input = read_input_file_for_project_as_string!();
    println!("Part1: {}", part1(&input).unwrap());
    println!("Part2: {}", part2(&input).unwrap());
}

fn part1(input: &str) -> R<usize> {
    let mut solution = 0usize;
    for line in input.lines() {
        let mut split = line.split('x').into_iter().map(|x| x.parse::<usize>().unwrap());
        let (l, h, w) = (split.next().unwrap(), split.next().unwrap(), split.next().unwrap());
        solution += wrapping_needed(l, w, h);
    }
    Ok(solution)
}

fn part2(input: &str) -> R<usize> {
    let mut solution = 0usize;
    for line in input.lines() {
        let mut split = line.split('x').into_iter().map(|x| x.parse::<usize>().unwrap());
        let (l, h, w) = (split.next().unwrap(), split.next().unwrap(), split.next().unwrap());
        solution += ribbon_needed(l, w, h);
    }
    Ok(solution)
}

fn wrapping_needed(l: usize, w: usize, h: usize) -> usize {
    let sides = &[l * w, w * h, l * h];
    sides.iter().map(|s| 2 * s).sum::<usize>() + *sides.iter().min().unwrap()
}

fn ribbon_needed(l: usize, w: usize, h: usize) -> usize {
    let mut sides = vec![l, w, h];
    let bow: usize = sides.iter().product();
    sides.sort();
    let wrap = sides.first().unwrap() * 2 + sides.get(1).unwrap() * 2;

    wrap + bow
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1_test() {
        assert_eq!(part1("2x3x4").unwrap(), 58);
        assert_eq!(part1("1x1x10").unwrap(), 43);
        assert_eq!(
            part1(
                r#"2x3x4
1x1x10"#
            )
            .unwrap(),
            58 + 43
        );
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2("2x3x4").unwrap(), 34);
        assert_eq!(part2("1x1x10").unwrap(), 14);
        assert_eq!(
            part2(
                r#"2x3x4
1x1x10"#
            )
            .unwrap(),
            34 + 14
        );
    }
}
