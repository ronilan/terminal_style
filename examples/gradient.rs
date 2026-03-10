use terminal_style::format::background_rgb;

fn main() -> Result<(), terminal_style::color::ColorConversionError> {
    println!("=== 24-bit TrueColor Gradient Demo ===\n");

    let width = 60;
    let height = 10;

    for y in 0..height {
        for x in 0..width {
            // Calculate RGB values for a smooth horizontal gradient
            // From Blue (left) to Red (right)
            let r = (x as f32 / width as f32 * 255.0) as u8;
            let g = (y as f32 / height as f32 * 255.0) as u8;
            let b = 255 - r;

            // Using background_rgb for "pixels"
            print!("{}", background_rgb([r, g, b], " ")?);
        }
        println!();
    }

    println!("\nThis gradient uses 24-bit TrueColor sequences (\\x1b[48;2;R;G;Bm).");
    println!("If you see smooth transitions, your terminal supports TrueColor.");

    Ok(())
}
