use crate::grid::Grid;
use image::ImageBuffer;
use image::RgbImage;

/// A struct for rendering a grid as an Image, optionally colored with some data.  Uses the
/// builder pattern.
pub struct ImageGridRenderer<'a> {
    /// The grid to render
    grid: &'a Grid,

    /// The width of the rendered cell in pixels, not including the borders.
    cell_width: usize,

    /// The height of the rendered cell in pixels.
    cell_height: usize,

    /// The border width, in pixels.
    border_width: usize,
}

impl<'a> ImageGridRenderer<'a> {
    /// Creates a new renderer for the Grid with default settings
    pub fn new(grid: &'a Grid) -> Self {
        Self {
            grid,
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
    pub fn render(&self) -> RgbImage {
        // FIRST, size and create the image
        let nr = self.grid.num_rows() as u32;
        let nc = self.grid.num_cols() as u32;
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
                for x1 in x..(x+bw) {
                    for y1 in y..(y+bw) {
                        image.put_pixel(x1, y1, black);
                    }
                }
            }
        }

        // NEXT, draw the east and south borders for each cell.
        for i in 0..self.grid.num_rows() {
            let y = self.iy(i);
            for j in 0..self.grid.num_cols() {
                let cell = self.grid.cell(i, j);
                let x = self.jx(j);

                // Draw east border
                if !self.grid.is_linked_east(cell) {
                    for y1 in y..(y + cellh) {
                        for x1 in (x + cellw)..(x + bcellw) {
                            image.put_pixel(x1, y1, black);
                        }
                    }
                }

                // Draw south border
                if !self.grid.is_linked_south(cell) {
                    for x1 in x..(x + cellw) {
                        for y1 in (y + cellh)..(y + bcellh) {
                            image.put_pixel(x1, y1, black);
                        }
                    }
                }
            }
        }

        image
    }

}
