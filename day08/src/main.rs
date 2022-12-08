use common::{read_input_file_for_project_as_string, R};

fn main() {
    let input = read_input_file_for_project_as_string!();
    println!("Part1: {:#?}", part1(&input).unwrap());
    println!("Part2: {:#?}", part2(&input).unwrap());
}

#[derive(Debug)]
struct Forest {
    trees: Vec<Tree>,
    width: usize,
    height: usize,
}

#[derive(Debug, Clone, Copy)]
struct Tree {
    height: usize,
    visible: bool,
}

impl Forest {
    fn new(trees: Vec<usize>, width: usize) -> Self {
        let len = trees.len();
        // Solve for height using length and width
        let height = len / width;
        // Convert the heights into Trees
        let trees = trees
            .into_iter()
            .enumerate()
            .map(|(i, height)| {
                // Check for Trees on the outside and mark them as visible right away
                let mut outside = false;
                if i < width || i >= len - width || i % width == 0 || i % width == width - 1 {
                    outside = true;
                }
                Tree {
                    height,
                    visible: outside,
                }
            })
            .collect::<Vec<Tree>>();
        Forest { trees, width, height }
    }
    // Gets a specific index of trees, If x/y are out of the bounds of the grid, return None
    fn get(&self, x: isize, y: isize) -> Option<Tree> {
        if x >= self.width as isize || y >= self.height as isize || x.is_negative() || y.is_negative() {
            None
        } else {
            // Numbers are not negative checked above
            let y: usize = y as usize;
            let x: usize = x as usize;
            Some(self.trees[y * self.width + x])
        }
    }
    // Returns a whole row of trees
    fn get_row_mut(&mut self, index: usize) -> Vec<&mut Tree> {
        assert!(index < self.height);
        let start = index * self.height;
        self.trees[start..start + self.width].as_mut().iter_mut().collect()
    }
    // Returns a whole column of trees
    fn get_col_mut(&mut self, index: usize) -> Vec<&mut Tree> {
        assert!(index < self.width);
        self.trees
            .iter_mut()
            .skip(index)
            .step_by(self.width)
            .collect::<Vec<_>>()
    }
}

fn part1(input: &str) -> R<usize> {
    let mut trees: Vec<usize> = vec![];
    let mut width = None;
    for line in input.lines() {
        if width.is_none() {
            // Set the width based on the length of the first line
            width = Some(line.len());
        }
        for char in line.chars() {
            trees.push(char.to_digit(10).unwrap() as usize)
        }
    }
    let mut forest = Forest::new(trees, width.unwrap());
    #[cfg(test)]
    {
        println!("{:#?}", forest);
        // Sanity check on  the get_row, get_col functions that runs during tests
        assert_eq!(
            forest.get_row_mut(0).iter().map(|x| x.height).collect::<Vec<usize>>(),
            vec![3, 0, 3, 7, 3]
        );
        assert_eq!(
            forest.get_col_mut(0).iter().map(|x| x.height).collect::<Vec<usize>>(),
            vec![3, 2, 6, 3, 3]
        );
    }
    for col in 0..forest.width {
        let mut max_height = 0;
        // Do a vision check from the top to bottom
        for mut tree in forest.get_col_mut(col) {
            if tree.height > max_height {
                max_height = tree.height;
                tree.visible = true;
            }
        }
        max_height = 0;
        // Do a vision check from bottom to top
        for mut tree in forest.get_col_mut(col).into_iter().rev() {
            if tree.height > max_height {
                max_height = tree.height;
                tree.visible = true;
            }
        }
    }
    for row in 0..forest.width {
        let mut max_height = 0;
        for mut tree in forest.get_row_mut(row) {
            if tree.height > max_height {
                max_height = tree.height;
                tree.visible = true;
            }
        }
        max_height = 0;
        for mut tree in forest.get_row_mut(row).into_iter().rev() {
            if tree.height > max_height {
                max_height = tree.height;
                tree.visible = true;
            }
        }
    }

    Ok(forest.trees.iter().filter(|t| t.visible).count())
}

fn part2(input: &str) -> R<usize> {
    let mut trees: Vec<usize> = vec![];
    let mut width = None;
    for line in input.lines() {
        if width.is_none() {
            width = Some(line.len());
        }
        for char in line.chars() {
            trees.push(char.to_digit(10).unwrap() as usize)
        }
    }
    let forest = Forest::new(trees, width.unwrap());

    let mut max_score = 0usize;
    for x in 0..forest.width {
        let x: isize = x.try_into().unwrap();
        for y in 0..forest.height {
            let y: isize = y.try_into().unwrap();
            let local_score = calc_tree_score(&forest, x, y);
            if local_score > max_score {
                max_score = local_score;
            }
        }
    }
    #[cfg(not(test))]
    {
        println!("{}", max_score);
        assert!(max_score > 113866); // Wrong answer, must be larger
        assert!(max_score > 178464); // Wrong answer, must be larger
    }
    Ok(max_score)
}

fn calc_tree_score(forest: &Forest, x: isize, y: isize) -> usize {
    let this_tree = forest.get(x, y).unwrap();
    // Calculate tree score
    // Initially we cant see any direction
    let (mut left, mut right, mut down, mut up) = (0, 0, 0, 0);
    let mut index = 1;
    while let Some(tree) = forest.get(x - index, y) {
        // If we can see a tree to our left, increment the count
        left += 1;
        if tree.height < this_tree.height {
            // If the tree is smaller than us, keep looking
            index += 1;
        } else {
            break;
        }
    }
    index = 1;
    while let Some(tree) = forest.get(x + index, y) {
        right += 1;
        if tree.height < this_tree.height {
            index += 1;
        } else {
            break;
        }
    }
    index = 1;
    while let Some(tree) = forest.get(x, y - index) {
        up += 1;
        if tree.height < this_tree.height {
            index += 1;
        } else {
            break;
        }
    }
    index = 1;
    while let Some(tree) = forest.get(x, y + index) {
        down += 1;
        if tree.height < this_tree.height {
            index += 1;
        } else {
            break;
        }
    }
    left * right * down * up
}

#[cfg(test)]
mod day8 {
    use super::*;
    const SAMPLE1: &str = r#"30373
25512
65332
33549
35390"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 21);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 8);
    }
}
