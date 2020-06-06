//! Implementation of RGBA Pixel type for use with Molt.
//!
//! The internal representation is an image::Rgba.  The TCL representation is
//! a string "#rrggbb?.aa?".  The alpha defaults to 255.

use image::Rgba;
use molt::types::*;
use std::fmt;
use std::str::FromStr;

/// A struct representing an `image::Rgba` pixel.  This is a simple wrapper around
/// the `image` type, but it allows us to define the Display and FromStr traits along
/// with a variety of convenience methods, and also to insulate the Molt code from the
/// underlying implementation.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct MoltPixel {
    /// The internal image pixel
    ipixel: Rgba<u8>,
}

impl MoltPixel {
    /// Create an RGB pixel with an alpha of 255
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            ipixel: image::Rgba([r, g, b, 255]),
        }
    }

    /// Create an RGB pixel with the given alpha
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            ipixel: image::Rgba([r, g, b, a]),
        }
    }

    /// Get the pixel's red component.
    pub fn red(&self) -> u8 {
        self.ipixel[0]
    }

    /// Get the pixel's green component.
    pub fn green(&self) -> u8 {
        self.ipixel[1]
    }

    /// Get the pixel's blue component.
    pub fn blue(&self) -> u8 {
        self.ipixel[2]
    }

    /// Get the pixel's alpha component.
    pub fn alpha(&self) -> u8 {
        self.ipixel[3]
    }

    /// Get the underlying image::Rgba pixel
    /// TODO: Should be "inner"?
    pub fn ipixel(&self) -> Rgba<u8> {
        self.ipixel
    }
}

impl fmt::Display for MoltPixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.alpha() == 255 {
            write!(
                f,
                "#{:02x}{:02x}{:02x}",
                self.red(),
                self.green(),
                self.blue()
            )
        } else {
            write!(
                f,
                "#{:02x}{:02x}{:02x}.{:02x}",
                self.red(),
                self.green(),
                self.blue(),
                self.alpha()
            )
        }
    }
}

impl FromStr for MoltPixel {
    type Err = String;

    fn from_str(pixel: &str) -> Result<Self, Self::Err> {
        // FIRST, check the length.
        let len = pixel.chars().count();

        if len != 7 && len != 10 {
            return Err("invalid pixel string".into());
        }

        // NEXT, check the leading character.
        let pixel = pixel.to_lowercase();
        let ch = pixel.chars().next().expect("first character");
        if ch != '#' {
            return Err("invalid pixel string".into());
        }

        // NEXT, get the RGB
        let r = parse_hex(&pixel[1..3])?;
        let g = parse_hex(&pixel[3..5])?;
        let b = parse_hex(&pixel[5..7])?;

        // NEXT, get the alpha, if any.
        let mut a = 255;

        if len == 10 {
            let mut chars = pixel.chars().skip(7);
            let ch = chars.next().expect("period");
            if ch != '.' {
                return Err("invalid pixel string".into());
            }
            a = parse_hex(&pixel[8..10])?;
        }

        Ok(MoltPixel::rgba(r, g, b, a))
    }
}

/// Parse a hex string, returning a String error on error.
fn parse_hex(hex: &str) -> Result<u8, String> {
    match u8::from_str_radix(hex, 16) {
        Err(_) => Err("invalid pixel string".into()),
        Ok(val) => Ok(val),
    }
}

impl MoltPixel {
    /// A convenience: retrieves the pixel value, converting it from
    /// `Option<MoltPixel>` into `Result<MoltPixel,Exception>`.
    pub fn from_molt(value: &Value) -> Result<Self, Exception> {
        if let Some(x) = value.as_copy::<MoltPixel>() {
            Ok(x)
        } else {
            Err(Exception::molt_err(Value::from("Not a pixel string")))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pixel_rgb() {
        let pix = MoltPixel::rgb(1, 2, 3);

        assert_eq!(pix.red(), 1);
        assert_eq!(pix.green(), 2);
        assert_eq!(pix.blue(), 3);
        assert_eq!(pix.alpha(), 255);

        assert_eq!(pix.ipixel(), image::Rgba([1, 2, 3, 255]));
    }

    #[test]
    fn test_pixel_rgba() {
        let pix = MoltPixel::rgba(4, 5, 6, 7);

        assert_eq!(pix.red(), 4);
        assert_eq!(pix.green(), 5);
        assert_eq!(pix.blue(), 6);
        assert_eq!(pix.alpha(), 7);

        assert_eq!(pix.ipixel(), image::Rgba([4, 5, 6, 7]));
    }

    #[test]
    fn test_pixel_display() {
        let pix = MoltPixel::rgb(0, 15, 255);
        assert_eq!(&format!("{}", pix), "#000fff");

        let pix = MoltPixel::rgba(0, 15, 255, 15);
        assert_eq!(&format!("{}", pix), "#000fff.0f");
    }

    #[test]
    fn test_pixel_fromstr() {
        assert_eq!(
            MoltPixel::from_str("#fa7268").unwrap(),
            MoltPixel::rgb(250, 114, 104)
        );

        assert_eq!(
            MoltPixel::from_str("#fa7268.0f").unwrap(),
            MoltPixel::rgba(250, 114, 104, 15)
        );

        // Wrong hex digits
        assert_eq!(
            MoltPixel::from_str("#faXY68"),
            Err("invalid pixel string".into())
        );

        // Wrong length
        assert_eq!(
            MoltPixel::from_str("#fa"),
            Err("invalid pixel string".into())
        );

        // Wrong lead character
        assert_eq!(
            MoltPixel::from_str("-012345"),
            Err("invalid pixel string".into())
        );

        // Wrong separator character
        assert_eq!(
            MoltPixel::from_str("#012345-67"),
            Err("invalid pixel string".into())
        );
    }
}
