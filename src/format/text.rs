// format.rs
use super::stylable::Stylable;
use crate::color::{ansi_from_color_definition, ColorConversionError, IntoColorString};

// --- Text formatting functions ---
pub fn bold<T: Stylable>(input: T) -> T::Output {
    input.apply(|s| format!("\x1b[1m{}\x1b[0m", s))
}

pub fn italic<T: Stylable>(input: T) -> T::Output {
    input.apply(|s| format!("\x1b[3m{}\x1b[0m", s))
}

pub fn faint<T: Stylable>(input: T) -> T::Output {
    input.apply(|s| format!("\x1b[2m{}\x1b[0m", s))
}

pub fn inverse<T: Stylable>(input: T) -> T::Output {
    input.apply(|s| format!("\x1b[7m{}\x1b[0m", s))
}

pub fn underline<T: Stylable>(input: T) -> T::Output {
    input.apply(|s| format!("\x1b[4m{}\x1b[0m", s))
}

// --- Coloring functions ---
pub fn color<C, T>(color_input: C, text: T) -> Result<T::Output, ColorConversionError>
where
    C: Copy + IntoColorString,
    T: Stylable,
{
    let f = |s: &str| -> Result<String, ColorConversionError> {
        let code = ansi_from_color_definition(color_input)?;
        Ok(format!("\x1b[38;5;{}m{}\x1b[0m", code, s))
    };
    text.apply_result(f)
}

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
