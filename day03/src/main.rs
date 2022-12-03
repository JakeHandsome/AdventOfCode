use common::read_input_file_for_project;
fn main() {
    let mut lines = read_input_file_for_project!();
    let mut part1 = 0u64;
    while let Some(Ok(line)) = lines.next() {
        let first = &line[..line.len() / 2];
        let second = &line[line.len() / 2..];

        let a: char = {
            let mut b: char = char::default();
            'outer1: for char in first.chars() {
                for char2 in second.chars() {
                    if char == char2 {
                        b = char;
                        break 'outer1;
                    }
                }
            }
            b
        };
        part1 += char_to_score(a);
    }
    println!("{}", part1);
    let mut part2 = 0u64;
    let mut lines2: Vec<_> = read_input_file_for_project!().collect();
    for chunk in lines2.chunks(3) {
        'outer2: for a in chunk[0].as_ref().unwrap().chars() {
            for b in chunk[1].as_ref().unwrap().chars() {
                for c in chunk[2].as_ref().unwrap().chars() {
                    if a == b && a == c {
                        part2 += char_to_score(a);
                        break 'outer2;
                    }
                }
            }
        }
    }
    println!("{}", part2);
}

fn char_to_score(c: char) -> u64 {
    let ascii: u64 = c as u64;
    if ascii <= 90 {
        ascii - 38
    } else {
        ascii - 96
    }
}
