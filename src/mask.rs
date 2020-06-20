//! A mask, for defining irregular mazes.

use crate::Cell;
use crate::sample;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Mask {
    num_rows: usize,
    num_cols: usize,
    num_cells: usize,
    cells: Vec<bool>,
}

impl Mask {
    /// Create a new mask, with all bits set.
    pub fn new(num_rows: usize, num_cols: usize) -> Self {
        // FIRST, initialize the cells vector
        let num_cells = num_rows * num_cols;
        let mut cells = Vec::with_capacity(num_cells);

        for _ in 0..num_cells {
            cells.push(true);
        }

        let mask = Self {
            num_rows,
            num_cols,
            num_cells,
            cells,
        };


        mask
    }

    /// The number of rows in the mask.
    pub fn num_rows(&self) -> usize {
        self.num_rows
    }

    /// The number of columns in the mask.
    pub fn num_cols(&self) -> usize {
        self.num_cols
    }

    /// The number of cells in the mask.
    pub fn num_cells(&self) -> usize {
        self.num_cells
    }

    /// Computes the cell from the row and column.
    pub fn cell(&self, i: usize, j: usize) -> Cell {
        assert!(i < self.num_rows && j < self.num_cols);
        i * self.num_cols + j
    }

    /// Computes the row index from the cell ID.
    pub fn i(&self, cell: Cell) -> usize {
        assert!(self.contains(cell));
        cell / self.num_cols
    }

    /// Computes the column index from the cell ID.
    pub fn j(&self, cell: Cell) -> usize {
        assert!(self.contains(cell));
        cell % self.num_cols
    }

    /// Computes the row and column indices from the cell ID.
    pub fn ij(&self, cell: Cell) -> (usize, usize) {
        assert!(self.contains(cell));
        (cell / self.num_cols, cell % self.num_cols)
    }

    /// Does the mask contain the location?
    pub fn contains(&self, cell: Cell) -> bool {
        // NOTE: No need to check against zero, since we're using an unsigned type.
        cell < self.num_cells
    }

    /// Sets the cell's alive/dead flag.
    pub fn set(&mut self, cell: Cell, flag: bool) {
        assert!(self.contains(cell));
        self.cells[cell] = flag;
    }

    /// Kills the given cell.
    pub fn kill(&mut self, cell: Cell) {
        self.set(cell, false);
    }

    /// Returns true if the cell is alive, and false otherwise.
    pub fn is_alive(&mut self, cell: Cell) -> bool {
        assert!(self.contains(cell));
        self.cells[cell]
    }

    /// Returns the number of cells that are alive.
    pub fn live_count(&self) -> usize {
        self.cells.iter().copied().filter(|flag| *flag).count()
    }

    /// Returns a random cell, guaranteed to be alive.  Only returns None if there
    /// are no live cells.
    pub fn random_cell(&self) -> Option<Cell> {
        let live_cells: Vec<Cell> = self.cells
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, flag)| *flag)
            .map(|(cell, _)| cell)
            .collect();

        if live_cells.len() > 0 {
            Some(sample(&live_cells))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_new() {
        let mask = Mask::new(5, 6);

        assert_eq!(mask.num_rows(), 5);
        assert_eq!(mask.num_cols(), 6);
        assert_eq!(mask.num_cells(), 30);
    }

    #[test]
    fn test_mask_cell() {
        let mask = Mask::new(5, 6);

        assert_eq!(mask.cell(0, 0), 0);
        assert_eq!(mask.cell(0, 3), 3);
        assert_eq!(mask.cell(1, 0), 6);
        assert_eq!(mask.cell(1, 3), 9);
        assert_eq!(mask.cell(2, 0), 12);
        assert_eq!(mask.cell(4, 5), mask.num_cells() - 1);
    }

    #[test]
    fn test_mask_i_j() {
        let mask = Mask::new(5, 6);

        assert_eq!(mask.i(0), 0);
        assert_eq!(mask.j(0), 0);

        assert_eq!(mask.i(3), 0);
        assert_eq!(mask.j(3), 3);

        assert_eq!(mask.i(6), 1);
        assert_eq!(mask.j(6), 0);

        assert_eq!(mask.i(9), 1);
        assert_eq!(mask.j(9), 3);
    }

    #[test]
    fn test_mask_cell_i_j() {
        let mask = Mask::new(5, 6);

        for i in 0..mask.num_rows() {
            for j in 0..mask.num_cols() {
                let cell = mask.cell(i, j);
                assert!(mask.contains(cell));
                assert_eq!(mask.i(cell), i);
                assert_eq!(mask.j(cell), j);
            }
        }
    }
}
