//! A library for generating and rendering and working with mazes.  The code is inspired
//! by _Mazes for Programmers_ by Jamis Buck, but isn't a straightforward translation.
use crate::grid::*;
use rand::{thread_rng, Rng};

pub mod grid;
pub mod pixel;

/// A Cell ID.
///
/// A unique integer ID for each cell in a grid.  Each grid type will provide a conversion between
/// the cell ID and whatever more natural indexing scheme exists for that type.  For example,
/// `Grid` provides a conversion between Cells and (i,j) row/column pairs.
pub type Cell = usize;

/// Algorithm to produce a Grid containing a binary-tree maze
pub fn binary_tree_maze(grid: &mut Grid) {
    for cell in 0..grid.num_cells() {
        let mut neighbors = Vec::new();

        if let Some(ncell) = grid.north_of(cell) {
            neighbors.push(ncell);
        }

        if let Some(ecell) = grid.east_of(cell) {
            neighbors.push(ecell);
        }

        if !neighbors.is_empty() {
            grid.link(cell, sample(&neighbors));
        }
    }
}

/// Algorithm to produce a Grid containing a sidewinder maze
pub fn sidewinder_maze(grid: &mut Grid) {
    for i in 0..grid.num_rows() {
        let mut run = Vec::new();

        for j in 0..grid.num_cols() {
            let cell = grid.cell(i, j);
            run.push(cell);

            let at_eastern_boundary = grid.east_of(cell).is_none();
            let at_northern_boundary = grid.north_of(cell).is_none();
            let should_close_out = at_eastern_boundary || (!at_northern_boundary && !flip());

            if should_close_out {
                let member = sample(&run);
                if let Some(ncell) = grid.north_of(member) {
                    grid.link(member, ncell);
                }
                run.clear();
            } else {
                grid.link(cell, grid.east_of(cell).expect("a cell"));
            }
        }
    }
}

/// Picks a random cell from a slice of cells.
pub fn sample(vec: &[Cell]) -> Cell {
    assert!(!vec.is_empty());

    if vec.len() == 1 {
        return vec[0];
    }

    let mut rng = thread_rng();
    let ind: usize = rng.gen_range(0, vec.len());
    vec[ind]
}

/// Flips a coin, returning true for heads and false for tails.
pub fn flip() -> bool {
    thread_rng().gen_bool(0.5)
}
