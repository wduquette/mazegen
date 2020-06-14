use mazegen::molt_grid::make_grid_object;
use mazegen::Cell;
use mazegen::Grid;
use mazegen::ImageGridRenderer;
use mazegen::TextGridRenderer;
use molt::check_args;
use molt::molt_err;
use molt::molt_ok;
use molt::types::*;
use molt::Interp;
use std::collections::HashMap;

fn main() {
    use std::env;

    // FIRST, get the command line arguments.
    let args: Vec<String> = env::args().collect();

    // NEXT, initialize the interpreter
    let mut interp = Interp::new();
    interp.add_command("doit", cmd_doit);
    interp.add_command("maze", cmd_maze);

    // Install a Molt extension
    mazegen::molt_grid::install(&mut interp);
    mazegen::molt_image::install(&mut interp);
    mazegen::molt_rand::install(&mut interp);

    // NEXT, evaluate the file, if any.
    if args.len() > 1 {
        molt_shell::script(&mut interp, &args[1..]);
    } else {
        molt_shell::repl(&mut interp);
    }
}

fn cmd_doit(_interp: &mut Interp, _: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(1, argv, 1, 1, "")?;

    // FIRST, produce a maze.
    let mut grid = Grid::new(10, 20);
    mazegen::recursive_backtracker(&mut grid);

    // NEXT, compute the longest path in the maze.
    let cellpath = grid.longest_path();

    // NEXT, compute the distances from the starting point of the path.
    let dists = grid.distances(cellpath[0]);

    // NEXT, prepare to produce output.
    let mut out = String::new();
    let textmapper = TextGridRenderer::new().auto_width(1).to_owned();

    // NEXT, render the maze with distances.
    out.push_str("Distances from start:\n");
    out.push_str(&textmapper.render_with(&grid, |c| dists[c]));

    // NEXT, render the dead ends.
    let dead_ends = grid.dead_ends();

    out.push_str("\nDead ends:\n");
    out.push_str(&textmapper.render_with(&grid, |c| {
        Some(if dead_ends.contains(&c) { "***" } else { "" })
    }));

    // NEXT, render the path
    let distpath: HashMap<Cell, usize> =
        cellpath.iter().map(|c| (*c, dists[*c].unwrap())).collect();

    out.push_str("\nPath, from start to finish:\n");
    out.push_str(&textmapper.render_with(&grid, |c| distpath.get(&c)));

    // NEXT, save an image of the path as temp.png.
    let image = ImageGridRenderer::new()
        .cell_size(30)
        .border_width(5)
        .render_with(&grid, |c| {
            Some(if distpath.contains_key(&c) { 100 } else { 0 })
        });

    if image.save("path.png").is_err() {
        return molt_err!("error saving grid image");
    }

    // NEXT, save an image of the grid with dead ends marked
    let image = ImageGridRenderer::new()
        .cell_size(30)
        .border_width(5)
        .render_with(&grid, |c| {
            Some(if dead_ends.contains(&c) { 100 } else { 0 })
        });

    if image.save("dead_ends.png").is_err() {
        return molt_err!("error saving grid image");
    }

    molt_ok!(out)
}

fn cmd_maze(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    interp.call_subcommand(ctx, argv, 1, &MAZE_SUBCOMMANDS)
}

const MAZE_SUBCOMMANDS: [Subcommand; 4] = [
    Subcommand("backtracker", cmd_maze_backtracker),
    Subcommand("bintree", cmd_maze_bintree),
    Subcommand("huntandkill", cmd_maze_huntandkill),
    Subcommand("sidewinder", cmd_maze_sidewinder),
];

fn cmd_maze_backtracker(interp: &mut Interp, _: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 5, 5, "name rows cols")?;

    let name = argv[2].as_str();
    let rows = argv[3].as_int()?;
    let cols = argv[4].as_int()?;

    if rows < 2 || cols < 2 {
        return molt_err!("expected a max of size at least 2x2, got {}x{}", rows, cols);
    }

    let mut grid = Grid::new(rows as usize, cols as usize);
    mazegen::recursive_backtracker(&mut grid);
    make_grid_object(interp, name, grid);

    molt_ok!(name)
}

fn cmd_maze_bintree(interp: &mut Interp, _: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 5, 5, "name rows cols")?;

    let name = argv[2].as_str();
    let rows = argv[3].as_int()?;
    let cols = argv[4].as_int()?;

    if rows < 2 || cols < 2 {
        return molt_err!("expected a max of size at least 2x2, got {}x{}", rows, cols);
    }

    let mut grid = Grid::new(rows as usize, cols as usize);
    mazegen::binary_tree_maze(&mut grid);
    make_grid_object(interp, name, grid);

    molt_ok!(name)
}

fn cmd_maze_huntandkill(interp: &mut Interp, _: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 5, 5, "name rows cols")?;

    let name = argv[2].as_str();
    let rows = argv[3].as_int()?;
    let cols = argv[4].as_int()?;

    if rows < 2 || cols < 2 {
        return molt_err!("expected a max of size at least 2x2, got {}x{}", rows, cols);
    }

    let mut grid = Grid::new(rows as usize, cols as usize);
    mazegen::hunt_and_kill(&mut grid);
    make_grid_object(interp, name, grid);

    molt_ok!(name)
}

fn cmd_maze_sidewinder(interp: &mut Interp, _: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 5, 5, "name rows cols")?;

    let name = argv[2].as_str();
    let rows = argv[3].as_int()?;
    let cols = argv[4].as_int()?;

    if rows < 2 || cols < 2 {
        return molt_err!("expected a max of size at least 2x2, got {}x{}", rows, cols);
    }

    let mut grid = Grid::new(rows as usize, cols as usize);
    mazegen::sidewinder_maze(&mut grid);
    make_grid_object(interp, name, grid);

    molt_ok!(name)
}
