use mazegen::Grid;
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

    // Install a Molt extension crate
    // molt_sample::install(&mut interp).expect("Could not install.");

    // NEXT, evaluate the file, if any.
    if args.len() > 1 {
        molt_shell::script(&mut interp, &args[1..]);
    } else {
        molt_shell::repl(&mut interp);
    }
}

pub fn cmd_doit(_interp: &mut Interp, _: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(1, argv, 1, 1, "")?;

    molt_ok!("did it")
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
    let ctx = interp.save_context(grid);
    interp.add_context_command(name, obj_grid, ctx);

    molt_ok!(name)
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
        Err(_) => molt_err!("error saving grid image")
    }
}
