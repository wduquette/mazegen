//! Molt Grid Commands
use crate::CellID;
use crate::Grid;
use crate::GridDirection;
use crate::ImageGridRenderer;
use crate::TextGridRenderer;
use molt::check_args;
use molt::molt_err;
use molt::molt_ok;
use molt::types::*;
use molt::Interp;

/// Installs the Molt grid commands into the interpreter.
pub fn install(interp: &mut Interp) {
    interp.add_command("grid", cmd_grid);
}

/// How i,j coordinates are represented.
#[derive(Clone,Copy,Eq,PartialEq)]
enum Coord {
    // As members of a flat list
    Flat,

    // As an {i j} pair, a single element in a list
    Pair
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

const OBJ_GRID_SUBCOMMANDS: [Subcommand; 17] = [
    Subcommand("cells", obj_grid_cells),
    Subcommand("cellto", obj_grid_cell_to),
    Subcommand("clear", obj_grid_clear),
    Subcommand("cols", obj_grid_cols),
    Subcommand("deadends", obj_grid_deadends),
    Subcommand("distances", obj_grid_distances),
    Subcommand("linked", obj_grid_linked),
    Subcommand("linkedto", obj_grid_linked_to),
    Subcommand("link", obj_grid_link),
    Subcommand("links", obj_grid_links),
    Subcommand("longest", obj_grid_longest),
    Subcommand("neighbors", obj_grid_neighbors),
    Subcommand("path", obj_grid_path),
    Subcommand("render", obj_grid_render),
    Subcommand("rows", obj_grid_rows),
    Subcommand("text", obj_grid_text),
    Subcommand("unlink", obj_grid_unlink),
];

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
    check_args(2, argv, 5, 5, "i j dir")?;
    let grid = interp.context::<Grid>(ctx);

    let i = get_grid_row(grid, &argv[2])?;
    let j = get_grid_col(grid, &argv[3])?;
    let dir = get_dir(&argv[4])?;

    let cell = grid.cell(i,j);

    if let Some(c) = grid.cell_to(cell, dir) {
        molt_ok!(pair(grid.ij(c)))
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

// $grid deadends
//
// Returns a list of the cells that are dead-ends (i.e., that link to one other cell).
fn obj_grid_deadends(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 2, 3, "?-flat|-pairs?")?;
    let grid = interp.context::<Grid>(ctx);

    let kind = if argv.len() == 3 {
        get_coord_type(&argv[2])?
    } else {
        Coord::Flat
    };

    molt_ok!(list_of_cells(grid, &grid.dead_ends(), kind))
}

// $grid distances *i j* ?-flat|-pairs?
//
// Gets the distances from a given cell as a list indexed by cell ID, or as a
// dictionary of cells and distances.  Defaults to -list.  If a cell is unreachable from
// the given cell, the distance is the empty string.
fn obj_grid_distances(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 4, 5, "i j ?opt?")?;
    let grid = interp.context::<Grid>(ctx);
    let i = get_grid_row(grid, &argv[2])?;
    let j = get_grid_col(grid, &argv[3])?;
    let kind = if argv.len() == 5 {
        get_coord_type(&argv[4])?
    } else {
        Coord::Flat
    };

    let cell = grid.cell(i,j);
    let dists = grid.distances(cell);


    // Do something simple, as this is going to change again almost immediately.
    let mut result = Vec::new();

    for cell in 0..grid.num_cells() {
        if let Some(val) = dists[cell] {
            let (i,j) = grid.ij(cell);

            if kind == Coord::Pair {
                result.push(Value::from(pair((i,j))));
            } else {
                result.push(Value::from(i as MoltInt));
                result.push(Value::from(j as MoltInt));
            }
            result.push(Value::from(val as MoltInt));
        }
    }

    molt_ok!(result)
}

