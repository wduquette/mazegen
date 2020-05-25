//! Molt Image Commands
use molt::Interp;
use molt::check_args;
use molt::molt_err;
use molt::molt_ok;
use molt::types::*;
use crate::Grid;
use crate::ImageGridRenderer;

/// Installs the Molt grid commands into the interpreter.
pub fn install(interp: &mut Interp) {
    interp.add_command("grid", cmd_grid);
}

/// Grid constructor: creates a new grid called "name" with the specified number of
/// rows and columns.
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

/// Makes a Molt object command for the given Grid with the given name.
pub fn make_grid_object(interp: &mut Interp, name: &str, grid: Grid) {
    let ctx = interp.save_context(grid);
    interp.add_context_command(name, obj_grid, ctx);
}

fn obj_grid(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    interp.call_subcommand(ctx, argv, 1, &OBJ_GRID_SUBCOMMANDS)
}

const OBJ_GRID_SUBCOMMANDS: [Subcommand; 6] = [
    Subcommand("cell", obj_grid_cell),
    Subcommand("cells", obj_grid_cells),
    Subcommand("cols", obj_grid_cols),
    Subcommand("render", obj_grid_render),
    Subcommand("rows", obj_grid_rows),
    Subcommand("text", obj_grid_text),
];

fn obj_grid_cell(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 4, 4, "i j")?;
    let grid = interp.context::<Grid>(ctx);

    let i = get_grid_row(grid, &argv[2])?;
    let j = get_grid_col(grid, &argv[3])?;

    molt_ok!(grid.cell(i, j) as MoltInt)
}

fn obj_grid_cells(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 2, 2, "")?;
    let grid = interp.context::<Grid>(ctx);
    molt_ok!(grid.num_cells() as MoltInt)
}

fn obj_grid_cols(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 2, 2, "")?;
    let grid = interp.context::<Grid>(ctx);
    molt_ok!(grid.num_cols() as MoltInt)
}

fn obj_grid_render(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 3, 0, "filename ?options...?")?;
    let filename = argv[2].as_str();
    let grid = interp.context::<Grid>(ctx);

    let opt_args = &argv[3..argv.len()];
    let mut queue = opt_args.iter();

    let mut renderer = ImageGridRenderer::new();

    while let Some(opt) = queue.next() {
        let val = if let Some(opt_val) = queue.next() {
            opt_val
        } else {
            return molt_err!("missing option value");
        };

        match opt.as_str() {
            "-cellsize" => {
                let size = val.as_int()?;
                if size < 1 {
                    return molt_err!("invalid -cellsize, expected positive integer");
                }
                renderer.cell_size(size as usize);
            }
            "-borderwidth" => {
                let wid = val.as_int()?;
                if wid < 1 {
                    return molt_err!("invalid -borderwidth, expected positive integer");
                }
                renderer.border_width(wid as usize);
            }
            _ => {
                return molt_err!("invalid option: \"{}\"", opt);
            }
        }
    }

    let image = renderer.render(&grid);

    match image.save(filename) {
        Ok(_) => molt_ok!(),
        Err(_) => molt_err!("error saving grid image"),
    }
}

fn obj_grid_rows(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 2, 2, "")?;
    let grid = interp.context::<Grid>(ctx);
    molt_ok!(grid.num_rows() as MoltInt)
}

fn obj_grid_text(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 2, 2, "")?;
    let grid = interp.context::<Grid>(ctx);
    molt_ok!(grid.to_string())
}

/// Get a grid row for the given grid.
fn get_grid_row(grid: &Grid, arg: &Value) -> Result<usize, Exception> {
    let num = arg.as_int()?;

    if num >= 0 && num < grid.num_rows() as MoltInt {
        Ok(num as usize)
    } else {
        molt_err!("expected grid row index, got \"{}\"", num)
    }
}

/// Get a grid column for the given grid.
fn get_grid_col(grid: &Grid, arg: &Value) -> Result<usize, Exception> {
    let num = arg.as_int()?;

    if num >= 0 && num < grid.num_cols() as MoltInt {
        Ok(num as usize)
    } else {
        molt_err!("expected grid column index, got \"{}\"", num)
    }
}
