use rand::{thread_rng, Rng};
use mazegen::Grid;

fn main() {
    let mut grid = Grid::new(10,10);
    binary_tree_maze(&mut grid);
    println!("Made {}", grid);
}

fn binary_tree_maze(grid: &mut Grid) {
    // TODO: consider cell iterator
    let mut rng = thread_rng();

    for cell in 0..grid.num_cells() {
        let mut neighbors = Vec::new();

        if let Some(ncell) = grid.north_of(cell) {
            neighbors.push(ncell);
        }

        if let Some(ecell) = grid.east_of(cell) {
            neighbors.push(ecell);
        }

        if neighbors.len() == 2 {
            let ind: usize = rng.gen_range(0, 2);
            grid.link(cell, neighbors[ind]);
        } else if neighbors.len() == 1 {
            grid.link(cell, neighbors[0]);
        }
    }
}
