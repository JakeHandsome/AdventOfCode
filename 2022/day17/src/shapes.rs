use crate::board::Board;

/// Marked are the squares to check. L for left, D for down, R for right, X | x being the 0,0 point. X if in the shape x if outside
pub enum Shapes {
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
    pub fn move_left(&self, board: &mut Board) {
        let (x, y) = board.current_piece_location;

        let x: isize = x.try_into().unwrap();
        let y: isize = y.try_into().unwrap();
        let can_move = match self {
            Shapes::L => {
                // Check 3 positions
                !board.read_position(x - 1, y)
                    && !board.read_position(x + 1, y + 1)
                    && !board.read_position(x + 1, y + 2)
            }
            Shapes::Cross => {
                // Check 3 positions
                !board.read_position(x, y) && !board.read_position(x - 1, y + 1) && !board.read_position(x, y + 2)
            }
            Shapes::Rect(_width, height) => {
                let mut result = true;
                // Check each block along the left side of the height of this block
                for i in 0..*height {
                    let i: isize = i.try_into().unwrap();
                    result = result && !board.read_position(x - 1, y + i);
                }
                result
            }
        };
        // Move one space to the left
        if can_move {
            board.current_piece_location.0 -= 1;
        }
    }
    /// Attempt to move a piece right if possible
    pub fn move_right(&self, board: &mut Board) {
        let (x, y) = board.current_piece_location;

        let x: isize = x.try_into().unwrap();
        let y: isize = y.try_into().unwrap();
        let can_move = match self {
            Shapes::L => {
                !board.read_position(x + 3, y)
                    && !board.read_position(x + 3, y + 1)
                    && !board.read_position(x + 3, y + 2)
            }
            Shapes::Cross => {
                !board.read_position(x + 2, y)
                    && !board.read_position(x + 3, y + 1)
                    && !board.read_position(x + 2, y + 3)
            }
            Shapes::Rect(width, height) => {
                let mut result = true;
                let width: isize = (*width).try_into().unwrap();
                // Check each block along the left side of the height of this bloc
                for i in 0..*height {
                    let i: isize = i.try_into().unwrap();
                    result = result && !board.read_position(x + width, y + i);
                }
                result
            }
        };
        if can_move {
            board.current_piece_location.0 += 1;
        }
    }
    /// Attempt to move down, if it cannot move down, return false
    pub fn move_down(&self, board: &mut Board) -> bool {
        let (x, y) = board.current_piece_location;

        let x: isize = x.try_into().unwrap();
        let y: isize = y.try_into().unwrap();
        !match self {
            Shapes::L => {
                board.read_position(x, y - 1) || board.read_position(x + 1, y - 1) || board.read_position(x + 2, y - 1)
            }
            Shapes::Cross => {
                board.read_position(x, y) || board.read_position(x + 1, y - 1) || board.read_position(x + 2, y)
            }
            Shapes::Rect(width, _height) => {
                let mut result = false;
                // Check each block along the bottom of this block
                for i in 0..*width {
                    let i: isize = i.try_into().unwrap();
                    result = result || board.read_position(x + i, y - 1);
                }
                result
            }
        }
    }

    /// Returns the height of the rock placed
    pub fn turn_to_rock(&self, board: &mut Board) -> usize {
        let (x, y) = board.current_piece_location;

        let positions = match self {
            Shapes::L => vec![(x, y), (x + 1, y), (x + 2, y), (x + 2, y + 1), (x + 2, y + 2)],
            Shapes::Cross => vec![(x + 1, y), (x, y + 1), (x + 1, y + 1), (x + 2, y + 1), (x + 1, y + 2)],
            Shapes::Rect(width, height) => {
                let mut res = vec![];
                let x_max = x + *width;
                let y_max = y + *height;
                for x in x..x_max {
                    for y in y..y_max {
                        res.push((x, y))
                    }
                }
                res
            }
        };
        for (x, y) in &positions {
            board.write_position((*x).try_into().unwrap(), (*y).try_into().unwrap(), true)
        }
        // return the max height of this block
        positions.into_iter().map(|(_, y)| y).max().unwrap()
    }
}
