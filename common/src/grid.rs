// Helper for parsing the text input as a grid

/// A grid representation of the input
#[derive(Debug, Clone)]
pub struct Grid {
    /// The raw text input
    pub inner: String,
    /// Rows in the grid
    pub rows: usize,
    /// Cols in the grid
    pub cols: usize,
}

impl Grid {
    /// Creates a new grid, assumes all rows and cols are same size
    pub fn new(input: String) -> Self {
        let cols = input.lines().next().unwrap().len();
        let rows = input.lines().count();
        let inner = input.replace('\n', "");
        Self { inner, rows, cols }
    }
}

impl Grid {
    /// Converts an x/y index into an offset into the string
    #[inline]
    pub fn index(&self, row: isize, col: isize) -> Option<usize> {
        if row < 0 || col < 0 || row >= self.rows as isize || col >= self.cols as isize {
            None
        } else {
            let index = (row * self.cols as isize + col) as usize;
            debug_assert!(index < self.inner.len(), "{},r{},c{}", index, row, col);
            Some(index)
        }
    }
    /// Gets a char from the string
    #[inline]
    pub fn get_char(&self, row: usize, col: usize) -> Option<char> {
        self.index(row as isize, col as isize)
            .map(|index| self.inner.as_bytes()[index] as char)
    }
}
