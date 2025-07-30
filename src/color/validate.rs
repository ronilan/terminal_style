use super::error::ColorConversionError;

/// Validates whether a given string is a valid 3- or 6-digit hexadecimal color.
///
/// Accepts strings with or without a leading `#`, and checks that the remaining
/// characters are valid ASCII hexadecimal digits (`0-9`, `a-f`, `A-F`), with a length of
/// exactly 3 or 6.
///
/// # Arguments
///
/// * `input` - A string slice that may represent a hex color value (e.g. `"#FFA07A"` or `"0F0"`).
///
/// # Returns
///
/// * `Ok(())` if the input is a valid 3- or 6-digit hex string.
/// * `Err(ColorConversionError::InvalidHex)` if the input is invalid.
///
/// # Examples
///
/// ```
/// assert!(terminal_style::color::validate_hex("#ffcc00").is_ok());
/// assert!(terminal_style::color::validate_hex("abc").is_ok());
/// assert!(terminal_style::color::validate_hex("xyz").is_err());
/// ```

pub fn validate_hex(input: &str) -> Result<(), ColorConversionError> {
    let hex = input.strip_prefix('#').unwrap_or(input);

    if !(hex.len() == 6 || hex.len() == 3) || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(ColorConversionError::InvalidHex(format!(
            "Expected 3 or 6-digit hex string, got: {}",
            input
        )));
    }

    Ok(())
}

/// Validates that an ANSI color value is within the 0–255 range.
///
/// # Arguments
///
/// * `value` - An integer value to validate.
///
/// # Returns
///
/// * `Ok(())` if the value is between 0 and 255 inclusive.
/// * `Err(ColorConversionError::InvalidAnsiValue)` if the value is out of range.
pub fn validate_ansi(value: i32) -> Result<(), ColorConversionError> {
    if !(0..=255).contains(&value) {
        return Err(ColorConversionError::InvalidAnsiValue(value));
    }

    Ok(())
}
