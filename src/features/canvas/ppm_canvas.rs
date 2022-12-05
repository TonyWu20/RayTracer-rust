use std::fmt::Display;

use crate::features::colors::Color;

use super::Canvas;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
/// Newtype to define a `Color` only for `PPMCanvas`
pub struct PPMColor(pub(crate) Color<u8>);

impl From<Color<u8>> for PPMColor {
    fn from(src: Color<u8>) -> Self {
        Self(src)
    }
}

impl Display for PPMColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0.r, self.0.g, self.0.b)
    }
}

pub struct PPMCanvas<const W: usize, const H: usize> {
    pixels: [[PPMColor; W]; H],
}

impl<const W: usize, const H: usize> PPMCanvas<W, H> {
    pub fn pixels(&self) -> [[PPMColor; W]; H] {
        self.pixels
    }
}

impl<const W: usize, const H: usize> From<Canvas<W, H>> for PPMCanvas<W, H> {
    fn from(src: Canvas<W, H>) -> Self {
        let ppm_pixels: Vec<[PPMColor; W]> = src
            .pixels()
            .iter()
            .map(|row| {
                let row = row
                    .iter()
                    .map(|&pixel| {
                        let u8_pixel = pixel.into();
                        // Wrap into `PPMColor`
                        PPMColor(u8_pixel)
                    })
                    .collect::<Vec<PPMColor>>();
                let row_array: [PPMColor; W] = row.try_into().unwrap();
                row_array
            })
            .collect();
        let ppm_pixels: [[PPMColor; W]; H] = ppm_pixels.try_into().unwrap();
        Self { pixels: ppm_pixels }
    }
}

impl<const W: usize, const H: usize> Display for PPMCanvas<W, H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let header = format!("P3\n{} {}\n255\n", W, H);
        let mut line_length = 0;
        let pixels: Vec<String> = self
            .pixels()
            .iter()
            .flat_map(|row| -> Vec<String> {
                row.iter()
                    .map(|pixel| -> String {
                        let pixel_output = format!("{}", pixel);
                        // The expected `line_length` after appended a formatted pixel.
                        let expect_length = line_length + pixel_output.len();
                        // Avoid the line length exceeds 70 characters.
                        let final_output = 
                        // Cases:
                        // 1. The expected length does not exceed 70, but already reach 63
                        // Because the largest string length for a pixel is "255 255 255" which takes 11 character,
                        // and the `\n` counts for 1 character, we should break the line if the current expected `line_length`
                        // has exceeded 63.
                        if expect_length < 70 && expect_length >= 63 {
                            // Start next line, `line_length` reset to 0;
                            line_length = 0;
                            format!("{}\n", pixel)
                        }
                        // 2. When the pixel string is appended, the line length limit is reached.
                        // Break the line before the string, and set the `line_length` to the current
                        // length of the string.
                        else if expect_length >= 70 
                        {
                            // The `line_length` reset to the current string length plus a space as the new line.
                            line_length = pixel_output.len() + 1;
                            format!("\n{} ", pixel)
                        }
                        // 3. The line will not be saturated with the appended string. Add `line_length`
                        // counter by `(pixel_output.len() + 1)`
                        else {
                            line_length += pixel_output.len() + 1;
                            format!("{} ", pixel)
                        };
                        final_output
                    })
                    .collect()
            })
            .collect();
        let pixels_string = pixels.concat();
        write!(f, "{}{}\n", header, pixels_string)
    }
}

#[cfg(test)]
mod test {
}
