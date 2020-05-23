//! This module defines Grid, a rectilinear Grid for building mazes with.

use crate::Cell;
use image::RgbImage;
use crate::pixel::ImageGridRenderer;
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
        ImageGridRenderer::new(self)
            .cell_size(10)
            .border_width(2)
            .render()
    }
}

// Output the maze dimensions and the maze itself using simply ASCII graphics.
impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Grid({}x{})", self.num_rows, self.num_cols)?;
        writeln!(f, "{}", TextGridRenderer::new(self).render())
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
pub struct TextGridRenderer<'a> {
    /// The grid to render
    grid: &'a Grid,

    /// The minimum width of the rendered cell in monospace characters.
    cell_width: usize,

    /// Whether to compute the width automatically.
    auto_width: bool,

    /// The margin, when computing auto width.
    margin: usize,

    // TODO: Could add character style, but this will do for now.
}

impl<'a> TextGridRenderer<'a> {
    /// Creates a new renderer for the Grid with default settings
    pub fn new(grid: &'a Grid) -> Self {
        Self {
            grid,
            cell_width: 3,
            auto_width: false,
            margin: 0,
        }
    }

    /// Adds the desired cell_width.
    pub fn cell_width(&mut self, cell_width: usize) -> &mut Self {
        self.cell_width = cell_width;
        self
    }

    /// Compute the width required to display each data value at its preferred width, plus
    /// a margin on each side, and size the output accordingly.  The computed width will
    /// not be less than the current cell_width.
    pub fn auto_width(&mut self, margin: usize) -> &mut Self {
        self.auto_width = true;
        self.margin = margin;
        self
    }

    /// Render the grid using the current parameters.
    pub fn render(&self) -> String {
        let data: &[usize] = &[];
        self.render_data(data)
    }

    /// Render the grid using the current parameters, writing each data item into the
    /// corresponding cell.  `data` must be empty or have a length of `num_cells`.
    pub fn render_data<T: Display>(&self, data: &[T]) -> String {
        assert!(data.is_empty() || data.len() == self.grid.num_cells());

        // FIRST, if compute the width automatically, if requested.
        let mut cwidth = self.cell_width;

        if self.auto_width && !data.is_empty() {
            let mut width = 0;

            // FIRST, get the max width in the data
            for val in data {
                width = std::cmp::max(width, val.to_string().len());
            }

            // NEXT, add the margin
            width += 2 * self.margin;

            // NEXT, don't use a width less than the established cell width.
            if width > cwidth {
                cwidth = width;
            }
        }

        // NEXT, create the String to hold the output.
        let mut buff = String::new();

        // NEXT, write the top border.
        buff.push('+');
        for _ in 0..self.grid.num_cols() {
            self.write_south(&mut buff, false, cwidth);
        }

        // NEXT, write each row.
        for i in 0..self.grid.num_rows() {
            buff.push_str("\n|");

            // FIRST, write the cell row
            for j in 0..self.grid.num_cols() {
                let cell = self.grid.cell(i, j);

                if !data.is_empty() {
                    self.write_cell(&mut buff, &data[cell], cwidth);
                } else {
                    self.write_cell(&mut buff, &"", cwidth);
                }

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

                self.write_south(&mut buff, self.grid.is_linked_south(cell), cwidth);
            }
        }

        buff.push('\n');

        // FINALLY, return the buff
        buff
    }

    /// Render the grid using the current parameters, writing each data item into the
    /// corresponding cell.  `data` must be empty or have a length of `num_cells`.
    pub fn render_with<F,T>(&self, f: F) -> String
        where F: Fn(Cell) -> Option<T>, T: Display
    {
        // FIRST, compute the labels and the max label width.
        let mut labwidth = 0;
        let mut labels = Vec::with_capacity(self.grid.num_cells());

        for c in 0..self.grid.num_cells() {
            if let Some(val) = f(c) {
                let label = val.to_string();
                labwidth = std::cmp::max(labwidth, label.chars().count());
                labels.push(Some(label));
            } else {
                labels.push(None);
            }
        }

        // NEXT, compute the desired cell width.
        let mut cwidth = self.cell_width;

        if self.auto_width {
            cwidth = std::cmp::max(cwidth, labwidth + 2 * self.margin);
        }

        // NEXT, create the String to hold the output.
        let mut buff = String::new();

        // NEXT, write the top border.
        buff.push('+');
        for _ in 0..self.grid.num_cols() {
            self.write_south(&mut buff, false, cwidth);
        }

        // NEXT, write each row.
        for i in 0..self.grid.num_rows() {
            buff.push_str("\n|");

            // FIRST, write the cell row
            for j in 0..self.grid.num_cols() {
                let cell = self.grid.cell(i, j);

                if let Some(label) = &labels[cell] {
                    self.write_cell(&mut buff, &label, cwidth);
                } else {
                    self.write_cell(&mut buff, &"", cwidth);
                }

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

                self.write_south(&mut buff, self.grid.is_linked_south(cell), cwidth);
            }
        }

        buff.push('\n');

        // FINALLY, return the buff
        buff
    }

    fn write_cell<T: Display>(&self, buff: &mut String, value: &T, width: usize) {
        // FIRST, format the data on a field with the given width.
        buff.push_str(&format!(
            "{datum:^width$}",
            datum = value,
            width = width
        ));
    }

    fn write_south(&self, buff: &mut String, open: bool, width: usize) {
        for _ in 0..width {
            buff.push(if open { ' ' } else { '-' });
        }
        buff.push('+');
    }
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
