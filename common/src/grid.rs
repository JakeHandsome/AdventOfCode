// Helper for parsing the text input as a grid

use std::collections::BTreeMap;

use anyhow::{bail, Error};

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
    pub fn new<S>(input: S) -> Self
    where
        S: Into<String>,
    {
        let input = input.into();
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
    /// Gets a char from the string
    #[inline]
    pub fn get_char_signed(&self, row: isize, col: isize) -> Option<char> {
        self.index(row, col).map(|index| self.inner.as_bytes()[index] as char)
    }
    /// Converts an offset into the string to a x/y
    #[inline]
    pub fn index_to_row_col(&self, index: usize) -> (usize, usize) {
        (index / self.cols, index % self.cols)
    }

    pub fn get_adjacent8(&self, row: usize, col: usize) -> [Option<char>; 8] {
        let r = row as isize;
        let c = col as isize;
        [
            self.get_char_signed(r + 1, c + 1),
            self.get_char_signed(r + 1, c),
            self.get_char_signed(r + 1, c - 1),
            self.get_char_signed(r - 1, c + 1),
            self.get_char_signed(r - 1, c),
            self.get_char_signed(r - 1, c - 1),
            self.get_char_signed(r, c + 1),
            self.get_char_signed(r, c - 1),
        ]
    }
    pub fn get_adjacent4(&self, row: usize, col: usize) -> [Option<char>; 4] {
        let r = row as isize;
        let c = col as isize;
        [
            self.get_char_signed(r + 1, c),
            self.get_char_signed(r - 1, c),
            self.get_char_signed(r, c + 1),
            self.get_char_signed(r, c - 1),
        ]
    }

    /// Sets a char in the string to a new char, requires string to be ASCII
    pub fn set_char(&mut self, row: usize, col: usize, c: char) -> anyhow::Result<()> {
        if self.inner.is_ascii() {
            if let Some(index) = self.index(row as isize, col as isize) {
                // SAFETY: String is checked to be ascii
                let vec = unsafe { self.inner.as_mut_vec() };
                vec[index] = c as u8;
                Ok(())
            } else {
                bail!("Invalid index to set char")
            }
        } else {
            bail!("Grid is made up of non-ascii chars, cannot replace single char")
        }
    }

    pub fn find_char(&self, c: char) -> Option<(usize, usize)> {
        self.inner.find(c).map(|index| self.index_to_row_col(index))
    }

    // Assumes all characters are unique
    pub fn char_positions(&self) -> BTreeMap<char, (usize, usize)> {
        let mut map = BTreeMap::new();
        for (i, x) in self.inner.chars().enumerate() {
            map.insert(x, self.index_to_row_col(i));
        }
        // Number of keys should be the same as number of characters
        assert_eq!(map.keys().len(), self.inner.len());
        map
    }
}
