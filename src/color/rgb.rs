/// Converts an RGB color array into a hex color string.
///
/// # Arguments
///
/// * `arr` - An array `[u8; 3]` representing red, green, and blue components.
///
/// # Returns
///
/// A hex color string in the format `#RRGGBB`.
///
/// # Example
///
/// ```
/// assert_eq!(terminal_style::color::rgb_to_hex([255, 0, 170]), "#FF00AA");
/// ```
pub fn rgb_to_hex(arr: [u8; 3]) -> String {
    format!("#{:02X}{:02X}{:02X}", arr[0], arr[1], arr[2])
}

/// Converts an RGB color array into an ANSI 8-bit color code.
///
/// This function maps RGB values either to a grayscale range (232–255) or to
/// the 6×6×6 ANSI color cube (16–231) depending on whether all components are equal.
///
///
/// # Arguments
///
/// * `arr` - An array `[u8; 3]` representing red, green, and blue components.
///
/// # Returns
///
/// An `u8` ANSI 8-bit color code (0–255).
///
/// # Example
///
/// ```
/// assert_eq!(terminal_style::color::rgb_to_ansi8([255, 0, 0]), 196); // Bright red
/// assert_eq!(terminal_style::color::rgb_to_ansi8([128, 128, 128]), 244); // Mid gray
/// ```
pub fn rgb_to_ansi8(arr: [u8; 3]) -> u8 {
    let first = &arr[0];
    let is_gray = arr.iter().all(|item| item == first);

    if is_gray {
        // Grayscale calculation: map to range 232–255
        let gray = ((arr[0] as f64) / 240.0 * 24.0).round() as u8;

        // Clamp gray to valid ANSI grayscale range
        // Keep values close to black and white in the 216 palette
        if gray < 1 {
            return 16;
        }
        if gray > 24 {
            return 231;
        }
        return 231 + gray;
    }

    // For non-gray, map RGB to 6×6×6 color cube (values 16–231)
    // the real ANSI palette is not equally spaced
    // it goes 0, 95, 135, 175, 215, 255
    // cutoffs for rgb allocation are are 75, 115, 155, 195, 235
    // for each r, g and b, if under 75 0, else generate "cuts" of 40
    // multiply by 6 powered.
    arr.iter().enumerate().fold(16, |val, (index, &x)| {
        let contribution = if x < 75 {
            0 // Too dark, contributes nothing
        } else {
            // Scale the component to 1–5 range, then weight by position
            let scaled = ((x - 75) as f64 / 200.0 * 5.0).floor() as u8 + 1;
            scaled * 6_u8.pow(2 - index as u32) // 36 for R, 6 for G, 1 for B
        };
        val + contribution
    })
}
