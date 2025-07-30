use terminal_style::format::{background, color};

fn main() -> Result<(), terminal_style::color::ColorConversionError> {
    println!("=== Color Examples ===\n");

    // Foreground colors using hex, rgb, and ansi
    println!("{}", color("#FF4500", "Hex input: OrangeRed")?); // Hex string
    println!("{}", color([0, 128, 0], "RGB input: Green")?); // RGB array
    println!("{}", color(21u8, "ANSI input: Blue-ish")?); // ANSI 8-bit code

    // Background colors using hex, rgb, and ansi
    println!("{}", background("#FFD700", "Hex BG: Gold background")?);
    println!("{}", background([75, 0, 130], "RGB BG: Indigo background")?);
    println!("{}", background(196u8, "ANSI BG: Bright Red background")?);

    Ok(())
}
