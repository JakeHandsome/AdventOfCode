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

fn calc_hash(val: &str) -> usize {
    let mut hash = 0;
    for c in val.chars() {
        hash += c as usize;
        hash *= 17;
        hash %= 256;
    }
    hash
}

fn part1(input: &str) -> anyhow::Result<usize> {
    Ok(input.replace('\n', "").split(',').map(calc_hash).sum())
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let input = input.replace('\n', "");
    let mut map = vec![vec![]; 256];
    for x in input.split(',') {
        if x.contains('=') {
            let mut split = x.split('=');
            let label = split.next().unwrap();
            let focal_len = split.next().unwrap().parse::<usize>().unwrap();
            let box_index = calc_hash(label);
            if let Some(x) = map[box_index].iter_mut().find(|(l, _)| *l == label) {
                // Something with this label already exists, update the focal len
                x.1 = focal_len;
            } else {
                // Add the new item
                map[box_index].push((label, focal_len));
            }
        } else {
            let label = &x[..=x.len() - 2];
            let box_index = calc_hash(label);
            // Find and remove shifting everything forward
            if let Some(i) = map[box_index].iter().position(|(l, _)| *l == label) {
                map[box_index].remove(i);
            }
        }
    }
    let mut sum = 0;
    for (i, boxx) in map.iter().enumerate() {
        let box_num = i + 1;
        for (slot, (_, focal_len)) in boxx.iter().enumerate() {
            sum += box_num * focal_len * (slot + 1);
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 1320);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 145);
    }
}
