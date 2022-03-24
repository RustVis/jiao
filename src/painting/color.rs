// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

use super::rgb::Rgb;
use super::rgba64::Rgba64;
use crate::util::{fuzzy_compare, fuzzy_is_zero};

/// The Color class provides colors based on RGB, HSV or CMYK values.
///
///
/// A color is normally specified in terms of RGB (red, green, and blue) components,
/// but it is also possible to specify it in terms of HSV (hue, saturation, and value) and
/// CMYK (cyan, magenta, yellow and black) components.
///
/// In addition a color can be specified using a color name.
/// The color name can be any of the SVG 1.0 color names.
///
/// The Color constructor creates the color based on RGB values. To create a Color
/// based on either HSV or CMYK values, use the `to_hsv()` and `to_cmyk()` functions respectively.
/// These functions return a copy of the color using the desired format.
/// In addition the `from_rgb()`, `from_hsv()` and `from_cmyk()` functions
/// create colors from the specified values.
/// Alternatively, a color can be converted to any of the three formats using
/// the `convert_to()` function (returning a copy of the color in the desired format),
/// or any of the `set_rgb()`, `set_hsv()` and `set_cmyk()` functions
/// altering this color's format.
/// The `spec()` function tells how the color was specified.
///
/// A color can be set by passing an RGB string (such as "#112233"),
/// or an ARGB string (such as "#ff112233") or a color name (such as "blue"),
/// to the `set_named_color()` function.
/// The color names are taken from the SVG 1.0 color names.
/// The `name()` function returns the name of the color in the format "#RRGGBB".
/// Colors can also be set using `set_rgb()`, `set_hsv()` and `set_cmyk()`.
/// To get a lighter or darker color use the `lighter()` and `darker()` functions respectively.
///
/// The `is_valid()` function indicates whether a Color is legal at all.
/// For example, a RGB color with RGB values out of range is illegal.
/// For performance reasons, Color mostly disregards illegal colors, and for that reason,
/// the result of using an invalid color is undefined.
///
/// The color components can be retrieved individually, e.g with `red()`, `hue()` and `cyan()`.
/// The values of the color components can also be retrieved in one go using
/// the `get_rgb()`, `get_hsv()` and `get_cmyk()` functions.
/// Using the RGB color model, the color components can in addition be accessed with `rgb()`.
///
/// There are several related non-members:
/// Rgb is a typdef for an unsigned int representing the RGB value triplet (r, g, b).
/// Note that it also can hold a value for the alpha-channel
/// (for more information, see the Alpha-Blended Drawing section).
///
/// # Integer vs. Floating Point Precision
/// Color supports floating point precision and provides floating point versions
/// of all the color components functions, e.g. `get_rgb_f()`, `huef()` and `from_cmyk_f()`.
/// Note that since the components are stored using 16-bit integers, there might be
/// minor deviations between the values set using, for example, `set_rgb_f()`
/// and the values returned by the `get_rgb_f()` function due to rounding.
///
/// While the integer based functions take values in the range 0-255
/// (except hue() which must have values within the range 0-359),
/// the floating point functions accept values in the range 0.0 - 1.0.
///
/// # The Extended RGB Color Model
/// The extended RGB color model, also known as the scRGB color space,
/// is the same the RGB color model except it allows values under 0.0, and over 1.0.
/// This makes it possible to represent colors that would otherwise be outside
/// the range of the RGB colorspace but still use the same values for colors inside
/// the RGB colorspace.
///
/// # The HSV Color Model
/// The RGB model is hardware-oriented. Its representation is close to what most monitors show.
/// In contrast, HSV represents color in a way more suited to the human perception of color.
/// For example, the relationships "stronger than", "darker than", and "the opposite of"
/// are easily expressed in HSV but are much harder to express in RGB.
///
/// HSV, like RGB, has three components:
/// - H, for hue, is in the range 0 to 359 if the color is chromatic (not gray),
/// or meaningless if it is gray. It represents degrees on the color wheel familiar to most people.
/// Red is 0 (degrees), green is 120, and blue is 240.
/// - S, for saturation, is in the range 0 to 255, and the bigger it is, the stronger the color is.
/// Grayish colors have saturation near 0; very strong colors have saturation near 255.
/// - V, for value, is in the range 0 to 255 and represents lightness or brightness of the color.
/// 0 is black; 255 is as far from black as possible.
///
/// Here are some examples: pure red is H=0, S=255, V=255; a dark red, moving slightly
/// towards the magenta, could be H=350 (equivalent to -10), S=255, V=180; a grayish light red
/// could have H about 0 (say 350-359 or 0-10), S about 50-100, and S=255.
///
/// `Color` returns a hue value of -1 for achromatic colors. If you pass a hue value
/// that is too large, `Color` forces it into range. Hue 360 or 720 is treated as 0;
/// hue 540 is treated as 180.
///
/// # The HSL Color Model
/// HSL is similar to HSV, however instead of the Value parameter, HSL specifies
/// a Lightness parameter which maps somewhat differently to the brightness of the color.
///
/// Similarly, the HSL saturation value is not in general the same as the HSV saturation value
/// for the same color. hslSaturation() provides the color's HSL saturation value,
/// while saturation() and hsvSaturation() provides the HSV saturation value.
///
/// The hue value is defined to be the same in HSL and HSV.
///
/// # The CMYK Color Model
/// While the RGB and HSV color models are used for display on computer monitors,
/// the CMYK model is used in the four-color printing process of printing presses and
/// some hard-copy devices.
///
/// CMYK has four components, all in the range 0-255: cyan (C), magenta (M), yellow (Y) and
/// black (K). Cyan, magenta and yellow are called subtractive colors; the CMYK color model
/// creates color by starting with a white surface and then subtracting color
/// by applying the appropriate components. While combining cyan, magenta and yellow
/// gives the color black, subtracting one or more will yield any other color.
/// When combined in various percentages, these three colors can create the entire spectrum of colors.
///
/// Mixing 100 percent of cyan, magenta and yellow does produce black, but the result
/// is unsatisfactory since it wastes ink, increases drying time, and gives a muddy colour
/// when printing. For that reason, black is added in professional printing to provide a solid black tone;
/// hence the term 'four color process'.
///
/// Default is RGBA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Color {
    inner: ColorInner,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ColorInner {
    Cmyk(ColorCmyk),
    Hsl(ColorHsl),
    Hsv(ColorHsv),
    Rgb(ColorRgb),
}

