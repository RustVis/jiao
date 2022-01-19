// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

/// Represents color value.
///
/// Supports mulitiple color space.
/// Default is RGBA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Color(ColorInner);

#[derive(Debug, Clone, PartialEq, Eq)]
enum ColorInner {
    Rgb(ColorRgb),
    Hsv(ColorHsv),
    Cmyk(ColorCmyk),
    Hsl(ColorHsl),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ColorRgb {
    alpha: u8,
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ColorHsv {
    alpha: u8,
    hue: u8,
    saturation: u8,
    value: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ColorCmyk {
    alpha: u8,
    cyan: u8,
    magenta: u8,
    yellow: u8,
    black: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ColorHsl {
    alpha: u8,
    hue: u8,
    saturation: u8,
    lightness: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Spec {
    Rgb,
    Hsv,
    Cmyk,
    Hsl,
}

pub const MAX_VALUE: u8 = u8::MAX;
pub const MAX_FLOAT_VALUE: f64 = MAX_VALUE as f64;
pub const MAX_HUE_VALUE: i32 = 360;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParseColorError {
    InvalidFormatError,
    OutOfRangeError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NameFormat {
    HexRgb,
    HexArgb,
}

fn check_float_range(value: f64) -> Result<(), ParseColorError> {
    if value < 0.0 || value > 1.0 {
        return Err(ParseColorError::OutOfRangeError);
    }
    Ok(())
}

impl fmt::Display for ParseColorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::InvalidFormatError => "Invalid format",
            Self::OutOfRangeError => "Out of range",
        };
        write!(f, "{}", s)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::from_rgb(0, 0, 0)
    }
}

impl Color {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates and returns a CMYK color based on this color.
    #[inline]
    pub fn to_cmyk(&self) -> Self {
        self.convert_to(Spec::Cmyk)
    }

    /// Creates and returns an RGB color based on this color.
    #[inline]
    pub fn to_rgb(&self) -> Self {
        self.convert_to(Spec::Rgb)
    }

    /// Creates and returns an HSL color based on this color.
    #[inline]
    pub fn to_hsl(&self) -> Self {
        self.convert_to(Spec::Hsl)
    }

    /// Creates and returns an HSV color based on this color.
    #[inline]
    pub fn to_hsv(&self) -> Self {
        self.convert_to(Spec::Hsv)
    }

    /// Create a copy of this color in the specified format.
    pub fn convert_to(&self, spec: Spec) -> Self {
        // TODO(Shaohua):
        Self::default()
    }

    /// Construct color from the given CMYK color values.
    ///
    /// All the values must be in the range 0-255.
    pub fn from_cmyk(cyan: u8, magenta: u8, yellow: u8, black: u8, alpha: u8) -> Self {
        Self(ColorInner::Cmyk(ColorCmyk {
            alpha,
            cyan,
            magenta,
            yellow,
            black,
        }))
    }

    /// Construct color from the given CMYK color values.
    ///
    /// All the values must be in the range 0.0-1.0.
    pub fn from_cmyk_f(
        cyan: f64,
        magenta: f64,
        yellow: f64,
        black: f64,
        alpha: f64,
    ) -> Result<Self, ParseColorError> {
        if cyan < 0.0
            || cyan > 1.0
            || magenta < 0.0
            || magenta > 1.0
            || yellow < 0.0
            || yellow > 1.0
            || black < 0.0
            || black > 1.0
            || alpha < 0.0
            || alpha > 1.0
        {
            return Err(ParseColorError::OutOfRangeError);
        }

        Ok(Self(ColorInner::Cmyk(ColorCmyk {
            alpha: (alpha * MAX_FLOAT_VALUE).round() as u8,
            cyan: (cyan * MAX_FLOAT_VALUE).round() as u8,
            magenta: (magenta * MAX_FLOAT_VALUE).round() as u8,
            yellow: (yellow * MAX_FLOAT_VALUE).round() as u8,
            black: (black * MAX_FLOAT_VALUE).round() as u8,
        })))
    }

    /// Construct color from the HSL color values.
    ///
    /// The value of saturation, lightness, and alpha must all be in the range 0-255;
    /// the value of hue must be in the range 0-359.
    fn from_hsl(
        hue: i32,
        saturation: u8,
        lightness: u8,
        alpha: u8,
    ) -> Result<Self, ParseColorError> {
        if hue < -1 || hue >= MAX_HUE_VALUE {
            return Err(ParseColorError::OutOfRangeError);
        }
        let real_hue = if hue == -1 {
            MAX_VALUE
        } else {
            (hue % MAX_HUE_VALUE * 100) as u8
        };

        Ok(Self(ColorInner::Hsl(ColorHsl {
            alpha,
            hue: real_hue,
            saturation,
            lightness,
        })))
    }

    /// Construct color from the HSL color values.
    ///
    /// All the values must be in range 0.0-1.0.
    fn from_hsl_f(
        hue: f64,
        saturation: f64,
        lightness: f64,
        alpha: f64,
    ) -> Result<Self, ParseColorError> {
        if (hue < 0.0 && hue != -1.0)
            || hue > 1.0
            || saturation < 0.0
            || saturation > 1.0
            || lightness < 0.0
            || lightness > 1.0
            || alpha < 0.0
            || alpha > 1.0
        {
            return Err(ParseColorError::OutOfRangeError);
        }
        let real_hue = if hue == -1.0 {
            MAX_VALUE
        } else {
            (hue * 36_000.0).round() as u8
        };

        Ok(Self(ColorInner::Hsl(ColorHsl {
            alpha: (alpha * MAX_FLOAT_VALUE).round() as u8,
            hue: real_hue,
            saturation: (saturation * MAX_FLOAT_VALUE).round() as u8,
            lightness: (lightness * MAX_FLOAT_VALUE).round() as u8,
        })))
    }

    /// Construct color from the HSV color values.
    ///
    /// The value of saturation, value, and alpha must all be in the range 0-255;
    /// the value of hue must be in the range 0-359.
    fn from_hsv(hue: i32, saturation: u8, value: u8, alpha: u8) -> Result<Self, ParseColorError> {
        if hue < -1 || hue >= MAX_HUE_VALUE {
            return Err(ParseColorError::OutOfRangeError);
        }
        let real_hue = if hue == -1 {
            MAX_VALUE
        } else {
            (hue % MAX_HUE_VALUE * 100) as u8
        };

        Ok(Self(ColorInner::Hsv(ColorHsv {
            alpha,
            hue: real_hue,
            saturation,
            value,
        })))
    }

    /// Construct color from the HSV color values.
    ///
    /// All the values must be in range 0.0-1.0.
    fn from_hsv_f(
        hue: f64,
        saturation: f64,
        value: f64,
        alpha: f64,
    ) -> Result<Self, ParseColorError> {
        if (hue < 0.0 && hue != -1.0)
            || hue > 1.0
            || saturation < 0.0
            || saturation > 1.0
            || value < 0.0
            || value > 1.0
            || alpha < 0.0
            || alpha > 1.0
        {
            return Err(ParseColorError::OutOfRangeError);
        }
        let real_hue = if hue == -1.0 {
            MAX_VALUE
        } else {
            (hue * 36_000.0).round() as u8
        };

        Ok(Self(ColorInner::Hsv(ColorHsv {
            alpha: (alpha * MAX_FLOAT_VALUE).round() as u8,
            hue: real_hue,
            saturation: (saturation * MAX_FLOAT_VALUE).round() as u8,
            value: (value * MAX_FLOAT_VALUE).round() as u8,
        })))
    }

    /// Construct color from the RGB color values.
    ///
    /// All the values must be in range 0-255.
    #[inline]
    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self::from_rgba(red, green, blue, MAX_VALUE)
    }

    /// Construct color from the RGBA color values.
    ///
    /// All the values must be in range 0-255.
    #[inline]
    pub fn from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self(ColorInner::Rgb(ColorRgb {
            alpha,
            red,
            green,
            blue,
        }))
    }

    /// Returns how the color was specified.
    pub fn spec(&self) -> Spec {
        match &self.0 {
            ColorInner::Rgb(_) => Spec::Rgb,
            ColorInner::Hsv(_) => Spec::Hsv,
            ColorInner::Cmyk(_) => Spec::Cmyk,
            ColorInner::Hsl(_) => Spec::Hsl,
        }
    }

    /// Returns the alpha color component of this color.
    #[inline]
    pub fn alpha(&self) -> u8 {
        match &self.0 {
            ColorInner::Rgb(c) => c.alpha,
            ColorInner::Hsv(c) => c.alpha,
            ColorInner::Cmyk(c) => c.alpha,
            ColorInner::Hsl(c) => c.alpha,
        }
    }

    /// Set alpha channel of this color.
    #[inline]
    pub fn set_alpha(&mut self, alpha: u8) {
        match &mut self.0 {
            ColorInner::Rgb(c) => c.alpha = alpha,
            ColorInner::Hsv(c) => c.alpha = alpha,
            ColorInner::Cmyk(c) => c.alpha = alpha,
            ColorInner::Hsl(c) => c.alpha = alpha,
        }
    }

    /// Returns the alpha color component of this color.
    #[inline]
    pub fn alpha_f(&self) -> f64 {
        self.alpha() as f64 / MAX_FLOAT_VALUE
    }

    /// Sets the alpha of this color to \a alpha. qreal alpha is specified in the range 0.0-1.0.
    #[inline]
    pub fn set_alpha_f(&mut self, alpha: f64) -> Result<(), ParseColorError> {
        check_float_range(alpha)?;
        let alpha_int = (alpha * MAX_FLOAT_VALUE).round() as u8;
        match &mut self.0 {
            ColorInner::Rgb(c) => c.alpha = alpha_int,
            ColorInner::Hsv(c) => c.alpha = alpha_int,
            ColorInner::Cmyk(c) => c.alpha = alpha_int,
            ColorInner::Hsl(c) => c.alpha = alpha_int,
        }
        Ok(())
    }
    /// Returns the red color component of this color.
    pub fn red(&self) -> u8 {
        match &self.0 {
            ColorInner::Rgb(c) => c.red,
            _ => self.to_rgb().red(),
        }
    }

    /// Returns the green color component of this color.
    pub fn green(&self) -> u8 {
        match &self.0 {
            ColorInner::Rgb(c) => c.green,
            _ => self.to_rgb().green(),
        }
    }

    /// Returns the blue color component of this color.
    pub fn blue(&self) -> u8 {
        match &self.0 {
            ColorInner::Rgb(c) => c.blue,
            _ => self.to_rgb().blue(),
        }
    }

    pub fn to_rgb_str(&self) -> String {
        debug_assert!(self.alpha() == MAX_VALUE);
        format!("#{:x}{:x}{:x}", self.red(), self.green(), self.blue())
    }
}

impl From<std::num::ParseIntError> for ParseColorError {
    fn from(_error: std::num::ParseIntError) -> Self {
        Self::InvalidFormatError
    }
}

impl std::string::ToString for Color {
    fn to_string(&self) -> String {
        String::from(format!(
            "rgba({}, {}, {}, {})",
            self.red(),
            self.green(),
            self.blue(),
            self.alpha()
        ))
    }
}

impl std::str::FromStr for Color {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace(" ", "");
        let s = s.trim();

