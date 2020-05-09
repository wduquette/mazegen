use image::ImageBuffer;
use image::RgbImage;
use rand::{thread_rng, Rng};
use std::collections::HashSet;
use std::fmt::Display;

/// A rectangular grid of cells, which can be used to represent a maze.
/// Each cell has its neighbors to the north, south, east, and west (as constrained by
/// the boundaries of the grid), and may be linked to any of its neighbors.  In graph
/// theory terms, each cell is a node; if two cells are linked there is a bidirectional
/// edge between them.
///
/// Each cell is identified by a unique integer cell ID, and also by an (i,j) row/column pair.
/// The cell ID and the (i,j) pair can easily be computed one from the other.
///
/// A Grid is created with a particular number of rows and columns.  Initially no cell is
/// linked to any other cell.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grid {
    num_rows: usize,
    num_cols: usize,
    num_cells: usize,
    cells: Vec<CellData>,
}

/// A Cell ID.
///
/// Internally, a Cell is just an index into the cells vector.  The Grid's
/// `cell` method returns the cell ID for a given (i,j) pair, and the `i`, `j`, and `ij`
/// methods do the opposite.
pub type Cell = usize;

impl Grid {
    pub fn new(num_rows: usize, num_cols: usize) -> Self {
        // FIRST, initialize the cells vector
        let num_cells = num_rows * num_cols;
        let cells = Vec::with_capacity(num_cells);

        let mut grid = Self {
            num_rows,
            num_cols,
            num_cells,
            cells,
        };

        for cell in 0..num_cells {
            let i = grid.i(cell);
            let j = grid.j(cell);

            let north = if i > 0 {
                Some(grid.cell(i - 1, j))
            } else {
                None
            };

            let south = if i < num_rows - 1 {
                Some(grid.cell(i + 1, j))
            } else {
                None
            };

            let east = if j < num_cols - 1 {
                Some(grid.cell(i, j + 1))
            } else {
                None
            };

            let west = if j > 0 {
                Some(grid.cell(i, j - 1))
            } else {
                None
            };

            grid.cells.push(CellData {
                cell,
                links: HashSet::new(),
                north,
                south,
                east,
                west,
            });
        }

        grid
    }

    /// The number of rows in the grid.
    pub fn num_rows(&self) -> usize {
        self.num_rows
    }

    /// The number of columns in the grid.
    pub fn num_cols(&self) -> usize {
        self.num_cols
    }

    /// The number of cells in the grid.
    pub fn num_cells(&self) -> usize {
        self.num_cells
    }

    /// Computes the cell from the row and column.
    pub fn cell(&self, i: usize, j: usize) -> Cell {
        assert!(i < self.num_rows && j < self.num_cols);
        i * self.num_cols + j
    }

    /// Computes the row index from the cell ID.
    pub fn i(&self, cell: Cell) -> usize {
        assert!(self.contains(cell));
        cell / self.num_cols
    }

    /// Computes the column index from the cell ID.
    pub fn j(&self, cell: Cell) -> usize {
        assert!(self.contains(cell));
        cell % self.num_cols
    }

    /// Computes the row and column indices from the cell ID.
    pub fn ij(&self, cell: Cell) -> (usize, usize) {
        assert!(self.contains(cell));
        (cell / self.num_cols, cell % self.num_cols)
    }

    // Links cell 1 to cell 2.
    // TODO: The linked cells should always be adjacent; but this implementation doesn't
    // require it.  Later in the book, the author talks about "braiding"; possibly,
    // braiding involves non-adjacent links.  If not, an assertion should be put in.
    pub fn link(&mut self, cell1: Cell, cell2: Cell) {
        assert!(self.contains(cell1));
        assert!(self.contains(cell2));

        self.cells[cell1].link(cell2);
        self.cells[cell2].link(cell1);
    }

    // Unlinks cell 1 from cell 2
    pub fn unlink(&mut self, cell1: Cell, cell2: Cell) {
        self.cells[cell1].unlink(cell2);
        self.cells[cell2].unlink(cell1);
    }

    // Gets the cells linked to this cell
    pub fn links(&self, cell: Cell) -> Vec<Cell> {
        assert!(self.contains(cell));
        self.cells[cell].links.iter().copied().collect()
    }

    // Indicates whether or not the cells are linked
    pub fn is_linked(&self, cell1: Cell, cell2: Cell) -> bool {
        assert!(self.contains(cell1));
        assert!(self.contains(cell2));

        self.cells[cell1].links.contains(&cell2)
    }

