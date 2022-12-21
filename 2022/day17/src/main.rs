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

#[derive(Debug)]
enum HorizontalMovement {
    Left,
    Right,
}
use HorizontalMovement::*;

impl HorizontalMovement {
    fn new(c: char) -> Self {
        match c {
            '<' => Left,
            '>' => Right,
            _ => unreachable!(),
        }
    }
}

const WIDTH: usize = 7;

struct Board {
    // This will be 7 wide and height + 6 (for room to spawn new pieces) tall
    data: Vec<bool>,
    /// This is the height of the blocks, starts at 0. This will be the answer
    height: usize,
    /// This is the bottom left square for each piece
    current_piece_location: (usize, usize),
}
impl Board {
    fn read_position(&self, x: isize, y: isize) -> bool {
        if x.is_negative() || y.is_negative() || x as usize >= WIDTH || y as usize >= self.height {
            true // Mark this square as occupied
        } else {
            let y = y as usize;
            let x = x as usize;
            self.data[y * WIDTH + x]
        }
    }
}
/// Marked are the squares to check. L for left, D for down, R for right, X | x being the 0,0 point. X if in the shape x if outside
enum Shapes {
    /// L shape.
    ///  .L#R
    ///  .L#R
    /// LX##R
    ///  DDD
    L,
    /// + shape.
    ///     L#R
    ///    L###R
    /// x/D/R#D/R
    ///      D
    Cross,
    /// 4x1
    ///  L####R
    ///   DDDD
    /// 1x4
    ///  L#R
    ///  L#R
    ///  L#R
    ///  L#R
    ///   D
    /// 2x2
    ///  L##R
    ///  L##R
    ///   DD
    Rect(usize, usize),
}

impl Shapes {
    /// Attempt to a move a piece left if possible
    fn move_left(&self, board: &mut Board) {
        let (x, y) = board.current_piece_location;

        let x: isize = x.try_into().unwrap();
        let y: isize = y.try_into().unwrap();
        let can_move = match self {
            Shapes::L => {
                // Check 3 positions
                board.read_position(x - 1, y) && board.read_position(x + 1, y + 1) && board.read_position(x + 1, y + 2)
            }
            Shapes::Cross => {
                // Check 3 positions
                board.read_position(x, y) && board.read_position(x - 1, y + 1) && board.read_position(x, y + 2)
            }
            Shapes::Rect(_width, height) => {
                let mut result = true;
                // Check each block along the left side of the height of this bloc
                for i in 0..*height {
                    result = result && board.read_position(x - 1, y + i as isize);
                }
                result
            }
        };
        // Move one space to the left
        if can_move {
            board.current_piece_location = (board.current_piece_location.0 - 1, board.current_piece_location.1);
        }
    }
    /// Attempt to move a piece right if possible
    fn move_right(&self, board: &mut Board) {
        let (x, y) = board.current_piece_location;

        let x: isize = x.try_into().unwrap();
        let y: isize = y.try_into().unwrap();
        let can_move = match self {
            Shapes::L => {
                board.read_position(x + 3, y) && board.read_position(x + 3, y + 1) && board.read_position(x + 3, y + 2)
            }
            Shapes::Cross => {
                board.read_position(x + 2, y) && board.read_position(x + 3, y + 1) && board.read_position(x + 2, y + 3)
            }
            Shapes::Rect(width, height) => {
                let mut result = true;
                // Check each block along the left side of the height of this bloc
                for i in 0..*height {
                    let i = i as isize;
                    result = result && board.read_position(x + *width as isize, y + i as isize);
                }
                result
            }
        };
        if can_move {
            board.current_piece_location = (board.current_piece_location.0 + 1, board.current_piece_location.1);
        }
    }
    /// Attempt to move down, if it cannot move down, return false
    fn move_down(&self, board: &mut Board) -> bool {
        let (x, y) = board.current_piece_location;

        let x: isize = x.try_into().unwrap();
        let y: isize = y.try_into().unwrap();
        match self {
            Shapes::L => {
                board.read_position(x, y - 1) && board.read_position(x + 1, y - 1) && board.read_position(x + 2, y - 1)
            }
            Shapes::Cross => {
                board.read_position(x, y) && board.read_position(x + 1, y - 1) && board.read_position(x + 2, y)
            }
            Shapes::Rect(width, _height) => {
                let mut result = true;
                // Check each block along the left side of the height of this bloc
                for i in 0..*width {
                    let i = i as isize;
                    result = result && board.read_position(x + i, y - 1 as isize);
                }
                result
            }
        }
    }
    /// Spawns a new piece in the spawn position
    fn spawn(&self, board: &mut Board) {
        match self {
            Shapes::L => todo!(),
            Shapes::Cross => todo!(),
            Shapes::Rect(x, y) => todo!(),
        }
    }
}

fn part1(input: &str) -> R<u64> {
    let jet_patterns = input
        .chars()
        .filter(|c| matches!(c, '<' | '>'))
        .map(|c| HorizontalMovement::new(c))
        .cycle();
    for (i, _) in jet_patterns.enumerate() {
        if i % 1_000_000_000 == 0 {
            println!("{:?}", i);
        }
    }

    Err(Box::new(AdventOfCodeError::new("Not implemented")))
}

fn part2(input: &str) -> R<u64> {
    Err(Box::new(AdventOfCodeError::new("Not implemented")))
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 3068);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 0);
    }
}