        let len = s.len();

        if len < 4 {
            return Err(ParseColorError::InvalidFormatError);
        }

        if &s[0..1] == "#" {
            return match len {
                9 => {
                    // #rrggbbaa
                    let red = u8::from_str_radix(&s[1..3], 16)?;
                    let green = u8::from_str_radix(&s[3..5], 16)?;
                    let blue = u8::from_str_radix(&s[5..7], 16)?;
                    let alpha = u8::from_str_radix(&s[7..9], 16)?;

                    Ok(Color::from_rgba(red, green, blue, alpha))
                }
                7 => {
                    // #rrggbb
                    let red = u8::from_str_radix(&s[1..3], 16)?;
                    let green = u8::from_str_radix(&s[3..5], 16)?;
                    let blue = u8::from_str_radix(&s[5..7], 16)?;

                    Ok(Color::from_rgb(red, green, blue))
                }
                4 => {
                    // #rgb
                    let red = u8::from_str_radix(&s[1..2], 16)?;
                    let green = u8::from_str_radix(&s[2..3], 16)?;
                    let blue = u8::from_str_radix(&s[3..4], 16)?;

                    // Duplicate bytes
                    let red = red * 17;
                    let green = green * 17;
                    let blue = blue * 17;

                    Ok(Color::from_rgb(red, green, blue))
                }
                _ => Err(ParseColorError::InvalidFormatError),
            };
        } else if len > 12 && &s[0..5] == "rgba(" && &s[len - 1..len] == ")" {
            // rgba(0,0,0,0)
            // rgba(101, 255, 255, 100)
            let parts: Vec<&str> = s[4..]
                .trim_matches(|p| p == '(' || p == ')')
                .split(',')
                .collect();
            if parts.len() != 4 {
                return Err(ParseColorError::InvalidFormatError);
            }

            let red = parts[0].parse::<u8>()?;
            let green = parts[1].parse::<u8>()?;
            let blue = parts[2].parse::<u8>()?;
            let alpha = parts[3].parse::<u8>()?;

            return Ok(Color::from_rgba(red, green, blue, alpha));
        } else if len > 9 && &s[0..4] == "rgb(" && &s[len - 1..len] == ")" {
            // rgb(0,0,0)
            // rgb(101, 255, 255)
            let parts: Vec<&str> = s[3..]
                .trim_matches(|p| p == '(' || p == ')')
                .split(',')
                .collect();
            if parts.len() != 3 {
                return Err(ParseColorError::InvalidFormatError);
            }

            let red = parts[0].parse::<u8>()?;
            let green = parts[1].parse::<u8>()?;
            let blue = parts[2].parse::<u8>()?;

            return Ok(Color::from_rgb(red, green, blue));
        }