// Links the two cells, which must be neighbors.
fn obj_grid_link(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 6, 6, "i1 j1 i2 j2")?;
    let grid = interp.context::<Grid>(ctx);

    let i1 = get_grid_row(grid, &argv[2])?;
    let j1 = get_grid_col(grid, &argv[3])?;
    let i2 = get_grid_row(grid, &argv[4])?;
    let j2 = get_grid_col(grid, &argv[5])?;

    let cell1 = grid.cell(i1,j1);
    let cell2 = grid.cell(i2,j2);

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
    check_args(2, argv, 6, 6, "i1 j1 i2 j2")?;
    let grid = interp.context::<Grid>(ctx);

    let i1 = get_grid_row(grid, &argv[2])?;
    let j1 = get_grid_col(grid, &argv[3])?;
    let i2 = get_grid_row(grid, &argv[4])?;
    let j2 = get_grid_col(grid, &argv[5])?;

    let cell1 = grid.cell(i1,j1);
    let cell2 = grid.cell(i2,j2);

    molt_ok!(grid.is_linked(cell1, cell2))
}

// Returns true if the cell is linked in the given direction, and false otherwise
fn obj_grid_linked_to(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 5, 5, "i j dir")?;
    let grid = interp.context::<Grid>(ctx);

    let i = get_grid_row(grid, &argv[2])?;
    let j = get_grid_col(grid, &argv[3])?;
    let dir = get_dir(&argv[4])?;

    let cell = grid.cell(i,j);

    molt_ok!(grid.is_linked_to(cell, dir))
}

// Gets a list of the IDs of the cell's linked neighbors
fn obj_grid_links(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 4, 5, "i j ?-flat|-pairs?")?;
    let grid = interp.context::<Grid>(ctx);

    let i = get_grid_row(grid, &argv[2])?;
    let j = get_grid_col(grid, &argv[3])?;

    let kind = if argv.len() == 5 {
        get_coord_type(&argv[4])?
    } else {
        Coord::Flat
    };

    let cell = grid.cell(i,j);
    molt_ok!(list_of_cells(grid, &grid.links(cell), kind))
}

// $grid longest
//
// Returns the longest path through the maze as a list of cell IDs
fn obj_grid_longest(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 2, 3, "?-flat|-pairs?")?;
    let grid = interp.context::<Grid>(ctx);

    let kind = if argv.len() == 3 {
        get_coord_type(&argv[2])?
    } else {
        Coord::Flat
    };

    molt_ok!(list_of_cells(grid, &grid.longest_path(), kind))
}

// Gets a list of the IDs of the cell's neighbors
fn obj_grid_neighbors(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 4, 5, "i j ?-flat|-pairs?")?;
    let grid = interp.context::<Grid>(ctx);

    let i = get_grid_row(grid, &argv[2])?;
    let j = get_grid_col(grid, &argv[3])?;

    let kind = if argv.len() == 5 {
        get_coord_type(&argv[4])?
    } else {
        Coord::Flat
    };

    let cell = grid.cell(i,j);

    molt_ok!(list_of_cells(grid, &grid.neighbors(cell), kind))
}

