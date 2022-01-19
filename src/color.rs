// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

use super::util::{fuzzy_compare, fuzzy_is_zero};

/// Represents color value.
///
/// Supports mulitiple color space.
/// Default is RGBA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Color(ColorInner);

#[derive(Debug, Clone, PartialEq, Eq)]
enum ColorInner {
    Cmyk(ColorCmyk),
    Hsl(ColorHsl),
    Hsv(ColorHsv),
    Rgb(ColorRgb),
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
        match &self.0 {
            ColorInner::Cmyk(c) => self.clone(),
            ColorInner::Hsl(_) => self.to_rgb().to_cmyk(),
            ColorInner::Hsv(_) => self.to_rgb().to_cmyk(),
            ColorInner::Rgb(c) => {
                if c.red == 0 || c.green == 0 || c.blue == 0 {
                    // Special case, div-by-zero.
                    return Self(ColorInner::Cmyk(ColorCmyk {
                        alpha: c.alpha,
                        cyan: 0,
                        magenta: 0,
                        yellow: 0,
                        black: MAX_VALUE,
                    }));
                }
                // rgb -> cmy
                let red = c.red as f64 / MAX_FLOAT_VALUE;
                let green = c.green as f64 / MAX_FLOAT_VALUE;
                let blue = c.blue as f64 / MAX_FLOAT_VALUE;
                let mut cyan = 1.0 - red;
                let mut magenta = 1.0 - green;
                let mut yellow = 1.0 - blue;

                // cmy -> cmyk
                let black = red.min(green.min(blue));
                let black_revert = 1.0 - black;
                cyan = (cyan - black) / black_revert;
                magenta = (magenta - black) / black_revert;
                yellow = (yellow - black) / black_revert;

                Self(ColorInner::Cmyk(ColorCmyk {
                    alpha: c.alpha,
                    cyan: (cyan * MAX_FLOAT_VALUE).round() as u8,
                    magenta: (magenta * MAX_FLOAT_VALUE).round() as u8,
                    yellow: (yellow * MAX_FLOAT_VALUE).round() as u8,
                    black: (black * MAX_FLOAT_VALUE).round() as u8,
                }))
            }
        }
    }

    /// Creates and returns an RGB color based on this color.
    #[inline]
    pub fn to_rgb(&self) -> Self {
        match &self.0 {
            ColorInner::Cmyk(c) => {
                let cyan = c.cyan as f64 / MAX_FLOAT_VALUE;
                let magenta = c.magenta as f64 / MAX_FLOAT_VALUE;
                let yellow = c.yellow as f64 / MAX_FLOAT_VALUE;
                let black = c.black as f64 / MAX_FLOAT_VALUE;

                let red = 1.0 - (cyan * (1.0 - black) + black);
                let green = 1.0 - (magenta * (1.0 - black) + black);
                let blue = 1.0 - (yellow * (1.0 - black) + black);

                Self(ColorInner::Rgb(ColorRgb {
                    alpha: c.alpha,
                    red: (red * MAX_FLOAT_VALUE).round() as u8,
                    green: (green * MAX_FLOAT_VALUE).round() as u8,
                    blue: (blue * MAX_FLOAT_VALUE).round() as u8,
                }))
            }
            ColorInner::Hsl(c) => {
                let mut red = 0;
                let mut green = 0;
                let mut blue = 0;
                if c.saturation == 0 || c.hue == MAX_VALUE {
                    // achromatic case
                    red = c.lightness;
                    green = c.lightness;
                    blue = c.lightness;
                } else if c.lightness == 0 {
                    red = 0;
                    green = 0;
                    blue = 0;
                } else {
                    // chromatic case
                    // FIXME(Shaohua): u8 out of range
                    let hue = if c.hue as u32 == 36_000 {
                        0.0
                    } else {
                        c.hue as f64 / 36_000.0
                    };
                    let saturation = c.saturation as f64 / MAX_FLOAT_VALUE;
                    let lightness = c.lightness as f64 / MAX_FLOAT_VALUE;
                    let temp2 = if lightness < 0.5 {
                        lightness * (1.0 + saturation)
                    } else {
                        lightness + saturation - (lightness * saturation)
                    };

                    let temp1 = (2.0 * lightness) - temp2;
                    let mut temp3 = [hue + (1.0 / 3.0), hue, hue - (1.0 / 3.0)];
                    let mut array = [0u8; 4];

                    for i in 0..3 {
                        if temp3[i] < 0.0 {
                            temp3[i] += 1.0;
                        } else if temp3[i] > 1.0 {
                            temp3[i] -= 1.0;
                        }

                        let sixtemp3 = temp3[i] * 6.0;
                        if sixtemp3 < 1.0 {
                            array[i + 1] = ((temp1 + (temp2 - temp1) * sixtemp3) * MAX_FLOAT_VALUE)
                                .round() as u8;
                        } else if (temp3[i] * 2.0) < 1.0 {
                            array[i + 1] = (temp2 * MAX_FLOAT_VALUE).round() as u8;
                        } else if (temp3[i] * 3.0) < 2.0 {
                            array[i + 1] = ((temp1
                                + (temp2 - temp1) * (2.0 / 3.0 - temp3[i]) * 6.0)
                                * MAX_FLOAT_VALUE)
                                .round() as u8;
                        } else {
                            array[i + 1] = (temp1 * MAX_FLOAT_VALUE).round() as u8;
                        }
                    }

                    red = array[1];
                    green = array[2];
                    blue = array[3];
                }

                Self(ColorInner::Rgb(ColorRgb {
                    alpha: c.alpha,
                    red,
                    green,
                    blue,
                }))
            }
            ColorInner::Hsv(_c) => {
                //
                self.clone()
            }
            ColorInner::Rgb(_) => self.clone(),
        }
    }

    /// Creates and returns an HSL color based on this color.
    #[inline]
    pub fn to_hsl(&self) -> Self {
        match &self.0 {
            ColorInner::Cmyk(_) => self.to_rgb().to_hsl(),
            ColorInner::Hsl(_) => self.clone(),
            ColorInner::Hsv(_) => self.to_rgb().to_hsl(),
            ColorInner::Rgb(c) => {
                let red = c.red as f64 / MAX_FLOAT_VALUE;
                let green = c.green as f64 / MAX_FLOAT_VALUE;
                let blue = c.blue as f64 / MAX_FLOAT_VALUE;
                let max_val = red.max(green.max(blue));
                let min_val = red.min(green.min(blue));
                let delta = max_val - min_val;
                let delta2 = max_val + min_val;
                let lightness = 0.5 * delta2;
                let mut hsl = ColorHsl {
                    alpha: c.alpha,
                    hue: 0,
                    saturation: 0,
                    lightness: (lightness * MAX_FLOAT_VALUE).round() as u8,
                };

                if fuzzy_is_zero(delta) {
                    // achromatic case, hue is undefined.
                    hsl.hue = MAX_VALUE;
                    hsl.saturation = 0;
                } else {
                    // chromatic case.
                    hsl.saturation = if lightness < 0.5 {
                        ((delta / delta2) * MAX_FLOAT_VALUE).round() as u8
                    } else {
                        (delta / (2.0 - delta2) * MAX_FLOAT_VALUE).round() as u8
                    };

                    let mut hue = 0.0;
                    if fuzzy_compare(red, max_val) {
                        hue = (green - blue) / delta;
                    } else if fuzzy_compare(green, max_val) {
                        hue = 2.0 + (blue - red) / delta;
                    } else if fuzzy_compare(blue, max_val) {
                        hue = 4.0 + (red - green) / delta;
                    } else {
                        // TODO(Shaohua): Throw an error
                    }

                    hue *= 60.0;
                    if hue < 0.0 {
                        hue += 360.0;
                    }
                    hsl.hue = (hue * 100.0).round() as u8;
                }

                Self(ColorInner::Hsl(hsl))
            }
        }
    }

    /// Creates and returns an HSV color based on this color.
    #[inline]
    pub fn to_hsv(&self) -> Self {
        match &self.0 {
            ColorInner::Cmyk(_) => self.to_rgb().to_hsl(),
            ColorInner::Hsl(_) => self.to_rgb().to_hsv(),
            ColorInner::Hsv(_) => self.clone(),
            ColorInner::Rgb(c) => {
                let red = c.red as f64 / MAX_FLOAT_VALUE;
                let green = c.green as f64 / MAX_FLOAT_VALUE;
                let blue = c.blue as f64 / MAX_FLOAT_VALUE;
                let max_val = red.max(green.max(blue));
                let min_val = red.min(green.min(blue));
                let delta = max_val - min_val;
                let value = max_val;

                let mut hsv = ColorHsv {
                    alpha: c.alpha,
                    hue: 0,
                    saturation: 0,
                    value: (value * MAX_FLOAT_VALUE).round() as u8,
                };

                if fuzzy_is_zero(delta) {
                    // achromatic case, hue is undefined.
                    hsv.hue = MAX_VALUE;
                    hsv.saturation = 0;
                } else {
                    // chromatic case.
                    hsv.saturation = ((delta / max_val) * MAX_FLOAT_VALUE).round() as u8;

                    let mut hue = 0.0;
                    if fuzzy_compare(red, max_val) {
                        hue = (green - blue) / delta;
                    } else if fuzzy_compare(green, max_val) {
                        hue = 2.0 + (blue - red) / delta;
                    } else if fuzzy_compare(blue, max_val) {
                        hue = 4.0 + (red - green) / delta;
                    } else {
                        // TODO(Shaohua): Throw an error
                    }

                    hue *= 60.0;
                    if hue < 0.0 {
                        hue += 360.0;
                    }
                    hsv.hue = (hue * 100.0).round() as u8;
                }

                Self(ColorInner::Hsv(hsv))
            }
        }
    }

    /// Create a copy of this color in the specified format.
    pub fn convert_to(&self, spec: Spec) -> Self {
        if spec == self.spec() {
            return self.clone();
        }
        match spec {
            Spec::Rgb => self.to_rgb(),
            Spec::Hsv => self.to_hsv(),
            Spec::Cmyk => self.to_cmyk(),
            Spec::Hsl => self.to_hsl(),
        }
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

    /// Construct color from the RGB color values.
    ///
    /// All the values must be in range 0.0-1.0.
    #[inline]
    pub fn from_rgb_f(red: f64, green: f64, blue: f64) -> Result<Self, ParseColorError> {
        Self::from_rgba_f(red, green, blue, 1.0)
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

    /// Construct color from the RGBA color values.
    ///
    /// All the values must be in range 0.0-1.0.
    #[inline]
    pub fn from_rgba_f(
        red: f64,
        green: f64,
        blue: f64,
        alpha: f64,
    ) -> Result<Self, ParseColorError> {
        if red < 0.0
            || red > 1.0
            || green < 0.0
            || green > 1.0
            || blue < 0.0
            || blue > 1.0
            || alpha < 0.0
            || alpha > 1.0
        {
            return Err(ParseColorError::OutOfRangeError);
        }

        Ok(Self(ColorInner::Rgb(ColorRgb {
            alpha: (alpha * MAX_FLOAT_VALUE).round() as u8,
            red: (red * MAX_FLOAT_VALUE).round() as u8,
            green: (green * MAX_FLOAT_VALUE).round() as u8,
            blue: (blue * MAX_FLOAT_VALUE).round() as u8,
        })))
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
