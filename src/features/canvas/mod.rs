use std::{error::Error, fmt::Display};

use approx::AbsDiffEq;

use super::colors::Color;

pub mod ppm_canvas;

#[derive(Debug, Clone, PartialEq)]
/// A canvas defined with `W` (width) and `H` (height).
pub struct Canvas<const W: usize, const H: usize> {
    pixels: [[Color<f64>; W]; H],
}

impl<const W: usize, const H: usize> Default for Canvas<W, H> {
    fn default() -> Self {
        let pixels = [[Color::<f64>::default(); W]; H];
        Self { pixels }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanvasIndexError {
    curr_x: usize,
    curr_y: usize,
    canvas_width: usize,
    canvas_height: usize,
}

impl CanvasIndexError {
    pub fn new(curr_x: usize, curr_y: usize, canvas_width: usize, canvas_height: usize) -> Self {
        Self {
            curr_x,
            curr_y,
            canvas_width,
            canvas_height,
        }
    }
}

impl Display for CanvasIndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid index at {}, {}; The canvas size is {} x {}.",
            self.curr_x, self.curr_y, self.canvas_width, self.canvas_height
        )
    }
}

impl Error for CanvasIndexError {}

impl<const W: usize, const H: usize> Canvas<W, H> {
    /// Returns the width of this [`Canvas`].
    pub fn width(&self) -> usize {
        W
    }

    /// Returns the height of this [`Canvas`].
    pub fn height(&self) -> usize {
        H
    }

    /// Returns a reference to the pixels of this [`Canvas`].

    /// Returns a pixel of the canvas at `(x,y)`.
    pub fn pixel_at(&self, x: usize, y: usize) -> Option<Color<f64>> {
        if y < H && x < W {
            Some(self.pixels[y][x])
        } else {
            None
        }
    }

    /// Writes a pixel to the canvas.
    /// # Arguments
    /// - x: usize
    /// - y: usize
    /// - color: Color<f64>
    /// # Errors
    ///
    /// This function will return the `CanvasIndexError` if the given (x, y) is out of bounds.
    pub fn write_pixel(
        &mut self,
        x: usize,
        y: usize,
        color: Color<f64>,
    ) -> Result<(), CanvasIndexError> {
        let pixel = self
            .pixels
            .get_mut(y)
            .ok_or_else(|| CanvasIndexError::new(x, y, W, H))?
            .get_mut(x)
            .ok_or_else(|| CanvasIndexError::new(x, y, W, H))?;
        *pixel = color;
        Ok(())
    }

    pub fn pixels(&self) -> [[Color<f64>; W]; H] {
        self.pixels
    }
}
