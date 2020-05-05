use molt::check_args;
use molt::molt_ok;
use molt::Interp;
use molt::types::*;
use mazegen::Grid;

fn main() {
    use std::env;

    // FIRST, get the command line arguments.
    let args: Vec<String> = env::args().collect();

    // NEXT, initialize the interpreter
    let mut interp = Interp::new();
    interp.add_command("doit", cmd_doit);
    interp.add_command("grid", cmd_grid);

    // Install a Molt extension crate
    // molt_sample::install(&mut interp).expect("Could not install.");

    // NEXT, evaluate the file, if any.
    if args.len() > 1 {
        molt_shell::script(&mut interp, &args[1..]);
    } else {
        molt_shell::repl(&mut interp);
    }
}

pub fn cmd_doit(_interp: &mut Interp,  _: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(1, argv, 1, 1, "")?;

    let mut grid = Grid::new(10, 20);
    mazegen::sidewinder_maze(&mut grid);
    let image = grid.to_image();
    image.save("temp.png").unwrap();

    molt_ok!(grid.to_string())
}

pub fn cmd_grid(interp: &mut Interp, _: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(1, argv, 2, 2, "name")?;

    let name = argv[1].as_str();
    let grid = Grid::new(10, 20);
    let ctx = interp.save_context(grid);
    interp.add_context_command(name, obj_grid, ctx);

    molt_ok!(name)
}

pub fn obj_grid(interp: &mut Interp, ctx : ContextID, argv: &[Value]) -> MoltResult {
    interp.call_subcommand(ctx, argv, 1, &GRID_SUBCOMMANDS)
}

const GRID_SUBCOMMANDS: [Subcommand; 3] = [
    Subcommand("text", obj_grid_text),
    Subcommand("rows", obj_grid_rows),
    Subcommand("cols", obj_grid_cols),
];

pub fn obj_grid_text(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 2, 2, "")?;

    let grid = interp.context::<Grid>(ctx);
    molt_ok!(grid.to_string())
}

pub fn obj_grid_rows(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 2, 2, "")?;

    let grid = interp.context::<Grid>(ctx);
    molt_ok!(grid.num_rows() as MoltInt)
}

pub fn obj_grid_cols(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 2, 2, "")?;
    let grid = interp.context::<Grid>(ctx);
    molt_ok!(grid.num_cols() as MoltInt)
}
