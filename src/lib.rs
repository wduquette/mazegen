//! A library for generating and rendering and working with mazes.  The code is inspired
//! by _Mazes for Programmers_ by Jamis Buck, but isn't a straightforward translation.
pub use crate::grid::*;
pub use crate::image_grid_renderer::*;
pub use crate::pixel::*;
pub use crate::text_grid_renderer::*;
use rand::{thread_rng, Rng};

mod grid;
mod image_grid_renderer;
pub mod molt_grid;
pub mod molt_image;
pub mod molt_rand;
mod pixel;
mod text_grid_renderer;

/// A Cell ID.
///
/// A unique integer ID for each cell in a grid.  Each grid type will provide a conversion between
/// the cell ID and whatever more natural indexing scheme exists for that type.  For example,
/// `Grid` provides a conversion between Cells and (i,j) row/column pairs.
pub type Cell = usize;

/// Algorithm to produce a Grid containing a binary-tree maze
pub fn binary_tree_maze(grid: &mut Grid) {
    grid.clear();

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
    grid.clear();

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

/// Hunt-and-Kill maze algorithm
pub fn hunt_and_kill(grid: &mut Grid) {
    grid.clear();

    // FIRST, Pick a random starting point.
    let mut current: Cell = thread_rng().gen_range(0, grid.num_cells());

    while current != grid.num_cells() {
        let unvisited_neighbors: Vec<Cell> = grid
            .neighbors(current)
            .into_iter()
            .filter(|c| grid.links(*c).is_empty())
            .collect();

        if !unvisited_neighbors.is_empty() {
            // Pick an unvisited neighbor as a random walk.
            let neighbor = sample(&unvisited_neighbors);
            grid.link(current, neighbor);
            current = neighbor;
        } else {
            // Sentinal value: use this to indicate nothing more to do.
            current = grid.num_cells();

            // Hunter Block
            for cell in 0..grid.num_cells() {
                let visited_neighbors: Vec<Cell> = grid
                    .neighbors(cell)
                    .into_iter()
                    .filter(|c| !grid.links(*c).is_empty())
                    .collect();

                if grid.links(cell).is_empty() && !visited_neighbors.is_empty() {
                    current = cell;
                    let neighbor = sample(&visited_neighbors);
                    grid.link(current, neighbor);
                    break;
                }
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
