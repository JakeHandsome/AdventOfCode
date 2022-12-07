use common::AdventOfCodeError;
use common::{read_input_file_for_project_as_string, R};
use slab_tree::*;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::rc::Rc;

fn main() {
    let input = read_input_file_for_project_as_string!();
    println!("Part1: {:#?}", part1(&input).unwrap());
    println!("Part2: {:#?}", part2(&input).unwrap());
}

#[derive(Debug)]
enum PathItem {
    File { name: String, size: usize },
    Folder { name: String },
}

fn part1(input: &str) -> R<usize> {
    let mut ids: HashMap<String, NodeId> = Default::default();
    // Skip first one becaus it is the root
    let mut tree = TreeBuilder::new()
        .with_root(PathItem::Folder { name: "/".to_string() })
        .build();
    let root_id = tree.root_id().expect("Root doesn't exist?");
    ids.insert("/".to_string(), root_id);
    let mut current_dir: String = "/".into();
    let mut iter = input.lines().into_iter().skip(1).peekable();
    while let Some(line) = iter.next() {
        if line == "$ ls" {
            // Advance iterator until the next element starts with $

            while iter.peek().is_some() && !iter.peek().unwrap().starts_with("$") {
                let line = iter.next().unwrap();
                if line.starts_with("dir") {
                    // Directory
                    let dir_name = current_dir.clone() + line.split(" ").last().unwrap() + "/";
                    let mut node = tree.get_mut(ids[&current_dir]).unwrap();
                    let new_dir = node.append(PathItem::Folder { name: dir_name.clone() });
                    ids.insert(dir_name, new_dir.node_id());
                } else {
                    let mut split = line.split(" ");
                    let mut node = tree.get_mut(ids[&current_dir]).unwrap();
                    node.append(PathItem::File {
                        size: split.next().unwrap().parse()?,
                        name: split.next().unwrap().to_string(),
                    });
                }
            }
        } else if line.starts_with("$ cd") {
            let new_dir = line.split(" ").last().unwrap();

            if new_dir == ".." {
                let mut split = current_dir.split("/").collect::<VecDeque<_>>();
                split.pop_back();
                split.pop_back();
                current_dir = split.into_iter().collect::<Vec<_>>().join("/") + "/";
            } else if new_dir == "/" {
                current_dir = "/".to_string()
            } else {
                current_dir = current_dir + new_dir + "/";
            }
        }
    }
    let mut s = String::new();
    tree.write_formatted(&mut s).unwrap();
    // Look at the longest directories and calc size, walk backwards through the tree calculating all sizes
    let mut directories = ids.keys().cloned().collect::<Vec<String>>();
    directories.sort_by(|a, b| {
        let a_slashes = a.chars().filter(|c| *c == '/').count();
        let b_slashes = b.chars().filter(|c| *c == '/').count();
        b_slashes.cmp(&a_slashes)
    });
    let mut dir_sizes: HashMap<String, usize> = HashMap::default();
    for directory in directories {
        let mut dir_size = 0usize;
        let node = tree.get(ids[&directory]).unwrap();
        for child in node.children() {
            match child.data() {
                PathItem::File { name, size } => dir_size += size,
                PathItem::Folder { name } => dir_size += dir_sizes[name],
            }
        }
        dir_sizes.insert(directory, dir_size);
    }
    Ok(dir_sizes.into_values().filter(|x| *x <= 100_000usize).sum())
}

fn part2(input: &str) -> R<usize> {
    let mut ids: HashMap<String, NodeId> = Default::default();
    // Skip first one becaus it is the root
    let mut tree = TreeBuilder::new()
        .with_root(PathItem::Folder { name: "/".to_string() })
        .build();
    let root_id = tree.root_id().expect("Root doesn't exist?");
    ids.insert("/".to_string(), root_id);
    let mut current_dir: String = "/".into();
    let mut iter = input.lines().into_iter().skip(1).peekable();
    while let Some(line) = iter.next() {
        if line == "$ ls" {
            // Advance iterator until the next element starts with $

            while iter.peek().is_some() && !iter.peek().unwrap().starts_with("$") {
                let line = iter.next().unwrap();
                if line.starts_with("dir") {
                    // Directory
                    let dir_name = current_dir.clone() + line.split(" ").last().unwrap() + "/";
                    let mut node = tree.get_mut(ids[&current_dir]).unwrap();
                    let new_dir = node.append(PathItem::Folder { name: dir_name.clone() });
                    ids.insert(dir_name, new_dir.node_id());
                } else {
                    let mut split = line.split(" ");
                    let mut node = tree.get_mut(ids[&current_dir]).unwrap();
                    node.append(PathItem::File {
                        size: split.next().unwrap().parse()?,
                        name: split.next().unwrap().to_string(),
                    });
                }
            }
        } else if line.starts_with("$ cd") {
            let new_dir = line.split(" ").last().unwrap();

            if new_dir == ".." {
                let mut split = current_dir.split("/").collect::<VecDeque<_>>();
                split.pop_back();
                split.pop_back();
                current_dir = split.into_iter().collect::<Vec<_>>().join("/") + "/";
            } else if new_dir == "/" {
                current_dir = "/".to_string()
            } else {
                current_dir = current_dir + new_dir + "/";
            }
        }
    }
    let mut s = String::new();
    tree.write_formatted(&mut s).unwrap();
    // Look at the longest directories and calc size, walk backwards through the tree calculating all sizes
    let mut directories = ids.keys().cloned().collect::<Vec<String>>();
    directories.sort_by(|a, b| {
        let a_slashes = a.chars().filter(|c| *c == '/').count();
        let b_slashes = b.chars().filter(|c| *c == '/').count();
        b_slashes.cmp(&a_slashes)
    });
    let mut dir_sizes: HashMap<String, usize> = HashMap::default();
    for directory in directories {
        let mut dir_size = 0usize;
        let node = tree.get(ids[&directory]).unwrap();
        for child in node.children() {
            match child.data() {
                PathItem::File { name, size } => dir_size += size,
                PathItem::Folder { name } => dir_size += dir_sizes[name],
            }
        }
        dir_sizes.insert(directory, dir_size);
    }
    let available_space = 70_000_000 - dir_sizes["/"];
    println!("{}", available_space);
    let mut smallest_dir = dir_sizes["/"];
    const UPDATE_SIZE: usize = 30_000_000;
    for dir_size in dir_sizes.into_values() {
        if available_space + dir_size >= UPDATE_SIZE && dir_size < smallest_dir {
            smallest_dir = dir_size
        }
    }
    Ok(smallest_dir)
}

#[cfg(test)]
mod day6_tests {
    use super::*;
    const SAMPLE1: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 95437);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 24933642);
    }
}
