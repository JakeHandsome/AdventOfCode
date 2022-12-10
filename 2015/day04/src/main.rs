use common::*;
use md5::*;
// For parallel iterators using multiple threads
use rayon::prelude::*;

fn main() {
    let input = read_input_file_for_project_as_string!();
    println!("Part1: {}", part1(&input).unwrap());
    println!("Part2: {}", part2(&input).unwrap());
}

fn part1(input: &str) -> R<usize> {
    let input = input.trim();
    let result = (0..100_000_000)
        .into_par_iter()
        .find_first(|&x| {
            let combined = format!("{}{}", input, x);
            let hash = format!("{:x}", compute(&combined));
            hash.starts_with("00000")
        })
        .unwrap()
        .to_owned();
    Ok(result)
}

fn part2(input: &str) -> R<usize> {
    let input = input.trim();
    let result = (0..100_000_000)
        .into_par_iter()
        .find_first(|&x| {
            let combined = format!("{}{}", input, x);
            let hash = format!("{:x}", compute(&combined));
            hash.starts_with("000000")
        })
        .unwrap()
        .to_owned();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#""#;
    #[test]
    fn p1_test() {
        assert_eq!(part1("abcdef").unwrap(), 609043);
        assert_eq!(part1("pqrstuv").unwrap(), 1048970);
    }
    #[test]
    fn p2_test() {}
}
