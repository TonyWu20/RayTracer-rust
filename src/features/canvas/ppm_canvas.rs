use std::{fmt::Display, marker::PhantomData};

use crate::features::colors::Color;

use super::{Canvas, CanvasFormat};

#[derive(Debug, Clone, Copy)]
/// Unit struct to represent `PPM` format
pub struct PPM;

/// Type alias `PPMCanvas<W,H>` as `Canvas<W,H,u8, PPM>`
pub type PPMCanvas<const W: usize, const H: usize> = Canvas<W, H, u8, PPM>;
/// Type alias `PPMColor` as `Color<u8>`
pub type PPMColor = Color<u8>;

impl CanvasFormat for PPM {}

impl Display for PPMColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

impl<const W: usize, const H: usize, U: CanvasFormat> From<Canvas<W, H, f64, U>>
    for PPMCanvas<W, H>
{
    fn from(src: Canvas<W, H, f64, U>) -> Self {
        let ppm_pixels: Vec<PPMColor> = src
            .pixels()
            .iter()
            .map(|&pixel| -> PPMColor { pixel.into() })
            .collect();
        Self {
            pixels: ppm_pixels,
            _format: PhantomData,
        }
    }
}

impl<const W: usize, const H: usize> Display for PPMCanvas<W, H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let header = format!("P3\n{} {}\n255\n", W, H);
        let mut line_length = 0;
        let pixels: Vec<String> = self
            .pixels()
            .iter()
            .map(|pixel| -> String {
                let pixel_output = format!("{}", pixel);
                // The expected `line_length` after appended a formatted pixel.
                let expect_length = line_length + pixel_output.len();
                // Avoid the line length exceeds 70 characters.
                // Cases:
                // 1. The expected length does not exceed 70, but already reach 63
                // Because the largest string length for a pixel is "255 255 255" which takes 11 character,
                // and the `\n` counts for 1 character, we should break the line if the current expected `line_length`
                // has exceeded 63.
                if (63..70).contains(&expect_length) {
                    // Start next line, `line_length` reset to 0;
                    line_length = 0;
                    format!("{}\n", pixel)
                }
                // 2. When the pixel string is appended, the line length limit is reached.
                // Break the line before the string, and set the `line_length` to the current
                // length of the string.
                else if expect_length >= 70 {
                    // The `line_length` reset to the current string length plus a space as the new line.
                    line_length = pixel_output.len() + 1;
                    format!("\n{} ", pixel)
                }
                // 3. The line will not be saturated with the appended string. Add `line_length`
                // counter by `(pixel_output.len() + 1)`
                else {
                    line_length += pixel_output.len() + 1;
                    format!("{} ", pixel)
                }
            })
            .collect();
        let pixels_string = pixels.concat();
        writeln!(f, "{}{}", header, pixels_string)
    }
}