// $grid path i1 j1 i2 j2 ?-flat|-pairs?
//
// Returns a path through the maze from i1,j1, to i2,j2 as a list of cell coordinates.
fn obj_grid_path(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 6, 7, "i1 j1 i2 j2 ?-flat|-pairs?")?;
    let grid = interp.context::<Grid>(ctx);

    let i1 = get_grid_row(grid, &argv[2])?;
    let j1 = get_grid_col(grid, &argv[3])?;
    let i2 = get_grid_row(grid, &argv[4])?;
    let j2 = get_grid_col(grid, &argv[5])?;

    let kind = if argv.len() == 7 {
        get_coord_type(&argv[6])?
    } else {
        Coord::Flat
    };

    let cell1 = grid.cell(i1,j1);
    let cell2 = grid.cell(i2,j2);

    molt_ok!(list_of_cells(grid, &grid.shortest_path(cell1, cell2), kind))
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
    check_args(2, argv, 2, 0, "?options...?")?;
    let grid = interp.context::<Grid>(ctx);

    let mut renderer = TextGridRenderer::new();

    let opt_args = &argv[2..argv.len()];
    let mut queue = opt_args.iter();

    enum Data {
        None,
        List(Value),
        Dict(Value),
    };

    let mut data = Data::None;

    while let Some(opt) = queue.next() {
        let val = if let Some(opt_val) = queue.next() {
            opt_val
        } else {
            return molt_err!("missing option value");
        };

        match opt.as_str() {
            "-cellwidth" => {
                let size = val.as_int()?;
                if size < 1 {
                    return molt_err!("invalid -cellwidth, expected positive integer");
                }
                renderer.cell_width(size as usize);
            }
            "-autowidth" => {
                let margin = val.as_int()?;
                if margin < 0 {
                    return molt_err!("invalid -autowidth, expected non-negative integer");
                }
                renderer.auto_width(margin as usize);
            }
            "-datalist" => {
                let list = val.as_list()?;
                if list.len() != grid.num_cells() {
                    return molt_err!("invalid -datalist, expected {} items", grid.num_cells());
                }
                data = Data::List(val.clone());
            }
            "-datadict" => {
                let _ = val.as_dict()?; // Just verify that it's a valid dict.
                data = Data::Dict(val.clone());
            }
            _ => {
                return molt_err!("invalid option: \"{}\"", opt);
            }
        }
    }

    match data {
        Data::None => molt_ok!(renderer.render(&grid)),
        Data::List(val) => {
            let list = val.as_list()?; // Already has list type.
            molt_ok!(renderer.render_with(&grid, |c| Some(list[c].as_str())))
        }
        Data::Dict(val) => {
            let dict = val.as_dict()?; // Already has dict type.
            molt_ok!(renderer.render_with(&grid, |c| dict
                .get(&Value::from(c as MoltInt))
                .map(|v| v.as_str())))
        }
    }
}

// Unlinks the two cells, which must be neighbors.
fn obj_grid_unlink(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 6, 6, "i1 j1 i2 j2")?;
    let grid = interp.context::<Grid>(ctx);

    let i1 = get_grid_row(grid, &argv[2])?;
    let j1 = get_grid_col(grid, &argv[3])?;
    let i2 = get_grid_row(grid, &argv[4])?;
    let j2 = get_grid_col(grid, &argv[5])?;

    let cell1 = grid.cell(i1,j1);
    let cell2 = grid.cell(i2,j2);

    if grid.neighbors(cell1).contains(&cell2) {
        grid.unlink(cell1, cell2);
        molt_ok!()
    } else {
        molt_err!("not a neighbor of cell {}: \"{}\"", cell1, cell2)
    }
}

//------------------------------------------------------------------------
// Helpers

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

/// Returns a list of cells as either a -flat or a -pairs list
fn list_of_cells(grid: &Grid, cells: &[CellID], kind: Coord) -> MoltList {
    match kind {
        Coord::Flat => flat_list_of_coords(grid, cells),
        Coord::Pair => list_of_coord_pairs(grid, cells),
    }
}

/// returns a -flat list of cell coordinates
fn flat_list_of_coords(grid: &Grid, cells: &[CellID]) -> MoltList {
    let mut list = Vec::new();

    for cell in cells {
        let (i,j) = grid.ij(*cell);
        list.push(Value::from(i as MoltInt));
        list.push(Value::from(j as MoltInt));
    }

    list
}

/// returns a -pairs list of cell coordinates
fn list_of_coord_pairs(grid: &Grid, cells: &[CellID]) -> MoltList {
    let mut list = Vec::new();

    for cell in cells {
        let (i,j) = grid.ij(*cell);
        let pair = vec![Value::from(i as MoltInt), Value::from(j as MoltInt)];
        list.push(Value::from(pair));
    }

    list
}

fn pair((i,j): (usize,usize)) -> MoltList {
    vec![Value::from(i as MoltInt), Value::from(j as MoltInt)]
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

// Gets an option that should be -flat or -pairs.  Returns true for pairs.
fn get_coord_type(value: &Value) -> Result<Coord, Exception> {
    match value.as_str() {
        "-flat" => Ok(Coord::Flat),
        "-pairs" => Ok(Coord::Pair),
        _ => molt_err!("invalid option, expected one of: -flat, -pairs")
    }
}
