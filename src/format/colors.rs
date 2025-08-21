use super::stylable::Stylable;
use crate::color::{ansi_from_color_definition, ColorConversionError, IntoColorString};

/// Foreground color
pub fn color<C, T>(color_input: C, text: T) -> Result<T::Output, ColorConversionError>
where
    C: Copy + IntoColorString,
    T: Stylable,
{
    // Closure now returns Result<String, ColorConversionError>
    let f = |s: &str| -> Result<String, ColorConversionError> {
        let code = ansi_from_color_definition(color_input)?;
        Ok(format!("\x1b[38;5;{}m{}\x1b[0m", code, s))
    };

    // Apply to the Stylable text, propagating errors
    text.apply_result(f)
}

/// Background color
pub fn background<C, T>(color_input: C, text: T) -> Result<T::Output, ColorConversionError>
where
    C: Copy + IntoColorString,
    T: Stylable,
{
    let f = |s: &str| -> Result<String, ColorConversionError> {
        let code = ansi_from_color_definition(color_input)?;
        Ok(format!("\x1b[48;5;{}m{}\x1b[0m", code, s))
    };

    text.apply_result(f)
}
