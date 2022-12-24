const WIDTH: usize = 7;
const MAX_PIECE_HEIGHT: usize = 4;
const SPAWN_X_OFFSET: usize = 2;
const SPAWN_Y_OFFSET: usize = 3;
const HEAD_ROOM: usize = SPAWN_Y_OFFSET + MAX_PIECE_HEIGHT;

pub struct Board {
    // This will be 7 wide and height + HEAD_ROOM tall
    pub data: Vec<bool>,
    /// This is the height of the blocks, starts at 0. This will be the answer
    pub height: usize,
    /// This is the bottom left square for each piece
    pub current_piece_location: (usize, usize),
}
impl Board {
    pub fn write_position(&mut self, x: isize, y: isize, val: bool) {
        if x.is_negative() || y.is_negative() {
        } else {
            let yu: usize = y.try_into().unwrap();
            let xu: usize = x.try_into().unwrap();
            if xu >= WIDTH || yu >= (self.height + HEAD_ROOM) {
            } else {
                self.data[yu * WIDTH + xu] = val;
            }
        }
    }
    pub fn read_position(&self, x: isize, y: isize) -> bool {
        if x.is_negative() || y.is_negative() {
            true // Mark this square as occupied
        } else {
            let yu: usize = y.try_into().unwrap();
            let xu: usize = x.try_into().unwrap();
            if xu >= WIDTH || yu >= (self.height + HEAD_ROOM) {
                true
            } else {
                self.data[yu * WIDTH + xu]
            }
        }
    }
    #[inline]
    pub fn x_y_to_index(x: usize, y: usize) -> usize {
        y * WIDTH + x
    }

    pub fn new() -> Self {
        let height = 0;
        let data = vec![false; (height + HEAD_ROOM) * WIDTH];

        Board {
            data,
            height,
            current_piece_location: (SPAWN_X_OFFSET, height + SPAWN_Y_OFFSET),
        }
    }
    pub fn spawn_new_rock(&mut self, last_rock_height: usize) {
        // Increase size of the data to account for new rock
        self.height = self.height.max(last_rock_height + 1);

        let new_size = ((self.height) + HEAD_ROOM) * WIDTH;
        if self.data.len() < new_size {
            for _ in 0..(new_size - self.data.len()) {
                self.data.push(false);
            }
            assert_eq!(new_size, self.data.len());
        }

        // Set current piece location to  the spawn
        self.current_piece_location = (SPAWN_X_OFFSET, (self.height) + SPAWN_Y_OFFSET);
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Print the bottom
        let mut a = vec![];
        a.push("+-------+\n".to_string());
        for (i, val) in self.data.iter().enumerate() {
            let mut c = match *val {
                true => '#',
                false => '.',
            };
            let (x, y) = self.current_piece_location;
            if Board::x_y_to_index(x, y) == i {
                c = 'x';
            }
            if i % 7 == 0 {
                a.push(format!("|{c}"));
            } else if i % 7 == 6 {
                a.push(format!("{c}|\n"));
            } else {
                a.push(format!("{c}"));
            }
        }
        let st = a
            .join("")
            .chars()
            .collect::<String>()
            .lines()
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
            .join("\n");
        f.write_fmt(format_args!("{}", st))
    }
}
