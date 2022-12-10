use common::read_input_file_for_project;
fn main() {
    part1_and2();
}

fn part1_and2() {
    let mut lines = read_input_file_for_project!();
    let mut part1 = 0usize;
    let mut part2 = 0usize;
    while let Some(Ok(line)) = lines.next() {
        let ranges = {
            let mut split = line.split(',');
            (split.next().unwrap(), split.last().unwrap())
        };
        let first = parse_range(ranges.0);
        let second = parse_range(ranges.1);
        if (first.0 >= second.0 && first.1 <= second.1) || (second.0 >= first.0 && second.1 <= first.1) {
            part1 += 1;
        }

        if !(first.1 < second.0 || second.1 < first.0) {
            part2 += 1;
        }
    }
    println!("{}", part1);
    println!("{}", part2);
}

fn parse_range(split: &str) -> (usize, usize) {
    let mut iter = split.split('-').map(|x| x.parse::<usize>().unwrap());
    (iter.next().unwrap(), iter.last().unwrap())
}
