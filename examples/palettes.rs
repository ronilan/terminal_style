use terminal_style::format::{background_ansi, color_ansi};

fn main() -> Result<(), terminal_style::color::ColorConversionError> {
    println!("=== 8-bit ANSI Palette (256 colors) ===\n");

    println!("Standard & High Intensity (0-15):");
    for code in 0..=15 {
        print!("{}", background_ansi(code, "  ")?);
    }
    println!("\n");

    println!("Color Cube (16-231):");
    for row in 0..6 {
        for col in 0..36 {
            let code = row * 36 + col + 16;
            print!("{}", background_ansi(code as u8, " ")?);
        }
        println!();
    }
    println!();

    println!("Grayscale Ramp (232-255):");
    for code in 232..=255 {
        print!("{}", background_ansi(code, "  ")?);
    }
    println!("\n");

    println!("Text Color Demo:");
    for code in 0..=15 {
        print!("{} ", color_ansi(code, "Text")?);
    }
    println!();

    Ok(())
}