impl ColorInner {
    const fn cmyk(cyan: u8, magenta: u8, yellow: u8, black: u8, alpha: u8) -> Self {
        Self::Cmyk(ColorCmyk {
            alpha,
            cyan,
            magenta,
            yellow,
            black,
        })
    }

    const fn hsl(hue: u16, saturation: u8, lightness: u8, alpha: u8) -> Self {
        Self::Hsl(ColorHsl {
            alpha,
            hue,
            saturation,
            lightness,
        })
    }

    const fn hsv(hue: u16, saturation: u8, value: u8, alpha: u8) -> Self {
        Self::Hsv(ColorHsv {
            alpha,
            hue,
            saturation,
            value,
        })
    }

    const fn rgb(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self::Rgb(ColorRgb {
            alpha,
            red,
            green,
            blue,
        })
    }
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
    hue: u16,
    saturation: u8,
    lightness: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ColorHsv {
    alpha: u8,
    hue: u16,
    saturation: u8,
    value: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ColorRgb {
    alpha: u8,
    red: u8,
    green: u8,
    blue: u8,
}

/// The type of color specified.
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Spec {
    Rgb = 1,
    Hsv = 2,
    Cmyk = 3,
    Hsl = 4,
}

pub const MAX_VALUE: u8 = u8::MAX;
pub const MAX_VALUE_F64: f64 = MAX_VALUE as f64;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParseColorError {
    InvalidFormatError,
    OutOfRangeError,
}

/// How to format the output of the `name()` function
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NameFormat {
    /// `#RRGGBB` A "#" character followed by three two-digit hexadecimal numbers (i.e. `#RRGGBB`).
    HexRgb = 0,

    /// `#AARRGGBB` A "#" character followed by four two-digit hexadecimal numbers (i.e. `#AARRGGBB`).
    HexArgb = 1,
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
    /// Constructs an invalid color with the RGB value (0, 0, 0).
    ///
    /// An invalid color is a color that is not properly set up for the underlying window system.
    ///
    /// The alpha value of an invalid color is unspecified.
    pub const fn new() -> Self {
        Self::from_rgb(0, 0, 0)
    }

    /// Constructs a named color in the same way as `set_named_color()` using the given name.
    pub fn from_name(name: &str) -> Self {
        unimplemented!()
    }

    /// Construct color from the given CMYK color values.
    ///
    /// All the values must be in the range 0-255.
    pub fn from_cmyk(cyan: u8, magenta: u8, yellow: u8, black: u8, alpha: u8) -> Self {
        Self {
            inner: ColorInner::cmyk(cyan, magenta, yellow, black, alpha),
        }
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

        Ok(Self {
            inner: ColorInner::cmyk(
                (cyan * MAX_VALUE_F64).round() as u8,
                (magenta * MAX_VALUE_F64).round() as u8,
                (yellow * MAX_VALUE_F64).round() as u8,
                (black * MAX_VALUE_F64).round() as u8,
                (alpha * MAX_VALUE_F64).round() as u8,
            ),
        })
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
        if hue < -1 || hue >= 360 {
            return Err(ParseColorError::OutOfRangeError);
        }
        let real_hue = if hue == -1 {
            u16::MAX
        } else {
            (hue % 360 * 100) as u16
        };

        Ok(Self {
            inner: ColorInner::hsl(real_hue, saturation, lightness, alpha),
        })
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
            u16::MAX
        } else {
            (hue * 36_000.0).round() as u16
        };

        Ok(Self {
            inner: ColorInner::hsl(
                real_hue,
                (saturation * MAX_VALUE_F64).round() as u8,
                (lightness * MAX_VALUE_F64).round() as u8,
                (alpha * MAX_VALUE_F64).round() as u8,
            ),
        })
    }

    /// Construct color from the HSV color values.
    ///
    /// The value of saturation, value, and alpha must all be in the range 0-255;
    /// the value of hue must be in the range 0-359.
    fn from_hsv(hue: i32, saturation: u8, value: u8, alpha: u8) -> Result<Self, ParseColorError> {
        if hue < -1 || hue >= 360 {
            return Err(ParseColorError::OutOfRangeError);
        }
        let real_hue = if hue == -1 {
            u16::MAX
        } else {
            ((hue % 360) * 100) as u16
        };

        Ok(Self {
            inner: ColorInner::hsv(real_hue, saturation, value, alpha),
        })
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
            u16::MAX
        } else {
            (hue * 36_000.0).round() as u16
        };

        Ok(Self {
            inner: ColorInner::hsv(
                real_hue,
                (saturation * MAX_VALUE_F64).round() as u8,
                (value * MAX_VALUE_F64).round() as u8,
                (alpha * MAX_VALUE_F64).round() as u8,
            ),
        })
    }

