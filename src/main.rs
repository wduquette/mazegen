use mazegen::*;

fn main() {
    let mut grid = Grid::new(10, 10);
    binary_tree_maze(&mut grid);
    println!("Made {}", grid);

    grid.clear();
    sidewinder_maze(&mut grid);
    println!("Made {}", grid);

    let image = grid.to_image();
    raster::save(&image, "temp.png").unwrap();
}
