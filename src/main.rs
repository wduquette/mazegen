use rand::{thread_rng, Rng};
use mazegen::Grid;
use mazegen::Loc;

fn main() {
    let mut grid = Grid::new(10,10);
    binary_tree_maze(&mut grid);
    println!("Made {}", grid);


}

fn binary_tree_maze(grid: &mut Grid) {
    // TODO: consider cell iterator
    let mut rng = thread_rng();

    for i in 0..grid.num_rows() {
        for j in 0..grid.num_cols() {
            let loc = Loc::new(i, j);
            let mut neighbors = Vec::new();

            if grid.contains(loc.north()) {
                neighbors.push(loc.north());
            }

            if grid.contains(loc.east()) {
                neighbors.push(loc.east());
            }

            if neighbors.len() == 2 {
                let ind: usize = rng.gen_range(0, 2);
                grid.link(loc, neighbors[ind]);
            } else if neighbors.len() == 1 {
                grid.link(loc, neighbors[0]);
            }
        }
    }
}
