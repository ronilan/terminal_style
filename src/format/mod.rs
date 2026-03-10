//! Terminal text styling and formatting.
//!
//! This module provides functions for applying colors and text styles (like bold, italic, etc.)
//! to various types of input (strings, vectors, 2D vectors) via the [`Stylable`] trait.

pub mod colors;
pub mod stylable;
pub mod text;

pub use colors::{
    background, background_ansi, background_rgb, color, color_ansi, color_rgb,
};
pub use stylable::Stylable;
pub use text::{bold, faint, inverse, italic, underline};
