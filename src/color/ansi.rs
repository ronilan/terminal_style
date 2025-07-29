use super::{
    error::ColorConversionError,
    hex::hex_to_ansi8,
    rgb::{rgb_to_ansi8, rgb_to_hex},
    validate::validate_hex,
};

/// Converts various types of color representations into an ANSI 8-bit color string.
///
/// This function uses the `IntoColorString` trait to accept multiple input types,
/// including RGB arrays, hex strings, and u8 ANSI values.
pub fn ansi_from_color_definition<T>(input: T) -> Result<String, ColorConversionError>
where
    T: IntoColorString,
{
    input.into_color_string()
}

/// A trait for converting different color formats into an ANSI 8-bit color string.
pub trait IntoColorString {
    fn into_color_string(self) -> Result<String, ColorConversionError>;
}

/// Implements conversion from an RGB array to an ANSI color string.
impl IntoColorString for [u8; 3] {
    fn into_color_string(self) -> Result<String, ColorConversionError> {
        Ok(rgb_to_ansi8(self).to_string())
    }
}

/// Implements conversion from a `String` hex color to an ANSI color string.
/// Validates the hex format before conversion.
impl IntoColorString for String {
    fn into_color_string(self) -> Result<String, ColorConversionError> {
        validate_hex(&self)?;
        Ok(hex_to_ansi8(&self).to_string())
    }
}

/// Implements conversion from a `&str` hex color to an ANSI color string.
/// Validates the hex format before conversion.
impl IntoColorString for &str {
    fn into_color_string(self) -> Result<String, ColorConversionError> {
        validate_hex(self)?;
        Ok(hex_to_ansi8(self).to_string())
    }
}

/// Implements conversion from an ANSI 8-bit value directly to a string.
impl IntoColorString for u8 {
    fn into_color_string(self) -> Result<String, ColorConversionError> {
        Ok(self.to_string())
    }
}

/// Converts an ANSI 8-bit color value to an RGB array.
///
/// Supports standard ANSI color ranges:
/// - 0–15: system colors
/// - 16–231: 6×6×6 color cube
/// - 232–255: grayscale ramp
pub fn ansi8_to_rgb(num: u8) -> [u8; 3] {
    match num {
        0..=6 => {
            // Low-intensity primary colors
            let mut bits = format!("{:03b}", num)
                .chars()
                .rev()
                .map(|c| c.to_digit(2).unwrap())
                .map(|b| (b * 255 / 2) as u8)
                .collect::<Vec<_>>();

            // Ensure array has 3 components
            while bits.len() < 3 {
                bits.push(0);
            }

            [bits[0], bits[1], bits[2]]
        }
        7 => [192, 192, 192], // Light gray
        8 => [127, 127, 127], // Dark gray
        9..=15 => {
            // High-intensity primary colors
            let bits = format!("{:03b}", num - 8)
                .chars()
                .rev()
                .map(|c| c.to_digit(2).unwrap())
                .map(|b| (b * 255) as u8)
                .collect::<Vec<_>>();

            [bits[0], bits[1], bits[2]]
        }
        16..=231 => {
            // 6×6×6 color cube (216 colors)
            let index = num - 16;
            let r = index / 36;
            let g = (index / 6) % 6;
            let b = index % 6;

            let scale = |x: u8| {
                if x == 0 {
                    0
                } else {
                    (x as u16 * 200 / 5 + 55) as u8
                }
            };

            [scale(r), scale(g), scale(b)]
        }
        232..=255 => {
            // Grayscale ramp (24 steps)
            let gray = ((num - 231) as u16 * 240 / 24).saturating_sub(2) as u8;
            [gray, gray, gray]
        } // No _ => branch needed because all u8 values are matched above.
    }
}

/// All values in the ANSI 8-bit range (0–255) are supported. This function
/// internally converts the ANSI code to RGB using [`ansi8_to_rgb`] and then
/// formats it as a hex string using [`rgb_to_hex`].
///
/// # Example
///
/// ```
/// assert_eq!(terminal_style::color::ansi8_to_hex(196), "#FF0000"); // Bright red
/// ```
pub fn ansi8_to_hex(num: u8) -> String {
    let rgb = ansi8_to_rgb(num);

    rgb_to_hex(rgb)
}
