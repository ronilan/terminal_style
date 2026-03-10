use super::{
    error::ColorConversionError,
    hex::hex_to_rgb,
    rgb::rgb_to_hex,
    validate::validate_hex,
};

/// Converts various types of color representations into an RGB color array.
///
/// This function uses the `IntoRgb` trait to accept multiple input types,
/// including RGB arrays, hex strings, and u8 ANSI values.
///
/// # Arguments
///
/// * `input` - Any type that implements [`IntoRgb`].
///
/// # Errors
///
/// Returns [`ColorConversionError`] if the input format is invalid.
///
/// # Examples
///
/// ```
/// use terminal_style::color::rgb_from_color_definition;
///
/// let rgb = rgb_from_color_definition("#FF0000").unwrap();
/// assert_eq!(rgb, [255, 0, 0]);
///
/// let from_ansi = rgb_from_color_definition(196u8).unwrap();
/// assert_eq!(from_ansi, [255, 0, 0]);
/// ```
pub fn rgb_from_color_definition<T>(input: T) -> Result<[u8; 3], ColorConversionError>
where
    T: IntoRgb,
{
    input.into_rgb()
}

/// A trait for converting different color formats into an RGB color array.
pub trait IntoRgb {
    /// Converts the type into an RGB color array `[u8; 3]`.
    fn into_rgb(self) -> Result<[u8; 3], ColorConversionError>;
}

/// Implements conversion from an RGB array to itself.
impl IntoRgb for [u8; 3] {
    fn into_rgb(self) -> Result<[u8; 3], ColorConversionError> {
        Ok(self)
    }
}

/// Implements conversion from a `String` hex color to an RGB array.
/// Validates the hex format before conversion.
impl IntoRgb for String {
    fn into_rgb(self) -> Result<[u8; 3], ColorConversionError> {
        validate_hex(&self)?;
        Ok(hex_to_rgb(&self))
    }
}

/// Implements conversion from a `&str` hex color to an RGB array.
/// Validates the hex format before conversion.
impl IntoRgb for &str {
    fn into_rgb(self) -> Result<[u8; 3], ColorConversionError> {
        validate_hex(self)?;
        Ok(hex_to_rgb(self))
    }
}

/// Implements conversion from an ANSI 8-bit value to an RGB array.
impl IntoRgb for u8 {
    fn into_rgb(self) -> Result<[u8; 3], ColorConversionError> {
        Ok(ansi8_to_rgb(self))
    }
}

/// Converts an ANSI 8-bit color value to an RGB array.
///
/// Supports standard ANSI color ranges:
/// - 0–15: System colors (8 standard + 8 high-intensity).
/// - 16–231: 6×6×6 color cube (216 colors).
/// - 232–255: Grayscale ramp (24 steps).
///
/// # Arguments
///
/// * `num` - An 8-bit ANSI color code (0-255).
///
/// # Returns
///
/// An RGB color array `[u8; 3]`.
///
/// # Examples
///
/// ```
/// use terminal_style::color::ansi8_to_rgb;
///
/// assert_eq!(ansi8_to_rgb(196), [255, 0, 0]); // Red
/// assert_eq!(ansi8_to_rgb(232), [8, 8, 8]);   // Near-black gray
/// ```
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

/// Converts an ANSI 8-bit color value to a hex color string.
///
/// Internally converts the ANSI code to RGB using [`ansi8_to_rgb`] and then
/// formats it as a hex string using [`rgb_to_hex`].
///
/// # Arguments
///
/// * `num` - An 8-bit ANSI color code (0-255).
///
/// # Returns
///
/// A hex color string in the format `#RRGGBB`.
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
