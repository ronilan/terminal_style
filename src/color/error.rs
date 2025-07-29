use std::fmt;

#[derive(Debug)]
pub enum ColorConversionError {
    InvalidHex(String),
    InvalidRgb(String),
    InvalidAnsiValue(u8),
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
