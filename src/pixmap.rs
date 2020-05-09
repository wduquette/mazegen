use crate::grid::Grid;
use image::ImageBuffer;
use image::RgbImage;

/// A struct for rendering a grid as an Image, optionally colored with some data.  Uses the
/// builder pattern.
pub struct ImageRenderer<'a> {
    /// The grid to render
    grid: &'a Grid,

    /// The width of the rendered cell in pixels, not including the borders.
    cell_width: usize,

    /// The height of the rendered cell in pixels.
    cell_height: usize,

    /// The border width, in pixels.
    border_width: usize,
}

impl<'a> ImageRenderer<'a> {
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
    pub fn cell_width(mut self, cell_width: usize) -> Self {
        assert!(cell_width > 0);
        self.cell_width = cell_width;
        self
    }

    /// Adds the desired cell_width.
    pub fn cell_height(mut self, cell_height: usize) -> Self {
        assert!(cell_height > 0);
        self.cell_height = cell_height;
        self
    }

    /// Adds the desired cell_width and height.
    pub fn cell_size(mut self, cell_size: usize) -> Self {
        assert!(cell_size > 0);
        self.cell_width = cell_size;
        self.cell_height = cell_size;
        self
    }

    /// Adds the desired cell_width and height.
    pub fn border_width(mut self, border_width: usize) -> Self {
        assert!(border_width > 0);
        self.border_width = border_width;
        self
    }

    /// Render the grid using the current parameters.
    pub fn render(self) -> RgbImage {
        // FIRST, size and create the image
        let size: u32 = 10;
        let width = 1 + size * self.grid.num_cols() as u32;
        let height = 1 + size * self.grid.num_rows() as u32;

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
        for i in 0..self.grid.num_rows() {
            let y = size * i as u32;
            for j in 0..self.grid.num_cols() {
                let cell = self.grid.cell(i, j);
                let x = size * j as u32;

                // Draw east border
                if !self.grid.is_linked_east(cell) {
                    for n in y..(y + size) {
                        image.put_pixel(x + size, n, black);
                    }
                }

                // Draw south border
                if !self.grid.is_linked_south(cell) {
                    for n in x..(x + size) {
                        image.put_pixel(n, y + size, black);
                    }
                }
            }
        }

        image
    }
}