    // Gets the neighbors to the north, south, east, and west of this cell.
    pub fn neighbors(&self, cell: Cell) -> Vec<Cell> {
        assert!(self.contains(cell));
        self.cells[cell].neighbors()
    }

    /// Does the grid contain the location?
    pub fn contains(&self, cell: Cell) -> bool {
        // NOTE: No need to check against zero, since we're using an unsigned type.
        cell < self.num_cells
    }

    /// Gets the cell to the north, if any.
    pub fn north_of(&self, cell: Cell) -> Option<Cell> {
        assert!(self.contains(cell));
        self.cells[cell].north
    }

    /// Gets the cell to the south, if any.
    pub fn south_of(&self, cell: Cell) -> Option<Cell> {
        assert!(self.contains(cell));
        self.cells[cell].south
    }

    /// Gets the cell to the east, if any.
    pub fn east_of(&self, cell: Cell) -> Option<Cell> {
        assert!(self.contains(cell));
        self.cells[cell].east
    }

    /// Gets the cell to the west, if any.
    pub fn west_of(&self, cell: Cell) -> Option<Cell> {
        assert!(self.contains(cell));
        self.cells[cell].west
    }

    /// Indicates whether this cell is linked to the cell to its north.
    /// Returns false if there is no cell to the north.
    pub fn is_linked_north(&self, cell: Cell) -> bool {
        assert!(self.contains(cell));
        if let Some(other) = self.cells[cell].north {
            self.cells[cell].links.contains(&other)
        } else {
            false
        }
    }

    /// Indicates whether this cell is linked to the cell to its south.
    /// Returns false if there is no cell to the south.
    pub fn is_linked_south(&self, cell: Cell) -> bool {
        assert!(self.contains(cell));
        if let Some(other) = self.cells[cell].south {
            self.cells[cell].links.contains(&other)
        } else {
            false
        }
    }

    /// Indicates whether this cell is linked to the cell to its east.
    /// Returns false if there is no cell to the east.
    pub fn is_linked_east(&self, cell: Cell) -> bool {
        assert!(self.contains(cell));
        if let Some(other) = self.cells[cell].east {
            self.cells[cell].links.contains(&other)
        } else {
            false
        }
    }

    /// Indicates whether this cell is linked to the cell to its west.
    /// Returns false if there is no cell to the west.
    pub fn is_linked_west(&self, cell: Cell) -> bool {
        assert!(self.contains(cell));
        if let Some(other) = self.cells[cell].west {
            self.cells[cell].links.contains(&other)
        } else {
            false
        }
    }

    /// Returns the grid to its initial state:  no cell is linked to any other cell.
    pub fn clear(&mut self) {
        for c in 0..self.num_cells {
            self.cells[c].links.clear();
        }
    }

    /// Computes the shortest distance from the cell to each other cell.
    /// Returns the distances as a vector of length `num_cells`.
    pub fn distances(&self, cell: Cell) -> Vec<Option<usize>> {
        // FIRST, create a working vector.  Initially, no distances are computed.
        let mut dists = Vec::<Option<usize>>::with_capacity(self.num_cells());

        for _ in 0..self.num_cells() {
            dists.push(None);
        }

        // NEXT, use a (simplified) Dijkstra's algorithm to compute the distances.
        // See "Mazes for Programmers" Ch. 3.
        dists[cell] = Some(0);
        let mut frontier = HashSet::new();
        frontier.insert(cell);

        while !frontier.is_empty() {
            let mut new_frontier = HashSet::new();

            for c in frontier {
                for d in self.links(c) {
                    if dists[d].is_none() {
                        dists[d] = Some(dists[c].expect("valid distance") + 1);
                        new_frontier.insert(d);
                    }
                }
            }
            frontier = new_frontier;
        }

        // NEXT, return the distances.
        dists
    }

