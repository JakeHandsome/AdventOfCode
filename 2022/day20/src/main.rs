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

fn part1(input: &str) -> R<i64> {
    let numbers = input.lines().map(|l| l.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let mut indexes = (0..numbers.len()).collect::<Vec<_>>();

    for (index, num) in numbers.iter().enumerate() {
        let pos = indexes.iter().position(|&x| x == index).unwrap();
        // Swap index around instead of numbers since we cannot modify the vec as we iterator over it
        indexes.remove(pos);
        let new_index = (pos as i64 + num).rem_euclid(indexes.len() as i64) as usize;
        indexes.insert(new_index, index)
    }
    let index_of_0 = numbers.iter().position(|x| *x == 0).unwrap();
    let zero_index = indexes.iter().position(|x| *x == index_of_0).unwrap();
    // Convert the index back into the number at that location
    let a = numbers[indexes[(zero_index + 1000) % indexes.len()]];
    let b = numbers[indexes[(zero_index + 2000) % indexes.len()]];
    let c = numbers[indexes[(zero_index + 3000) % indexes.len()]];
    Ok(a + b + c)
}

fn part2(input: &str) -> R<i64> {
    // When parsing the number just multiply by the key
    const KEY: i64 = 811589153;
    let numbers = input
        .lines()
        .map(|l| l.parse::<i64>().unwrap() * KEY)
        .collect::<Vec<_>>();
    let mut indexes = (0..numbers.len()).collect::<Vec<_>>();
    // Do 10 iterations
    for _ in 0..10 {
        for (index, num) in numbers.iter().enumerate() {
            let pos = indexes.iter().position(|&x| x == index).unwrap();
            // Swap index around instead of numbers since we cannot modify the vec as we iterator over it
            indexes.remove(pos);
            let new_index = (pos as i64 + num).rem_euclid(indexes.len() as i64) as usize;
            indexes.insert(new_index, index)
        }
    }
    let index_of_0 = numbers.iter().position(|x| *x == 0).unwrap();
    let zero_index = indexes.iter().position(|x| *x == index_of_0).unwrap();
    // Convert the index back into the number at that location
    let a = numbers[indexes[(zero_index + 1000) % indexes.len()]];
    let b = numbers[indexes[(zero_index + 2000) % indexes.len()]];
    let c = numbers[indexes[(zero_index + 3000) % indexes.len()]];
    Ok(a + b + c)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"1
2
-3
3
-2
0
4"#;
    #[test]
    fn p1_test() -> R<()> {
        assert_eq!(part1(SAMPLE1)?, 3);
        Ok(())
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 1623178306);
    }
    #[test]
    fn modulus() {
        let a: isize = 7; // or any other integer type
        let b = 4;

        assert_eq!(a.rem_euclid(b), 3);
        assert_eq!((-a).rem_euclid(b), 1);
        assert_eq!(a.rem_euclid(-b), 3);
        assert_eq!((-a).rem_euclid(-b), 1);
        assert_eq!(0isize.rem_euclid(10), 0);
        assert_eq!(11isize.rem_euclid(10), 1);
        assert_eq!((-1isize).rem_euclid(10), 9);
    }
}
