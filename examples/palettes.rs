use terminal_style::format::{background, color};

fn main() -> Result<(), terminal_style::color::ColorConversionError> {
    println!("=== Palettes Examples ===\n");

    for code in 0..=15 {
        print!("{}", background(code, " ")?);
    }
    println!();

    for row in 0..6 {
        for col in 0..36 {
            let code = row * 36 + col + 16;
            print!("{}", background(code as u8, " ")?);
        }
        println!();
    }
    println!();

    for code in 232..=255 {
        print!("{}", background(code, " ")?);
    }
    println!();
    println!();

    for code in 0..=15 {
        let ch = ((code - 16) % 94 + 33) as u8 as char; // printable ASCII range: 33–126
        print!("{}", color(code as u8, ch.to_string().as_str())?);
    }
    println!();

    for row in 0..6 {
        for col in 0..36 {
            let code = row * 36 + col + 16;

            let ch = ((code - 16) % 94 + 33) as u8 as char; // printable ASCII range: 33–126
            print!("{}", color(code as u8, ch.to_string().as_str())?);
        }
        println!();
    }
    println!();

    for code in 232..=255 {
        let ch = ((code - 16) % 94 + 33) as u8 as char; // printable ASCII range: 33–126
        print!("{}", color(code as u8, ch.to_string().as_str())?);
    }
    println!();

    Ok(())
}