    /// Computes the shortest path from the first cell to the second, returning the path
    /// as a vector of cells.  If there is no path, the vector will be empty.
    pub fn shortest_path(&self, start: Cell, goal: Cell) -> Vec<Cell> {
        // FIRST, compute distances from the starting cell.
        let dists = self.distances(start);

        // NEXT, compute a path from the goal back to start.
        let mut path = Vec::new();

        let mut current = goal;
        path.push(current);

        while current != start {
            let old_len = path.len();

            // FIRST, get the next step in the path.
            let cdist = dists[current].expect("valid distance");
            for neighbor in self.links(current) {
                let ndist = dists[neighbor].expect("valid distance");

                if ndist < cdist {
                    path.push(neighbor);
                    current = neighbor;
                    break;
                }
            }

            // NEXT, if we didn't add a new step to the path then there is no path to start.
            if path.len() == old_len {
                path.clear();
                break;
            }
        }

        // FINALLY, return the computed path.
        path.reverse();
        path
    }

    pub fn to_image(&self) -> RgbImage {
        // FIRST, size and create the image
        let size: u32 = 10;
        let width = 1 + size * self.num_cols() as u32;
        let height = 1 + size * self.num_rows() as u32;

        let mut image: RgbImage = ImageBuffer::new(width, height);
        let black = image::Rgb([0, 0, 0]);
        let white = image::Rgb([255, 255, 255]);

        // NEXT, clear the image to white.
        for y in 0..height {
            for x in 0..width {
                // NOTE: set_pixel returns an error result if the coordinates are out of bounds.
                // That should probably be a panic instead, since there's no excuse for it.
                // NOTE: set_pixel takes a Color, not &Color; and Color isn't Copy.
                // Consequently you need to create a new Color for each pixel.  Derpy.
                image.put_pixel(x, y, white);
            }
        }

        // NEXT, draw the top and left lines, and the intersection points
        for x in 0..width {
            image.put_pixel(x, 0, black);
        }
        for y in 0..height {
            image.put_pixel(0, y, black);
        }
        for y in (size..height).step_by(size as usize) {
            for x in (size..width).step_by(size as usize) {
                image.put_pixel(x, y, black);
            }
        }

        // NEXT, draw the east and south borders for each cell.
        for i in 0..self.num_rows() {
            let y = size * i as u32;
            for j in 0..self.num_cols() {
                let cell = self.cell(i, j);
                let x = size * j as u32;

                // Draw east border
                if !self.is_linked_east(cell) {
                    for n in y..(y + size) {
                        image.put_pixel(x + size, n, black);
                    }
                }

                // Draw south border
                if !self.is_linked_south(cell) {
                    for n in x..(x + size) {
                        image.put_pixel(n, y + size, black);
                    }
                }
            }
        }

        image
    }
}

// Output the maze dimensions and the maze itself using simply ASCII graphics.
impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Grid({}x{})", self.num_rows, self.num_cols)?;
        writeln!(f, "{}", GridTextRenderer::<usize>::new(self).render())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct CellData {
    cell: Cell,
    links: HashSet<Cell>,
    north: Option<Cell>,
    south: Option<Cell>,
    east: Option<Cell>,
    west: Option<Cell>,
}

impl CellData {
    /// Inserts a link to the given
    fn link(&mut self, other: Cell) {
        self.links.insert(other);
    }

    /// Removes a link to the given cell
    fn unlink(&mut self, other: Cell) {
        self.links.remove(&other);
    }

    fn neighbors(&self) -> Vec<Cell> {
        let mut vec = Vec::new();

        if let Some(cell) = self.north {
            vec.push(cell)
        }

        if let Some(cell) = self.south {
            vec.push(cell)
        }

        if let Some(cell) = self.east {
            vec.push(cell)
        }

        if let Some(cell) = self.west {
            vec.push(cell)
        }

        vec
    }
}

/// A struct for rendering a grid, optionally with some data.  Uses the builder pattern.
pub struct GridTextRenderer<'a, T>
where
    T: Display,
{
    /// The grid to render
    grid: &'a Grid,

    /// The  width of the rendered cell in monospace characters.  1+
    cell_width: usize,

    /// A vector of T data for use when rendering the grid.
    /// TODO: Would a closure be better?
    data: &'a [T],

    /// Whether to compute the width automatically.
    auto_width: bool,

    /// The margin, when computing auto width.
    margin: usize,
    // TODO: Could add character style, but this will do for now.
}

