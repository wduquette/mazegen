use mazegen::grid::Grid;
use mazegen::grid::GridTextRenderer;
use molt::check_args;
use molt::molt_err;
use molt::molt_ok;
use molt::types::*;
use molt::Interp;

fn main() {
    use std::env;

    // FIRST, get the command line arguments.
    let args: Vec<String> = env::args().collect();

    // NEXT, initialize the interpreter
    let mut interp = Interp::new();
    interp.add_command("doit", cmd_doit);
    interp.add_command("grid", cmd_grid);
    interp.add_command("maze", cmd_maze);

    // Install a Molt extension crate
    // molt_sample::install(&mut interp).expect("Could not install.");

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

    let mut grid = Grid::new(10, 20);

    mazegen::sidewinder_maze(&mut grid);
    let dists = grid.distances(grid.cell(9, 0));

    let data: Vec<String> = dists
        .iter()
        .map(|x| {
            if x.is_some() {
                x.unwrap().to_string()
            } else {
                "".into()
            }
        })
        .collect();

    let mut out = GridTextRenderer::<String>::new(&grid)
        .auto_width(1)
        .data(&data)
        .render();

    let path: Vec<(usize,usize)> = grid.shortest_path(grid.cell(9, 0), grid.cell(0,19))
        .iter()
        .map(|x| grid.ij(*x))
        .collect();

    out.push_str(&format!("{:?}", path));
    out.push('\n');

    molt_ok!(out)
}

fn cmd_maze(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    interp.call_subcommand(ctx, argv, 1, &MAZE_SUBCOMMANDS)
}

const MAZE_SUBCOMMANDS: [Subcommand; 2] = [
    Subcommand("bintree", cmd_maze_bintree),
    Subcommand("sidewinder", cmd_maze_sidewinder),
];

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

pub fn cmd_grid(interp: &mut Interp, _: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(1, argv, 4, 4, "name rows cols")?;

    let name = argv[1].as_str();
    let rows = argv[2].as_int()?;
    let cols = argv[3].as_int()?;

    if rows < 2 || cols < 2 {
        return molt_err!(
            "expected a grid of size at least 2x2, got {}x{}",
            rows,
            cols
        );
    }

    let grid = Grid::new(rows as usize, cols as usize);
    make_grid_object(interp, name, grid);
    molt_ok!(name)
}

fn make_grid_object(interp: &mut Interp, name: &str, grid: Grid) {
    let ctx = interp.save_context(grid);
    interp.add_context_command(name, obj_grid, ctx);
}

fn obj_grid(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    interp.call_subcommand(ctx, argv, 1, &GRID_SUBCOMMANDS)
}

const GRID_SUBCOMMANDS: [Subcommand; 4] = [
    Subcommand("text", obj_grid_text),
    Subcommand("rows", obj_grid_rows),
    Subcommand("cols", obj_grid_cols),
    Subcommand("render", obj_grid_render),
];

fn obj_grid_text(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 2, 2, "")?;
    let grid = interp.context::<Grid>(ctx);
    molt_ok!(grid.to_string())
}

fn obj_grid_rows(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 2, 2, "")?;
    let grid = interp.context::<Grid>(ctx);
    molt_ok!(grid.num_rows() as MoltInt)
}

fn obj_grid_cols(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 2, 2, "")?;
    let grid = interp.context::<Grid>(ctx);
    molt_ok!(grid.num_cols() as MoltInt)
}

fn obj_grid_render(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 3, 3, "filename")?;
    let filename = argv[2].as_str();
    let grid = interp.context::<Grid>(ctx);
    let image = grid.to_image();
    match image.save(filename) {
        Ok(_) => molt_ok!(),
        Err(_) => molt_err!("error saving grid image"),
    }
}
