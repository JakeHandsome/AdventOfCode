use common::*;
use std::ops::Range;

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

#[derive(Default, Debug)]
struct Mapping(Vec<(Range<usize>, usize)>);

impl Mapping {
    fn get(&self, key: usize) -> usize {
        // If the mapping exist in any range, map it to the output range
        for (range, dest) in &self.0 {
            if range.contains(&key) {
                return key - range.start + dest;
            }
        }
        // If mapping didn't exist return the key as it is a 1:1 mapping
        key
    }
}

fn part1(input: &str) -> R<usize> {
    let mut lines = input.lines();
    // First line is seeds
    let seeds = parse_seeds(&mut lines);
    let mappings = parse_mappings(lines);
    Ok(seeds
        .into_iter()
        .map(|seed| {
            let mut seed = seed;
            for mapping in &mappings {
                seed = mapping.get(seed);
            }
            seed
        })
        .min()
        .unwrap())
}

fn parse_mappings(lines: std::str::Lines<'_>) -> Vec<Mapping> {
    let mut mappings = vec![];
    let mut current_mapping = Mapping::default();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if line.contains("map") {
            mappings.push(current_mapping);
            current_mapping = Mapping::default();
            continue;
        }
        let numbers = line.split(' ').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let (dest, source, len) = (numbers[0], numbers[1], numbers[2]);
        let input_range = source..source + len;
        let map = (input_range, dest);
        current_mapping.0.push(map)
    }
    mappings.push(current_mapping);
    mappings
}

//  Takes about ~3 seconds to run in release on 7950x due to the large number of seeds to check
fn part2(input: &str) -> R<usize> {
    let mut lines = input.lines();
    // First line is seeds
    let seeds = parse_seeds(&mut lines);
    // Convert the pairs of numbers into ranges and flat_map them to make a new iterator over all numbers. This is the only change needed for pt2
    let seeds = seeds.par_chunks(2).flat_map(|x| x[0]..x[0] + x[1]);
    let mappings = parse_mappings(lines);
    Ok(seeds
        .map(|seed| {
            let mut seed = seed;
            for mapping in &mappings {
                seed = mapping.get(seed);
            }
            seed
        })
        .min()
        .unwrap())
}

fn parse_seeds(lines: &mut std::str::Lines<'_>) -> Vec<usize> {
    let seeds = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    seeds
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 35);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 46);
    }
}
