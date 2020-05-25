//! Molt Grid Commands
use crate::Grid;
use crate::GridDirection;
use crate::ImageGridRenderer;
use molt::check_args;
use molt::molt_err;
use molt::molt_ok;
use molt::types::*;
use molt::Interp;

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

const OBJ_GRID_SUBCOMMANDS: [Subcommand; 19] = [
    Subcommand("cell", obj_grid_cell),
    Subcommand("cells", obj_grid_cells),
    Subcommand("cellto", obj_grid_cell_to),
    Subcommand("clear", obj_grid_clear),
    Subcommand("cols", obj_grid_cols),
    Subcommand("distances", obj_grid_distances),
    Subcommand("i", obj_grid_i),
    Subcommand("ij", obj_grid_ij),
    Subcommand("linked", obj_grid_linked),
    Subcommand("linkedto", obj_grid_linked_to),
    Subcommand("j", obj_grid_j),
    Subcommand("link", obj_grid_link),
    Subcommand("links", obj_grid_links),
    Subcommand("longest", obj_grid_longest),
    Subcommand("neighbors", obj_grid_neighbors),
    Subcommand("render", obj_grid_render),
    Subcommand("rows", obj_grid_rows),
    Subcommand("text", obj_grid_text),
    Subcommand("unlink", obj_grid_unlink),
];

// Converts an (i,j) pair into a cell ID
fn obj_grid_cell(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 4, 4, "i j")?;
    let grid = interp.context::<Grid>(ctx);

    let i = get_grid_row(grid, &argv[2])?;
    let j = get_grid_col(grid, &argv[3])?;

    molt_ok!(grid.cell(i, j) as MoltInt)
}

// Gets the number of cells in the grid.  Cells have IDs in the range `[0..cells)`.
fn obj_grid_cells(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 2, 2, "")?;
    let grid = interp.context::<Grid>(ctx);
    molt_ok!(grid.num_cells() as MoltInt)
}

// Returns the cell in the given direction, or the empty string if none.
fn obj_grid_cell_to(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 4, 4, "cell dir")?;
    let grid = interp.context::<Grid>(ctx);

    let cell = get_grid_cell(grid, &argv[2])?;
    let dir = get_dir(&argv[3])?;

    if let Some(c) = grid.cell_to(cell, dir) {
        molt_ok!(c as MoltInt)
    } else {
        molt_ok!(Value::empty())
    }
}

// Clears the links in the grid.
fn obj_grid_clear(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 2, 2, "")?;
    let grid = interp.context::<Grid>(ctx);

    grid.clear();

    molt_ok!()
}

// Gets the number of columns in the grid.  Columns are indexed `[0..cols)`.
fn obj_grid_cols(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 2, 2, "")?;
    let grid = interp.context::<Grid>(ctx);
    molt_ok!(grid.num_cols() as MoltInt)
}

// $grid distances *cell* ?-list|-dict?
//
// Gets the distances from a given cell as a list indexed by cell ID, or as a
// dictionary of cells and distances.  Defaults to -list.  If a cell is unreachable from
// the given cell, the distance is the empty string.
fn obj_grid_distances(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 3, 4, "cell ?opt?")?;
    let grid = interp.context::<Grid>(ctx);
    let cell = get_grid_cell(grid, &argv[2])?;

    let as_dict = if argv.len() == 4 {
        let opt = argv[3].as_str();
        match opt {
            "-list" => false,
            "-dict" => true,
            _ => {
                return molt_err!("invalid option \"{}\", should be one of: -list, -dict", opt);
            }
        }
    } else {
        false
    };

    if as_dict {
        let dict: MoltDict = grid
            .distances(cell)
            .iter()
            .enumerate()
            .map(|(k, v)| (Value::from(k as MoltInt), from_option(*v)))
            .collect();
        molt_ok!(dict)
    } else {
        let list: MoltList = grid
            .distances(cell)
            .iter()
            .map(|c| from_option(*c))
            .collect();
        molt_ok!(list)
    }
}

// Gets the cell's row coordinate given its cell ID
fn obj_grid_i(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 3, 3, "cell")?;
    let grid = interp.context::<Grid>(ctx);

    let cell = get_grid_cell(grid, &argv[2])?;

    molt_ok!(grid.i(cell) as MoltInt)
}

// Gets the cell's row/col coordinate pair given its cell ID
fn obj_grid_ij(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 3, 3, "cell")?;
    let grid = interp.context::<Grid>(ctx);

    let cell = get_grid_cell(grid, &argv[2])?;

    let (i, j) = grid.ij(cell);

    molt_ok!(vec![Value::from(i as MoltInt), Value::from(j as MoltInt)])
}