impl<'a, T> GridTextRenderer<'a, T>
where
    T: Display,
{
    /// Creates a new renderer for the Grid with default settings
    pub fn new(grid: &'a Grid) -> Self {
        Self {
            grid,
            cell_width: 3,
            data: &[],
            auto_width: false,
            margin: 0,
        }
    }

    /// Adds the desired cell_width.
    pub fn cell_width(mut self, cell_width: usize) -> Self {
        self.cell_width = cell_width;
        self
    }

    /// Adds a data vector to render in each cell.
    ///
    /// # Panics
    ///
    /// This method panics if the vector doesn't have the same number of items
    /// as the Grid has cells.
    pub fn data(mut self, data: &'a [T]) -> Self {
        assert!(data.is_empty() || data.len() == self.grid.num_cells());
        self.data = data;
        self
    }

    /// Compute the width required to display each data value at its preferred width, plus
    /// a margin on each side, and size the output accordingly.  The computed width will
    /// not be less than the current cell_width.
    pub fn auto_width(mut self, margin: usize) -> Self {
        self.auto_width = true;
        self.margin = margin;
        self
    }

    /// Render the grid using the current parameters.
    pub fn render(mut self) -> String {
        // FIRST, if compute the width automatically, if requested.
        if self.auto_width && !self.data.is_empty() {
            let mut width = 0;

            // FIRST, get the max width in the data
            for val in self.data {
                width = std::cmp::max(width, val.to_string().len());
            }

            // NEXT, add the margin
            width += 2 * self.margin;

            // NEXT, don't use a width less than the established cell width.
            if width > self.cell_width {
                self.cell_width = width;
            }
        }

        // NEXT, create the String to hold the output.
        let mut buff = String::new();

        // NEXT, write the top border.
        buff.push('+');
        for _ in 0..self.grid.num_cols() {
            self.write_south(&mut buff, false);
        }

        // NEXT, write each row.
        for i in 0..self.grid.num_rows() {
            buff.push_str("\n|");

            // FIRST, write the cell row
            for j in 0..self.grid.num_cols() {
                let cell = self.grid.cell(i, j);

                self.write_cell(&mut buff, cell);

                if self.grid.is_linked_east(cell) {
                    buff.push(' ');
                } else {
                    buff.push('|');
                }
            }

            // NEXT, write the row of borders below
            buff.push_str("\n+");

            for j in 0..self.grid.num_cols() {
                let cell = self.grid.cell(i, j);

                self.write_south(&mut buff, self.grid.is_linked_south(cell));
            }
        }

        buff.push('\n');

        // FINALLY, return the buff
        buff
    }

    fn write_cell(&self, buff: &mut String, cell: Cell) {
        // FIRST, if there's no data just output spaces.
        if self.data.is_empty() {
            for _ in 0..self.cell_width {
                buff.push(' ');
            }
            return;
        }

        // NEXT, format the data on a field with the given width.
        buff.push_str(&format!(
            "{datum:^width$}",
            datum = self.data[cell],
            width = self.cell_width
        ));
    }

    fn write_south(&self, buff: &mut String, open: bool) {
        for _ in 0..self.cell_width {
            buff.push(if open { ' ' } else { '-' });
        }
        buff.push('+');
    }
}

/// Algorithm to produce a Grid containing a binary-tree maze
pub fn binary_tree_maze(grid: &mut Grid) {
    for cell in 0..grid.num_cells() {
        let mut neighbors = Vec::new();

        if let Some(ncell) = grid.north_of(cell) {
            neighbors.push(ncell);
        }

        if let Some(ecell) = grid.east_of(cell) {
            neighbors.push(ecell);
        }

        if !neighbors.is_empty() {
            grid.link(cell, sample(&neighbors));
        }
    }
}

/// Algorithm to produce a Grid containing a sidewinder maze
pub fn sidewinder_maze(grid: &mut Grid) {
    for i in 0..grid.num_rows() {
        let mut run = Vec::new();

        for j in 0..grid.num_cols() {
            let cell = grid.cell(i, j);
            run.push(cell);

            let at_eastern_boundary = grid.east_of(cell).is_none();
            let at_northern_boundary = grid.north_of(cell).is_none();
            let should_close_out = at_eastern_boundary || (!at_northern_boundary && !flip());

            if should_close_out {
                let member = sample(&run);
                if let Some(ncell) = grid.north_of(member) {
                    grid.link(member, ncell);
                }
                run.clear();
            } else {
                grid.link(cell, grid.east_of(cell).expect("a cell"));
            }
        }
    }
}

pub fn sample(vec: &[Cell]) -> Cell {
    assert!(!vec.is_empty());

    if vec.len() == 1 {
        return vec[0];
    }

    let mut rng = thread_rng();
    let ind: usize = rng.gen_range(0, vec.len());
    vec[ind]
}

