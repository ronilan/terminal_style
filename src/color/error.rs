use std::fmt;

/// Errors that can occur during color conversion or validation.
#[derive(Debug, PartialEq)]
pub enum ColorConversionError {
    /// Invalid hexadecimal color string.
    InvalidHex(String),
    /// Invalid RGB component value.
    InvalidRgb(String),
    /// ANSI 8-bit color value out of range (0-255).
    InvalidAnsiValue(i32),
    /// Could not determine the color format of the input.
    UnknownFormat(String),
}

impl fmt::Display for ColorConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColorConversionError::InvalidHex(s) => write!(f, "Invalid hex string: {}", s),
            ColorConversionError::InvalidRgb(s) => write!(f, "Invalid RGB value: {}", s),
            ColorConversionError::InvalidAnsiValue(v) => write!(f, "Invalid ANSI value: {}", v),
            ColorConversionError::UnknownFormat(s) => write!(f, "Unknown format: {}", s),
        }
    }
}

impl std::error::Error for ColorConversionError {}
