//! A bit mask, for defining irregular mazes.
//!
//! * Could use bit-vec crate as basis; would save memory.
//! * Could define all the various flavors of iterators.
//! * Could define a matrix trait, to be shared with Grid.


use crate::Cell;
use crate::sample;
use std::ops::Index;
use std::ops::IndexMut;

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
    fn cell(&self, (i,j): (usize,usize)) -> Cell {
        assert!(i < self.num_rows && j < self.num_cols);
        i * self.num_cols + j
    }

    /// Computes the row and column indices from the cell ID.
    fn ij(&self, cell: Cell) -> (usize,usize) {
        assert!(cell < self.num_cells);
        (cell / self.num_cols, cell % self.num_cols)
    }

    /// Does the mask contain the location?
    pub fn contains(&self, (i,j): (usize,usize)) -> bool {
        // NOTE: No need to check against zero, since we're using an unsigned type.
        i < self.num_rows && j < self.num_cols
    }

    /// Sets the cell's alive/dead flag.
    pub fn set(&mut self, ij: (usize,usize), flag: bool) {
        assert!(self.contains(ij));
        let cell = self.cell(ij);
        self.cells[cell] = flag;
    }

    /// Returns true if the cell is alive, and false otherwise.
    pub fn is_alive(&mut self, ij: (usize,usize)) -> bool {
        assert!(self.contains(ij));
        let cell = self.cell(ij);
        self.cells[cell]
    }

    /// Returns the number of cells that are alive.
    pub fn live_count(&self) -> usize {
        self.cells.iter().copied().filter(|flag| *flag).count()
    }

    /// Returns a list of the live cells in the mask.
    pub fn live_cells(&self) -> Vec<(usize,usize)> {
        self.cells
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, flag)| *flag)
            .map(|(cell, _)| self.ij(cell))
            .collect()
    }

    /// Returns a random cell, guaranteed to be alive.  Only returns None if there
    /// are no live cells.
    pub fn random_cell(&self) -> Option<(usize,usize)> {
        let live_cells = self.live_cells();

        if live_cells.len() > 0 {
            Some(sample(&live_cells))
        } else {
            None
        }
    }
}

impl Index<(usize,usize)> for Mask {
    type Output = bool;

    fn index(&self, idx: (usize,usize)) -> &Self::Output {
        let cell = self.cell(idx);
        &self.cells[cell]
    }
}

impl IndexMut<(usize,usize)> for Mask {
    fn index_mut(&mut self, idx: (usize,usize)) -> &mut Self::Output {
        let cell = self.cell(idx);
        &mut self.cells[cell]
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

        assert_eq!(mask.cell((0, 0)), 0);
        assert_eq!(mask.cell((0, 3)), 3);
        assert_eq!(mask.cell((1, 0)), 6);
        assert_eq!(mask.cell((1, 3)), 9);
        assert_eq!(mask.cell((2, 0)), 12);
        assert_eq!(mask.cell((4, 5)), mask.num_cells() - 1);
    }

    #[test]
    fn test_mask_set_is_alive() {
        let mut mask = Mask::new(5, 6);

        for i in 0..mask.num_rows() {
            for j in 0..mask.num_cols() {
                let cell = (i,j);
                assert!(mask.is_alive(cell));
                mask.set(cell, false);
                assert!(!mask.is_alive(cell));
            }

        }
    }

    #[test]
    fn test_mask_live_count() {
        let mut mask = Mask::new(5, 6);
        assert_eq!(mask.live_count(), 30);

        mask.set((0,0), false);
        assert_eq!(mask.live_count(), 29);
    }

    #[test]
    fn test_mask_index_cell() {
        let mut mask = Mask::new(5, 6);

        for i in 0..mask.num_rows() {
            for j in 0..mask.num_cols() {
                let cell = (i,j);
                assert!(mask[cell]);
                mask[cell] = false;
                assert!(!mask[cell]);
            }
        }
    }

    #[test]
    fn test_live_cells() {
        let mut mask = Mask::new(2, 2);

        assert_eq!(mask.live_cells(), vec![(0,0), (0,1), (1,0), (1,1)]);

        mask[(1,0)] = false;
        assert_eq!(mask.live_cells(), vec![(0,0), (0,1), (1,1)]);
    }
}
