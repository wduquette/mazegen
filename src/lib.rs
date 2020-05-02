use std::fmt::Display;
use std::collections::HashSet;

pub fn hello() {
    println!("Hello, world!");
}

#[derive(Debug,PartialEq,Eq,Clone)]
pub struct Grid {
    num_rows: usize,
    num_cols: usize,
    cells: Vec<Cell>,
}

/// TODO: Consider making this generic.  The link information is wholly contained in
/// the Grid structure; but there's a generic data record for each cell so that the
/// client can retain data related to the cell.
impl Grid {
    pub fn new(num_rows: usize, num_cols: usize) -> Self {
        // FIRST, initialize the cells vector
        let mut cells = Vec::with_capacity(num_rows * num_cols);

        for i in 0..num_rows {
            for j in 0..num_cols {
                cells.push(Cell::new(i, j));
            }
        }

        // NEXT, return the grid
        Self {
            num_rows,
            num_cols,
            cells,
        }
    }

    pub fn num_rows(&self) -> usize {
        self.num_rows
    }

    pub fn num_cols(&self) -> usize {
        self.num_cols
    }

    // Links location 1 to location 2
    pub fn link(&mut self, loc1: Loc, loc2: Loc) {
        let index1 = self.index(loc1);
        let index2 = self.index(loc2);

        self.cells[index1].link(loc2);
        self.cells[index2].link(loc1);
    }

    // Unlinks location 1 from location 2
    pub fn unlink(&mut self, loc1: Loc, loc2: Loc) {
        let index1 = self.index(loc1);
        let index2 = self.index(loc2);

        self.cells[index1].unlink(loc2);
        self.cells[index2].unlink(loc1);
    }

    // Gets a reference to the cell at the given location
    pub fn cell(&self, loc: Loc) -> Option<&Cell> {
        if  self.contains(loc) {
            Some(&self.cells[self.index(loc)])
        } else {
            None
        }
    }

    /// Does the grid contain the location?
    pub fn contains(&self, loc: Loc) -> bool {
        // NOTE: No need to check against zero, since we're using an unsigned type.
        loc.row() < self.num_rows && loc.col() < self.num_cols
    }

    /// Returns the index of the given location in the cells vector.
    fn index(&self, loc: Loc) -> usize {
        loc.row()*self.num_rows + loc.col()
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Grid({}x{})", self.num_rows, self.num_cols)
    }
}

/// New type: the coordinates of a maze cell.
/// TODO: could probably use a smaller integer type.
/// TODO: could possibly use a signed type, so that mazes can grow in all directions.
/// If I do that, though, I'll need a different data structure in Grid.
#[derive(Debug,PartialEq,Eq,Copy,Clone,Hash)]
pub struct Loc {
    row: usize,
    col: usize,
}

impl Loc {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }

    /// Returns the location to the north
    pub fn north(&self) -> Loc {
        // TODO: Problem: if in top row, there is no valid location to the north because
        // the row is negative and the variable is unsigned
        Loc::new(self.row - 1, self.col)
    }

    /// Returns the location to the south
    pub fn south(&self) -> Loc {
        Loc::new(self.row + 1, self.col)
    }

    /// Returns the location to the east
    pub fn east(&self) -> Loc {
        Loc::new(self.row, self.col + 1)
    }

    /// Returns the location to the west
    pub fn west(&self) -> Loc {
        // TODO: Problem: if in leftmost column, there is no valid location to the west because
        // the column is negative and the variable is unsigned
        Loc::new(self.row, self.col - 1)
    }
}



#[derive(Debug,PartialEq,Eq,Clone)]
pub struct Cell {
    loc: Loc,
    links: HashSet<Loc>,
}

impl Cell {
    /// Creates a new Cell
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            loc: Loc::new(row, col),
            links: HashSet::new(),
        }
    }

    pub fn loc(&self) -> Loc {
        self.loc
    }

    /// Returns the cell's row index
    pub fn row(&self) -> usize {
        self.loc.row()
    }

    /// Returns the cell's column index
    pub fn col(&self) -> usize {
        self.loc.col()
    }

    /// Inserts a link to the given location
    fn link(&mut self, other: Loc) {
        self.links.insert(other);
    }

    /// Removes a link to the given location
    fn unlink(&mut self, other: Loc) {
        self.links.remove(&other);
    }

    /// Returns an interator over the links
    pub fn links(&self) -> std::collections::hash_set::Iter<Loc> {
        self.links.iter()
    }

    /// Indicates whether or not this cell is linked to another
    pub fn linked(&self, loc: Loc) -> bool {
        self.links.contains(&loc)
    }

}