    /// Construct color from the RGB color values.
    ///
    /// All the values must be in range 0-255.
    pub const fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self::from_rgba(red, green, blue, MAX_VALUE)
    }

    /// Construct color from the RGB color values.
    ///
    /// All the values must be in range 0.0-1.0.
    pub fn from_rgb_f(red: f64, green: f64, blue: f64) -> Result<Self, ParseColorError> {
        Self::from_rgba_f(red, green, blue, 1.0)
    }

    /// Construct color from the RGBA color values.
    ///
    /// All the values must be in range 0-255.
    pub const fn from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            inner: ColorInner::rgb(red, green, blue, alpha),
        }
    }

    /// Construct color from the RGBA color values.
    ///
    /// All the values must be in range 0.0-1.0.
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

        Ok(Self {
            inner: ColorInner::rgb(
                (red * MAX_VALUE_F64).round() as u8,
                (green * MAX_VALUE_F64).round() as u8,
                (blue * MAX_VALUE_F64).round() as u8,
                (alpha * MAX_VALUE_F64).round() as u8,
            ),
        })
    }

    /// Constructs a color with the value color.
    ///
    /// The alpha component is ignored and set to solid.
    pub fn from_rgb32(rgb: Rgb) -> Self {
        Self::from_rgba(rgb.red(), rgb.green(), rgb.blue(), 0xff)
    }

    /// Constructs a color with the value rgba64.
    pub fn from_rgba64(rgba64: Rgba64) -> Self {
        Self::from_rgba(
            rgba64.red8(),
            rgba64.green8(),
            rgba64.blue8(),
            rgba64.alpha8(),
        )
    }

    /// Creates and returns a CMYK color based on this color.
    pub fn to_cmyk(&self) -> Self {
        match &self.inner {
            ColorInner::Cmyk(c) => self.clone(),
            ColorInner::Hsl(_) => self.to_rgb().to_cmyk(),
            ColorInner::Hsv(_) => self.to_rgb().to_cmyk(),
            ColorInner::Rgb(c) => {
                if c.red == 0 || c.green == 0 || c.blue == 0 {
                    // Special case, div-by-zero.
                    return Self {
                        inner: ColorInner::cmyk(0, 0, 0, MAX_VALUE, c.alpha),
                    };
                }
                // rgb -> cmy
                let red = c.red as f64 / MAX_VALUE_F64;
                let green = c.green as f64 / MAX_VALUE_F64;
                let blue = c.blue as f64 / MAX_VALUE_F64;
                let mut cyan = 1.0 - red;
                let mut magenta = 1.0 - green;
                let mut yellow = 1.0 - blue;

                // cmy -> cmyk
                let black = red.min(green.min(blue));
                let black_revert = 1.0 - black;
                cyan = (cyan - black) / black_revert;
                magenta = (magenta - black) / black_revert;
                yellow = (yellow - black) / black_revert;

                Self {
                    inner: ColorInner::cmyk(
                        (cyan * MAX_VALUE_F64).round() as u8,
                        (magenta * MAX_VALUE_F64).round() as u8,
                        (yellow * MAX_VALUE_F64).round() as u8,
                        (black * MAX_VALUE_F64).round() as u8,
                        c.alpha,
                    ),
                }
            }
        }
    }

    /// Creates and returns an RGB color based on this color.
    pub fn to_rgb(&self) -> Self {
        match &self.inner {
            ColorInner::Cmyk(c) => {
                let cyan = c.cyan as f64 / MAX_VALUE_F64;
                let magenta = c.magenta as f64 / MAX_VALUE_F64;
                let yellow = c.yellow as f64 / MAX_VALUE_F64;
                let black = c.black as f64 / MAX_VALUE_F64;

                let red = 1.0 - (cyan * (1.0 - black) + black);
                let green = 1.0 - (magenta * (1.0 - black) + black);
                let blue = 1.0 - (yellow * (1.0 - black) + black);

                Self {
                    inner: ColorInner::rgb(
                        (red * MAX_VALUE_F64).round() as u8,
                        (green * MAX_VALUE_F64).round() as u8,
                        (blue * MAX_VALUE_F64).round() as u8,
                        c.alpha,
                    ),
                }
            }
            ColorInner::Hsl(c) => {
                let mut red = 0;
                let mut green = 0;
                let mut blue = 0;
                if c.saturation == 0 || c.hue == u16::MAX {
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
                    let hue = if c.hue == 36_000 {
                        0.0
                    } else {
                        c.hue as f64 / 36_000.0
                    };
                    let saturation = c.saturation as f64 / MAX_VALUE_F64;
                    let lightness = c.lightness as f64 / MAX_VALUE_F64;
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
                            array[i + 1] = ((temp1 + (temp2 - temp1) * sixtemp3) * MAX_VALUE_F64)
                                .round() as u8;
                        } else if (temp3[i] * 2.0) < 1.0 {
                            array[i + 1] = (temp2 * MAX_VALUE_F64).round() as u8;
                        } else if (temp3[i] * 3.0) < 2.0 {
                            array[i + 1] = ((temp1
                                + (temp2 - temp1) * (2.0 / 3.0 - temp3[i]) * 6.0)
                                * MAX_VALUE_F64)
                                .round() as u8;
                        } else {
                            array[i + 1] = (temp1 * MAX_VALUE_F64).round() as u8;
                        }
                    }

                    red = array[1];
                    green = array[2];
                    blue = array[3];
                }

                Self {
                    inner: ColorInner::rgb(red, green, blue, c.alpha),
                }
            }
            ColorInner::Hsv(_c) => {
                //
                self.clone()
            }
            ColorInner::Rgb(_) => self.clone(),
        }
    }

    /// Creates and returns an HSL color based on this color.
    pub fn to_hsl(&self) -> Self {
        match &self.inner {
            ColorInner::Cmyk(_) => self.to_rgb().to_hsl(),
            ColorInner::Hsl(_) => self.clone(),
            ColorInner::Hsv(_) => self.to_rgb().to_hsl(),
            ColorInner::Rgb(c) => {
                let red = c.red as f64 / MAX_VALUE_F64;
                let green = c.green as f64 / MAX_VALUE_F64;
                let blue = c.blue as f64 / MAX_VALUE_F64;
                let max_val = red.max(green.max(blue));
                let min_val = red.min(green.min(blue));
                let delta = max_val - min_val;
                let delta2 = max_val + min_val;
                let lightness = 0.5 * delta2;
                let mut hsl = ColorHsl {
                    alpha: c.alpha,
                    hue: 0,
                    saturation: 0,
                    lightness: (lightness * MAX_VALUE_F64).round() as u8,
                };

                if fuzzy_is_zero(delta) {
                    // achromatic case, hue is undefined.
                    hsl.hue = 360;
                    hsl.saturation = 0;
                } else {
                    // chromatic case.
                    hsl.saturation = if lightness < 0.5 {
                        ((delta / delta2) * MAX_VALUE_F64).round() as u8
                    } else {
                        (delta / (2.0 - delta2) * MAX_VALUE_F64).round() as u8
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
                    hsl.hue = (hue * 100.0).round() as u16;
                }

                Self {
                    inner: ColorInner::Hsl(hsl),
                }
            }
        }
    }

    /// Creates and returns an HSV color based on this color.
    pub fn to_hsv(&self) -> Self {
        match &self.inner {
            ColorInner::Cmyk(_) => self.to_rgb().to_hsl(),
            ColorInner::Hsl(_) => self.to_rgb().to_hsv(),
            ColorInner::Hsv(_) => self.clone(),
            ColorInner::Rgb(c) => {
                let red = c.red as f64 / MAX_VALUE_F64;
                let green = c.green as f64 / MAX_VALUE_F64;
                let blue = c.blue as f64 / MAX_VALUE_F64;
                let max_val = red.max(green.max(blue));
                let min_val = red.min(green.min(blue));
                let delta = max_val - min_val;
                let value = max_val;

                let mut hsv = ColorHsv {
                    alpha: c.alpha,
                    hue: 0,
                    saturation: 0,
                    value: (value * MAX_VALUE_F64).round() as u8,
                };

                if fuzzy_is_zero(delta) {
                    // achromatic case, hue is undefined.
                    hsv.hue = 360;
                    hsv.saturation = 0;
                } else {
                    // chromatic case.
                    hsv.saturation = ((delta / max_val) * MAX_VALUE_F64).round() as u8;

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
                    hsv.hue = (hue * 100.0).round() as u16;
                }

                Self {
                    inner: ColorInner::Hsv(hsv),
                }
            }
        }
    }

    /// Returns the alpha color component of this color.
    pub fn alpha(&self) -> u8 {
        match &self.inner {
            ColorInner::Rgb(c) => c.alpha,
            ColorInner::Hsv(c) => c.alpha,
            ColorInner::Cmyk(c) => c.alpha,
            ColorInner::Hsl(c) => c.alpha,
        }
    }

    /// Returns the alpha color component of this color.
    pub fn alpha_f(&self) -> f64 {
        self.alpha() as f64 / MAX_VALUE_F64
    }

    /// Returns the black color component of this color.
    pub fn black(&self) -> u8 {
        unimplemented!()
    }

    /// Returns the black color component of this color.
    pub fn black_f(&self) -> f64 {
        unimplemented!()
    }

    /// Returns the blue color component of this color.
    pub fn blue(&self) -> u8 {
        match &self.inner {
            ColorInner::Rgb(c) => c.blue,
            _ => self.to_rgb().blue(),
        }
    }

    /// Returns the blue color component of this color.
    pub fn blue_f(&self) -> f64 {
        unimplemented!()
    }

    /// Returns a string list containing the color names we knows about.
    pub fn color_names() -> Vec<String> {
        unimplemented!()
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

    /// Returns the cyan color component of this color.
    pub fn cyan(&self) -> u8 {
        unimplemented!()
    }

    /// Returns the cyan color component of this color.
    pub fn cyan_f(&self) -> f64 {
        unimplemented!()
    }

    /// Returns a darker color (with factor 200), but does not change this object.
    pub fn darker(&self) -> Self {
        self.darker_by(200)
    }

    /// Returns a darker (or lighter) color, but does not change this object.
    ///
    /// If the factor is greater than 100, this functions returns a darker color.
    /// Setting factor to 300 returns a color that has one-third the brightness.
    ///
    /// If the factor is less than 100, the return color is lighter,
    /// but we recommend using the lighter() function for this purpose.
    /// If the factor is 0 or negative, the return value is unspecified.
    ///
    /// The function converts the current color to HSV, divides the value (V) component
    /// by factor and converts the color back to it's original color spec.
    pub fn darker_by(&self, factor: i32) -> Self {
        unimplemented!()
    }

    /// Sets the contents to the cyan, magenta, yellow, black, and alpha-channel (transparency)
    /// components of the color's CMYK value.
    ///
    /// These components can be retrieved individually using the `cyan()`, `magenta()`,
    /// `yellow()`, `black()` and `alpha()` functions.
    pub fn get_cmyk(
        &self,
        cyan: &mut u8,
        magenta: &mut u8,
        yellow: &mut u8,
        black: &mut u8,
        alpha: &mut u8,
    ) {
        unimplemented!()
    }

    /// Sets the contents to the cyan, magenta, yellow, black, and alpha-channel (transparency)
    /// components of the color's CMYK value.
    ///
    /// These components can be retrieved individually using the `cyan_f()`,
    /// `magenta_f()`, `yellow_f()`, `black_f()` and `alpha_f()` functions.
    pub fn get_cmyk_f(
        &self,
        cyan: &mut f64,
        magenta: &mut f64,
        yellow: &mut f64,
        black: &mut f64,
        alpha: &mut f64,
    ) {
        unimplemented!()
    }

    /// Sets the contents to the hue, saturation, lightness, and alpha-channel (transparency)
    /// components of the color's HSL value.
    ///
    /// These components can be retrieved individually using the `hsl_hue()`, `hsl_saturation()`,
    /// `lightness()` and `alpha()` functions.
    pub fn get_hsl(&self, hue: &mut i32, saturation: &mut u8, lightness: &mut u8, alpha: &mut u8) {
        match &self.inner {
            ColorInner::Hsl(c) => {
                *hue = if c.hue == u16::MAX {
                    -1
                } else {
                    c.hue as i32 / 100
                };
                *saturation = c.saturation;
                *lightness = c.lightness;
                *alpha = c.alpha;
            }
            _ => self.to_hsl().get_hsl(hue, saturation, lightness, alpha),
        }
    }

    /// Sets the contents to the hue, saturation, lightness, and alpha-channel (transparency)
    /// components of the color's HSL value.
    ///
    /// These components can be retrieved individually using the `hsl_hue_f()`,
    /// `hsl_saturation_f()`, `lightness_f()` and `alpha_f()` functions.
    pub fn get_hsl_f(
        &self,
        hue: &mut f64,
        saturation: &mut f64,
        lightness: &mut f64,
        alpha: &mut f64,
    ) {
        match &self.inner {
            ColorInner::Hsl(c) => {
                *hue = if c.hue == u16::MAX {
                    -1.0
                } else {
                    c.hue as f64 / 36_000.0
                };
                *saturation = c.saturation as f64 / MAX_VALUE_F64;
                *lightness = c.lightness as f64 / MAX_VALUE_F64;
                *alpha = c.alpha as f64 / MAX_VALUE_F64;
            }
            _ => self.to_hsl().get_hsl_f(hue, saturation, lightness, alpha),
        }
    }

    /// Sets the contents to the hue, saturation, value, and alpha-channel (transparency)
    /// components of the color's HSV value.
    ///
    /// These components can be retrieved individually using the `hue()`, `saturation()`,
    /// `value()` and `alpha()` functions.
    pub fn get_hsv(&self, hue: &mut i32, saturation: &mut u8, value: &mut u8, alpha: &mut u8) {
        match &self.inner {
            ColorInner::Hsv(c) => {
                *hue = if c.hue == u16::MAX {
                    -1
                } else {
                    (c.hue / 100) as i32
                };
                *saturation = c.saturation;
                *value = c.value;
                *alpha = c.alpha;
            }
            _ => self.to_hsv().get_hsv(hue, saturation, value, alpha),
        }
    }

    /// Sets the contents to the hue, saturation, value, and alpha-channel (transparency)
    /// components of the color's HSV value.
    ///
    /// These components can be retrieved individually using the `hue_f()`, `saturation_f()`,
    /// `value_f()` and `alpha_f()` functions.
    pub fn get_hsv_f(&self, hue: &mut f64, saturation: &mut f64, value: &mut f64, alpha: &mut f64) {
        match &self.inner {
            ColorInner::Hsv(c) => {
                *hue = if c.hue == u16::MAX {
                    -1.0
                } else {
                    c.hue as f64 / 36_000.0
                };
                *saturation = c.saturation as f64 / MAX_VALUE_F64;
                *value = c.value as f64 / MAX_VALUE_F64;
                *alpha = c.alpha as f64 / MAX_VALUE_F64;
            }
            _ => self.to_hsv().get_hsv_f(hue, saturation, value, alpha),
        }
    }

    /// Sets the contents to the red, green, blue, and alpha-channel (transparency)
    /// components of the color's RGB value.
    ///
    /// These components can be retrieved individually using the `red()`, `green()`,
    /// `blue()` and `alpha()` functions.
    pub fn get_rgb(&self, red: &mut u8, green: &mut u8, blue: &mut u8, alpha: &mut u8) {
        unimplemented!()
    }

    /// Sets the contents to the red, green, blue, and alpha-channel (transparency)
    /// components of the color's RGB value.
    ///
    /// These components can be retrieved individually using the `red_f()`, `green_f()`,
    /// `blue_f()` and `alpha_f()` functions.
    pub fn get_rgb_f(&self, red: &mut f64, green: &mut f64, blue: &mut f64, alpha: &mut f64) {
        unimplemented!()
    }

    /// Returns the green color component of this color.
    pub fn green(&self) -> u8 {
        match &self.inner {
            ColorInner::Rgb(c) => c.green,
            _ => self.to_rgb().green(),
        }
    }

    /// Returns the green color component of this color.
    pub fn green_f(&self) -> f64 {
        unimplemented!()
    }

    /// Returns the HSL hue color component of this color.
    pub fn hsl_hue(&self) -> u8 {
        unimplemented!()
    }

    /// Returns the HSL hue color component of this color.
    pub fn hsl_hue_f(&self) -> f64 {
        unimplemented!()
    }

    /// Returns the HSL saturation color component of this color.
    pub fn hsl_saturation(&self) -> u8 {
        unimplemented!()
    }

    /// Returns the HSL saturation color component of this color.
    pub fn hsl_saturation_f(&self) -> f64 {
        unimplemented!()
    }

    /// Returns the HSV hue color component of this color.
    pub fn hsv_hue(&self) -> u8 {
        unimplemented!()
    }

    /// Returns the HSV hue color component of this color.
    pub fn hsv_hue_f(&self) -> f64 {
        unimplemented!()
    }

    /// Returns the HSV saturation color component of this color.
    pub fn hsv_saturation(&self) -> u8 {
        unimplemented!()
    }

    /// Returns the HSV saturation color component of this color.
    pub fn hsv_saturation_v(&self) -> f64 {
        unimplemented!()
    }

    /// Returns the HSV hue color component of this color.
    ///
    /// The color is implicitly converted to HSV.
    pub fn hue(&self) -> u8 {
        unimplemented!()
    }

    /// Returns the HSV hue color component of this color.
    ///
    /// The color is implicitly converted to HSV.
    pub fn hue_f(&self) -> f64 {
        unimplemented!()
    }

    /// Returns true if the name is a valid color name and can be used
    /// to construct a valid Color object, otherwise returns false.
    ///
    /// It uses the same algorithm used in `set_named_color()`.
    pub fn is_valid_color(name: &str) -> bool {
        unimplemented!()
    }

    /// Returns a lighter color (with factor 50), but does not change this object.
    pub fn lighter(&self) -> Self {
        self.lighter_by(50)
    }

    /// Returns a lighter (or darker) color, but does not change this object.
    ///
    /// If the factor is greater than 100, this functions returns a lighter color.
    /// Setting factor to 150 returns a color that is 50% brighter.
    /// If the factor is less than 100, the return color is darker,
    /// but we recommend using the `darker()` function for this purpose.
    /// If the factor is 0 or negative, the return value is unspecified.
    ///
    /// The function converts the current color to HSV, multiplies the value (V) component
    /// by factor and converts the color back to it's original color spec.
    pub fn lighter_by(&self, factor: i32) -> Self {
        unimplemented!()
    }

    /// Returns the lightness color component of this color.
    pub fn lightness(&self) -> u8 {
        unimplemented!()
    }

    /// Returns the lightness color component of this color.
    pub fn lightness_f(&self) -> f64 {
        unimplemented!()
    }

    /// Returns the magenta color component of this color.
    pub fn magenta(&self) -> u8 {
        unimplemented!()
    }

    /// Returns the magenta color component of this color.
    pub fn magenta_f(&self) -> f64 {
        unimplemented!()
    }

    /// Returns the name of the color in the format "#RRGGBB".
    ///
    /// i.e. a "#" character followed by three two-digit hexadecimal numbers.
    pub fn name(&self) -> String {
        self.name_with_format(NameFormat::HexRgb)
    }

    /// Returns the name of the color in the specified format.
    pub fn name_with_format(&self, format: NameFormat) -> String {
        let value = self.rgba().int() as u64;
        match format {
            NameFormat::HexRgb => format!("#{:x}", value & 0x0ffffff),
            // it's called rgba() but it does return AARRGGBB
            NameFormat::HexArgb => format!("#{:x}", value & 0x0ffffffff),
        }
    }

    /// Returns the red color component of this color.
    pub fn red(&self) -> u8 {
        match &self.inner {
            ColorInner::Rgb(c) => c.red,
            _ => self.to_rgb().red(),
        }
    }

    /// Returns the red color component of this color.
    pub fn red_f(&self) -> f64 {
        unimplemented!()
    }

    /// Returns the RGB value of the color.
    ///
    /// The alpha value is opaque.
    pub fn rgb(&self) -> Rgb {
        let color = self.to_rgb();
        Rgb::new(color.red(), color.green(), color.blue())
    }

    /// Returns the RGB64 value of the color, including its alpha.
    ///
    /// For an invalid color, the alpha value of the returned color is unspecified.
    pub fn rgba64(&self) -> Rgba64 {
        unimplemented!()
    }

    /// Returns the RGB value of the color, including its alpha.
    ///
    /// For an invalid color, the alpha value of the returned color is unspecified.
    pub fn rgba(&self) -> Rgb {
        let color = self.to_rgb();
        Rgb::with_alpha(color.red(), color.green(), color.blue(), color.alpha())
    }

    /// Returns the HSV saturation color component of this color.
    ///
    /// The color is implicitly converted to HSV.
    pub fn saturation(&self) -> u8 {
        unimplemented!()
    }

    /// Returns the HSV saturation color component of this color.
    ///
    /// The color is implicitly converted to HSV.
    pub fn saturation_f(&self) -> f64 {
        unimplemented!()
    }

    /// Set alpha channel of this color.
    pub fn set_alpha(&mut self, alpha: u8) {
        match &mut self.inner {
            ColorInner::Rgb(c) => c.alpha = alpha,
            ColorInner::Hsv(c) => c.alpha = alpha,
            ColorInner::Cmyk(c) => c.alpha = alpha,
            ColorInner::Hsl(c) => c.alpha = alpha,
        }
    }

    /// Sets the alpha of this color to alpha.
    ///
    /// qreal alpha is specified in the range 0.0-1.0.
    pub fn set_alpha_f(&mut self, alpha: f64) -> Result<(), ParseColorError> {
        check_float_range(alpha)?;
        let alpha_int = (alpha * MAX_VALUE_F64).round() as u8;
        match &mut self.inner {
            ColorInner::Rgb(c) => c.alpha = alpha_int,
            ColorInner::Hsv(c) => c.alpha = alpha_int,
            ColorInner::Cmyk(c) => c.alpha = alpha_int,
            ColorInner::Hsl(c) => c.alpha = alpha_int,
        }
        Ok(())
    }

    /// Sets the blue color component of this color to blue.
    ///
    /// Integer components are specified in the range 0-255.
    pub fn set_blue(&mut self, blue: u8) {
        unimplemented!()
    }

    /// Sets the blue color component of this color to blue.
    pub fn set_blue_f(&mut self, blue: f64) {
        unimplemented!()
    }

    /// Sets the color to CMYK values, cyan, magenta, yellow, black, and alpha-channel.
    ///
    /// All the values must be in the range 0-255.
    pub fn set_cmyk(&mut self, cyan: u8, magenta: u8, yellow: u8, black: u8, alpha: u8) {
        unimplemented!()
    }

    /// Sets the color to CMYK values, cyan, magenta, yellow, black, and alpha-channel.
    ///
    /// All the values must be in the range 0.0-1.0.
    pub fn set_cmyk_f(&mut self, cyan: f64, magenta: f64, yellow: f64, black: f64, alpha: f64) {
        unimplemented!()
    }

    /// Sets the green color component of this color to green.
    ///
    /// Integer components are specified in the range 0-255.
    pub fn set_green(&mut self, green: u8) {
        unimplemented!()
    }

    /// Sets the green color component of this color to green.
    pub fn set_green_f(&mut self, green: f64) {
        unimplemented!()
    }

    /// Sets a HSL color value.
    ///
    /// The saturation, value and alpha values must be in the range 0-255,
    /// and the hue value must be greater than -1.
    pub fn set_hsl(
        &mut self,
        hue: i32,
        saturation: u8,
        lightness: u8,
        alpha: u8,
    ) -> Result<(), ParseColorError> {
        if hue < -1 {
            return Err(ParseColorError::OutOfRangeError);
        }

        let real_hue = if hue == -1 {
            u16::MAX
        } else {
            (hue % 360) as u16 * 100
        };
        self.inner = ColorInner::hsl(real_hue, saturation, lightness, alpha);
        return Ok(());
    }

    /// Sets a HSL color lightness.
    ///
    /// All the values must be in the range 0.0-1.0.
    pub fn set_hsl_f(
        &mut self,
        hue: f64,
        saturation: f64,
        lightness: f64,
        alpha: f64,
    ) -> Result<(), ParseColorError> {
        if (hue < 0.0 || hue > 1.0) && hue != -1.0
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
            u16::MAX
        } else {
            (hue * 36_000.0).round() as u16
        };
        self.inner = ColorInner::hsl(
            real_hue,
            (saturation * MAX_VALUE_F64).round() as u8,
            (lightness * MAX_VALUE_F64).round() as u8,
            (alpha * MAX_VALUE_F64).round() as u8,
        );
        return Ok(());
    }

    /// Sets a HSV color value.
    ///
    /// The saturation, value and alpha-channel values must be in the range 0-255,
    /// and the hue value must be greater than -1.
    pub fn set_hsv(
        &mut self,
        hue: i32,
        saturation: u8,
        value: u8,
        alpha: u8,
    ) -> Result<(), ParseColorError> {
        if hue < -1 || hue > 360 {
            return Err(ParseColorError::OutOfRangeError);
        }

        let real_hue = if hue == -1 {
            u16::MAX
        } else {
            (hue % 360) as u16 * 100
        };
        self.inner = ColorInner::hsv(real_hue, saturation, value, alpha);
        return Ok(());
    }

    /// Sets a HSV color value.
    ///
    /// All the values must be in the range 0.0-1.0.
    pub fn set_hsv_f(
        &mut self,
        hue: f64,
        saturation: f64,
        value: f64,
        alpha: f64,
    ) -> Result<(), ParseColorError> {
        if (hue < 0.0 || hue > 1.0) && hue != -1.0
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
            u16::MAX
        } else {
            (hue * 36_000.0).round() as u16
        };
        self.inner = ColorInner::hsv(
            real_hue,
            (saturation * MAX_VALUE_F64).round() as u8,
            (value * MAX_VALUE_F64).round() as u8,
            (alpha * MAX_VALUE_F64).round() as u8,
        );
        return Ok(());
    }

    /// Sets the RGB value of this Color to name.
    ///
    /// May be in one of these formats:
    /// - `#RGB` (each of R, G, and B is a single hex digit)
    /// - `#RRGGBB`
    /// - `#AARRGGBB`
    /// - `#RRRGGGBBB`
    /// - `#RRRRGGGGBBBB`
    /// - A name from the list of colors defined in the list of SVG color keyword names
    /// provided by the World Wide Web Consortium; for example, "steelblue" or "gainsboro".
    /// These color names work on all platforms.
    /// - transparent: representing the absence of a color.
    ///
    /// The color is invalid if name cannot be parsed.
    pub fn set_named_color(&mut self, name: &str) {
        unimplemented!()
    }

    /// Sets the red color component of this color to red.
    ///
    /// Integer components are specified in the range 0-255.
    pub fn set_red(&mut self, red: u8) {
        unimplemented!()
    }

    /// Sets the red color component of this color to red.
    pub fn set_red_f(&mut self, red: f64) {
        unimplemented!()
    }

    /// Sets the RGB value.
    ///
    /// All the values must be in the range 0-255.
    pub fn set_rgb(&mut self, red: u8, green: u8, blue: u8) {
        unimplemented!()
    }

    /// Sets the RGB value to rgb.
    ///
    /// The alpha value is set to opaque.
    pub fn set_rgb32(&mut self, rgb: Rgb) {
        unimplemented!()
    }

    /// Sets the RGB64 value to rgba, including its alpha.
    pub fn set_rgba64(&mut self, rgba: Rgba64) {
        unimplemented!()
    }

    /// Sets the color channels of this color to (red, green, blue).
    ///
    /// The alpha value must be in the range 0.0-1.0.
    pub fn set_rgb_f(red: f64, green: f64, blue: f64, alpha: f64) {
        unimplemented!()
    }

    /// Sets the RGB value to rgba, including its alpha.
    pub fn set_rgba(&mut self, rgb: Rgb) {
        unimplemented!()
    }

    /// Returns how the color was specified.
    pub fn spec(&self) -> Spec {
        match &self.inner {
            ColorInner::Rgb(_) => Spec::Rgb,
            ColorInner::Hsv(_) => Spec::Hsv,
            ColorInner::Cmyk(_) => Spec::Cmyk,
            ColorInner::Hsl(_) => Spec::Hsl,
        }
    }

    /// Returns the value color component of this color.
    pub fn value(&self) -> u8 {
        unimplemented!()
    }

    /// Returns the value color component of this color.
    pub fn value_f(&self) -> f64 {
        unimplemented!()
    }

    /// Returns the yellow color component of this color.
    pub fn yellow(&self) -> u8 {
        unimplemented!()
    }

    /// Returns the yellow color component of this color.
    pub fn yellow_f(&self) -> f64 {
        unimplemented!()
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
    use serde::{Deserialize, Serialize};
    use std::str::FromStr;

    use super::{Color, NameFormat};

    #[test]
    fn test_parse_color() {
        let color = Color::from_rgba(255, 255, 255, 100);
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
        #[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
        struct Rectangle {
            x: i32,
            y: i32,
            color: Option<Color>,
        }

        let r = Rectangle {
            x: 1,
            y: 2,
            color: Some(Color::from_rgb(101, 102, 103)),
        };
        let s = serde_json::to_string_pretty(&r).unwrap();
        let r2: Rectangle = serde_json::from_str(&s).unwrap();
        assert_eq!(r, r2);
    }

    #[test]
    fn test_name() {
        let color = Color::from_rgb(240, 248, 255);
        assert_eq!(color.name(), "#f0f8ff");
        assert_eq!(color.name_with_format(NameFormat::HexArgb), "#fff0f8ff");
    }
}
