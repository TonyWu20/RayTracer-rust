use std::{
    error::Error,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::Scalar;

use super::colors::Color;

pub mod ppm_canvas;

fn dimension<const W: usize, const H: usize>() -> usize {
    W * H
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A canvas defined with `W` (width) and `H` (height).
pub struct Canvas<const W: usize, const H: usize, T: Scalar, F: CanvasFormat> {
    pixels: Vec<Color<T>>,
    _format: PhantomData<F>,
}
#[derive(Debug)]
pub struct Plain;

pub trait CanvasFormat: Debug {}
impl CanvasFormat for Plain {}

pub type RawCanvas<const W: usize, const H: usize, T> = Canvas<W, H, T, Plain>;

impl<const W: usize, const H: usize, T: Scalar, F: CanvasFormat> Default for Canvas<W, H, T, F> {
    fn default() -> Self {
        let mut pixels: Vec<Color<T>> = Vec::with_capacity(W * H);
        for _ in 0..W * H {
            pixels.push(Color::default())
        }
        Self {
            pixels,
            _format: PhantomData,
        }
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

impl<const W: usize, const H: usize, T: Scalar, F: CanvasFormat> Canvas<W, H, T, F> {
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
    pub fn pixel_at(&self, x: usize, y: usize) -> Option<&Color<T>> {
        if y < H && x < W {
            Some(self.pixels.get(y * W + x).unwrap())
        } else {
            None
        }
    }
    fn mut_pixel_at(&mut self, x: usize, y: usize) -> Option<&mut Color<T>> {
        if y < H && x < W {
            Some(self.pixels.get_mut(y * W + x).unwrap())
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
        color: Color<T>,
    ) -> Result<(), CanvasIndexError> {
        let pixel: &mut Color<T> = self
            .mut_pixel_at(x, y)
            .ok_or_else(|| CanvasIndexError::new(x, y, W, H))?;
        *pixel = color;
        Ok(())
    }

    pub fn pixels(&self) -> &[Color<T>] {
        &self.pixels
    }
}
