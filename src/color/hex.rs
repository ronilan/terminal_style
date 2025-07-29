use super::rgb::rgb_to_ansi8;

/// Converts a hex color string (e.g. "#FF00AA" or "F0A") to an RGB array.
///
/// This function assumes that the input is always valid and performs no error checking.
/// It supports both 6-character ("RRGGBB") and 3-character ("RGB") hex codes, with or without a leading `#`.
///
/// # Examples
///
/// ```
/// assert_eq!(terminal_style::color::hex_to_rgb("#FF00AA"), [255, 0, 170]);
/// assert_eq!(terminal_style::color::hex_to_rgb("F0A"), [255, 0, 170]);
/// ```
pub fn hex_to_rgb(hex: &str) -> [u8; 3] {
    let hex = hex.trim_start_matches('#');

    if hex.len() == 6 {
        // Full 6-digit hex code: extract and convert each component
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
        [r, g, b]
    } else {
        // 3-digit shorthand: duplicate each character
        let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).unwrap();
        let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).unwrap();
        let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).unwrap();
        [r, g, b]
    }
}

/// Converts a hex color string to an ANSI 8-bit color value.
///
/// This function first converts the hex color to RGB using [`hex_to_rgb`],
/// then maps the RGB value to the corresponding ANSI 8-bit color using [`rgb_to_ansi8`].
///
/// # Examples
///
/// ```
/// assert_eq!(terminal_style::color::hex_to_ansi8("#FF0000"), 196); // Red
/// assert_eq!(terminal_style::color::hex_to_ansi8("0F0"), 46);      // Green
/// ```
pub fn hex_to_ansi8(hex: &str) -> u8 {
    let rgb = hex_to_rgb(hex);
    rgb_to_ansi8(rgb)
}
