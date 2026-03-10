use terminal_style::format::{background_ansi, background_rgb, color_ansi, color_rgb};

fn main() -> Result<(), terminal_style::color::ColorConversionError> {
    println!("=== Color Examples (TrueColor vs ANSI) ===\n");

    // TrueColor (24-bit) - Smooth gradients, 16 million colors
    println!(
        "{}",
        color_rgb([255, 20, 147], "TrueColor Foreground: Deep Pink")?
    );
    println!(
        "{}",
        background_rgb([138, 43, 226], "TrueColor Background: Blue Violet")?
    );

    println!();

    // 8-bit ANSI (256-color) - Quantized to nearest palette index
    println!(
        "{}",
        color_ansi([255, 20, 147], "8-bit ANSI Foreground: Deep Pink")?
    );
    println!(
        "{}",
        background_ansi([138, 43, 226], "8-bit ANSI Background: Blue Violet")?
    );

    println!("\n=== Mixed Inputs ===");
    // Functions accept Hex, RGB [u8; 3], or ANSI u8
    println!("{}", color_rgb("#00FF00", "Hex -> TrueColor")?);
    println!("{}", color_ansi("#00FF00", "Hex -> 8-bit ANSI")?);
    println!("{}", color_rgb(196u8, "ANSI u8 -> TrueColor (upconverted)")?);

    Ok(())
}
