use crate::color::{ansi_from_color_definition, ColorConversionError, IntoColorString};

pub fn color<T>(color_input: T, text: &str) -> Result<String, ColorConversionError>
where
    T: IntoColorString,
{
    let ansi_code = ansi_from_color_definition(color_input)?;
    Ok(format!("\x1b[38;5;{}m{}\x1b[0m", ansi_code, text))
}

pub fn background<T>(color_input: T, text: &str) -> Result<String, ColorConversionError>
where
    T: IntoColorString,
{
    let ansi_code = ansi_from_color_definition(color_input)?;
    Ok(format!("\x1b[48;5;{}m{}\x1b[0m", ansi_code, text))
}
