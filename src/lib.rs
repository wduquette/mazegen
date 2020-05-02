use std::fmt::Display;
use std::collections::HashSet;

pub fn hello() {
    println!("Hello, world!");
}

#[derive(Debug,PartialEq,Eq,Clone)]
pub struct Grid {
    num_rows: usize,
    num_cols: usize,
    num_cells: usize,
    cells: Vec<CellData>,
}

// A Cell is an index into the cells vector
pub type Cell = usize;

/// TODO: Consider making this generic.  The link information is wholly contained in
/// the Grid structure; but there's a generic data record for each cell so that the
/// client can retain data related to the cell.
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
                Some(dbg!(grid.cell(i - 1,j)))
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

    // Links cell 1 to cell 2.
    // The presumption is that the cells are adjacent.
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
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Grid({}x{}={})", self.num_rows, self.num_cols, self.num_cells)
        // TODO
    }
}

#[derive(Debug,PartialEq,Eq,Clone)]
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
        let grid = Grid::new(5,6);

        assert_eq!(grid.cell(0,0), 0);
        assert_eq!(grid.cell(0,3), 3);
        assert_eq!(grid.cell(1,0), 6);
        assert_eq!(grid.cell(1,3), 9);
        assert_eq!(grid.cell(2,0), 12);
        assert_eq!(grid.cell(4,5), grid.num_cells() - 1);
    }

    #[test]
    fn test_grid_i_j() {
        let grid = Grid::new(5,6);

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
        let grid = Grid::new(5,6);

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
        let grid = Grid::new(5,6);

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
        let mut grid = Grid::new(5,6);

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
}
