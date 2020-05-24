use crate::grid::Grid;
use crate::Cell;
use image::ImageBuffer;
use image::RgbImage;

/// A struct for rendering a grid as an Image, optionally colored with some data.  Uses the
/// builder pattern.
#[derive(Debug, Default, Copy, Clone)]
pub struct ImageGridRenderer {
    /// The width of the rendered cell in pixels, not including the borders.
    cell_width: usize,

    /// The height of the rendered cell in pixels.
    cell_height: usize,

    /// The border width, in pixels.
    border_width: usize,
}

impl ImageGridRenderer {
    /// Creates a new renderer for the Grid with default settings
    pub fn new() -> Self {
        Self {
            cell_width: 10,
            cell_height: 10,
            border_width: 1,
        }
    }

    /// Adds the desired cell_width.
    pub fn cell_width(&mut self, cell_width: usize) -> &mut Self {
        assert!(cell_width > 0);
        self.cell_width = cell_width;
        self
    }

    /// Adds the desired cell_width.
    pub fn cell_height(&mut self, cell_height: usize) -> &mut Self {
        assert!(cell_height > 0);
        self.cell_height = cell_height;
        self
    }

    /// Adds the desired cell_width and height.
    pub fn cell_size(&mut self, cell_size: usize) -> &mut Self {
        assert!(cell_size > 0);
        self.cell_width = cell_size;
        self.cell_height = cell_size;
        self
    }

    /// Adds the desired cell_width and height.
    pub fn border_width(&mut self, border_width: usize) -> &mut Self {
        assert!(border_width > 0);
        self.border_width = border_width;
        self
    }

    fn iy(&self, i: usize) -> u32 {
        (self.border_width + i * (self.cell_height + self.border_width)) as u32
    }

    fn jx(&self, j: usize) -> u32 {
        (self.border_width + j * (self.cell_width + self.border_width)) as u32
    }

    /// Render the grid using the current parameters.
    pub fn render(&self, grid: &Grid) -> RgbImage {
        self.render_with(grid, |_| None)
    }

    /// Render the grid using the current parameters.  Fill the cells by scaling the data in
    /// the data set from min to max.
    #[allow(clippy::cognitive_complexity)]
    pub fn render_with<F>(&self, grid: &Grid, f: F) -> RgbImage
    where
        F: Fn(Cell) -> Option<i64>,
    {
        // FIRST, size and create the image
        let nr = grid.num_rows() as u32;
        let nc = grid.num_cols() as u32;
        let bw = self.border_width as u32;
        let cellw = self.cell_width as u32;
        let cellh = self.cell_height as u32;
        let bcellw = (self.border_width + self.cell_width) as u32;
        let bcellh = (self.border_width + self.cell_height) as u32;
        let width = bw * (nc + 1) + cellw * nc;
        let height = bw * (nr + 1) + cellh * nr;

        let mut image: RgbImage = ImageBuffer::new(width, height);
        let black = image::Rgb([0, 0, 0]);
        let white = image::Rgb([255, 255, 255]);

        // NEXT, are we rendering data?
        let mut data_min = std::i64::MAX;
        let mut data_max = std::i64::MIN;
        let mut range: f64 = 0.0;

        for c in 0..grid.num_cells() {
            if let Some(val) = f(c) {
                data_min = std::cmp::min(val, data_min);
                data_max = std::cmp::max(val, data_min);
            }
        }

        if data_min < data_max {
            // We have a range of data; we can plot colors.
            range = (data_max - data_min) as f64;
        }

        // NEXT, clear the image to white.
        for y in 0..height {
            for x in 0..width {
                image.put_pixel(x, y, white);
            }
        }

        // NEXT, draw the top and left lines, and the intersection points
        for x in 0..width {
            for y in 0..bw {
                image.put_pixel(x, y, black);
            }
        }
        for y in 0..height {
            for x in 0..bw {
                image.put_pixel(x, y, black);
            }
        }
        for y in (bcellh..height).step_by(bcellh as usize) {
            for x in (bcellw..width).step_by(bcellw as usize) {
                for x1 in x..(x + bw) {
                    for y1 in y..(y + bw) {
                        image.put_pixel(x1, y1, black);
                    }
                }
            }
        }

        // NEXT, draw the east and south borders for each cell, and fill each cell with data
        // (if we have data).
        for i in 0..grid.num_rows() {
            let y = self.iy(i);
            for j in 0..grid.num_cols() {
                let cell = grid.cell(i, j);
                let x = self.jx(j);

                // Fill the cell with the data color.
                let mut floor = white;

                if let Some(value) = f(cell) {
                    let val = 255.0 * (value as f64) / range;

                    let scaled: u8;

                    if val < 0.0 {
                        scaled = 0;
                    } else if val > 255.0 {
                        scaled = 255;
                    } else {
                        scaled = val as u8;
                    }

                    floor = image::Rgb([255 - scaled, 255 - scaled, 255]);

                    for y1 in y..(y + cellh) {
                        for x1 in x..(x + cellw) {
                            image.put_pixel(x1, y1, floor);
                        }
                    }
                }

                // Draw east border
                let pixel = if grid.is_linked_east(cell) {
                    floor
                } else {
                    black
                };

                for y1 in y..(y + cellh) {
                    for x1 in (x + cellw)..(x + bcellw) {
                        image.put_pixel(x1, y1, pixel);
                    }
                }

                // Draw south border
                let pixel = if grid.is_linked_south(cell) {
                    floor
                } else {
                    black
                };

                for x1 in x..(x + cellw) {
                    for y1 in (y + cellh)..(y + bcellh) {
                        image.put_pixel(x1, y1, pixel);
                    }
                }
            }
        }

        image
    }
}