// Gets the cell's column coordinate given its cell ID
fn obj_grid_j(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 3, 3, "cell")?;
    let grid = interp.context::<Grid>(ctx);

    let cell = get_grid_cell(grid, &argv[2])?;

    molt_ok!(grid.j(cell) as MoltInt)
}

// Links the two cells, which must be neighbors.
fn obj_grid_link(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 4, 4, "cell1 cell2")?;
    let grid = interp.context::<Grid>(ctx);

    let cell1 = get_grid_cell(grid, &argv[2])?;
    let cell2 = get_grid_cell(grid, &argv[3])?;

    if grid.neighbors(cell1).contains(&cell2) {
        grid.link(cell1, cell2);
        molt_ok!()
    } else {
        molt_err!("not a neighbor of cell {}: \"{}\"", cell1, cell2)
    }
}

// Returns true if the cells are linked, and false otherwise
fn obj_grid_linked(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 4, 4, "cell1 cell2")?;
    let grid = interp.context::<Grid>(ctx);

    let cell1 = get_grid_cell(grid, &argv[2])?;
    let cell2 = get_grid_cell(grid, &argv[3])?;

    molt_ok!(grid.is_linked(cell1, cell2))
}

// Returns true if the cell is linked in the given direction, and false otherwise
fn obj_grid_linked_to(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 4, 4, "cell dir")?;
    let grid = interp.context::<Grid>(ctx);

    let cell = get_grid_cell(grid, &argv[2])?;
    let dir = get_dir(&argv[3])?;

    molt_ok!(grid.is_linked_to(cell, dir))
}

// Gets a list of the IDs of the cell's linked neighbors
fn obj_grid_links(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 3, 3, "cell")?;
    let grid = interp.context::<Grid>(ctx);

    let cell = get_grid_cell(grid, &argv[2])?;

    let list: MoltList = grid
        .links(cell)
        .iter()
        .map(|c| Value::from(*c as MoltInt))
        .collect();

    molt_ok!(list)
}

// $grid longest
//
// Returns the longest path through the maze as a list of cell IDs
fn obj_grid_longest(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 2, 2, "")?;
    let grid = interp.context::<Grid>(ctx);

    let list: MoltList = grid
        .longest_path()
        .iter()
        .map(|c| Value::from(*c as MoltInt))
        .collect();
    molt_ok!(list)
}

// Gets a list of the IDs of the cell's neighbors
fn obj_grid_neighbors(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 3, 3, "cell")?;
    let grid = interp.context::<Grid>(ctx);

    let cell = get_grid_cell(grid, &argv[2])?;

    let list: MoltList = grid
        .neighbors(cell)
        .iter()
        .map(|c| Value::from(*c as MoltInt))
        .collect();

    molt_ok!(list)
}

// Renders the grid as an image, saving it to disk.
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

// Gets the number of rows in the grid.  Rows are indexed `[0..rows)`.
fn obj_grid_rows(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 2, 2, "")?;
    let grid = interp.context::<Grid>(ctx);
    molt_ok!(grid.num_rows() as MoltInt)
}

// Renders the grid as a text string, which is returned.
fn obj_grid_text(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 2, 2, "")?;
    let grid = interp.context::<Grid>(ctx);
    molt_ok!(grid.to_string())
}

// Unlinks the two cells, which must be neighbors.
fn obj_grid_unlink(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 4, 4, "cell1 cell2")?;
    let grid = interp.context::<Grid>(ctx);

    let cell1 = get_grid_cell(grid, &argv[2])?;
    let cell2 = get_grid_cell(grid, &argv[3])?;

    if grid.neighbors(cell1).contains(&cell2) {
        grid.unlink(cell1, cell2);
        molt_ok!()
    } else {
        molt_err!("not a neighbor of cell {}: \"{}\"", cell1, cell2)
    }
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

/// Get a grid cell for the given grid.
fn get_grid_cell(grid: &Grid, arg: &Value) -> Result<usize, Exception> {
    let num = arg.as_int()?;

    if num >= 0 && num < grid.num_cells() as MoltInt {
        Ok(num as usize)
    } else {
        molt_err!("expected grid cell ID, got \"{}\"", num)
    }
}

fn from_option(val: Option<usize>) -> Value {
    if let Some(t) = val {
        Value::from(t as MoltInt)
    } else {
        Value::empty()
    }
}

fn get_dir(value: &Value) -> Result<GridDirection, Exception> {
    if let Some(x) = value.as_copy::<GridDirection>() {
        Ok(x)
    } else {
        Err(Exception::molt_err(Value::from(
            "expected a grid direction",
        )))
    }
}