        return Err(ParseColorError::InvalidFormatError);
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = self.to_string();
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        let color = Color::from_str(&s).map_err(de::Error::custom)?;
        Ok(color)
    }
}

#[cfg(test)]
mod tests {
    use serde_derive::{Deserialize, Serialize};
    use std::str::FromStr;

    use super::Color;

    #[test]
    fn test_parse_color() {
        let color = Color::rgba(255, 255, 255, 100);
        println!("color: {:?}", color);
        println!("color: {}", color.to_string());

        let colors = [
            "#fea",
            "#ffeeaa",
            "#ffeeaa99",
            "rgb ( 255, 255, 200)",
            "rgba ( 255, 255, 200, 255)",
            "rgb ( 255)",
        ];

        for color_str in &colors {
            let color = color_str.parse::<Color>();
            println!("color_str: {}, color: {:?}", color_str, color);
        }
    }

    #[test]
    fn test_parse_color_structs() {
        #[derive(Debug, Default, Clone, Copy, PartialEq, Deserialize, Serialize)]
        struct Rectangle {
            x: i32,
            y: i32,
            color: Option<Color>,
        }

        let r = Rectangle {
            x: 1,
            y: 2,
            color: Some(Color::rgb(101, 102, 103)),
        };
        let s = serde_json::to_string_pretty(&r).unwrap();
        println!("s: {}", s);
        let r2: Rectangle = serde_json::from_str(&s).unwrap();
        assert_eq!(r, r2);
    }
}
