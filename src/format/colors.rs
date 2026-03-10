use super::stylable::Stylable;
use crate::color::{rgb_from_color_definition, rgb_to_ansi8, ColorConversionError, IntoRgb};

/// Applies a foreground color using **24-bit TrueColor** ANSI sequences.
///
/// This function generates the sequence `\x1b[38;2;R;G;Bm`.
///
/// # Arguments
///
/// * `color_input` - Any type implementing [`IntoRgb`] (e.g., `#RRGGBB`, `[r, g, b]`, or `u8` ANSI).
/// * `text` - Any type implementing [`Stylable`] (e.g., `&str`, `String`, `Vec<String>`, `Vec<Vec<String>>`).
///
/// # Errors
///
/// Returns [`ColorConversionError`] if the color input is invalid.
///
/// # Example
///
/// ```
/// use terminal_style::format::color_rgb;
///
/// let s = color_rgb("#FF0000", "Red Text").unwrap();
/// assert_eq!(s, "\x1b[38;2;255;0;0mRed Text\x1b[0m");
/// ```
pub fn color_rgb<C, T>(color_input: C, text: T) -> Result<T::Output, ColorConversionError>
where
    C: Copy + IntoRgb,
    T: Stylable,
{
    let f = |s: &str| -> Result<String, ColorConversionError> {
        let [r, g, b] = rgb_from_color_definition(color_input)?;
        Ok(format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, s))
    };
    text.apply_result(f)
}

/// Applies a background color using **24-bit TrueColor** ANSI sequences.
///
/// This function generates the sequence `\x1b[48;2;R;G;Bm`.
///
/// # Arguments
///
/// * `color_input` - Any type implementing [`IntoRgb`].
/// * `text` - Any type implementing [`Stylable`].
///
/// # Errors
///
/// Returns [`ColorConversionError`] if the color input is invalid.
///
/// # Example
///
/// ```
/// use terminal_style::format::background_rgb;
///
/// let s = background_rgb([0, 0, 255], "Blue BG").unwrap();
/// assert_eq!(s, "\x1b[48;2;0;0;255mBlue BG\x1b[0m");
/// ```
pub fn background_rgb<C, T>(color_input: C, text: T) -> Result<T::Output, ColorConversionError>
where
    C: Copy + IntoRgb,
    T: Stylable,
{
    let f = |s: &str| -> Result<String, ColorConversionError> {
        let [r, g, b] = rgb_from_color_definition(color_input)?;
        Ok(format!("\x1b[48;2;{};{};{}m{}\x1b[0m", r, g, b, s))
    };
    text.apply_result(f)
}

/// Applies a foreground color using **8-bit ANSI** (256-color) sequences.
///
/// This function quantizes the input color to the nearest 8-bit ANSI index
/// and generates the sequence `\x1b[38;5;Nm`.
///
/// # Arguments
///
/// * `color_input` - Any type implementing [`IntoRgb`].
/// * `text` - Any type implementing [`Stylable`].
///
/// # Errors
///
/// Returns [`ColorConversionError`] if the color input is invalid.
///
/// # Example
///
/// ```
/// use terminal_style::format::color_ansi;
///
/// let s = color_ansi([255, 0, 0], "Red Text").unwrap();
/// assert_eq!(s, "\x1b[38;5;196mRed Text\x1b[0m");
/// ```
pub fn color_ansi<C, T>(color_input: C, text: T) -> Result<T::Output, ColorConversionError>
where
    C: Copy + IntoRgb,
    T: Stylable,
{
    let f = |s: &str| -> Result<String, ColorConversionError> {
        let rgb = rgb_from_color_definition(color_input)?;
        let code = rgb_to_ansi8(rgb);
        Ok(format!("\x1b[38;5;{}m{}\x1b[0m", code, s))
    };
    text.apply_result(f)
}

/// Applies a background color using **8-bit ANSI** (256-color) sequences.
///
/// This function quantizes the input color to the nearest 8-bit ANSI index
/// and generates the sequence `\x1b[48;5;Nm`.
///
/// # Arguments
///
/// * `color_input` - Any type implementing [`IntoRgb`].
/// * `text` - Any type implementing [`Stylable`].
///
/// # Errors
///
/// Returns [`ColorConversionError`] if the color input is invalid.
///
/// # Example
///
/// ```
/// use terminal_style::format::background_ansi;
///
/// let s = background_ansi("#0000FF", "Blue BG").unwrap();
/// assert_eq!(s, "\x1b[48;5;21mBlue BG\x1b[0m");
/// ```
pub fn background_ansi<C, T>(color_input: C, text: T) -> Result<T::Output, ColorConversionError>
where
    C: Copy + IntoRgb,
    T: Stylable,
{
    let f = |s: &str| -> Result<String, ColorConversionError> {
        let rgb = rgb_from_color_definition(color_input)?;
        let code = rgb_to_ansi8(rgb);
        Ok(format!("\x1b[48;5;{}m{}\x1b[0m", code, s))
    };
    text.apply_result(f)
}

/// Applies a foreground color using the default format (**TrueColor**).
///
/// Alias for [`color_rgb`].
pub fn color<C, T>(color_input: C, text: T) -> Result<T::Output, ColorConversionError>
where
    C: Copy + IntoRgb,
    T: Stylable,
{
    color_rgb(color_input, text)
}

/// Applies a background color using the default format (**TrueColor**).
///
/// Alias for [`background_rgb`].
pub fn background<C, T>(color_input: C, text: T) -> Result<T::Output, ColorConversionError>
where
    C: Copy + IntoRgb,
    T: Stylable,
{
    background_rgb(color_input, text)
}
