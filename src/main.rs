use rand::{thread_rng, Rng};
use mazegen::{Grid, Cell};

fn main() {
    let mut grid = Grid::new(10,10);
    binary_tree_maze(&mut grid);
    println!("Made {}", grid);
}

fn binary_tree_maze(grid: &mut Grid) {
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

fn sample(vec: &[Cell]) -> Cell {
    assert!(!vec.is_empty());

    if vec.len() == 1 {
        return vec[0];
    }

    let mut rng = thread_rng();
    let ind: usize = rng.gen_range(0, vec.len());
    vec[ind]
}
