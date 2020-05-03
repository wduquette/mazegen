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
