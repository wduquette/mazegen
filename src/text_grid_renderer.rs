use crate::grid::Grid;
use crate::Cell;
use std::collections::HashMap;
use std::fmt::Display;

/// A struct for rendering a grid, optionally with some data.  Uses the builder pattern.
#[derive(Debug, Default, Copy, Clone)]
pub struct TextGridRenderer {
    /// The minimum width of the rendered cell in monospace characters.
    cell_width: usize,

    /// Whether to compute the width automatically.
    auto_width: bool,

    /// The margin, when computing auto width.
    margin: usize,
    // TODO: Could add character style, but this will do for now.
}

impl TextGridRenderer {
    /// Creates a new renderer for the Grid with default settings
    pub fn new() -> Self {
        Self {
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
    pub fn render(&self, grid: &Grid) -> String {
        self.render_with(grid, |_| None as Option<usize>)
    }

    /// Render the grid using the current parameters, writing each data item into the
    /// corresponding cell.  `data` must be empty or have a length of `num_cells`.
    pub fn render_with<F, T>(&self, grid: &Grid, f: F) -> String
    where
        F: Fn(Cell) -> Option<T>,
        T: Display,
    {
        // FIRST, compute the labels and the max label width.
        let mut labwidth = 0;
        let mut labels = HashMap::new();

        for c in 0..grid.num_cells() {
            if let Some(val) = f(c) {
                let label = val.to_string();
                labwidth = std::cmp::max(labwidth, label.chars().count());
                labels.insert(c, label);
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
        for _ in 0..grid.num_cols() {
            self.write_south(&mut buff, false, cwidth);
        }

        // NEXT, write each row.
        for i in 0..grid.num_rows() {
            buff.push_str("\n|");

            // FIRST, write the cell row
            for j in 0..grid.num_cols() {
                let cell = grid.cell(i, j);

                if let Some(label) = labels.get(&cell) {
                    self.write_cell(&mut buff, &label, cwidth);
                } else {
                    self.write_cell(&mut buff, &"", cwidth);
                }

                if grid.is_linked_east(cell) {
                    buff.push(' ');
                } else {
                    buff.push('|');
                }
            }

            // NEXT, write the row of borders below
            buff.push_str("\n+");

            for j in 0..grid.num_cols() {
                let cell = grid.cell(i, j);

                self.write_south(&mut buff, grid.is_linked_south(cell), cwidth);
            }
        }

        buff.push('\n');

        // FINALLY, return the buff
        buff
    }

    fn write_cell<T: Display>(&self, buff: &mut String, value: &T, width: usize) {
        // FIRST, format the data on a field with the given width.
        buff.push_str(&format!("{datum:^width$}", datum = value, width = width));
    }

    fn write_south(&self, buff: &mut String, open: bool, width: usize) {
        for _ in 0..width {
            buff.push(if open { ' ' } else { '-' });
        }
        buff.push('+');
    }
}
