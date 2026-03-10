//! Utilities for color conversion and validation.
//!
//! This module provides functions for converting between different color formats:
//! - **Hex**: `#RRGGBB` or `#RGB` strings.
//! - **RGB**: `[u8; 3]` arrays.
//! - **ANSI**: 8-bit color codes (0-255).
//!
//! It also handles validation of these formats and defines the [`ColorConversionError`] type.

pub mod ansi;
pub mod error;
pub mod hex;
pub mod rgb;
pub mod validate;

pub use ansi::{ansi8_to_hex, ansi8_to_rgb, rgb_from_color_definition, IntoRgb};
pub use error::ColorConversionError;
pub use hex::{hex_to_ansi8, hex_to_rgb};
pub use rgb::{rgb_to_ansi8, rgb_to_hex};
pub use validate::{validate_ansi, validate_hex};
