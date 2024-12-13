use std::{
    char,
    fmt::{format, Display},
    ops::Index,
};

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
    let mut memory = vec![];
    for (index, size) in input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).expect("Should be a number"))
        .enumerate()
    {
        // Even numbers are file odds are empty
        if index % 2 == 0 {
            (0..size).for_each(|_| memory.push(Some(index / 2)));
        } else {
            (0..size).for_each(|_| memory.push(None));
        }
    }
    //debug_print(&memory);
    let mut start = 0;
    let mut current = memory.len() - 1;
    while start < current {
        while memory.index(start).is_some() {
            start += 1;
        }
        while memory.index(current).is_none() {
            current -= 1;
        }
        memory.swap(start, current);
    }
    // debug_print(&memory);
    Ok(memory
        .into_iter()
        .flatten()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + i * x))
}

#[allow(dead_code)]
fn debug_print(memory: &[Option<usize>]) {
    for a in memory {
        if let Some(x) = a {
            print!("{x}");
        } else {
            print!(".");
        }
    }
    println!();
}

#[derive(Debug)]
enum Entry {
    File { id: usize, len: usize },
    Free { len: usize },
}

impl Entry {
    fn set_len(&mut self, new_len: usize) {
        match self {
            Entry::File { id: _, len } => *len = new_len,
            Entry::Free { len } => *len = new_len,
        }
    }
}
impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(test)]
        {
            let (c, len) = match self {
                Entry::File { id, len } => (char::from_digit(*id as u32, 10).unwrap_or('?'), len),
                Entry::Free { len } => ('.', len),
            };
            write!(f, "{}", std::iter::repeat(c).take(*len).collect::<String>())
        }
        #[cfg(not(test))]
        {
            write!(f, "{:?}", &self)
        }
    }
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let mut memory = vec![];
    for (index, size) in input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).expect("Should be a number"))
        .enumerate()
    {
        // Even numbers are file odds are empty
        if index % 2 == 0 {
            memory.push(Entry::File {
                id: index / 2,
                len: size as usize,
            });
        } else {
            memory.push(Entry::Free { len: size as usize });
        }
    }
    #[cfg(test)]
    {
        for e in &memory {
            print!("{e}");
        }
        println!();
    }
    // Take each file from end to start
    //   check each empty area from start to end
    //   If it fits swap and decrment the index
    let mut end = memory.len() - 1;
    while end > 0 {
        if let Entry::File { id: _, len: file_len } = memory[end] {
            let mut start = 0;
            loop {
                if let Entry::Free { len: free_len } = memory[start] {
                    match free_len.cmp(&file_len) {
                        std::cmp::Ordering::Equal => {
                            #[cfg(test)]
                            {
                                println!("  Swapping {} and {}", memory[start], memory[end]);
                            }
                            memory.swap(start, end);
                            #[cfg(test)]
                            {
                                debug_print2(&memory);
                            }
                            end -= 1;
                            break;
                        }
                        std::cmp::Ordering::Greater => {
                            let new_len = free_len - file_len;
                            memory[start].set_len(file_len);
                            #[cfg(test)]
                            {
                                println!("  Swapping {} and {}", memory[start], memory[end]);
                            }
                            memory.swap(start, end);
                            // Insert empty space after
                            let new = Entry::Free { len: new_len };
                            #[cfg(test)]
                            {
                                debug_print2(&memory);
                                println!("  Inserting {new}");
                            }
                            memory.insert(start + 1, new);
                            #[cfg(test)]
                            debug_print2(&memory);
                            break;
                        }
                        std::cmp::Ordering::Less => (),
                    }
                }
                if start == end {
                    break;
                }
                start += 1;
            }
        }
        end -= 1;
    }

    let ans = memory
        .into_iter()
        .flat_map(|x| {
            let mut a = vec![];
            let (id, len) = match x {
                Entry::File { id, len } => (id, len),
                Entry::Free { len } => (0, len),
            };
            (0..len).for_each(|_| a.push(id));
            a
        })
        .enumerate()
        .fold(0, |acc, (i, c)| acc + i * c);

    Ok(ans)
}

#[allow(dead_code, unused_variables)]
fn debug_print2(memory: &[Entry]) {
    #[cfg(test)]
    {
        print!("  ");
        for e in memory {
            print!("{e}");
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"2333133121414131402"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 1928);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 2858);
    }
}
