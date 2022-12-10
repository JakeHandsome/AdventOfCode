use common::*;

fn main() {
    let input = read_input_file_for_project_as_string!();
    println!("Part1: {}", part1(&input).unwrap());
    println!("Part2: {}", part2(&input).unwrap());
}

fn part1(input: &str) -> R<usize> {
    let mut sum = 0usize;
    for line in input.lines() {
        let vowels = line
            .chars()
            .filter(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
            .count();
        let has_invalid_strings =
            line.contains("ab") || line.contains("cd") | line.contains("pq") || line.contains("xy");
        let has_repeated_chars = String::from(line).as_bytes().windows(2).any(|x| x[0] == x[1]);
        if vowels >= 3 && !has_invalid_strings && has_repeated_chars {
            sum += 1;
        }
    }
    Ok(sum)
}

fn part2(input: &str) -> R<usize> {
    let mut sum = 0usize;
    for line in input.lines() {
        let has_char_sandwhich = String::from(line).as_bytes().windows(3).any(|x| x[0] == x[2]);
        let has_repeated_char_sequence = check_repeated_char_seq(line);
        if has_char_sandwhich && has_repeated_char_sequence {
            sum += 1;
        }
    }
    Ok(sum)
}

fn check_repeated_char_seq(line: &str) -> bool {
    let mut sub_strings = vec![];
    String::from(line).as_bytes().windows(2).for_each(|w| {
        let a: String = w.iter().map(|x| *x as char).collect();
        sub_strings.push(a);
    });
    for (i, s) in sub_strings.iter().enumerate().take(sub_strings.len() - 2) {
        for s2 in &sub_strings[i + 2..sub_strings.len()] {
            if s == s2 {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"ugknbfddgicrmopn
aaa
jchzalrnumimnmhp
haegwjzuvuyypxyu
dvszwmarrgswjxmb"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 2);
    }
    #[test]
    fn p2_test() {
        assert_eq!(
            part2(
                r#"qjhvhtzxzqqjkmpb
aaa
xxyxx
uurcxstgmygtbstg
ieodomkazucvgmuy"#
            )
            .unwrap(),
            2
        );
    }
}
