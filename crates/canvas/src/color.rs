// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

const MAX_VALUE: u8 = 255;

#[derive(Debug)]
pub enum ParseColorError {
    InvalidFormatError,
    OutOfRangeError,
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
        Self {
            red: 0,
            green: 0,
            blue: 0,
            alpha: MAX_VALUE,
        }
    }
}

impl Color {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha: MAX_VALUE,
        }
    }

    pub fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn red(&self) -> u8 {
        self.red
    }

    pub fn green(&self) -> u8 {
        self.green
    }

    pub fn blue(&self) -> u8 {
        self.blue
    }

    pub fn alpha(&self) -> u8 {
        self.alpha
    }

    pub fn to_rgb_str(&self) -> String {
        debug_assert!(self.alpha == MAX_VALUE);
        format!("#{:x}{:x}{:x}", self.red, self.green, self.blue)
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
            self.red, self.green, self.blue, self.alpha
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

                    Ok(Color::rgba(red, green, blue, alpha))
                }
                7 => {
                    // #rrggbb
                    let red = u8::from_str_radix(&s[1..3], 16)?;
                    let green = u8::from_str_radix(&s[3..5], 16)?;
                    let blue = u8::from_str_radix(&s[5..7], 16)?;

                    Ok(Color::rgba(red, green, blue, MAX_VALUE))
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

                    Ok(Color::rgba(red, green, blue, MAX_VALUE))
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

            return Ok(Color::rgba(red, green, blue, alpha));
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

            return Ok(Color::rgba(red, green, blue, MAX_VALUE));
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
