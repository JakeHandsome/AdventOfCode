use common::{read_input_file_for_project_as_string, R};
use slab_tree::*;

use std::collections::{HashMap, VecDeque};

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
    let (tree, ids) = parse_file_system_to_tree(input)?;
    let dir_sizes = calculate_size_of_each_directory(tree, ids)?;
    Ok(dir_sizes.into_values().filter(|x| *x <= 100_000usize).sum())
}

fn part2(input: &str) -> R<usize> {
    let (tree, ids) = parse_file_system_to_tree(input)?;
    let dir_sizes = calculate_size_of_each_directory(tree, ids)?;

    let available_space = 70_000_000 - dir_sizes["/"];
    let mut smallest_dir = dir_sizes["/"];
    const UPDATE_SIZE: usize = 30_000_000;
    for dir_size in dir_sizes.into_values() {
        if available_space + dir_size >= UPDATE_SIZE && dir_size < smallest_dir {
            smallest_dir = dir_size
        }
    }
    Ok(smallest_dir)
}

/// From the given commands, parse the file tree
/// Returns A tree object that has all the files and folder in it and a HashMap that has all the paths to ther nodeId for quick lookup
fn parse_file_system_to_tree(input: &str) -> R<(Tree<PathItem>, HashMap<String, NodeId>)> {
    let mut ids: HashMap<String, NodeId> = Default::default();
    let mut tree = TreeBuilder::new()
        .with_root(PathItem::Folder { name: "/".to_string() })
        .build();
    let root_id = tree.root_id().expect("Root doesn't exist?");
    ids.insert("/".to_string(), root_id);
    let mut current_dir: String = "/".into();
    // Skip first one becaus it is the root
    let mut iter = input.lines().into_iter().skip(1).peekable();
    while let Some(line) = iter.next() {
        if line == "$ ls" {
            // Advance iterator until the next element starts with $
            while iter.peek().is_some() && !iter.peek().unwrap().starts_with('$') {
                let line = iter.next().unwrap();
                // Use the current_dir to get the current node we are in so we append the correct spot
                let mut current_node = tree.get_mut(ids[&current_dir]).unwrap();
                if line.starts_with("dir") {
                    // Create a Folder PathItem
                    let dir_name = current_dir.clone() + line.split(' ').last().unwrap() + "/";
                    let new_dir = current_node.append(PathItem::Folder { name: dir_name.clone() });
                    ids.insert(dir_name, new_dir.node_id());
                } else {
                    // Create a FilePathItem
                    let mut split = line.split(' ');
                    current_node.append(PathItem::File {
                        size: split.next().unwrap().parse()?,
                        name: split.next().unwrap().to_string(),
                    });
                }
            }
        } else if line.starts_with("$ cd") {
            let new_dir = line.split(' ').last().unwrap();
            if new_dir == ".." {
                // remove the last directory from the current dir
                let mut split = current_dir.split('/').collect::<VecDeque<_>>();
                split.pop_back();
                split.pop_back();
                current_dir = split.into_iter().collect::<Vec<_>>().join("/") + "/";
            } else if new_dir == "/" {
                // Set the current dir to home
                current_dir = "/".to_string()
            } else {
                // add the new directory to the current directory
                current_dir = current_dir + new_dir + "/";
            }
        }
    }
    Ok((tree, ids))
}

/// Caclulates the size of each directory
/// Takes in a tree object and map of all directories and thier nodeID
///
/// Starts at the longest directories and find the size, these directories have no sub directories. Saving its size to a hash map
/// When calculating a higher up directory, used the saved size value from the previous calculation
///
/// Returns a hashmap of with Key being Directory name and value being the size
fn calculate_size_of_each_directory(tree: Tree<PathItem>, ids: HashMap<String, NodeId>) -> R<HashMap<String, usize>> {
    let mut directories = ids.keys().cloned().collect::<Vec<String>>();
    // Sort by directories with the most '/' characters so they  will be proccessed first
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
                // If this a file, increment the size
                PathItem::File { name: _, size } => dir_size += size,
                // If this a directory, lookup its size and add it to this directory
                PathItem::Folder { name } => dir_size += dir_sizes[name],
            }
        }
        dir_sizes.insert(directory, dir_size);
    }
    Ok(dir_sizes)
}

#[cfg(test)]
mod day7_tests {
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