// Flips a coin, returning true/false
pub fn flip() -> bool {
    thread_rng().gen_bool(0.5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_new() {
        let grid = Grid::new(5, 6);

        assert_eq!(grid.num_rows(), 5);
        assert_eq!(grid.num_cols(), 6);
        assert_eq!(grid.num_cells(), 30);
    }

    #[test]
    fn test_grid_cell() {
        let grid = Grid::new(5, 6);

        assert_eq!(grid.cell(0, 0), 0);
        assert_eq!(grid.cell(0, 3), 3);
        assert_eq!(grid.cell(1, 0), 6);
        assert_eq!(grid.cell(1, 3), 9);
        assert_eq!(grid.cell(2, 0), 12);
        assert_eq!(grid.cell(4, 5), grid.num_cells() - 1);
    }

    #[test]
    fn test_grid_i_j() {
        let grid = Grid::new(5, 6);

        assert_eq!(grid.i(0), 0);
        assert_eq!(grid.j(0), 0);

        assert_eq!(grid.i(3), 0);
        assert_eq!(grid.j(3), 3);

        assert_eq!(grid.i(6), 1);
        assert_eq!(grid.j(6), 0);

        assert_eq!(grid.i(9), 1);
        assert_eq!(grid.j(9), 3);
    }

    #[test]
    fn test_grid_cell_i_j() {
        let grid = Grid::new(5, 6);

        for i in 0..grid.num_rows() {
            for j in 0..grid.num_cols() {
                let cell = grid.cell(i, j);
                assert!(grid.contains(cell));
                assert_eq!(grid.i(cell), i);
                assert_eq!(grid.j(cell), j);
            }
        }
    }

    #[test]
    fn test_grid_neighbors() {
        let grid = Grid::new(5, 6);

        for cell in 0..grid.num_cells() {
            let mut count = 0;
            let neighbors = grid.neighbors(cell);

            if let Some(other) = grid.north_of(cell) {
                count += 1;
                assert!(grid.contains(other));
                assert!(neighbors.contains(&other));
            }

            if let Some(other) = grid.south_of(cell) {
                count += 1;
                assert!(grid.contains(other));
                assert!(neighbors.contains(&other));
            }

            if let Some(other) = grid.east_of(cell) {
                count += 1;
                assert!(grid.contains(dbg!(other)));
                assert!(neighbors.contains(&other));
            }

            if let Some(other) = grid.west_of(cell) {
                count += 1;
                assert!(grid.contains(other));
                assert!(neighbors.contains(&other));
            }

            assert_eq!(neighbors.len(), count);
        }
    }

    #[test]
    fn test_grid_linking() {
        let mut grid = Grid::new(5, 6);

        // Initially, no cells are linked.
        for c1 in 0..grid.num_cells {
            for c2 in 0..grid.num_cells {
                assert!(!grid.is_linked(c1, c2));
            }
        }

        // Link each cell to its eastern neighbor (if any)
        for c1 in 0..grid.num_cells {
            if let Some(c2) = grid.east_of(c1) {
                grid.link(c1, c2);
                assert!(grid.is_linked(c1, c2));
                assert!(grid.is_linked(c2, c1));

                grid.unlink(c1, c2);
                assert!(!grid.is_linked(c1, c2));
                assert!(!grid.is_linked(c2, c1));
            }
        }
    }

    #[test]
    fn test_grid_is_linked_direction() {
        let mut grid = Grid::new(5, 6);

        // Add north, south, east, and west links.
        let cell = grid.cell(3, 3);
        grid.link(cell, grid.north_of(cell).unwrap());
        grid.link(cell, grid.south_of(cell).unwrap());
        grid.link(cell, grid.east_of(cell).unwrap());
        grid.link(cell, grid.west_of(cell).unwrap());

        for c in 0..grid.num_cells() {
            if let Some(other) = grid.north_of(c) {
                assert_eq!(grid.is_linked_north(c), grid.is_linked(c, other));
            }

            if let Some(other) = grid.south_of(c) {
                assert_eq!(grid.is_linked_south(c), grid.is_linked(c, other));
            }

            if let Some(other) = grid.east_of(c) {
                assert_eq!(grid.is_linked_east(c), grid.is_linked(c, other));
            }

            if let Some(other) = grid.west_of(c) {
                assert_eq!(grid.is_linked_west(c), grid.is_linked(c, other));
            }
        }
    }
}
